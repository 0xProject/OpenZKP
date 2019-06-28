#![allow(non_snake_case)] // TODO - Migrate to naming system which the rust complier doesn't complain
                          // about
use crate::{
    channel::*, fft::*, field::*, merkle::*, polynomial::*, u256::U256, utils::Reversible,
};
use itertools::Itertools;
use rayon::prelude::*;

pub struct TraceTable {
    pub ROWS:     usize,
    pub COLS:     usize,
    pub elements: Vec<FieldElement>,
}

impl TraceTable {
    pub fn new(ROWS: usize, COLS: usize, elements: Vec<FieldElement>) -> Self {
        Self {
            ROWS,
            COLS,
            elements,
        }
    }
}

/// Parameters for Stark proof generation
///
/// Contains various tuning parameters that determine how proofs are computed.
/// These can trade off between security, prover time, verifier time and
/// proof size.
///
/// **Note**: This does not including the constraint system or anything
/// about the claim to be proven.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofParams {
    /// The blowup factor
    ///
    /// The size of the low-degree-extension domain compared to the trace
    /// domain. Should be a power of two. Recommended values are 16, 32 or 64.
    pub blowup: usize,

    /// Proof of work difficulty
    ///
    /// The difficulty of the proof of work step in number of leading zero bits
    /// required.
    pub pow_bits: u8,

    /// Number of queries made to the oracles
    pub queries: usize,

    /// Number of FRI reductions between steps
    ///
    /// After the initial LDE polynomial is committed, several rounds of FRI
    /// degree reduction are done. Entries in the vector specify how many
    /// reductions are done between commitments.
    ///
    /// After `fri_layout.sum()` reductions are done, the remaining polynomial
    /// is written explicitly in coefficient form.
    pub fri_layout: Vec<usize>,
}

// This struct contains two evaluation systems which allow different
// functionality, first it contains a default function which directly evaluates
// the constraint function Second it contains a function designed to be used as
// the core of a loop on precomputed values to get the C function. If the proof
// system wants to used a looped eval for speedup it can set the loop bool to
// true, otherwise the system will preform all computation directly
#[allow(clippy::type_complexity)]
pub struct Constraint<'a> {
    pub NCONSTRAINTS: usize,
    pub eval: &'a Fn(
        &FieldElement,      // X point
        &[&[FieldElement]], // Polynomials
        usize,              // Claim Index
        FieldElement,       // Claim
        &[FieldElement],    // Constraint_coefficient
    ) -> FieldElement,
    pub eval_loop: Option<
        &'a Fn(
            &[&[FieldElement]], // Evaluated polynomials (LDEn)
            &[FieldElement],    // Constraint Coefficents
            usize,              // Claim index
            &FieldElement,      // Claim
        ) -> Vec<FieldElement>,
    >,
}

impl<'a> Constraint<'a> {
    #[allow(clippy::type_complexity)]
    pub fn new(
        NCONSTRAINTS: usize,
        eval: &'a Fn(
            &FieldElement,
            &[&[FieldElement]],
            usize,
            FieldElement,
            &[FieldElement],
        ) -> FieldElement,
        eval_loop: Option<
            &'a Fn(&[&[FieldElement]], &[FieldElement], usize, &FieldElement) -> Vec<FieldElement>,
        >,
    ) -> Self {
        Self {
            NCONSTRAINTS,
            eval,
            eval_loop,
        }
    }
}

