use crate::{
    channel::{ProverChannel, RandomGenerator, Writable},
    fft::{bit_reversal_permute, fft_cofactor_bit_reversed, ifft},
    hash::Hash,
    hashable::Hashable,
    merkle::{self, make_tree},
    mmap_vec::MmapVec,
    polynomial::{DensePolynomial, SparsePolynomial},
    utils::Reversible,
    TraceTable,
};
use itertools::Itertools;
use primefield::FieldElement;
use rayon::prelude::*;
use std::{
    cmp::max,
    marker::{Send, Sync},
    prelude::v1::*,
    vec,
};
use u256::U256;

// This trait is for objects where the object is grouped into hashable sets
// based on index before getting made into a merkle tree, with domain size
// being the max index [ie the one which if you iterate up to it splits the
// whole range]
pub trait Groupable<LeafType: Hashable> {
    fn get_leaf_hash(&self, index: usize) -> Hash {
        self.get_leaf(index).hash()
    }
    fn get_leaf(&self, index: usize) -> LeafType;
    fn domain_size(&self) -> usize;
}

// This trait is applied to give groupable objects a merkle tree based on their
// groupings
pub trait Merkleizable<NodeHash: Hashable> {
    fn merkleize(self) -> Vec<Hash>;
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

    /// The highest degree of any constraint polynomial.
    ///
    /// The polynomial constraints are not directly commited too on the trace
    /// domain, instead they are calculated via "Deep fri" which samples and
    /// commits too information outside of the domain.
    ///
    /// This information on constraint degree allows the out of domain sampling
    /// to provide the right number points.
    pub constraints_degree_bound: usize,
}

pub struct Constraint {
    pub base:        Box<dyn Fn(&[DensePolynomial]) -> DensePolynomial>,
    pub denominator: SparsePolynomial,
    pub numerator:   SparsePolynomial,
}

// This groupable impl allows the fri tree layers to get grouped and use the
// same merkleize system
impl Groupable<Vec<U256>> for (usize, &[FieldElement]) {
    fn get_leaf(&self, index: usize) -> Vec<U256> {
        let (coset_size, layer) = *self;
        let mut internal_leaf = Vec::with_capacity(coset_size);
        for j in 0..coset_size {
            internal_leaf.push(layer[(index * coset_size + j)].as_montgomery().clone());
        }
        internal_leaf
    }

    fn domain_size(&self) -> usize {
        self.1.len() / self.0
    }
}

impl Groupable<Vec<U256>> for &[MmapVec<FieldElement>] {
    fn get_leaf(&self, index: usize) -> Vec<U256> {
        let mut ret = Vec::with_capacity(self.len());
        for item in self.iter() {
            ret.push(item[index].as_montgomery().clone())
        }
        ret
    }

    fn domain_size(&self) -> usize {
        self[0].len()
    }
}

impl Groupable<U256> for &[FieldElement] {
    fn get_leaf(&self, index: usize) -> U256 {
        self[index].as_montgomery().clone()
    }

    fn domain_size(&self) -> usize {
        self.len()
    }
}

impl<NodeHash, LeafType> Merkleizable<NodeHash> for LeafType
where
    NodeHash: Hashable + Send + Sync,
    LeafType: Groupable<NodeHash> + Send + Sync,
{
    fn merkleize(self) -> Vec<Hash> {
        let eval_domain_size = self.domain_size();
        let mut leaves = Vec::with_capacity(eval_domain_size);
        (0..eval_domain_size)
            .into_par_iter()
            .map(|index| self.get_leaf_hash(index))
            .collect_into_vec(&mut leaves);
        make_tree(leaves.as_slice())
    }
}

