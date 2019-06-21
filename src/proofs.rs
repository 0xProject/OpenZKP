#![allow(non_snake_case)] // TODO - Migrate to naming system which the rust complier doesn't complain
                          // about
use crate::{
    channel::*, fft::*, field::*, merkle::*, polynomial::*, proof_of_work::*, u256::U256,
    utils::Reversible,
};
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
        u64,                // Claim Index
        FieldElement,       // Claim
        &[FieldElement],    // Constraint_coefficient
    ) -> FieldElement,
    pub eval_loop: Option<
        &'a Fn(
            &[&[FieldElement]], // Evaluated polynomials (LDEn)
            &[FieldElement],    // Constraint Coefficents
            u64,                // Claim index
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
            u64,
            FieldElement,
            &[FieldElement],
        ) -> FieldElement,
        eval_loop: Option<
            &'a Fn(&[&[FieldElement]], &[FieldElement], u64, &FieldElement) -> Vec<FieldElement>,
        >,
    ) -> Self {
        Self {
            NCONSTRAINTS,
            eval,
            eval_loop,
        }
    }
}

#[allow(clippy::cognitive_complexity)]
pub fn stark_proof(
    trace: &TraceTable,
    constraints: &Constraint,
    claim_index: u64,
    claim_value: FieldElement,
    beta: u64,
) -> Channel {
    let trace_len = (trace.elements.len() / trace.COLS) as u64;
    let omega = FieldElement::root(U256::from(trace_len * beta)).unwrap();
    let g = omega.pow(U256::from(beta));
    let eval_domain_size = trace_len * beta;
    let gen = FieldElement::GENERATOR;

    let eval_x = geometric_series(&FieldElement::ONE, &omega, eval_domain_size as usize);

    let mut TPn = vec![Vec::new(); trace.COLS];
    (0..trace.COLS)
        .into_par_iter()
        .map(|x| {
            let mut hold_col = Vec::with_capacity(trace_len as usize);
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
    let ret: Vec<(usize, Vec<(usize, Vec<FieldElement>)>)> = (0..(beta as usize))
        .into_par_iter()
        .map(|j| {
            (
                j,
                (0..(trace.COLS as usize))
                    .into_par_iter()
                    .map(|x| {
                        (
                            x,
                            fft_cofactor(
                                TPn[x].as_slice(),
                                &gen * (&omega.pow(U256::from(j as u64))),
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
                LDEn[x_element.0]
                    [((i * beta + (j_element.0 as u64)) % (eval_domain_size)) as usize] =
                    x_element.1[i as usize].clone();
            }
        }
    }

    let mut leaves = Vec::with_capacity(eval_domain_size as usize);
    (0..(eval_domain_size as usize))
        .into_par_iter()
        .map(|i| leaf_list(i as u64, &LDEn))
        .collect_into_vec(&mut leaves);

    let leaf_pointer: Vec<&[U256]> = leaves.iter().map(|x| x.as_slice()).collect();
    let tree = make_tree(leaf_pointer.as_slice());
    let mut public_input = [claim_index.to_be_bytes()].concat();
    public_input.extend_from_slice(&claim_value.0.to_bytes_be());
    let mut proof = Channel::new(public_input.as_slice());
    proof.write(&tree[1]);
    let mut constraint_coefficients = Vec::with_capacity(constraints.NCONSTRAINTS);
    for _i in 0..constraints.NCONSTRAINTS {
        constraint_coefficients.push(proof.element());
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
            CC = vec![FieldElement::ZERO; eval_domain_size as usize];
            for i in 0..eval_domain_size {
                CC[i as usize] = (constraints.eval)(
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
    let mut c_leaves = Vec::with_capacity(eval_domain_size as usize);
    (0..(eval_domain_size as usize))
        .into_par_iter()
        .map(|i| leaf_single(i as u64, &CC))
        .collect_into_vec(&mut c_leaves);

    let c_tree = make_tree(c_leaves.as_slice());
    proof.write(&c_tree[1]);
    let oods_point = proof.element();
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
        proof.write_element(v);
    }

    let mut oods_coefficients = Vec::with_capacity(2 * trace.COLS + 1);
    for _i in 0..=2 * trace.COLS {
        oods_coefficients.push(proof.element());
    }

    let mut CO = Vec::with_capacity(eval_domain_size as usize);
    let x = FieldElement::GENERATOR;
    let mut x_omega_cycle = Vec::with_capacity(eval_domain_size as usize);
    let mut x_oods_cycle: Vec<FieldElement> = Vec::with_capacity(eval_domain_size as usize);
    let mut x_oods_cycle_g: Vec<FieldElement> = Vec::with_capacity(eval_domain_size as usize);

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

    (0..(eval_domain_size as usize))
        .into_par_iter()
        .map(|i| {
            let A = &x_oods_cycle[i as usize];
            let B = &x_oods_cycle_g[i as usize];
            let mut r = FieldElement::ZERO;

            for x in 0..trace.COLS {
                r += &oods_coefficients[2 * x] * (&LDEn[x][i as usize] - &oods_values[2 * x]) * A;
                r += &oods_coefficients[2 * x + 1]
                    * (&LDEn[x][i as usize] - &oods_values[2 * x + 1])
                    * B;
            }
            r += &oods_coefficients[oods_coefficients.len() - 1]
                * (&CC[i as usize] - &oods_values[oods_values.len() - 1])
                * A;

            r
        })
        .collect_into_vec(&mut CO);
    // Fri Layers
    debug_assert!(eval_domain_size.is_power_of_two());
    let mut fri = Vec::with_capacity(64 - (eval_domain_size.leading_zeros() as usize));
    fri.push(CO);
    let fri_tree_1 = fri_tree(&(fri[0].as_slice()), 8);
    proof.write(&fri_tree_1[1]);

    let mut eval_point = proof.element();
    fri.push(fri_layer(
        &fri[0].as_slice(),
        &eval_point,
        eval_domain_size,
        eval_x.as_slice(),
    ));
    fri.push(fri_layer(
        &fri[1].as_slice(),
        &(eval_point.square()),
        eval_domain_size,
        eval_x.as_slice(),
    ));
    fri.push(fri_layer(
        fri[2].as_slice(),
        &(eval_point.square().square()),
        eval_domain_size,
        eval_x.as_slice(),
    ));
    let fri_tree_2 = fri_tree(&(fri[3].as_slice()), 4);

    proof.write(&fri_tree_2[1]);

    eval_point = proof.element();
    fri.push(fri_layer(
        &(fri[3].as_slice()),
        &eval_point,
        eval_domain_size,
        eval_x.as_slice(),
    ));
    fri.push(fri_layer(
        &(fri[4].as_slice()),
        &(eval_point.square()),
        eval_domain_size,
        eval_x.as_slice(),
    ));
    // Five fri layers have reduced the size of the evaluation domain and polynomial
    // by 32x
    let last_layer_degree_bound = trace_len / 32;
    let mut last_layer_coefficient = ifft(&(fri[5].as_slice()));
    last_layer_coefficient.truncate(last_layer_degree_bound as usize);
    proof.write_element_list(last_layer_coefficient.as_slice());
    debug_assert_eq!(last_layer_coefficient.len() as u64, last_layer_degree_bound);
    // Security parameter proof of work is at 12 bits
    let proof_of_work = pow_find_nonce(12, &proof);
    debug_assert!(pow_verify(proof_of_work, 12, &proof));
    proof.write(&proof_of_work.to_be_bytes());

    let num_queries = 20;
    let query_indices = get_indices(
        num_queries,
        (64 - eval_domain_size.leading_zeros() - 1) as u32,
        &mut proof,
    );

    for index in query_indices.iter() {
        for x in 0..trace.COLS {
            proof.write_element(
                &LDEn[x as usize][(index.clone()).bit_reverse()
                    >> ((LDEn[x].len().leading_zeros()) + 1) as usize],
            );
        }
    }

    let mut decommitment = crate::merkle::proof(&tree, &(query_indices.as_slice()));
    for x in decommitment.iter() {
        proof.write(x);
    }

    for index in query_indices.iter() {
        proof.write_element(
            &CC[index.clone().bit_reverse() >> ((CC.len().leading_zeros()) + 1) as usize],
        );
    }
    decommitment = crate::merkle::proof(&c_tree, &(query_indices.as_slice()));
    for x in decommitment.iter() {
        proof.write(x);
    }

    let fri_indices: Vec<usize> = query_indices
        .iter()
        .map(|x| x / ((beta / 2) as usize))
        .collect();
    for i in fri_indices.iter() {
        for j in 0..((beta / 2) as usize) {
            let n = i * ((beta / 2) as usize) + j;
            if query_indices.binary_search(&n).is_ok() {
                continue;
            } else {
                proof.write_element(&fri[0][((n as u64).bit_reverse() >> 50) as usize]);
            }
        }
    }
    decommitment = crate::merkle::proof(&fri_tree_1, &(fri_indices.as_slice()));
    for x in decommitment.iter() {
        proof.write(x);
    }

    let fri_low_indices: Vec<usize> = query_indices.iter().map(|x| x / 32).collect();
    for i in fri_low_indices.iter() {
        for j in 0..4 {
            let n = i * 4 + j;
            if fri_indices.binary_search(&n).is_ok() {
                continue;
            } else {
                proof.write_element(&fri[3][((n as u64).bit_reverse() >> 53) as usize]);
            }
        }
    }
    decommitment = crate::merkle::proof(&fri_tree_2, &(fri_low_indices.as_slice()));
    for x in decommitment.iter() {
        proof.write(x);
    }

    proof
}

fn leaf_list(i: u64, LDEn: &[Vec<FieldElement>]) -> Vec<U256> {
    let mut ret = Vec::with_capacity(LDEn.len());
    for item in LDEn.iter() {
        ret.push(
            item[(i.bit_reverse() >> (item.len().leading_zeros() + 1)) as usize]
                .0
                .clone(),
        )
    }
    ret
}

fn leaf_single(i: u64, CC: &[FieldElement]) -> U256 {
    CC[(i.bit_reverse() >> (CC.len().leading_zeros() + 1)) as usize]
        .0
        .clone()
}

// TODO Better variable names
#[allow(clippy::many_single_char_names)]
fn fri_layer(
    previous: &[FieldElement],
    evaluation_point: &FieldElement,
    eval_domain_size: u64,
    eval_x: &[FieldElement],
) -> Vec<FieldElement> {
    let n = previous.len() as u64;
    let s = eval_domain_size / n;
    let mut next = vec![FieldElement::ZERO; (n / 2) as usize];
    (0..(n as usize) / 2)
        .into_par_iter()
        .map(|i| {
            let j = (n / 2 + (i as u64)) % n;
            let m = eval_x.len() as u64;
            let ind = ((m - (i as u64)) * s) % m;
            let x_inv = &eval_x[ind as usize];
            let a = &previous[i as usize];
            let b = &previous[j as usize];
            (a + b) + evaluation_point * x_inv * (a - b)
        })
        .collect_into_vec(&mut next);
    next
}

fn fri_tree(layer: &[FieldElement], coset_size: u64) -> Vec<[u8; 32]> {
    let n = layer.len();
    let bits = 64 - (n as u64).leading_zeros(); // Floored base 2 log
    let mut internal_leaves = Vec::new();
    for i in (0..n).step_by(coset_size as usize) {
        let mut internal_leaf = Vec::with_capacity(coset_size as usize);
        for j in 0..coset_size {
            internal_leaf.push(
                layer[(((i as u64) + j).bit_reverse() >> (64 - bits + 1)) as usize]
                    .0
                    .clone(),
            );
        }
        internal_leaves.push(internal_leaf);
    }
    let leaf_pointer: Vec<&[U256]> = internal_leaves.iter().map(|x| x.as_slice()).collect();
    make_tree(leaf_pointer.as_slice())
}

fn get_indices(num: usize, bits: u32, proof: &mut Channel) -> Vec<usize> {
    let mut query_indices = Vec::with_capacity(num + 3);
    while query_indices.len() < num {
        let val = U256::from_bytes_be(&proof.bytes());
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
        let claim_index = 1000_u64;
        let claim_fib = FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        ));
        let witness = FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000000000000cafebabe"
        ));
        let expected = hex!("185ab1df82b4464206cae58761c4ab6873322fd0f77ee9e4e8e479e7a1f18707");
        let actual = stark_proof(
            &get_trace_table(1024, witness),
            &get_constraint(),
            claim_index,
            claim_fib,
            2_u64.pow(4),
        );
        assert_eq!(actual.digest, expected);
    }

    #[test]
    fn fib_test_1024_changed_witness() {
        let claim_index = 1000_u64;
        let claim_fib = FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        ));
        let witness = FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000f00dbabe0cafebabe"
        ));
        let expected = hex!("d5a91d6ba26d105e60adc24206d87c8ee05b11a8674a064c90cba96470573955");
        let actual = stark_proof(
            &get_trace_table(1024, witness),
            &get_constraint(),
            claim_index,
            claim_fib,
            2_u64.pow(5),
        );
        assert_eq!(actual.digest, expected);
    }

    #[test]
    fn fib_test_4096() {
        let claim_index = 4000_u64;
        let claim_fib = FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        ));
        let witness = FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000f00dbabe0cafebabe"
        ));
        let expected = hex!("006493c24997323b161b5ca20fbe0b2d0826fa8fd56e6eaf47f872ed29bfc5cb");
        let actual = stark_proof(
            &get_trace_table(4096, witness),
            &get_constraint(),
            claim_index,
            claim_fib,
            2_u64.pow(4),
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