// TODO - Split into smaller functions
#[allow(clippy::cognitive_complexity)]
pub fn stark_proof(
    trace: &TraceTable,
    constraints: &Constraint,
    claim_index: usize,
    claim_value: FieldElement,
    params: &ProofParams,
) -> Channel {
    let trace_len = trace.elements.len() / trace.COLS;
    let omega = FieldElement::root(U256::from((trace_len * params.blowup) as u64)).unwrap();
    let g = omega.pow(U256::from(params.blowup as u64));
    let eval_domain_size = trace_len * params.blowup;
    let gen = FieldElement::GENERATOR;

    let eval_x = geometric_series(&FieldElement::ONE, &omega, eval_domain_size);

    let mut TPn = vec![Vec::new(); trace.COLS];
    (0..trace.COLS)
        .into_par_iter()
        .map(|x| {
            let mut hold_col = Vec::with_capacity(trace_len);
            for i in (0..trace.elements.len()).step_by(trace.COLS) {
                hold_col.push(trace.elements[x + i].clone());
            }
            ifft(hold_col.as_slice())
        })
        .collect_into_vec(&mut TPn);

    let mut LDEn = vec![vec![FieldElement::ZERO; eval_x.len()]; trace.COLS];
    // OPT - Use some system to make this occur inline instead of storing then
    // processing
    #[allow(clippy::type_complexity)]
    let ret: Vec<(usize, Vec<(usize, Vec<FieldElement>)>)> = (0..params.blowup)
        .into_par_iter()
        .map(|j| {
            (
                j,
                (0..trace.COLS)
                    .into_par_iter()
                    .map(|x| {
                        (
                            x,
                            fft_cofactor(
                                TPn[x].as_slice(),
                                &(&gen * &omega.pow(U256::from(j as u64))),
                            ),
                        )
                    })
                    .collect(),
            )
        })
        .collect();

    for j_element in ret {
        for x_element in j_element.1 {
            for i in 0..trace_len {
                LDEn[x_element.0][(i * params.blowup + j_element.0) % eval_domain_size] =
                    x_element.1[i].clone();
            }
        }
    }

    let mut leaves = Vec::with_capacity(eval_domain_size);
    (0..eval_domain_size)
        .into_par_iter()
        .map(|i| leaf_list(i, &LDEn))
        .collect_into_vec(&mut leaves);

    let leaf_pointer: Vec<&[U256]> = leaves.iter().map(|x| x.as_slice()).collect();
    let tree = make_tree(leaf_pointer.as_slice());
    let mut public_input = [claim_index.to_be_bytes()].concat();
    public_input.extend_from_slice(&claim_value.0.to_bytes_be());
    let mut proof = Channel::new(public_input.as_slice());
    proof.write(&tree[1]);
    let mut constraint_coefficients = Vec::with_capacity(constraints.NCONSTRAINTS);
    for _i in 0..constraints.NCONSTRAINTS {
        constraint_coefficients.push(proof.read());
    }

    let mut CC;
    let mut x = gen.clone();
    let sliced_poly: Vec<&[FieldElement]> = TPn.iter().map(|x| x.as_slice()).collect();
    let sliced_eval: Vec<&[FieldElement]> = LDEn.iter().map(|x| x.as_slice()).collect();

    match constraints.eval_loop {
        Some(x) => {
            CC = (x)(
                sliced_eval.as_slice(),
                constraint_coefficients.as_slice(),
                claim_index,
                &claim_value,
            )
        }
        None => {
            CC = vec![FieldElement::ZERO; eval_domain_size];
            for constraint_element in CC.iter_mut() {
                *constraint_element = (constraints.eval)(
                    // This will perform the polynomial evaluation on each step
                    &x,
                    sliced_poly.as_slice(),
                    claim_index,
                    claim_value.clone(),
                    constraint_coefficients.as_slice(),
                );

                x *= &omega;
            }
        }
    }
    let mut c_leaves = Vec::with_capacity(eval_domain_size);
    (0..eval_domain_size)
        .into_par_iter()
        .map(|i| leaf_single(i, &CC))
        .collect_into_vec(&mut c_leaves);

    let c_tree = make_tree(c_leaves.as_slice());
    proof.write(&c_tree[1]);
    let oods_point: FieldElement = proof.read();
    let oods_point_g = &oods_point * &g;
    let mut oods_values = Vec::with_capacity(2 * trace.COLS + 1);
    for item in TPn.iter() {
        let mut evaled = eval_poly(oods_point.clone(), item.as_slice());
        oods_values.push(evaled.clone());
        evaled = eval_poly(oods_point_g.clone(), item.as_slice());
        oods_values.push(evaled.clone());
    }

    oods_values.push((constraints.eval)(
        &oods_point,
        sliced_poly.as_slice(),
        claim_index,
        claim_value.clone(),
        constraint_coefficients.as_slice(),
    )); // Gets eval_C of the oods point via direct computation

    for v in oods_values.iter() {
        proof.write(v);
    }

    let mut oods_coefficients = Vec::with_capacity(2 * trace.COLS + 1);
    for _i in 0..=2 * trace.COLS {
        oods_coefficients.push(proof.read());
    }

    let mut CO = Vec::with_capacity(eval_domain_size);
    let x = FieldElement::GENERATOR;
    let mut x_omega_cycle = Vec::with_capacity(eval_domain_size);
    let mut x_oods_cycle: Vec<FieldElement> = Vec::with_capacity(eval_domain_size);
    let mut x_oods_cycle_g: Vec<FieldElement> = Vec::with_capacity(eval_domain_size);

    eval_x
        .par_iter()
        .map(|i| i * &x)
        .collect_into_vec(&mut x_omega_cycle);
    x_omega_cycle
        .par_iter()
        .map(|i| (i - &oods_point, i - &oods_point * &g))
        .unzip_into_vecs(&mut x_oods_cycle, &mut x_oods_cycle_g);

    let pool = vec![&x_oods_cycle, &x_oods_cycle_g];

    let mut held = Vec::with_capacity(3);
    pool.par_iter()
        .map(|i| invert_batch(i))
        .collect_into_vec(&mut held);

    x_oods_cycle_g = held.pop().unwrap();
    x_oods_cycle = held.pop().unwrap();

    (0..eval_domain_size)
        .into_par_iter()
        .map(|i| {
            let A = &x_oods_cycle[i];
            let B = &x_oods_cycle_g[i];
            let mut r = FieldElement::ZERO;

            for x in 0..trace.COLS {
                r += &oods_coefficients[2 * x] * (&LDEn[x][i] - &oods_values[2 * x]) * A;
                r += &oods_coefficients[2 * x + 1] * (&LDEn[x][i] - &oods_values[2 * x + 1]) * B;
            }
            r += &oods_coefficients[oods_coefficients.len() - 1]
                * (&CC[i] - &oods_values[oods_values.len() - 1])
                * A;

            r
        })
        .collect_into_vec(&mut CO);
    // Fri Layers
    debug_assert!(eval_domain_size.is_power_of_two());
    let mut fri = Vec::with_capacity(64 - (eval_domain_size.leading_zeros() as usize));
    fri.push(CO);
    let mut fri_trees: Vec<Vec<[u8; 32]>> = Vec::with_capacity(params.fri_layout.len());
    let held_tree = fri_tree(&(fri[fri.len() - 1].as_slice()), params.blowup / 2);
    proof.write(&held_tree[1]);
    fri_trees.push(held_tree);

    let mut halvings = 0;
    let mut fri_const = params.blowup / 4;
    for &x in params.fri_layout.iter().dropping_back(1) {
        let mut eval_point = if x == 0 {
            FieldElement::ONE
        } else {
            proof.read()
        };
        for _ in 0..x {
            fri.push(fri_layer(
                &fri[fri.len() - 1].as_slice(),
                &eval_point,
                eval_domain_size,
                eval_x.as_slice(),
            ));
            eval_point = eval_point.square();
        }
        let held_tree = fri_tree(&(fri[fri.len() - 1].as_slice()), fri_const);

        proof.write(&held_tree[1]);
        fri_trees.push(held_tree);
        fri_const /= 2;
        halvings += x;
    }

    // Gets the coefficient representation of the last number of fri reductions
    let mut eval_point = proof.read();
    for _ in 0..params.fri_layout[params.fri_layout.len() - 1] {
        fri.push(fri_layer(
            &fri[fri.len() - 1].as_slice(),
            &eval_point,
            eval_domain_size,
            eval_x.as_slice(),
        ));
        eval_point = eval_point.square();
    }
    halvings += params.fri_layout[params.fri_layout.len() - 1];

    // Gets the coefficient representation of the last number of fri reductions

    let last_layer_degree_bound = trace_len / (2_usize.pow(halvings as u32));
    let mut last_layer_coefficient = ifft(&(fri[halvings].as_slice()));
    last_layer_coefficient.truncate(last_layer_degree_bound);
    proof.write(last_layer_coefficient.as_slice());
    debug_assert_eq!(last_layer_coefficient.len(), last_layer_degree_bound);

    let proof_of_work = proof.pow_find_nonce(params.pow_bits);
    debug_assert!(&proof.pow_verify(proof_of_work, params.pow_bits));
    proof.write(proof_of_work);

    let num_queries = params.queries;
    let query_indices = get_indices(
        num_queries,
        64 - eval_domain_size.leading_zeros() - 1,
        &mut proof,
    );

    for index in query_indices.iter() {
        for low_degree_extension in LDEn.iter() {
            proof.write(
                &low_degree_extension[(index.clone()).bit_reverse()
                    >> ((low_degree_extension.len().leading_zeros()) + 1)],
            );
        }
    }

    let decommitment = crate::merkle::proof(&tree, &(query_indices.as_slice()));
    for x in decommitment.iter() {
        proof.write(x);
    }

    for index in query_indices.iter() {
        proof.write(&CC[index.clone().bit_reverse() >> ((CC.len().leading_zeros()) + 1) as usize]);
    }
    let decommitment = crate::merkle::proof(&c_tree, &(query_indices.as_slice()));
    for x in decommitment.iter() {
        proof.write(x);
    }
    let mut fri_indices: Vec<usize> = query_indices
        .iter()
        .map(|x| x / (params.blowup / 2))
        .collect();

    let mut current_fri = 0;
    let mut previous_indices = query_indices.clone();
    for (k, next_tree) in fri_trees.iter().enumerate() {
        if k != 0 {
            current_fri += params.fri_layout[k - 1];
        }

        for i in fri_indices.iter() {
            for j in 0..(params.blowup / 2_usize.pow(k as u32 + 1)) {
                let n = i * (params.blowup / 2_usize.pow(k as u32 + 1)) + j;

                if previous_indices.binary_search(&n).is_ok() {
                    continue;
                } else {
                    proof.write(
                        &fri[current_fri][((n as u64).bit_reverse()
                            >> (u64::from(fri[current_fri].len().leading_zeros() + 1)))
                            as usize],
                    );
                }
            }
        }
        let decommitment = crate::merkle::proof(&next_tree, &(fri_indices.as_slice()));
        for proof_element in decommitment.iter() {
            proof.write(proof_element);
        }
        previous_indices = fri_indices.clone();
        fri_indices = fri_indices.iter().map(|ind| ind / 4).collect();
    }

    proof
}