pub fn stark_proof<Public>(
    trace: &TraceTable,
    constraints: &[Constraint],
    public: &Public,
    params: &ProofParams,
) -> ProverChannel
where
    for<'a> ProverChannel: Writable<&'a Public>,
    for<'a> ProverChannel: Writable<&'a Hash>,
{
    // Compute some constants.
    let eval_domain_size = trace.num_rows() * params.blowup;
    let omega = FieldElement::root(eval_domain_size).expect("No generator for extended domain.");
    let eval_x = geometric_series(&FieldElement::ONE, &omega, eval_domain_size);

    // Initialize a proof channel with the public input.
    let mut proof = ProverChannel::new();
    proof.write(public);

    // 1. Trace commitment.
    //

    // Compute the low degree extension of the trace table.
    let trace_polynomials = interpolate_trace_table(&trace);
    let trace_lde = calculate_low_degree_extensions(&trace_polynomials, params.blowup);

    // Construct a merkle tree over the LDE trace
    // and write the root to the channel.
    let tree = trace_lde.as_slice().merkleize();
    proof.write(&tree[1]);

    // 2. Constraint commitment
    //

    // Read constraint coefficients from the channel.
    let mut constraint_coefficients = Vec::with_capacity(2 * constraints.len());
    for _ in constraints {
        constraint_coefficients.push(proof.get_random());
        constraint_coefficients.push(proof.get_random());
    }

    let constraint_polynomial = get_constraint_polynomial(
        &trace_polynomials,
        constraints,
        &constraint_coefficients,
        params.constraints_degree_bound,
    );

    let constraint_lde = evalute_polynomial_on_domain(&constraint_polynomial, params.blowup);
    // Construct a merkle tree over the LDE combined constraints
    // and write the root to the channel.
    let c_tree = constraint_lde.as_slice().merkleize();
    proof.write(&c_tree[1]);

    // 3. Out of domain sampling
    //

    // Read the out of domain sampling point from the channel.
    // (and do a bunch more things)
    // TODO: expand
    let (oods_point, oods_coefficients) =
        get_out_of_domain_information(&mut proof, &trace_polynomials, &constraint_polynomial);

    // Divide out the OODS points from the constraints and combine.
    let oods_constraint_lde = calculate_fri_polynomial(
        &trace_polynomials,
        &constraint_polynomial,
        &oods_point,
        &oods_coefficients,
        params.blowup,
    );

    // 4. FRI layers
    let (fri_layers, fri_trees) = perform_fri_layering(
        oods_constraint_lde.as_slice(),
        &mut proof,
        &params,
        eval_x.as_slice(),
    );

    // 5. Proof of work
    let proof_of_work = proof.pow_find_nonce(params.pow_bits);
    debug_assert!(&proof.pow_verify(proof_of_work, params.pow_bits));
    proof.write(proof_of_work);

    // 6. Query decommitments
    //

    // Fetch query indices from channel.
    let query_indices = get_indices(
        params.queries,
        64 - eval_domain_size.leading_zeros() - 1,
        &mut proof,
    );

    // Decommit the trace table values.
    decommit_with_queries_and_proof(
        query_indices.as_slice(),
        trace_lde.as_slice(),
        tree.as_slice(),
        &mut proof,
    );

    // Decommit the constraint values
    decommit_with_queries_and_proof(
        query_indices.as_slice(),
        constraint_lde.as_slice(),
        c_tree.as_slice(),
        &mut proof,
    );

    // Decommit the FRI layer values
    decommit_fri_layers_and_trees(
        fri_layers.as_slice(),
        fri_trees.as_slice(),
        query_indices.as_slice(),
        &params,
        &mut proof,
    );

    // Q.E.D.
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
        let mask = 2usize.pow(bits) - 1;
        query_indices.push((val.clone() >> (0x100 - 0x040)).as_usize() & mask);
        query_indices.push((val.clone() >> (0x100 - 0x080)).as_usize() & mask);
        query_indices.push((val.clone() >> (0x100 - 0x0C0)).as_usize() & mask);
        query_indices.push(val.as_usize() & mask);
    }
    query_indices.truncate(num);
    (&mut query_indices).sort_unstable();
    query_indices
}

pub fn geometric_series(base: &FieldElement, step: &FieldElement, len: usize) -> Vec<FieldElement> {
    const PARALLELIZATION: usize = 16_usize;
    // OPT - Set based on the cores available and how well the work is spread
    let step_len = max(1, len / PARALLELIZATION);
    let mut range = vec![FieldElement::ZERO; len];
    range
        .par_chunks_mut(step_len)
        .enumerate()
        .for_each(|(i, slice)| {
            let mut hold = base * step.pow(i * step_len);
            for element in slice.iter_mut() {
                *element = hold.clone();
                hold *= step;
            }
        });
    range
}

