use crate::{
    channel::{ProverChannel, RandomGenerator, Writable},
    fft::{bit_reversal_permute, fft_cofactor_bit_reversed, ifft},
    merkle::{self, make_tree, Hashable},
    polynomial::eval_poly,
    utils::Reversible,
};
use itertools::Itertools;
use primefield::{invert_batch, FieldElement};
use rayon::prelude::*;
use u256::U256;

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
    pub num_constraints: usize,
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
        num_constraints: usize,
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
            num_constraints,
            eval,
            eval_loop,
        }
    }
}

// This groupable impl allows the fri tree layers to get grouped and use the
// same merkleize system
impl Groupable<Vec<U256>> for (usize, &[FieldElement]) {
    fn make_group(&self, index: usize) -> Vec<U256> {
        let (coset_size, layer) = *self;
        let mut internal_leaf = Vec::with_capacity(coset_size);
        for j in 0..coset_size {
            internal_leaf.push(layer[(index * coset_size + j)].0.clone());
        }
        internal_leaf
    }

    fn domain_size(&self) -> usize {
        self.1.len() / self.0
    }
}

impl Groupable<Vec<U256>> for &[Vec<FieldElement>] {
    fn make_group(&self, index: usize) -> Vec<U256> {
        let mut ret = Vec::with_capacity(self.len());
        for item in self.iter() {
            ret.push(item[index].0.clone())
        }
        ret
    }

    fn domain_size(&self) -> usize {
        self[0].len()
    }
}