fn leaf_list(i: usize, LDEn: &[Vec<FieldElement>]) -> Vec<U256> {
    let mut ret = Vec::with_capacity(LDEn.len());
    for item in LDEn.iter() {
        ret.push(
            item[i.bit_reverse() >> (item.len().leading_zeros() + 1)]
                .0
                .clone(),
        )
    }
    ret
}

fn leaf_single(i: usize, CC: &[FieldElement]) -> U256 {
    CC[i.bit_reverse() >> (CC.len().leading_zeros() + 1)]
        .0
        .clone()
}

// TODO Better variable names
#[allow(clippy::many_single_char_names)]
fn fri_layer(
    previous: &[FieldElement],
    evaluation_point: &FieldElement,
    eval_domain_size: usize,
    eval_x: &[FieldElement],
) -> Vec<FieldElement> {
    let len = previous.len();
    let step = eval_domain_size / len;
    let mut next = vec![FieldElement::ZERO; len / 2];
    // OPT - Use parallel extend or chunked alg
    (0..(len / 2))
        .into_par_iter()
        .map(|index| {
            let negative_index = (len / 2 + index) % len;
            let inverse_index = ((len - index) % len) * step;
            // OPT: Check if computed x_inv is faster
            let x_inv = &eval_x[inverse_index];
            let value = &previous[index];
            let neg_x_value = &previous[negative_index];
            debug_assert_eq!(x_inv, &eval_x[index * step].inv().unwrap());
            debug_assert_eq!(eval_x[negative_index * step], -&eval_x[index * step]);
            (value + neg_x_value) + evaluation_point * x_inv * (value - neg_x_value)
        })
        .collect_into_vec(&mut next);
    next
}