pub fn interpolate_trace_table(table: &TraceTable) -> Vec<DensePolynomial> {
    let mut result: Vec<DensePolynomial> = Vec::with_capacity(table.num_columns());
    (0..table.num_columns())
        .into_par_iter()
        // OPT: Use and FFT that can transform the entire table in one pass,
        // working on whole rows at a time. That is, it is vectorized over rows.
        // OPT: Use an in-place FFT. We don't need the trace table after this,
        // so it can be replaced by a matrix of coefficients.
        // OPT: Avoid double vector allocation here. Implement From<Vec<FieldElement>> for
        // DensePolynomial?
        .map(|j| DensePolynomial::new(&ifft(table.column_to_mmapvec(j).as_slice())))
        .collect_into_vec(&mut result);
    result
}

fn calculate_low_degree_extensions(
    trace_polynomials: &[DensePolynomial],
    blowup: usize,
) -> Vec<MmapVec<FieldElement>> {
    let mut low_degree_extensions: Vec<MmapVec<FieldElement>> =
        Vec::with_capacity(trace_polynomials.len());
    trace_polynomials
        .par_iter()
        .map(|p| evalute_polynomial_on_domain(&p, blowup))
        .collect_into_vec(&mut low_degree_extensions);
    low_degree_extensions
}

fn evalute_polynomial_on_domain(
    constraint_polynomial: &DensePolynomial,
    blowup: usize,
) -> MmapVec<FieldElement> {
    let extended_domain_length = constraint_polynomial.len() * blowup;
    let extended_domain_generator = FieldElement::root(extended_domain_length)
        .expect("No generator for extended_domain_length.");
    let shift_factor = FieldElement::GENERATOR;

    let mut result: MmapVec<FieldElement> = MmapVec::with_capacity(extended_domain_length);
    for index in 0..blowup {
        let reverse_index = index.bit_reverse_at(blowup);
        let cofactor =
            &shift_factor * extended_domain_generator.pow(U256::from(reverse_index as u64));
        result.extend(fft_cofactor_bit_reversed(
            constraint_polynomial.coefficients(),
            &cofactor,
        ));
    }
    result
}

pub fn get_constraint_polynomial(
    trace_polynomials: &[DensePolynomial],
    constraints: &[Constraint],
    constraint_coefficients: &[FieldElement],
    constraints_degree_bound: usize,
) -> DensePolynomial {
    let mut constraint_polynomial =
        DensePolynomial::new(&vec![FieldElement::ZERO; constraints_degree_bound]);
    let trace_length = trace_polynomials[0].len();
    for (i, constraint) in constraints.iter().enumerate() {
        let mut p = (constraint.base)(trace_polynomials);
        p *= constraint.numerator.clone();
        p /= constraint.denominator.clone();
        constraint_polynomial += &(&constraint_coefficients[2 * i] * &p);
        p *= SparsePolynomial::new(&[(
            FieldElement::ONE,
            (constraints_degree_bound - 1) * trace_length + constraint.denominator.degree()
                - constraint.numerator.degree(),
        )]);
        constraint_polynomial += &constraint_coefficients[2 * i + 1] * &p;
    }
    constraint_polynomial
}

fn get_out_of_domain_information(
    proof: &mut ProverChannel,
    trace_polynomials: &[DensePolynomial],
    constraint_polynomial: &DensePolynomial,
) -> (FieldElement, Vec<FieldElement>) {
    let oods_point: FieldElement = proof.get_random();
    let g = FieldElement::root(trace_polynomials[0].len())
        .expect("No root for trace polynomial length.");
    let oods_point_g = &oods_point * &g;
    let mut oods_values = Vec::with_capacity(2 * trace_polynomials.len() + 1);
    for trace_polynomial in trace_polynomials.iter() {
        let mut evaled = trace_polynomial.evaluate(&oods_point);
        oods_values.push(evaled.clone());
        evaled = trace_polynomial.evaluate(&oods_point_g);
        oods_values.push(evaled.clone());
    }

    oods_values.push(constraint_polynomial.evaluate(&oods_point));

    for v in oods_values.iter() {
        proof.write(v);
    }

    let mut oods_coefficients = Vec::with_capacity(2 * trace_polynomials.len() + 1);
    for _ in 0..=2 * trace_polynomials.len() {
        oods_coefficients.push(proof.get_random());
    }
    (oods_point, oods_coefficients)
}