impl Groupable<U256> for &[FieldElement] {
    fn make_group(&self, index: usize) -> U256 {
        self[index].0.clone()
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

// TODO: Naming
#[allow(non_snake_case)]
pub fn stark_proof(
    trace: &TraceTable,
    constraints: &Constraint,
    claim_index: usize,
    claim_value: FieldElement,
    params: &ProofParams,
) -> ProverChannel {
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
    let mut proof = ProverChannel::new(public_input.as_slice());
    proof.write(&tree[1]);

    let mut constraint_coefficients = Vec::with_capacity(constraints.num_constraints);
    for _i in 0..constraints.num_constraints {
        constraint_coefficients.push(proof.get_random());
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
    (0..(len / 2))
        .into_par_iter()
        .map(|index| {
            let value = &previous[2 * index];
            let neg_x_value = &previous[2 * index + 1];
            let x_inv = &eval_x[index.bit_reverse_at(len / 2) * step].inv().unwrap();
            (value + neg_x_value) + evaluation_point * x_inv * (value - neg_x_value)
        })
        .collect_into_vec(&mut next);
    next
}

fn get_indices(num: usize, bits: u32, proof: &mut ProverChannel) -> Vec<usize> {
    let mut query_indices = Vec::with_capacity(num + 3);
    while query_indices.len() < num {
        let val: U256 = proof.get_random();
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

// TODO: Naming
#[allow(non_snake_case)]
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

// TODO: Naming
#[allow(non_snake_case)]
fn calculate_low_degree_extensions(
    trace_poly: &[&[FieldElement]],
    params: &ProofParams,
    eval_x: &[FieldElement],
) -> Vec<Vec<FieldElement>> {
    let trace_len = trace_poly[0].len();
    let omega = FieldElement::root(U256::from((trace_len * params.blowup) as u64)).unwrap();
    let gen = FieldElement::GENERATOR;

    let mut LDEn = vec![Vec::with_capacity(eval_x.len()); trace_poly.len()];
    LDEn.par_iter_mut().enumerate().for_each(|(x, col)| {
        for index in 0..params.blowup {
            let reverse_index = index.bit_reverse_at(params.blowup);
            let cofactor = &gen * omega.pow(U256::from(reverse_index as u64));
            col.extend(fft_cofactor_bit_reversed(trace_poly[x], &cofactor));
        }
    });

    LDEn
}

// TODO: Naming
#[allow(non_snake_case)]
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
    proof: &mut ProverChannel,
    trace_poly: &[&[FieldElement]],
    constraint_coefficients: &[FieldElement],
    claim_index: usize,
    claim_value: &FieldElement,
    constraints: &Constraint,
    g: &FieldElement,
) -> (FieldElement, Vec<FieldElement>, Vec<FieldElement>) {
    let oods_point: FieldElement = proof.get_random();
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
        oods_coefficients.push(proof.get_random());
    }
    (oods_point, oods_coefficients, oods_values)
}

// TODO: Naming
#[allow(non_snake_case)]
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
        .map(|index| {
            let i = index.bit_reverse_at(eval_domain_size);
            let A = &x_oods_cycle[i];
            let B = &x_oods_cycle_g[i];
            let mut r = FieldElement::ZERO;

            for x in 0..lde_poly.len() {
                r += &oods_coefficients[2 * x] * (&lde_poly[x][index] - &oods_values[2 * x]) * A;
                r += &oods_coefficients[2 * x + 1]
                    * (&lde_poly[x][index] - &oods_values[2 * x + 1])
                    * B;
            }
            r += &oods_coefficients[oods_coefficients.len() - 1]
                * (&constraint_on_domain[index] - &oods_values[oods_values.len() - 1])
                * A;

            r
        })
        .collect_into_vec(&mut CO);
    CO
}

fn perform_fri_layering(
    constraints_out_of_domain: &[FieldElement],
    proof: &mut ProverChannel,
    params: &ProofParams,
    eval_x: &[FieldElement],
) -> (Vec<Vec<FieldElement>>, Vec<Vec<[u8; 32]>>) {
    let eval_domain_size = constraints_out_of_domain.len();
    let trace_len = eval_domain_size / params.blowup;

    debug_assert!(eval_domain_size.is_power_of_two());
    let mut fri: Vec<Vec<FieldElement>> =
        Vec::with_capacity(64 - (eval_domain_size.leading_zeros() as usize));
    fri.push(constraints_out_of_domain.to_vec());
    let mut fri_trees: Vec<Vec<[u8; 32]>> = Vec::with_capacity(params.fri_layout.len());
    let held_tree = (params.blowup / 2, fri[fri.len() - 1].as_slice()).merkleize();
    proof.write(&held_tree[1]);
    fri_trees.push(held_tree);

    let mut halvings = 0;
    let mut fri_const = params.blowup / 4;
    for &x in params.fri_layout.iter().dropping_back(1) {
        let mut eval_point = if x == 0 {
            FieldElement::ONE
        } else {
            proof.get_random()
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
        let held_tree = (fri_const, fri[fri.len() - 1].as_slice()).merkleize();

        proof.write(&held_tree[1]);
        fri_trees.push(held_tree);
        fri_const /= 2;
        halvings += x;
    }

    // Gets the coefficient representation of the last number of fri reductions
    let mut eval_point = proof.get_random();
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

    let mut last_layer = fri[halvings].clone();
    bit_reversal_permute(&mut last_layer);
    let mut last_layer_coefficient = ifft(&last_layer);
    last_layer_coefficient.truncate(last_layer_degree_bound);
    proof.write(last_layer_coefficient.as_slice());
    debug_assert_eq!(last_layer_coefficient.len(), last_layer_degree_bound);
    (fri, fri_trees)
}

fn decommit_with_queries_and_proof<R: Hashable, T: Groupable<R>>(
    queries: &[usize],
    source: T,
    tree: &[[u8; 32]],
    proof: &mut ProverChannel,
) where
    ProverChannel: Writable<R>,
{
    for &index in queries.iter() {
        proof.write((&source).make_group(index));
    }
    decommit_proof(merkle::proof(tree, queries, source), proof);
}

// Note - This function exists because rust gets confused by the intersection of
// the write types and the others.
fn decommit_proof(decommitment: Vec<[u8; 32]>, proof: &mut ProverChannel) {
    for x in decommitment.iter() {
        proof.write(x);
    }
}

fn decommit_fri_layers_and_trees(
    fri_layers: &[Vec<FieldElement>],
    fri_trees: &[Vec<[u8; 32]>],
    query_indices: &[usize],
    params: &ProofParams,
    proof: &mut ProverChannel,
) {
    let mut fri_indices: Vec<usize> = query_indices
        .to_vec()
        .iter()
        .map(|x| x / (params.blowup / 2))
        .collect();

    let mut current_fri = 0;
    let mut previous_indices = query_indices.to_vec().clone();
    let mut fri_const = params.blowup / 2;
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
                    proof.write(&fri_layers[current_fri][n]);
                }
            }
        }

        let decommitment = merkle::proof(
            &next_tree,
            &(fri_indices.as_slice()),
            (fri_const, fri_layers[current_fri].as_slice()),
        );

        for proof_element in decommitment.iter() {
            proof.write(proof_element);
        }
        fri_const /= 2;
        previous_indices = fri_indices.clone();
        fri_indices = fri_indices.iter().map(|ind| ind / 4).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fibonacci::*;
    use hex_literal::*;
    use u256::{u256h, U256};