fn fri_tree(layer: &[FieldElement], coset_size: usize) -> Vec<[u8; 32]> {
    let n = layer.len();
    let bits = 64 - (n as u64).leading_zeros(); // Floored base 2 log
    let mut internal_leaves = Vec::new();
    for i in (0..n).step_by(coset_size) {
        let mut internal_leaf = Vec::with_capacity(coset_size);
        for j in 0..coset_size {
            internal_leaf.push(layer[(i + j).bit_reverse() >> (64 - bits + 1)].0.clone());
        }
        internal_leaves.push(internal_leaf);
    }
    let leaf_pointer: Vec<&[U256]> = internal_leaves.iter().map(|x| x.as_slice()).collect();
    make_tree(leaf_pointer.as_slice())
}

fn get_indices(num: usize, bits: u32, proof: &mut Channel) -> Vec<usize> {
    let mut query_indices = Vec::with_capacity(num + 3);
    while query_indices.len() < num {
        let val: U256 = proof.read();
        query_indices.push(((val.clone() >> (0x100 - 0x040)).c0 & (2_u64.pow(bits) - 1)) as usize);
        query_indices.push(((val.clone() >> (0x100 - 0x080)).c0 & (2_u64.pow(bits) - 1)) as usize);
        query_indices.push(((val.clone() >> (0x100 - 0x0C0)).c0 & (2_u64.pow(bits) - 1)) as usize);
        query_indices.push((val.c0 & (2_u64.pow(bits) - 1)) as usize);
    }
    query_indices.truncate(num);
    (&mut query_indices).sort_unstable();
    query_indices
}

