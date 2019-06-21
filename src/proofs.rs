#![allow(non_snake_case)] // TODO - Migrate to naming system which the rust complier doesn't complain about
use crate::channel::*;
use crate::fft::*;
use crate::field::*;
use crate::merkle::*;
use crate::polynomial::*;
use crate::proof_of_work::*;
use crate::u256::U256;
use crate::utils::Reversible;
use rayon::prelude::*;

pub struct TraceTable {
    pub ROWS: usize,
    pub COLS: usize,
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

pub struct ProofParams {
    pub beta: u64,
    pub pow_bits: u64,
    pub queries: usize,
    pub fri_layout: Vec<usize>,
}

impl ProofParams {
    pub fn new(beta: u64, pow_bits: u64, queries: usize, fri_layout: Vec<usize>) -> Self {
        Self {
            beta,
            pow_bits,
            queries,
            fri_layout,
        }
    }
}

// This struct contains two evaluation systems which allow different functionality, first it contains a default function which directly evaluates the constraint function
// Second it contains a function designed to be used as the core of a loop on precomputed values to get the C function.
// If the proof system wants to used a looped eval for speedup it can set the loop bool to true, otherwise the system will preform all computation directly
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

#[allow(clippy::cognitive_complexity)] // TODO - Split into smaller functions
pub fn stark_proof(
    trace: &TraceTable,
    constraints: &Constraint,
    claim_index: u64,
    claim_value: FieldElement,
    params: ProofParams,
) -> Channel {
    let beta = params.beta;
    let trace_len = (trace.elements.len() / trace.COLS) as u64;
    let omega = FieldElement::root(U256::from(trace_len * beta)).unwrap();
    let g = omega.pow(U256::from(beta)).unwrap();
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
            ifft(g.clone(), hold_col.as_slice())
        })
        .collect_into_vec(&mut TPn);

    let mut LDEn = vec![vec![FieldElement::ZERO; eval_x.len()]; trace.COLS];
    // OPT - Use some system to make this occur inline instead of storing then processing
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
                                g.clone(),
                                TPn[x].as_slice(),
                                &gen * (&omega.pow(U256::from(j as u64)).unwrap()),
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
    let mut halvings = 0;
    let mut fri_const = params.beta/2;
    let mut fri_trees : Vec<Vec<[u8; 32]>> = Vec::with_capacity(params.fri_layout.len());
    let mut eval_point = FieldElement::ONE;
    for x in params.fri_layout.as_slice()[..(params.fri_layout.len() - 1)].iter() {
        if *x != 0 {
            eval_point = proof.element();
        }
        for _ in 0..*x {
            fri.push(fri_layer(
                &fri[fri.len()-1].as_slice(),
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
        halvings += *x;
    }

    // Gets the coefficient representation of the last number of fri reductions
    let mut eval_point = proof.element();
    for _ in 0..params.fri_layout[params.fri_layout.len() - 1] {
        fri.push(fri_layer(
            &fri[fri.len() -1].as_slice(),
            &eval_point,
            eval_domain_size,
            eval_x.as_slice(),
        ));
        eval_point = eval_point.square();
    }
    halvings += params.fri_layout[params.fri_layout.len() - 1];

    let last_layer_degree_bound = trace_len / (2_u64.pow(halvings as u32));
    let mut last_layer_coefficient = ifft(
        FieldElement::root(U256::from(fri[halvings].len() as u64)).unwrap(),
        &(fri[halvings].as_slice()),
    );
    last_layer_coefficient.truncate(last_layer_degree_bound as usize);
    proof.write_element_list(last_layer_coefficient.as_slice());
    debug_assert_eq!(last_layer_coefficient.len() as u64, last_layer_degree_bound);

    let proof_of_work = pow_find_nonce(params.pow_bits, &proof);
    debug_assert!(pow_verify(proof_of_work, 12, &proof));
    proof.write(&proof_of_work.to_be_bytes());

    let num_queries = params.queries;
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
    let mut fri_indices: Vec<usize> = query_indices
        .iter()
        .map(|x| x / ((beta / 2) as usize))
        .collect();
    
    let mut current_fri = 0;
    let mut previous_indicies = query_indices.clone();
    for (k, x) in params.fri_layout.as_slice()[..(params.fri_layout.len() -1)].iter().enumerate() {
        current_fri += *x;
        for i in fri_indices.iter() {
            for j in 0..((beta / 2_u64.pow(k as u32 + 1)) as usize) {
                let n = i * ((beta / 2_u64.pow(k as u32 + 1)) as usize) + j;

                if previous_indicies.binary_search(&n).is_ok() {
                    continue;
                } else {
                    proof.write_element(&fri[current_fri][((n as u64).bit_reverse() >> (u64::from(fri[current_fri].len().leading_zeros() +1))) as usize]);
                }
            }
        }
        decommitment = crate::merkle::proof(&fri_trees[k], &(fri_indices.as_slice()));
        for x in decommitment.iter() {
            proof.write(x);
        }
        previous_indicies = fri_indices.clone();
        fri_indices = fri_indices.iter().map(|ind| ind / 4).collect();
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

fn fri_layer(
    previous: &[FieldElement],
    evaluation_point: &FieldElement,
    eval_domain_size: u64,
    eval_x: &[FieldElement],
) -> Vec<FieldElement> {
    let len = previous.len() as u64;
    let s = eval_domain_size / len;
    let mut next = vec![FieldElement::ZERO; (len / 2) as usize];
    (0..(len as usize) / 2)
        .into_par_iter()
        .map(|index| {
            let permuted_index = (len / 2 + (index as u64)) % len;
            let m = eval_x.len() as u64;
            let ind = ((m - (index as u64)) * s) % m;
            let x_inv = &eval_x[ind as usize];
            let value = &previous[index as usize];
            let permuted_value = &previous[permuted_index as usize];
            (value + permuted_value) + evaluation_point * x_inv * (value - permuted_value)
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
    (0..PARALLELIZATION)
        .into_par_iter()
        .map(|i| {
            let mut hold = Vec::with_capacity(step_len);
            // OPT - Avoid temporary vectors
            hold.push(base * step.pow(U256::from((i * step_len) as u64)).unwrap());
            for j in 1..(step_len) {
                hold.push(&hold[j - 1] * step);
            }
            hold
        })
        .flatten()
        .collect()
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::channel::*;
    use crate::fibonacci::*;
    use crate::field::*;
    use crate::u256::U256;
    use crate::u256h;
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
            ProofParams::new(2_u64.pow(4), 12, 20, vec![0, 3, 2]),
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
        let expected = hex!("5c8e2f6353526e422744a8c11a7a94db1829cb2bfac78bae774b5576c88279c9");
        let actual = stark_proof(
            &get_trace_table(1024, witness),
            &get_constraint(),
            claim_index,
            claim_fib,
            ProofParams::new(2_u64.pow(5), 12, 20, vec![0, 3, 2]),
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
        let expected = hex!("427499a0cd50a90fe7fdf2f039f6dffd71fcc930392151d2eb0ea611c3f312b5");
        let actual = stark_proof(
            &get_trace_table(4096, witness),
            &get_constraint(),
            claim_index,
            claim_fib,
            ProofParams::new(2_u64.pow(4), 12, 20, vec![0, 3, 2, 1]),
        );
        assert_eq!(actual.digest, expected);
    }
}