fn divide_out_point(p: &DensePolynomial, x: &FieldElement) -> DensePolynomial {
    let denominator = SparsePolynomial::new(&[(-x, 0), (FieldElement::ONE, 1)]);
    let mut result = p - SparsePolynomial::new(&[(p.evaluate(x), 0)]);
    result /= denominator;
    result
}

fn calculate_fri_polynomial(
    trace_polynomials: &[DensePolynomial],
    constraint_polynomial: &DensePolynomial,
    oods_point: &FieldElement,
    oods_coefficients: &[FieldElement],
    blowup: usize,
) -> Vec<FieldElement> {
    let trace_length = trace_polynomials[0].len();
    let trace_generator = FieldElement::root(trace_length).unwrap();
    let shifted_oods_point = &trace_generator * oods_point;

    let mut fri_polynomial = DensePolynomial::new(&[FieldElement::ZERO]);
    for (i, trace_polynomial) in trace_polynomials.iter().enumerate() {
        fri_polynomial +=
            &oods_coefficients[2 * i] * &divide_out_point(trace_polynomial, oods_point);
        fri_polynomial += &oods_coefficients[2 * i + 1]
            * &divide_out_point(trace_polynomial, &shifted_oods_point);
    }

    fri_polynomial += &oods_coefficients[oods_coefficients.len() - 1]
        * &divide_out_point(constraint_polynomial, oods_point);

    evalute_polynomial_on_domain(&fri_polynomial, blowup).to_vec()
}