pub fn geometric_series(base: &FieldElement, step: &FieldElement, len: usize) -> Vec<FieldElement> {
    const PARALLELIZATION: usize = 16_usize;
    // OPT - Set based on the cores available and how well the work is spread
    let step_len = len / PARALLELIZATION;
    let mut range = vec![FieldElement::ZERO; len];
    range
        .par_chunks_mut(step_len)
        .enumerate()
        .for_each(|(i, slice)| {
            let mut hold = base * step.pow(U256::from((i * step_len) as u64));
            for element in slice.iter_mut() {
                *element = hold.clone();
                hold *= step;
            }
        });
    range
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{fibonacci::*, u256::U256, u256h};
    use hex_literal::*;

    #[test]
    fn fib_test_1024_python_witness() {
        let claim_index = 1000;
        let claim_fib = FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        ));
        let witness = FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000000000000cafebabe"
        ));
        let expected = hex!("fcf1924f84656e5068ab9cbd44ae084b235bb990eefc0fd0183c77d5645e830e");
        let actual = stark_proof(
            &get_trace_table(1024, witness),
            &get_constraint(),
            claim_index,
            claim_fib,
            &ProofParams {
                blowup:     16,
                pow_bits:   12,
                queries:    20,
                fri_layout: vec![3, 2],
            },
        );
        assert_eq!(actual.digest, expected);
    }

    #[test]
    fn fib_test_1024_changed_witness() {
        let claim_index = 1000;
        let claim_fib = FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        ));
        let witness = FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000f00dbabe0cafebabe"
        ));
        let expected = hex!("5c8e2f6353526e422744a8c11a7a94db1829cb2bfac78bae774b5576c88279c9");
        let actual = stark_proof(
            &get_trace_table(1024, witness),
            &get_constraint(),
            claim_index,
            claim_fib,
            &ProofParams {
                blowup:     32,
                pow_bits:   12,
                queries:    20,
                fri_layout: vec![3, 2],
            },
        );
        assert_eq!(actual.digest, expected);
    }

    #[test]
    fn fib_test_4096() {
        let claim_index = 4000;
        let claim_fib = FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        ));
        let witness = FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000f00dbabe0cafebabe"
        ));
        let expected = hex!("427499a0cd50a90fe7fdf2f039f6dffd71fcc930392151d2eb0ea611c3f312b5");
        let actual = stark_proof(
            &get_trace_table(4096, witness),
            &get_constraint(),
            claim_index,
            claim_fib,
            &ProofParams {
                blowup:     16,
                pow_bits:   12,
                queries:    20,
                fri_layout: vec![3, 2, 1],
            },
        );
        assert_eq!(actual.digest, expected);
    }

    #[test]
    fn geometric_series_test() {
        let base = FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        ));
        let step = FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000f00dbabe0cafebabe"
        ));

        let domain = geometric_series(&base, &step, 32);
        let mut hold = base.clone();
        for item in domain {
            assert_eq!(item, hold);
            hold *= &step;
        }
    }
}
