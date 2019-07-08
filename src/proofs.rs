#![allow(non_snake_case)] // TODO - Migrate to naming system which the rust complier doesn't complain
                          // about
use crate::{
    channel::*, fft::*, field::*, merkle::*, polynomial::*, u256::U256, utils::Reversible,
};
use itertools::Itertools;
use rayon::prelude::*;

// This trait is for objects where the object is grouped into hashable sets
// based on index before getting made into a merkle tree, with domain size
// being the max index [ie the one which if you iterate up to it splits the
// whole range]
pub trait Groupable<T: Hashable> {
    fn make_group(&self, index: usize) -> T;
    fn domain_size(&self) -> usize;
}

// This trait is applied to give groupable objects a merkle tree based on their
// groupings
pub trait Merkleizable<R: Hashable> {
    fn merkleize(self) -> Vec<[u8; 32]>;
}

pub struct TraceTable {
    pub rows:     usize,
    pub cols:     usize,
    pub elements: Vec<FieldElement>,
}

impl TraceTable {
    pub fn new(rows: usize, cols: usize, elements: Vec<FieldElement>) -> Self {
        Self {
            rows,
            cols,
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
// true, otherwise the system will perform all computation directly
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

impl Groupable<Vec<U256>> for &[Vec<FieldElement>] {
    fn make_group(&self, index: usize) -> Vec<U256> {
        let mut ret = Vec::with_capacity(self.len());
        for item in self.iter() {
            ret.push(item[index.bit_reverse_at(item.len())].0.clone())
        }
        ret
    }

    fn domain_size(&self) -> usize {
        self[0].len()
    }
}

impl Groupable<U256> for &[FieldElement] {
    fn make_group(&self, index: usize) -> U256 {
        self[index.bit_reverse_at(self.len())].0.clone()
    }

    fn domain_size(&self) -> usize {
        self.len()
    }
}

impl<
        R: Hashable + std::marker::Send + std::marker::Sync,
        T: Groupable<R> + std::marker::Send + std::marker::Sync,
    > Merkleizable<R> for T
{
    fn merkleize(self) -> Vec<[u8; 32]> {
        let eval_domain_size = self.domain_size();
        let mut leaves = Vec::with_capacity(eval_domain_size);
        (0..eval_domain_size)
            .into_par_iter()
            .map(|index| self.make_group(index))
            .collect_into_vec(&mut leaves);
        make_tree(leaves.as_slice())
    }
}

pub fn stark_proof(
    trace: &TraceTable,
    constraints: &Constraint,
    claim_index: usize,
    claim_value: FieldElement,
    params: &ProofParams,
) -> Channel {
    let trace_len = trace.elements.len() / trace.cols;
    let omega = FieldElement::root(U256::from((trace_len * params.blowup) as u64)).unwrap();
    let g = omega.pow(U256::from(params.blowup as u64));
    let eval_domain_size = trace_len * params.blowup;

    let eval_x = geometric_series(&FieldElement::ONE, &omega, eval_domain_size);

    let TPn = interpolate_trace_table(&trace);
    let TPn_reference: Vec<&[FieldElement]> = TPn.iter().map(|x| x.as_slice()).collect();
    let LDEn = calculate_low_degree_extensions(TPn_reference.as_slice(), &params, &eval_x);

    let tree = LDEn.as_slice().merkleize();

    let mut public_input = [claim_index.to_be_bytes()].concat();
    public_input.extend_from_slice(&claim_value.0.to_bytes_be());
    let mut proof = Channel::new(public_input.as_slice());
    proof.write(&tree[1]);

    let mut constraint_coefficients = Vec::with_capacity(constraints.NCONSTRAINTS);
    for _i in 0..constraints.NCONSTRAINTS {
        constraint_coefficients.push(proof.read());
    }

    let LDEn_reference: Vec<&[FieldElement]> = LDEn.iter().map(|x| x.as_slice()).collect();
    let CC = calculate_constraints_on_domain(
        TPn_reference.as_slice(),
        LDEn_reference.as_slice(),
        constraints,
        constraint_coefficients.as_slice(),
        claim_index,
        &claim_value,
        params.blowup,
    );

    let c_tree = CC.as_slice().merkleize();
    proof.write(&c_tree[1]);

    let (oods_point, oods_coefficients, oods_values) = get_out_of_domain_information(
        &mut proof,
        TPn_reference.as_slice(),
        constraint_coefficients.as_slice(),
        claim_index,
        &claim_value,
        &constraints,
        &g,
    );

    let CO = calculate_out_of_domain_constraints(
        LDEn_reference.as_slice(),
        CC.as_slice(),
        &oods_point,
        oods_coefficients.as_slice(),
        oods_values.as_slice(),
        eval_x.as_slice(),
        params.blowup,
    );

    let (fri_layers, fri_trees) =
        perform_fri_layering(CO.as_slice(), &mut proof, &params, eval_x.as_slice());

    let proof_of_work = proof.pow_find_nonce(params.pow_bits);
    debug_assert!(&proof.pow_verify(proof_of_work, params.pow_bits));
    proof.write(proof_of_work);

    let query_indices = get_indices(
        params.queries,
        64 - eval_domain_size.leading_zeros() - 1,
        &mut proof,
    );
    decommit_with_queries_and_proof(
        query_indices.as_slice(),
        LDEn.as_slice(),
        tree.as_slice(),
        &mut proof,
    );
    decommit_with_queries_and_proof(
        query_indices.as_slice(),
        CC.as_slice(),
        c_tree.as_slice(),
        &mut proof,
    );
    decommit_fri_layers_and_trees(
        fri_layers.as_slice(),
        fri_trees.as_slice(),
        query_indices.as_slice(),
        &params,
        &mut proof,
    );
    proof
}

fn fri_layer(
    previous: &[FieldElement],
    evaluation_point: &FieldElement,
    eval_domain_size: usize,
    eval_x: &[FieldElement],
) -> Vec<FieldElement> {
    let len = previous.len();
    let step = eval_domain_size / len;
    let mut next = Vec::with_capacity(len / 2);
    next.par_extend((0..(len / 2)).into_par_iter().map(|index| {
        let negative_index = (len / 2 + index) % len;
        let inverse_index = ((len - index) % len) * step;
        // OPT: Check if computed x_inv is faster
        let x_inv = &eval_x[inverse_index];
        let value = &previous[index];
        let neg_x_value = &previous[negative_index];
        debug_assert_eq!(x_inv, &eval_x[index * step].inv().unwrap());
        debug_assert_eq!(eval_x[negative_index * step], -&eval_x[index * step]);
        (value + neg_x_value) + evaluation_point * x_inv * (value - neg_x_value)
    }));
    next
}

fn fri_tree(layer: &[FieldElement], coset_size: usize) -> Vec<[u8; 32]> {
    let n = layer.len();
    let mut internal_leaves = Vec::new();
    for i in (0..n).step_by(coset_size) {
        let mut internal_leaf = Vec::with_capacity(coset_size);
        for j in 0..coset_size {
            internal_leaf.push(layer[(i + j).bit_reverse_at(n)].0.clone());
        }
        internal_leaves.push(internal_leaf);
    }
    let leaf_reference: Vec<&[U256]> = internal_leaves.iter().map(|x| x.as_slice()).collect();
    make_tree(leaf_reference.as_slice())
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

fn interpolate_trace_table(table: &TraceTable) -> Vec<Vec<FieldElement>> {
    let trace_len = table.elements.len() / table.cols;
    let mut TPn = vec![Vec::new(); table.cols];
    (0..table.cols)
        .into_par_iter()
        .map(|x| {
            let mut hold_col = Vec::with_capacity(trace_len);
            for i in (0..table.elements.len()).step_by(table.cols) {
                hold_col.push(table.elements[x + i].clone());
            }
            ifft(hold_col.as_slice())
        })
        .collect_into_vec(&mut TPn);
    TPn
}

fn calculate_low_degree_extensions(
    trace_poly: &[&[FieldElement]],
    params: &ProofParams,
    eval_x: &[FieldElement],
) -> Vec<Vec<FieldElement>> {
    let trace_len = trace_poly[0].len();
    let omega = FieldElement::root(U256::from((trace_len * params.blowup) as u64)).unwrap();
    let gen = FieldElement::GENERATOR;

    let mut LDEn = vec![vec![FieldElement::ZERO; eval_x.len()]; trace_poly.len()];
    // OPT - Refactor to not allocate in this loop.
    LDEn.par_iter_mut().enumerate().for_each(|(x, col)| {
        let data_holder: Vec<Vec<FieldElement>> = (0..params.blowup)
            .into_par_iter()
            .map(|j| fft_cofactor(trace_poly[x], &(&gen * &omega.pow(U256::from(j as u64)))))
            .collect();

        col.par_chunks_mut(params.blowup)
            .enumerate()
            .for_each(|(i, chunk)| {
                for (j, item) in chunk.iter_mut().enumerate() {
                    *item = data_holder[j][i].clone();
                }
            });
    });

    LDEn
}

fn calculate_constraints_on_domain(
    trace_poly: &[&[FieldElement]],
    lde_poly: &[&[FieldElement]],
    constraints: &Constraint,
    constraint_coefficients: &[FieldElement],
    claim_index: usize,
    claim_value: &FieldElement,
    blowup: usize,
) -> Vec<FieldElement> {
    let mut CC;
    let trace_len = trace_poly[0].len();
    let mut x = FieldElement::GENERATOR;
    let omega = FieldElement::root(U256::from((trace_len * blowup) as u64)).unwrap();
    let eval_domain_size = trace_len * blowup;

    match constraints.eval_loop {
        Some(x) => CC = (x)(lde_poly, constraint_coefficients, claim_index, &claim_value),
        None => {
            CC = vec![FieldElement::ZERO; eval_domain_size];
            for constraint_element in CC.iter_mut() {
                *constraint_element = (constraints.eval)(
                    // This will perform the polynomial evaluation on each step
                    &x,
                    trace_poly,
                    claim_index,
                    claim_value.clone(),
                    constraint_coefficients,
                );

                x *= &omega;
            }
        }
    }
    CC
}

fn get_out_of_domain_information(
    proof: &mut Channel,
    trace_poly: &[&[FieldElement]],
    constraint_coefficients: &[FieldElement],
    claim_index: usize,
    claim_value: &FieldElement,
    constraints: &Constraint,
    g: &FieldElement,
) -> (FieldElement, Vec<FieldElement>, Vec<FieldElement>) {
    let oods_point: FieldElement = proof.read();
    let oods_point_g = &oods_point * g;
    let mut oods_values = Vec::with_capacity(2 * trace_poly.len() + 1);
    for item in trace_poly.iter() {
        let mut evaled = eval_poly(oods_point.clone(), item);
        oods_values.push(evaled.clone());
        evaled = eval_poly(oods_point_g.clone(), item);
        oods_values.push(evaled.clone());
    }

    oods_values.push((constraints.eval)(
        &oods_point,
        trace_poly,
        claim_index,
        claim_value.clone(),
        constraint_coefficients,
    )); // Gets eval_C of the oods point via direct computation

    for v in oods_values.iter() {
        proof.write(v);
    }

    let mut oods_coefficients = Vec::with_capacity(2 * trace_poly.len() + 1);
    for _i in 0..=2 * trace_poly.len() {
        oods_coefficients.push(proof.read());
    }
    (oods_point, oods_coefficients, oods_values)
}

fn calculate_out_of_domain_constraints(
    lde_poly: &[&[FieldElement]],
    constraint_on_domain: &[FieldElement],
    oods_point: &FieldElement,
    oods_coefficients: &[FieldElement],
    oods_values: &[FieldElement],
    eval_x: &[FieldElement],
    blowup: usize,
) -> Vec<FieldElement> {
    let eval_domain_size = eval_x.len();
    let trace_len = eval_domain_size / blowup;
    let omega = FieldElement::root(U256::from((trace_len * blowup) as u64)).unwrap();
    let g = omega.pow(U256::from(blowup as u64));

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
        .map(|i| (i - oods_point, i - oods_point * &g))
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

            for x in 0..lde_poly.len() {
                r += &oods_coefficients[2 * x] * (&lde_poly[x][i] - &oods_values[2 * x]) * A;
                r +=
                    &oods_coefficients[2 * x + 1] * (&lde_poly[x][i] - &oods_values[2 * x + 1]) * B;
            }
            r += &oods_coefficients[oods_coefficients.len() - 1]
                * (&constraint_on_domain[i] - &oods_values[oods_values.len() - 1])
                * A;

            r
        })
        .collect_into_vec(&mut CO);
    CO
}

fn perform_fri_layering(
    constraints_out_of_domain: &[FieldElement],
    proof: &mut Channel,
    params: &ProofParams,
    eval_x: &[FieldElement],
) -> (Vec<Vec<FieldElement>>, Vec<Vec<[u8; 32]>>) {
    let eval_domain_size = constraints_out_of_domain.len();
    let trace_len = eval_domain_size / params.blowup;

    // Fri Layers
    debug_assert!(eval_domain_size.is_power_of_two());
    let mut fri: Vec<Vec<FieldElement>> =
        Vec::with_capacity(64 - (eval_domain_size.leading_zeros() as usize));
    fri.push(constraints_out_of_domain.to_vec());
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
                eval_x,
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
            eval_x,
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
    (fri, fri_trees)
}

fn decommit_with_queries_and_proof<R: Hashable, T: Groupable<R>>(
    queries: &[usize],
    source: T,
    tree: &[[u8; 32]],
    proof: &mut Channel,
) where
    Channel: Writable<R>,
{
    for &index in queries.iter() {
        proof.write((&source).make_group(index));
    }
    decommit_proof(crate::merkle::proof(tree, queries), proof);
}

// Note - This function exists because rust gets confused by the intersection of
// the write types and the others.
fn decommit_proof(decommitment: Vec<[u8; 32]>, proof: &mut Channel) {
    for x in decommitment.iter() {
        proof.write(x);
    }
}

fn decommit_fri_layers_and_trees(
    fri_layers: &[Vec<FieldElement>],
    fri_trees: &[Vec<[u8; 32]>],
    query_indices: &[usize],
    params: &ProofParams,
    proof: &mut Channel,
) {
    let mut fri_indices: Vec<usize> = query_indices
        .to_vec()
        .iter()
        .map(|x| x / (params.blowup / 2))
        .collect();

    let mut current_fri = 0;
    let mut previous_indices = query_indices.to_vec().clone();
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
                        &fri_layers[current_fri][((n as u64).bit_reverse()
                            >> (u64::from(fri_layers[current_fri].len().leading_zeros() + 1)))
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