fn perform_fri_layering(
    constraints_out_of_domain: &[FieldElement],
    proof: &mut ProverChannel,
    params: &ProofParams,
    eval_x: &[FieldElement],
) -> (Vec<Vec<FieldElement>>, Vec<Vec<Hash>>) {
    let eval_domain_size = constraints_out_of_domain.len();
    let trace_len = eval_domain_size / params.blowup;

    debug_assert!(eval_domain_size.is_power_of_two());
    let mut fri: Vec<Vec<FieldElement>> =
        Vec::with_capacity(64 - (eval_domain_size.leading_zeros() as usize));
    fri.push(constraints_out_of_domain.to_vec());
    let mut fri_trees: Vec<Vec<Hash>> = Vec::with_capacity(params.fri_layout.len());
    let held_tree = (
        2_usize.pow(params.fri_layout[0] as u32),
        fri[fri.len() - 1].as_slice(),
    )
        .merkleize();
    proof.write(&held_tree[1]);
    fri_trees.push(held_tree);

    let mut halvings = 0;
    for (k, &x) in params.fri_layout.iter().enumerate().dropping_back(1) {
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
        let held_tree = (
            2_usize.pow(params.fri_layout[k + 1] as u32),
            fri[fri.len() - 1].as_slice(),
        )
            .merkleize();

        proof.write(&held_tree[1]);
        fri_trees.push(held_tree);
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

    let mut last_layer = fri[fri.len() - 1].clone();
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
    tree: &[Hash],
    proof: &mut ProverChannel,
) where
    ProverChannel: Writable<R>,
{
    for &index in queries.iter() {
        proof.write((&source).get_leaf(index));
    }
    decommit_proof(merkle::proof(tree, queries, source), proof);
}

// Note - This function exists because rust gets confused by the intersection of
// the write types and the others.
fn decommit_proof(decommitment: Vec<Hash>, proof: &mut ProverChannel) {
    for x in decommitment.iter() {
        proof.write(x);
    }
}

fn decommit_fri_layers_and_trees(
    fri_layers: &[Vec<FieldElement>],
    fri_trees: &[Vec<Hash>],
    query_indices: &[usize],
    params: &ProofParams,
    proof: &mut ProverChannel,
) {
    let mut fri_indices: Vec<usize> = query_indices
        .to_vec()
        .iter()
        .map(|x| x / 2_usize.pow((params.fri_layout[0]) as u32))
        .collect();

    let mut current_fri = 0;
    let mut previous_indices = query_indices.to_vec().clone();
    for (k, next_tree) in fri_trees.iter().enumerate() {
        let fri_const = 2_usize.pow(params.fri_layout[k] as u32);
        if k != 0 {
            current_fri += params.fri_layout[k - 1];
        }

        fri_indices.dedup();
        for i in fri_indices.iter() {
            for j in 0..fri_const {
                let n = i * fri_const + j;

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
        previous_indices = fri_indices.clone();
        if k + 1 < params.fri_layout.len() {
            fri_indices = fri_indices
                .iter()
                .map(|ind| ind / 2_usize.pow((params.fri_layout[k + 1]) as u32))
                .collect();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        fibonacci::{get_fibonacci_constraints, get_trace_table, PrivateInput, PublicInput},
        verifier::check_proof,
    };
    use macros_decl::{field_element, hex, u256h};
    use u256::U256;

    #[test]
    fn starkware_fibonacci() {
        // All the constants for this tests are copied from files in
        // https://github.com/0xProject/evm-verifier/commit/9bf369139b0edc23ab7ab7e8db8164c5a05a83df.
        // Copied from solidity/contracts/fibonacci/fibonacci_private_input1.json
        let private = PrivateInput {
            secret: field_element!("83d36de9"),
        };
        let tt = get_trace_table(1024, &private);
        let public = PublicInput {
            index: 1000,
            value: tt[(1000, 0)].clone(),
        };
        // Copied from solidity/contracts/fibonacci/fibonacci_public_input1.json
        assert_eq!(
            tt[(1000, 0)],
            field_element!("04d5f1f669b34fb7252d5a9d0d9786b2638c27eaa04e820b38b088057960cca1")
        );
        let constraints = &get_fibonacci_constraints(&public);
        let actual = stark_proof(&tt, &constraints, &public, &ProofParams {
            blowup:                   16,
            pow_bits:                 0,
            queries:                  20,
            fri_layout:               vec![3, 2],
            constraints_degree_bound: 1,
        });

        // Commitment hashes from
        // solidity/test/fibonacci/proof/fibonacci_proof_annotations.txt
        assert_eq!(
            actual.proof[0..32],
            hex!("4ef92de4d2d3594d35f0123ed8187d60542188f5000000000000000000000000")
        );
        assert_eq!(
            actual.proof[32..64],
            hex!("f2f6338add62aac3311361aa5d4cf2da2ae04fb6000000000000000000000000")
        );
        assert_eq!(
            actual.proof[224..256],
            hex!("e793b5a749cf7d10eb2d43faf4ab472f3ed20c1e000000000000000000000000")
        );
        assert_eq!(
            actual.proof[256..288],
            hex!("2333baba2fa0573e00bca54c2b5508f540a37781000000000000000000000000")
        );
    }

    #[test]
    fn fib_test_1024_python_witness() {
        let private = PrivateInput {
            secret: FieldElement::from(u256h!(
                "00000000000000000000000000000000000000000000000000000000cafebabe"
            )),
        };
        let tt = get_trace_table(1024, &private);
        let public = PublicInput {
            index: 1000,
            value: tt[(1000, 0)].clone(),
        };
        let constraints = &get_fibonacci_constraints(&public);
        let expected = hex!("fcf1924f84656e5068ab9cbd44ae084b235bb990eefc0fd0183c77d5645e830e");

        let actual = stark_proof(&tt, &constraints, &public, &ProofParams {
            blowup:                   16,
            pow_bits:                 12,
            queries:                  20,
            fri_layout:               vec![3, 2],
            constraints_degree_bound: 1,
        });
        assert_eq!(actual.coin.digest, expected);
    }

    #[test]
    fn fib_test_1024_changed_witness() {
        let private = PrivateInput {
            secret: FieldElement::from(u256h!(
                "00000000000000000000000000000000000000000000000f00dbabe0cafebabe"
            )),
        };
        let tt = get_trace_table(1024, &private);
        let public = PublicInput {
            index: 1000,
            value: tt[(1000, 0)].clone(),
        };
        let actual = stark_proof(
            &get_trace_table(1024, &private),
            &get_fibonacci_constraints(&public),
            &public,
            &ProofParams {
                blowup: 16, /* TODO - The blowup in the fib constraints is hardcoded to 16,
                             * we should set this back to 32 to get wider coverage when
                             * that's fixed */
                pow_bits:                 12,
                queries:                  20,
                fri_layout:               vec![3, 2],
                constraints_degree_bound: 1,
            },
        );

        assert!(check_proof(
            actual,
            &get_fibonacci_constraints(&public),
            &public,
            &ProofParams {
                blowup: 16, /* TODO - The blowup in the fib constraints is hardcoded to 16,
                             * we should set this back to 32 to get wider coverage when
                             * that's fixed */
                pow_bits:                 12,
                queries:                  20,
                fri_layout:               vec![3, 2],
                constraints_degree_bound: 1,
            },
            2,
            1024
        ));
    }

    #[test]
    fn fib_test_4096() {
        let private = PrivateInput {
            secret: FieldElement::from(u256h!(
                "00000000000000000000000000000000000000000000000f00dbabe0cafebabe"
            )),
        };
        let tt = get_trace_table(4096, &private);
        let public = PublicInput {
            index: 4000,
            value: tt[(4000, 0)].clone(),
        };
        let constraints = get_fibonacci_constraints(&public);
        let actual = stark_proof(&tt, &constraints, &public, &ProofParams {
            blowup:                   16,
            pow_bits:                 12,
            queries:                  20,
            fri_layout:               vec![2, 1, 4, 2],
            constraints_degree_bound: 1,
        });

        assert!(check_proof(
            actual,
            &constraints,
            &public,
            &ProofParams {
                blowup:                   16,
                pow_bits:                 12,
                queries:                  20,
                fri_layout:               vec![2, 1, 4, 2],
                constraints_degree_bound: 1,
            },
            2,
            4096
        ));
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

    // TODO: What are we actually testing here? Should we add these as debug_assert
    // to the main implementation? Should we break up the implementation so we
    // can test the individual steps?
    #[test]
    // TODO: Naming
    #[allow(non_snake_case)]
    // TODO - See if it's possible to do context cloning and break this into smaller tests
    #[allow(clippy::cognitive_complexity)]
    fn fib_proof_test() {
        let public = PublicInput {
            index: 1000,
            value: FieldElement::from(u256h!(
                "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
            )),
        };
        let private = PrivateInput {
            secret: FieldElement::from(u256h!(
                "00000000000000000000000000000000000000000000000000000000cafebabe"
            )),
        };

        let trace_len = 1024;
        let constraints = get_fibonacci_constraints(&public);
        let params = ProofParams {
            blowup:                   16,
            pow_bits:                 12,
            queries:                  20,
            fri_layout:               vec![3, 2],
            constraints_degree_bound: 1,
        };

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
        let trace = get_trace_table(1024, &private);
        assert_eq!(trace[(1000, 0)], public.value);

        let TPn = interpolate_trace_table(&trace);
        let TP0 = TPn[0].clone();
        let TP1 = TPn[1].clone();
        // Checks that the trace table polynomial interpolation is working
        assert_eq!(TP0.evaluate(&trace_x[1000]), trace[(1000, 0)]);

        let LDEn = calculate_low_degree_extensions(&TPn, params.blowup);

        // Checks that the low degree extension calculation is working
        let i = 13644usize;
        let reverse_i = i.bit_reverse_at(eval_domain_size);
        assert_eq!(TP0.evaluate(&eval_offset_x[reverse_i]), LDEn[0][i]);
        assert_eq!(TP1.evaluate(&eval_offset_x[reverse_i]), LDEn[1][i]);

        // Checks that the groupable trait is properly grouping for &[Vec<FieldElement>]
        assert_eq!(
            (LDEn.as_slice().get_leaf(3243))[0].clone(),
            u256h!("01ddd9e389a326817ad1d2a5311e1bc2cf7fa734ebdc2961085b5acfa87a58ff")
        );
        assert_eq!(
            (LDEn.as_slice().get_leaf(3243))[1].clone(),
            u256h!("03dbc6c47df0606997c2cefb20c4277caf2b76bca1d31c13432f71cdd93b3718")
        );

        let tree = LDEn.merkleize();
        // Checks that the merklelizable implementation is working [implicit check of
        // most previous steps]
        assert_eq!(
            tree[1].as_bytes(),
            hex!("018dc61f748b1a6c440827876f30f63cb6c4c188000000000000000000000000")
        );

        let mut public_input = [(public.index as u64).to_be_bytes()].concat();
        public_input.extend_from_slice(&public.value.as_montgomery().to_bytes_be());

        let mut proof = ProverChannel::new();
        proof.initialize(&public_input.as_slice());
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

        let mut constraint_coefficients = Vec::with_capacity(2 * constraints.len());
        for _ in &constraints {
            constraint_coefficients.push(proof.get_random());
            constraint_coefficients.push(proof.get_random());
        }

        let constraint_polynomial = get_constraint_polynomial(
            &TPn,
            &constraints,
            &constraint_coefficients,
            params.constraints_degree_bound,
        );
        assert_eq!(constraint_polynomial.len(), 1024);
        let CC = evalute_polynomial_on_domain(&constraint_polynomial, params.blowup);
        // Checks that our constraints are properly calculated on the domain
        assert_eq!(
            CC[123.bit_reverse_at(eval_domain_size)].clone(),
            field_element!("05b841208b357e29ac1fe7a654efebe1ae152104571e695f311a353d4d5cabfb")
        );

        let c_tree = CC.as_slice().merkleize();
        // Checks both that the merkle tree is working for this groupable type and that
        // the constraints are properly calculated on the domain
        assert_eq!(
            hex::encode(c_tree[1].as_bytes()),
            "e276ce1357d4030a4c84cdfdb4dd77845d3f80e9000000000000000000000000"
        );
        proof.write(&c_tree[1]);

        let (oods_point, oods_coefficients) =
            get_out_of_domain_information(&mut proof, &TPn, &constraint_polynomial);
        // Checks that we have derived the right out of domain sample point
        assert_eq!(
            U256::from(oods_point.clone()),
            u256h!("05d677ea387ec4ebd08ec49c414f53f569f406f51e28db2c566fdd99537a97e4")
        );
        // Checks that our get out of domain function call has written the right values
        // to the proof
        assert_eq!(
            hex::encode(proof.coin.digest),
            "c1b7a613149f857c524a724ebb54121352b9e720bf794ecebf2d78ee4e3f938b"
        );

        let CO = calculate_fri_polynomial(
            &TPn,
            &constraint_polynomial,
            &oods_point,
            &oods_coefficients,
            params.blowup,
        );
        // Checks that our out of domain evaluated constraints calculated right
        assert_eq!(
            CO[4321.bit_reverse_at(eval_domain_size)].clone(),
            field_element!("03c6b730c58b55f44bbf3cb7ea82b2e6a0a8b23558e908b5466dfe42e821ee96")
        );

        let (fri_layers, fri_trees) =
            perform_fri_layering(CO.as_slice(), &mut proof, &params, eval_x.as_slice());

        // Checks that the first fri merkle tree root is right
        assert_eq!(
            hex::encode(fri_trees[0][1].as_bytes()),
            "620a934880b6c7d893acf17a21cc9c10058a7add000000000000000000000000"
        );
        // Checks that the second fri merkle tree root is right
        assert_eq!(
            hex::encode(fri_trees[1][1].as_bytes()),
            "effd58adf9f2dac6bfd338772d0d7750c0c6f8b2000000000000000000000000"
        );
        // Checks that the fri layering function decommited the right values.
        assert_eq!(
            hex::encode(proof.coin.digest),
            "3c6cecef72873e7d73933e73279d36ca77c5a0c7497311eba735722549238334"
        );

        let proof_of_work = proof.pow_find_nonce(params.pow_bits);
        // Checks that the pow function is working [may also fail if the previous steps
        // have perturbed the channel's random]
        assert_eq!(proof_of_work, 281);
        proof.write(proof_of_work);

        let query_indices = get_indices(
            params.queries,
            64 - eval_domain_size.leading_zeros() - 1,
            &mut proof,
        );
        // Checks that the get query_indices is working
        assert_eq!(query_indices[19], 16377);

        decommit_with_queries_and_proof(
            query_indices.as_slice(),
            LDEn.as_slice(),
            tree.as_slice(),
            &mut proof,
        );
        // Checks that our first decommitment is successful
        assert_eq!(
            hex::encode(proof.coin.digest),
            "c0bf8d8ba4d15bd0e73892e3d6e90bd4f477f9135a7be39ba7e9471e6ac68a44"
        );

        decommit_with_queries_and_proof(
            query_indices.as_slice(),
            CC.as_slice(),
            c_tree.as_slice(),
            &mut proof,
        );
        // Checks that our second decommitment is successful
        assert_eq!(
            hex::encode(proof.coin.digest),
            "f2d3e6593dc23fa32655040ad5023739e15fff1d645bb809467cfccb676d6343"
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
            hex::encode(proof.coin.digest),
            "fcf1924f84656e5068ab9cbd44ae084b235bb990eefc0fd0183c77d5645e830e"
        );
    }
}