    #[test]
    fn fib_test_1024_python_witness() {
        let claim_index = 1000;
        let claim_value = FieldElement::from(u256h!(
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
            claim_value,
            &ProofParams {
                blowup:     16,
                pow_bits:   12,
                queries:    20,
                fri_layout: vec![3, 2],
            },
        );
        assert_eq!(actual.coin.digest, expected);
    }

    #[test]
    fn fib_test_1024_changed_witness() {
        let claim_index = 1000;
        let claim_value = FieldElement::from(u256h!(
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
            claim_value,
            &ProofParams {
                blowup:     32,
                pow_bits:   12,
                queries:    20,
                fri_layout: vec![3, 2],
            },
        );
        assert_eq!(actual.coin.digest, expected);
    }

    #[test]
    fn fib_test_4096() {
        let claim_index = 4000;
        let claim_value = FieldElement::from(u256h!(
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
            claim_value,
            &ProofParams {
                blowup:     16,
                pow_bits:   12,
                queries:    20,
                fri_layout: vec![3, 2, 1],
            },
        );
        assert_eq!(actual.coin.digest, expected);
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

    #[test]
    // TODO: Naming
    #[allow(non_snake_case)]
    // TODO - See if it's possible to do context cloning and break this into smaller tests
    #[allow(clippy::cognitive_complexity)]
    fn fib_proof_test() {
        let claim_index = 1000;
        let claim_value = FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        ));
        let witness = FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000000000000cafebabe"
        ));

        let constraints = get_constraint();
        let params = ProofParams {
            blowup:     16,
            pow_bits:   12,
            queries:    20,
            fri_layout: vec![3, 2],
        };

        let trace_len = 1024;
        let omega = FieldElement::from(u256h!(
            "0393a32b34832dbad650df250f673d7c5edd09f076fc314a3e5a42f0606082e1"
        ));
        let g = FieldElement::from(u256h!(
            "0659d83946a03edd72406af6711825f5653d9e35dc125289a206c054ec89c4f1"
        ));
        let eval_domain_size = trace_len * params.blowup;
        let gen = FieldElement::from(U256::from(3_u64));

        let eval_x = geometric_series(&FieldElement::ONE, &omega, eval_domain_size);
        let eval_offset_x = geometric_series(&gen, &omega, eval_domain_size);
        let trace_x = geometric_series(&FieldElement::ONE, &g, trace_len);
        // Checks that the geometric series is working
        assert_eq!(
            U256::from(eval_x[500].clone()),
            u256h!("068a24ef8b13c6b23a4fe31235667142494bc0eecbb59ed9866a44ac47fb2f6b")
        );

        // Second check that the trace table function is working.
        let trace = get_trace_table(1024, witness);
        assert_eq!(trace.elements[2000], claim_value);

        let TPn = interpolate_trace_table(&trace);
        let TP0 = TPn[0].as_slice();
        let TP1 = TPn[1].as_slice();
        // Checks that the trace table polynomial interpolation is working
        assert_eq!(eval_poly(trace_x[1000].clone(), TP0), trace.elements[2000]);

        let TPn_reference: Vec<&[FieldElement]> = TPn.iter().map(|x| x.as_slice()).collect();
        let LDEn = calculate_low_degree_extensions(TPn_reference.as_slice(), &params, &eval_x);

        // Checks that the low degree extension calculation is working
        let LDE0 = LDEn[0].as_slice();
        let LDE1 = LDEn[1].as_slice();
        let i = 13644usize;
        let reverse_i = i.bit_reverse_at(eval_domain_size);
        assert_eq!(eval_poly(eval_offset_x[reverse_i].clone(), TP0), LDE0[i]);
        assert_eq!(eval_poly(eval_offset_x[reverse_i].clone(), TP1), LDE1[i]);

        // Checks that the groupable trait is properly grouping for &[Vec<FieldElement>]
        assert_eq!(
            (LDEn.as_slice().make_group(3243))[0].clone(),
            u256h!("01ddd9e389a326817ad1d2a5311e1bc2cf7fa734ebdc2961085b5acfa87a58ff")
        );
        assert_eq!(
            (LDEn.as_slice().make_group(3243))[1].clone(),
            u256h!("03dbc6c47df0606997c2cefb20c4277caf2b76bca1d31c13432f71cdd93b3718")
        );

        let tree = LDEn.merkleize();
        // Checks that the merklelizable implementation is working [implicit check of
        // most previous steps]
        assert_eq!(
            tree[1],
            hex!("018dc61f748b1a6c440827876f30f63cb6c4c188000000000000000000000000")
        );

        let mut public_input = [(claim_index as u64).to_be_bytes()].concat();
        public_input.extend_from_slice(&claim_value.0.to_bytes_be());

        let mut proof = ProverChannel::new(&public_input.as_slice());
        // Checks that the channel is inited properly
        assert_eq!(
            proof.coin.digest,
            hex!("c891a11ddbc6c425fad523a7a4aeafa505d7aa1638cfffbd5b747100bc69e367")
        );
        proof.write(&tree[1]);
        // Checks that the channel allows writing of [u8; 32] properly
        assert_eq!(
            proof.coin.digest,
            hex!("b7d80385fa0c8879473cdf987ea7970bb807aec78bb91af39a1504d965ad8e92")
        );
        let test_element: FieldElement = proof.get_random();
        // Checks that the channel is pulling field elements properly
        assert_eq!(
            U256::from(test_element),
            u256h!("0529fc64b01be65623ef376bfa31d62b9a75ba2f51b5fda79e55e2ac05dfa80f")
        );

        let mut constraint_coefficients = Vec::with_capacity(constraints.num_constraints);
        for _i in 0..constraints.num_constraints {
            constraint_coefficients.push(proof.get_random());
        }

        let LDEn_reference: Vec<&[FieldElement]> = LDEn.iter().map(|x| x.as_slice()).collect();
        let CC = calculate_constraints_on_domain(
            TPn_reference.as_slice(),
            LDEn_reference.as_slice(),
            &constraints,
            constraint_coefficients.as_slice(),
            claim_index,
            &claim_value,
            params.blowup,
        );
        // Checks that our constraints are properly calculated on the domain
        assert_eq!(
            CC[123.bit_reverse_at(eval_domain_size)].clone(),
            FieldElement(u256h!(
                "019fb62b06446e919d7909f4896febce72978ff860e1ed61b4418091617677d3"
            ))
        );

        let c_tree = CC.as_slice().merkleize();
        // Checks both that the merkle tree is working for this groupable type and that
        // the constraints are properly calculated on the domain
        assert_eq!(
            c_tree[1],
            hex!("46318de7dbdafda87c1052d50989d15f8e61a5b8000000000000000000000000")
        );
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
        // Checks that we have derived the right out of domain sample point
        assert_eq!(
            U256::from(oods_point.clone()),
            u256h!("031dc8fc2f57e3f39f6951a04a04294a7c63c988573dc058eea4cbf3e6268353")
        );
        // Checks that our get out of domain function call has written the right values
        // to the proof
        assert_eq!(
            proof.coin.digest,
            hex!("f556f04f342598411b5626a797a114a64b3a15a5ab0d4f2a6b350b941d56d071")
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
        // Checks that our out of domain evaluated constraints calculated right
        assert_eq!(
            CO[4321.bit_reverse_at(eval_domain_size)].clone(),
            FieldElement(u256h!(
                "023b8ba264d4a1255e1dedd6e5819e86230562b85d5a7af8fb994053a2debdde"
            ))
        );

        let (fri_layers, fri_trees) =
            perform_fri_layering(CO.as_slice(), &mut proof, &params, eval_x.as_slice());

        // Checks that the first fri merkle tree root is right
        assert_eq!(
            fri_trees[0][1],
            hex!("f5110a80f0fabf114678f7e643a2be01f88661fe000000000000000000000000")
        );
        // Checks that the second fri merkle tree root is right
        assert_eq!(
            fri_trees[1][1],
            hex!("27ad2f6a19d18a7e4535905f1ee0bf0d39e8e444000000000000000000000000")
        );
        // Checks that the fri layering function decommited the right values.
        assert_eq!(
            proof.coin.digest,
            hex!("e2c7e50f3d1dcaad74678d8abb489675849ead08e2f848429a136304d9550bb6")
        );

        let proof_of_work = proof.pow_find_nonce(params.pow_bits);
        // Checks that the pow function is working [may also fail if the previous steps
        // have perturbed the channel's random]
        assert_eq!(proof_of_work, 3465);
        proof.write(proof_of_work);

        let query_indices = get_indices(
            params.queries,
            64 - eval_domain_size.leading_zeros() - 1,
            &mut proof,
        );
        // Checks that the get query_indices is working
        assert_eq!(query_indices[19], 16056);

        decommit_with_queries_and_proof(
            query_indices.as_slice(),
            LDEn.as_slice(),
            tree.as_slice(),
            &mut proof,
        );
        // Checks that our first decommitment is successful
        assert_eq!(
            proof.coin.digest,
            hex!("804a12f5f778c9d2b076d07a8c516dd8e1a57c35ef2df10f55df58764812799d")
        );

        decommit_with_queries_and_proof(
            query_indices.as_slice(),
            CC.as_slice(),
            c_tree.as_slice(),
            &mut proof,
        );
        // Checks that our second decommitment is successful
        assert_eq!(
            proof.coin.digest,
            hex!("ea73885255f98e9a51f6549fb74e076181971e426190660cdc45bac337423cb6")
        );

        decommit_fri_layers_and_trees(
            fri_layers.as_slice(),
            fri_trees.as_slice(),
            query_indices.as_slice(),
            &params,
            &mut proof,
        );
        // Checks that our fri decommitment is successful
        assert_eq!(
            proof.coin.digest,
            hex!("3d3b54ffd1c5e6f579648398b4a9bb67166d83d24c76e6adf74fa0feaf4e16d9")
        );
    }
}
