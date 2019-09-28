use crate::{
    verify,
    algebraic_dag::AlgebraicGraph,
    channel::{ProverChannel, RandomGenerator, Writable},
    constraint_system::{Provable, Verifiable},
    constraints::Constraints,
    polynomial::DensePolynomial,
    proof_of_work,
    proof_params::ProofParams,
    TraceTable,
};
use hash::{Hash, Hashable, MaskedKeccak};
use itertools::Itertools;
use log::info;
use merkle_tree::{Tree, VectorCommitment};
use mmap_vec::MmapVec;
use primefield::{
    fft::{ifft_permuted, permute, permute_index},
    geometric_series::geometric_series,
    FieldElement,
};
use rayon::prelude::*;
use std::{prelude::v1::*, vec};
use u256::U256;

#[derive(Clone, Debug)]
struct PolyLDE(Vec<MmapVec<FieldElement>>);

/// Merkle trees over trace table LDE and constraint LDE
// Clippy false positive
#[allow(clippy::use_self)]
impl VectorCommitment for PolyLDE {
    // TODO: Copy free implementation. Maybe have index as a leaf type.
    type Leaf = Vec<U256>;

    fn len(&self) -> usize {
        self.0.first().map_or(0, MmapVec::len)
    }

    fn leaf(&self, index: usize) -> Self::Leaf {
        let mut ret = Vec::with_capacity(self.0.len());
        for item in &self.0 {
            ret.push(item[index].as_montgomery().clone())
        }
        ret
    }

    fn leaf_hash(&self, index: usize) -> Hash {
        if self.0.len() == 1 {
            // For a single element, return its hash.
            self.0[0][index].hash()
        } else {
            // Concatenate the element hashes and hash the result.
            let mut hasher = MaskedKeccak::new();
            for value in &self.0 {
                hasher.update(value[index].hash().as_bytes());
            }
            hasher.hash()
        }
    }
}

#[derive(Clone, Debug)]
struct FriLeaves {
    coset_size: usize,
    layer:      MmapVec<FieldElement>,
}

type FriTree = Tree<FriLeaves>;

// Merkle tree for FRI layers with coset size
impl VectorCommitment for FriLeaves {
    type Leaf = Vec<U256>;

    fn len(&self) -> usize {
        debug_assert_eq!(self.layer.len() % self.coset_size, 0);
        self.layer.len() / self.coset_size
    }

    fn leaf(&self, index: usize) -> Self::Leaf {
        let mut internal_leaf = Vec::with_capacity(self.coset_size);
        for j in 0..self.coset_size {
            internal_leaf.push(
                self.layer[(index * self.coset_size + j)]
                    .as_montgomery()
                    .clone(),
            );
        }
        internal_leaf
    }

    fn leaf_hash(&self, index: usize) -> Hash {
        if self.coset_size == 1 {
            // For a single element, return its hash.
            self.layer[index].hash()
        } else {
            // Concatenate the element hashes and hash the result.
            let mut hasher = MaskedKeccak::new();
            for j in 0..self.coset_size {
                hasher.update(self.layer[(index * self.coset_size + j)].hash().as_bytes());
            }
            hasher.hash()
        }
    }
}
// False positive: `for<'a>` annotation is required.
#[allow(single_use_lifetimes)]
// TODO: Simplify
#[allow(clippy::cognitive_complexity)]
pub fn stark_proof<Claim: Verifiable, Witness: Provable<Claim>>(
    claim: &Claim,
    witness: &Witness,
    params: &ProofParams,
) -> ProverChannel
where
    for<'a> &'a Claim: Into<Vec<u8>>,
{
    let trace = witness.trace(claim);
    let constraints = claim.constraints();
    proof(&claim.into(), &constraints, &trace, params)
}

// False positives on the Latex math.
#[allow(clippy::doc_markdown)]
/// # Produce a Stark proof.
///
/// ## Input
///
/// A `ConstraintSystem` which captures the claim that is made.
/// A `TraceTable` which is the witness to this claim.
/// A `ProofParams` object which configures the proof.
///
/// ## Output
///
/// A `ProverChannel`.
///
/// ## Proof construction
///
/// A new `ProverChannel` is initialized with the public input.
///
/// ### Step 1: Low degree extension of the trace table.
///
/// The trace table is interpolated to an evaluation domain that is larger by a
/// factor `params.blowup`. It is also offset by a cofactor (currently fixed to
/// the default generator of the field, `3`).
///
/// $$
/// T_{i, j} = P_j(\omega_{\text{trace}}^i)
/// $$
///
/// <!-- TODO: Introduce trace table -->
///
/// A merkle tree is constructed over this evaluation domain and commited to the
/// channel.
///
/// $$
/// \text{Leaf}_i = (T_0(x_i), T_1(x_i), \dots )
/// $$
///
/// where $x_i = 3 \cdot \omega_{\mathrm{lde}}^i$.
///
/// <!-- TODO: The indices should be bit-reversed. -->
///
/// ### Step 2: Constraint commitment
///
/// For each constraint, two random value $\alpha_i$ and $\beta_i$ are drawn
/// from the channel. The constraints are combined as
///
/// $$
/// C(x) = \sum_i (\alpha_i + \beta_i \cdot x^{d_i}) \cdot C_i(x)
/// $$
///
/// <!-- TODO: Introduce constraints -->
///
/// where $d_i$ is the adjustment degree,
///
/// $$
/// d_i = \mathrm{target\\_degree} - \deg C_i
/// $$
///
/// The adjustment degrees are there to prevent make sure that the final
/// polynomial is a sum of all constraint polynomials aligned on the lowest
/// coefficient, and on the highest coefficient. This guarantees that constraint
/// degrees are enforced exactly. (Non-enforcement on the low end would mean a
/// term of negative degree $x^{-1}$ would be accepted).
///
/// <!-- TODO: Introduce target degree -->
///
/// The resulting polynomial $C$ is now split in $\mathrm{d}$ polynomials such
/// that
///
/// $$
/// C(x) = A_0(x^{\mathrm{d}}) + x \cdot A_1(x^{\mathrm{d}}) + x^2 \cdot
/// A_2(x^{\mathrm{d}}) + \cdots + x^{{\mathrm{d}} -1}\cdot
/// A_{\mathrm{d}}(x^{\mathrm{d}})
/// $$
///
/// where $\deg A_i \leq \text{trace\_length}$.
///
/// For a linear constraint system this does nothing and we have $A_0 = C$, for
/// a quadratic constraint system $A_0$ and $A_1$ contain all the odd and even
/// coefficients of $C$ respectively.
///
/// A merkle tree is constructed over the LDE values of the $A$ polynomials and
/// commited to the channel.
///
/// $$
/// \text{Leaf}_i = (A_0(x_i), A_1(x_i), \dots )
/// $$
///
/// ### Step 3: Divide out the deep points and combine
///
/// A random value $z$ is drawn from the channel.
///
/// For each trace polynomial, $T_i(z)$ and $T_i(\omega \cdot z)$ are written to
/// the proof. For each combined constraint polynomial, $A_i(z^{\mathrm{d}})$ is
/// written to the proof.
///
/// The points are then divided out of the polynomials, with each trace
/// polynomial being treated twice:
///
/// $$
/// T_i'(x) = \frac{T_i(x) - T_i(z)}{x - z}
/// $$
///
/// $$
/// T_i''(x) = \frac{T_i(x) - T_i(\omega \cdot z)}{x - \omega \cdot z}
/// $$
///
/// Similarly for the constraint polynomials:
///
/// $$
/// A_i'(x) = \frac{A_i(x) - A_i(z^{\mathrm{d}})}{x - z^{\mathrm{d}}}
/// $$
///
/// For each trace polynomial, two random values $\alpha_i$ and $\beta_i$ are
/// drawn from the channel. For each constraint polynomial, one random value
/// $\gamma_i$ is drawn.
///
/// All polynomial are combined in a single final polynomial:
///
/// $$
/// P(x) = \sum_i \left( \alpha_i \cdot T_i'(x) + \beta_i \cdot T_i''(x)\right)
/// + \sum_i \gamma_i \cdot A_i'(x) $$
///
/// <!-- TODO: Mention degree bounds on polynomials. Wouldn't this be -2 because
/// of the divided out points? -->
///
/// ### Step 4: Create FRI layers
///
/// The final polynomial $P$ is evaluated on the LDE domain. A Merkle tree is
/// constructed of these values and committed to the proof.
///
/// A random value $\alpha$ is drawn from the channel. Take $P_0$ to be our
/// final polynomial, then
///
/// $$
/// P_{i+1}(x^2) = \left( P_i(x) + P_i(-x) \right) + \frac{\alpha}{x} \left(
/// P_i(x) - P_i(-x) \right)
/// $$
///
/// This is the same as taking all the odd coefficients, multiplying them by
/// $\alpha$ and adding them to the even coefficients.
///
/// This reduction step can be repeated using $\alpha^2, \alpha^4, \dots$
/// instead of $\alpha$. Once sufficient reductions are made, a new Merkle tree
/// is constructed, committed too, a new random value $\alpha$ is drawn and the
/// FRI layering process repeats.
///
/// The number of reduction steps in between each commitment is specified using
/// the `params.fri_layout` parameter. The default recommendation is to do three
/// reductions between each layer, as this optimizes proof size.
///
/// Once the degree of the polynomial is sufficiently low, it is written to the
/// channel in coefficient form.
///
/// ### Step 5: Proof of work
///
/// A random challenge is drawn from the channel and a proof of work is solved.
/// The solution is written to the channel. The difficulty is specified by the
/// `params.pow_bits` parameter.
///
/// ### Step 6: Decommit queries
///
/// Random values $x_i$ from the LDE domain are drawn from the channel to form
/// our queries. The total number of queries is specified by `params.queries`.
/// The values are sorted.
///
/// <!-- TODO: Sorted by bit-reversed index -->
///
/// The trace polynomial values at the query locations are written to the
/// channel:
///
/// $$
/// T_0(x_0), T_1(x_0), \dots, T_0(x_1), T_1(x_1), \dots
/// $$
///
/// A merkle proof is provided linking these values to the earlier commitment.
///
/// Similarly, the combined constraint polynomial values are written to the
/// channel:
///
/// $$
/// A_0(x_0), A_1(x_0), \dots, A_0(x_1), A_1(x_1), \dots
/// $$
///
/// And again a merkle proof is provided linking these values to the earlier
/// commitment.
///
/// Then the values of the final polynomial are provided:
///
/// $$
/// P(x_0), P(x_1), \dots
/// $$
///
/// the merkle proof for these values links them to the commitment at the start
/// of the FRI layer.
///
/// Now the set of points $x_i$ is squared while maintaining the sorted order.
/// Duplicate points are removed. This is repeated unit we reach the reduction
/// for the next FRI commitment.
///
/// Values for the next committed FRI layer are provided:
///
/// $$
/// P_i(x_0), P_i(x_1), \dots
/// $$
///
/// with merkle proofs to that layer. This process is repeated for all FRI layer
/// commitments.
// TODO: Simplify
#[allow(clippy::cognitive_complexity)]
pub fn proof(
    channel_seed: &[u8],
    constraints: &Constraints,
    trace: &TraceTable,
    parameters: &ProofParams,
) -> ProverChannel {
    let params = parameters;

    // TODO: Verify input
    //  * Constraint trace length matches trace table length
    //  * Fri layout is less than trace length * blowup
    //  * Trace(_, _) items in constraint are valid.
    //  * Trace table satisfies constraints (expensive check, should be optional)

    info!("Starting Stark proof.");
    info!("Proof parameters: {:?}", params);
    // TODO: Use a proper size human formating function
    #[allow(clippy::cast_precision_loss)]
    let size_mb = (trace.num_rows() * trace.num_columns() * 32) as f64 / 1_000_000_f64;
    info!(
        "Trace table {} rows {} columns ({} MB)",
        trace.num_rows(),
        trace.num_columns(),
        size_mb
    );
    info!("{} constraints", constraints.len(),);

    info!("Initialize channel with claim.");
    let mut proof = ProverChannel::new();
    proof.initialize(channel_seed);

    // 1. Trace commitment.

    // Compute the low degree extension of the trace table.
    info!("Compute the low degree extension of the trace table.");
    let trace_polynomials = trace.interpolate();
    info!(
        "Trace degrees: {:?}",
        trace_polynomials
            .iter()
            .map(DensePolynomial::degree)
            .collect::<Vec<_>>()
    );
    let trace_lde = PolyLDE(
        trace_polynomials
            .par_iter()
            .map(|p| p.low_degree_extension(params.blowup))
            .collect::<Vec<_>>(),
    );

    // Construct a merkle tree over the LDE trace
    // and write the root to the channel.
    info!("Construct a merkle tree over the LDE trace and write the root to the channel.");
    let (commitment, tree) = trace_lde.commit().unwrap();
    proof.write(&commitment);

    // 2. Constraint commitment

    // Read constraint coefficients from the channel.
    info!("Read constraint coefficients from the channel.");
    let mut constraint_coefficients = Vec::with_capacity(2 * constraints.len());
    for _ in 0..constraints.len() {
        constraint_coefficients.push(proof.get_random());
        constraint_coefficients.push(proof.get_random());
    }

    info!("Compute constraint polynomials.");
    let constraint_polynomials = get_constraint_polynomials(
        &tree.leaves(),
        &constraints,
        &constraint_coefficients,
        trace.num_rows(),
    );
    info!(
        "Constraint degrees: {:?}",
        constraint_polynomials
            .iter()
            .map(DensePolynomial::degree)
            .collect::<Vec<_>>()
    );

    // OPT: It may be faster to compute the constraint LDE from the trace LDE,
    // instead of using an FFT.
    info!("Compute the low degree extension of constraint polynomials.");
    let constraint_lde = PolyLDE(
        constraint_polynomials
            .par_iter()
            .map(|p| p.low_degree_extension(params.blowup))
            .collect::<Vec<_>>(),
    );
    // Construct a merkle tree over the LDE combined constraints
    // and write the root to the channel.
    info!("Compute the merkle tree over the LDE constraint polynomials.");
    let (commitment, c_tree) = constraint_lde.commit().unwrap();
    proof.write(&commitment);

    // 3. Out of domain sampling
    info!("Divide out OODS point and combine polynomials.");
    let oods_polynomial = oods_combine(&mut proof, &trace_polynomials, &constraint_polynomials);
    info!("Oods poly degree: {}", oods_polynomial.degree());

    // 4. FRI layers with trees
    info!("FRI layers with trees.");
    let first_fri_layer = oods_polynomial.low_degree_extension(params.blowup);
    let fri_trees = perform_fri_layering(first_fri_layer, &mut proof, &params);

    // 5. Proof of work
    info!("Proof of work.");
    let pow_seed: proof_of_work::ChallengeSeed = proof.get_random();
    let pow_challenge = pow_seed.with_difficulty(params.pow_bits);
    let pow_response = pow_challenge.solve();
    debug_assert!(pow_challenge.verify(pow_response));
    proof.write(pow_response);

    // 6. Query decommitments
    //

    // Fetch query indices from channel.
    info!("Fetch query indices from channel.");
    let eval_domain_size = trace.num_rows() * params.blowup;
    let query_indices = get_indices(
        params.queries,
        64 - eval_domain_size.leading_zeros() - 1,
        &mut proof,
    );
    info!("Query indices: {:?}", query_indices);

    // Decommit the trace table values.
    info!("Decommit the trace table values.");
    for &index in &query_indices {
        proof.write(tree.leaf(index));
    }
    proof.write(&tree.open(&query_indices).unwrap());

    // Decommit the constraint values
    info!("Decommit the constraint values.");
    for &index in &query_indices {
        proof.write(c_tree.leaf(index));
    }
    proof.write(&c_tree.open(&query_indices).unwrap());

    // Decommit the FRI layer values
    info!("Decommit the FRI layer values.");
    decommit_fri_layers_and_trees(fri_trees.as_slice(), query_indices.as_slice(), &mut proof);

    // Verify proof
    info!("Verify proof.");
    // TODO - Bubble up errors so we can see where verification fails.
    assert!(verify(channel_seed, proof.proof.as_slice(), constraints, params,
        (trace.num_rows(), trace.num_columns())
    ).is_ok());

    // Q.E.D.
    // TODO: Return bytes, or a result structure
    proof
}

fn extract_trace_coset(trace_lde: &PolyLDE, size: usize) -> TraceTable {
    let trace_lde: &[MmapVec<FieldElement>] = &trace_lde.0;
    let lde_size = trace_lde[0].len();
    let mut trace_coset = TraceTable::new(size, trace_lde.len());
    // OPT: Benchmark with flipped order of loops
    for i in 0..trace_coset.num_rows() {
        for j in 0..trace_coset.num_columns() {
            let lde = &trace_lde[j];
            let index = i * lde_size / size;
            let index = permute_index(lde.len(), index);
            trace_coset[(i, j)] = lde[index].clone();
        }
    }
    trace_coset
}

fn get_indices(num: usize, bits: u32, proof: &mut ProverChannel) -> Vec<usize> {
    let mut query_indices = Vec::with_capacity(num + 3);
    while query_indices.len() < num {
        let val: U256 = proof.get_random();
        let mask = 2_usize.pow(bits) - 1;
        query_indices.push((val.clone() >> (0x100 - 0x040)).as_usize() & mask);
        query_indices.push((val.clone() >> (0x100 - 0x080)).as_usize() & mask);
        query_indices.push((val.clone() >> (0x100 - 0x0C0)).as_usize() & mask);
        query_indices.push(val.as_usize() & mask);
    }
    query_indices.truncate(num);
    (&mut query_indices).sort_unstable();
    query_indices
}

fn get_constraint_polynomials(
    trace_lde: &PolyLDE,
    constraints: &Constraints,
    constraint_coefficients: &[FieldElement],
    trace_length: usize,
) -> Vec<DensePolynomial> {
    // OPT: Better parallelization strategies. Probably the best would be to
    // split to domain up in smaller cosets and solve their expressions
    // independently. This will make all periods and therefore lookup tables
    // smaller.
    const CHUNK_SIZE: usize = 65536;

    // We need to evaluate on a power of two degree
    let constraint_degree = constraints.degree();
    let eval_degree = constraint_degree.next_power_of_two();
    let coset_size = trace_length * eval_degree;

    info!("Compute offset trace table");
    let trace_coset = extract_trace_coset(trace_lde, coset_size);

    info!("Combine rational expressions");
    let combined_constraints = constraints.combine(constraint_coefficients);
    let mut dag = AlgebraicGraph::new(
        &FieldElement::GENERATOR,
        trace_coset.num_rows(),
        eval_degree,
    );
    let result = dag.expression(combined_constraints);
    dag.lookup_tables();
    // TODO: Track and use result reference.
    let _ = dag.tree_shake(result);
    dag.init(0);

    // Evaluate on the coset trace table
    info!("Evaluate on the coset trace table");
    let mut result: MmapVec<FieldElement> = MmapVec::with_capacity(coset_size);
    result.resize(coset_size, FieldElement::ZERO);
    let values = &mut result;
    values
        .par_chunks_mut(CHUNK_SIZE)
        .enumerate()
        .for_each(|(mut i, chunk)| {
            i *= CHUNK_SIZE;
            let mut dag = dag.clone();
            dag.init(i);
            for value in chunk {
                *value = dag.next(&trace_coset);
                i += 1;
            }
        });

    info!("Convert from values to coefficients");
    ifft_permuted(values);
    permute(values);
    // OPT: Merge with even-odd separation loop.
    for (f, y) in geometric_series(&FieldElement::ONE, &FieldElement::GENERATOR.inv().unwrap())
        .zip(values.iter_mut())
    {
        // Shift out the generator from the evaluation domain.
        *y *= &f;
    }

    // Convert to even and odd coefficient polynomials
    let mut constraint_polynomials: Vec<MmapVec<FieldElement>> =
        vec![MmapVec::with_capacity(trace_length); constraint_degree];
    let (coefficients, zeros) = values.split_at(constraint_degree * trace_length);
    assert!(zeros.iter().all(|z| z == &FieldElement::ZERO));
    for chunk in coefficients.chunks_exact(constraint_degree) {
        for (i, coefficient) in chunk.iter().enumerate() {
            constraint_polynomials[i].push(coefficient.clone());
        }
    }
    constraint_polynomials
        .into_iter()
        .map(DensePolynomial::from_mmap_vec)
        .collect()
}

fn oods_combine(
    proof: &mut ProverChannel,
    trace_polynomials: &[DensePolynomial],
    constraint_polynomials: &[DensePolynomial],
) -> DensePolynomial {
    // Fetch the oods sampling point
    let trace_length = trace_polynomials[0].len();
    let oods_point: FieldElement = proof.get_random();
    let g = FieldElement::root(trace_length).expect("No root for trace polynomial length.");
    let oods_point_g = &oods_point * &g;
    let oods_point_pow = oods_point.pow(constraint_polynomials.len());

    // Write point evaluations to proof
    // OPT: Parallelization
    for trace_polynomial in trace_polynomials {
        proof.write(&trace_polynomial.evaluate(&oods_point));
        proof.write(&trace_polynomial.evaluate(&oods_point_g));
    }
    for constraint_polynomial in constraint_polynomials {
        proof.write(&constraint_polynomial.evaluate(&oods_point_pow));
    }

    // Read coefficients
    let n_coefficients = 2 * trace_polynomials.len() + constraint_polynomials.len();
    let mut oods_coefficients: Vec<FieldElement> = Vec::with_capacity(n_coefficients);
    for _ in 0..n_coefficients {
        oods_coefficients.push(proof.get_random());
    }
    let (trace_coefficients, constraint_coefficients) =
        oods_coefficients.split_at(2 * trace_polynomials.len());

    // Divide out points and linear sum the polynomials
    // OPT: Parallelization
    let mut combined_polynomial = DensePolynomial::zeros(trace_length);
    for (trace_polynomial, (coefficient_0, coefficient_1)) in trace_polynomials
        .iter()
        .zip(trace_coefficients.iter().tuples())
    {
        trace_polynomial.divide_out_point_into(
            &oods_point,
            coefficient_0,
            &mut combined_polynomial,
        );
        trace_polynomial.divide_out_point_into(
            &oods_point_g,
            coefficient_1,
            &mut combined_polynomial,
        );
    }
    for (constraint_polynomial, coefficient) in constraint_polynomials
        .iter()
        .zip(constraint_coefficients.iter())
    {
        constraint_polynomial.divide_out_point_into(
            &oods_point_pow,
            coefficient,
            &mut combined_polynomial,
        );
    }
    combined_polynomial
}

fn perform_fri_layering(
    first_layer: MmapVec<FieldElement>,
    proof: &mut ProverChannel,
    params: &ProofParams,
) -> Vec<FriTree> {
    let mut fri_trees: Vec<FriTree> = Vec::with_capacity(params.fri_layout.len());

    // Compute 1/x for the fri layer. We only compute the even coordinates.
    // OPT: Can these be efficiently computed on the fly?
    let x_inv = {
        let n = first_layer.len();
        let root_inv = FieldElement::root(n).unwrap().inv().unwrap();
        let mut x_inv = MmapVec::with_capacity(n / 2);
        let mut accumulator = FieldElement::ONE;
        for _ in 0..n / 2 {
            x_inv.push(accumulator.clone());
            accumulator *= &root_inv;
        }
        permute(&mut x_inv);
        x_inv
    };

    let mut next_layer = first_layer;
    for &n_reductions in &params.fri_layout {
        // Allocate next and swap ownership
        let mut layer = MmapVec::with_capacity(next_layer.len() / (1 << n_reductions));
        std::mem::swap(&mut layer, &mut next_layer);

        // Create tree from layer
        // FRI layout values are small.
        #[allow(clippy::cast_possible_truncation)]
        let coset_size = 2_usize.pow(n_reductions as u32);
        let tree = FriTree::from_leaves(FriLeaves { coset_size, layer }).unwrap();
        fri_trees.push(tree);
        let tree = fri_trees.last().unwrap();
        let layer = &tree.leaves().layer;

        // Write commitment and pull coefficient
        proof.write(tree.commitment());
        let coefficient = proof.get_random();

        // Fold layer up to three times
        // TODO: Capture the pattern in a macro and DRY.
        // OPT: Parallelization
        // OPT: The structure in x_inv should allow faster methods,
        // like in a radix-4 and radix-8 fft.
        let layer = layer.iter();
        match n_reductions {
            1 => {
                next_layer.extend(
                    layer
                        .tuples()
                        .zip(x_inv.iter())
                        .map(|((p0, p1), x_inv)| (p0 + p1) + &coefficient * x_inv * (p0 - p1)),
                )
            }
            2 => {
                let coefficient_2 = coefficient.pow(2);
                next_layer.extend(
                    layer
                        .tuples()
                        .zip(x_inv.iter())
                        .map(|((p0, p1), x_inv)| (p0 + p1) + &coefficient * x_inv * (p0 - p1))
                        .tuples()
                        .zip(x_inv.iter())
                        .map(|((p0, p1), x_inv)| (&p0 + &p1) + &coefficient_2 * x_inv * (p0 - p1)),
                )
            }
            3 => {
                let coefficient_2 = coefficient.square();
                let coefficient_4 = coefficient_2.square();
                next_layer.extend(
                    layer
                        .tuples()
                        .zip(x_inv.iter())
                        .map(|((p0, p1), x_inv)| (p0 + p1) + &coefficient * x_inv * (p0 - p1))
                        .tuples()
                        .zip(x_inv.iter())
                        .map(|((p0, p1), x_inv)| (&p0 + &p1) + &coefficient_2 * x_inv * (p0 - p1))
                        .tuples()
                        .zip(x_inv.iter())
                        .map(|((p0, p1), x_inv)| (&p0 + &p1) + &coefficient_4 * x_inv * (p0 - p1)),
                )
            }
            // TODO: Is there a use case for 4 layer folds?
            4 => {
                let coefficient_2 = coefficient.square();
                let coefficient_4 = coefficient_2.square();
                let coefficient_8 = coefficient_4.square();
                next_layer.extend(
                    layer
                        .tuples()
                        .zip(x_inv.iter())
                        .map(|((p0, p1), x_inv)| (p0 + p1) + &coefficient * x_inv * (p0 - p1))
                        .tuples()
                        .zip(x_inv.iter())
                        .map(|((p0, p1), x_inv)| (&p0 + &p1) + &coefficient_2 * x_inv * (p0 - p1))
                        .tuples()
                        .zip(x_inv.iter())
                        .map(|((p0, p1), x_inv)| (&p0 + &p1) + &coefficient_4 * x_inv * (p0 - p1))
                        .tuples()
                        .zip(x_inv.iter())
                        .map(|((p0, p1), x_inv)| (&p0 + &p1) + &coefficient_8 * x_inv * (p0 - p1)),
                )
            }
            _ => unimplemented!(),
        };
    }

    // Write the final layer coefficients
    let n_coefficients = next_layer.len() / params.blowup;
    let points = &mut next_layer[0..n_coefficients];
    permute(points);
    ifft_permuted(points);
    permute(points);
    proof.write(&*points);

    fri_trees
}

fn decommit_fri_layers_and_trees(
    fri_trees: &[FriTree],
    query_indices: &[usize],
    proof: &mut ProverChannel,
) {
    let mut previous_indices: Vec<usize> = query_indices.to_vec();

    for tree in fri_trees {
        let coset_size = tree.leaves().coset_size;

        let new_indices: Vec<usize> = previous_indices
            .iter()
            .map(|x| x / coset_size)
            .dedup()
            .collect();

        for i in &new_indices {
            // TODO: Write entire tree.leaf(i)
            for j in 0..coset_size {
                let n = i * coset_size + j;
                match previous_indices.binary_search(&n) {
                    Ok(_) => (),
                    _ => proof.write(&tree.leaves().layer[n]),
                };
            }
        }
        proof.write(&tree.open(&new_indices).unwrap());
        previous_indices = new_indices;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{fibonacci, verifier::check_proof};
    use macros_decl::{field_element, hex, u256h};
    use primefield::{fft::permute_index, geometric_series::geometric_series};
    use u256::U256;

    #[test]
    fn starkware_fibonacci() {
        // All the constants for this tests are copied from files in
        // https://github.com/0xProject/evm-verifier/commit/9bf369139b0edc23ab7ab7e8db8164c5a05a83df.
        // Copied from solidity/contracts/fibonacci/fibonacci_private_input1.json
        let witness = fibonacci::Witness {
            secret: field_element!("83d36de9"),
        };
        // Copied from solidity/contracts/fibonacci/fibonacci_public_input1.json
        let claim = fibonacci::Claim {
            index: 1000,
            value: field_element!(
                "04d5f1f669b34fb7252d5a9d0d9786b2638c27eaa04e820b38b088057960cca1"
            ),
        };
        let actual = stark_proof(&claim, &witness, &ProofParams {
            blowup:     16,
            pow_bits:   0,
            queries:    20,
            fri_layout: vec![3, 2],
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
        let index = 1000;
        let secret = field_element!("cafebabe");
        let value = fibonacci::get_value(index, &secret);

        let witness = fibonacci::Witness { secret };
        let claim = fibonacci::Claim { index, value };

        let actual = stark_proof(&claim, &witness, &ProofParams {
            blowup:     16,
            pow_bits:   12,
            queries:    20,
            fri_layout: vec![3, 2],
        });

        assert_eq!(
            actual.coin.digest,
            hex!("fcf1924f84656e5068ab9cbd44ae084b235bb990eefc0fd0183c77d5645e830e")
        );
    }

    #[test]
    fn fib_test_1024_changed_witness() {
        let index = 1000;
        let secret = field_element!("0f00dbabe0cafebabe");
        let value = fibonacci::get_value(index, &secret);

        let witness = fibonacci::Witness { secret };
        let claim = fibonacci::Claim { index, value };

        let actual = stark_proof(&claim, &witness, &ProofParams {
            blowup: 16, /* TODO - The blowup in the fib constraints is hardcoded to 16,
                         * we should set this back to 32 to get wider coverage when
                         * that's fixed */
            pow_bits:   12,
            queries:    20,
            fri_layout: vec![3, 2],
        });

        assert!(check_proof(actual.proof.as_slice(), &claim, &ProofParams {
            blowup: 16, /* TODO - The blowup in the fib constraints is hardcoded to 16,
                         * we should set this back to 32 to get wider coverage when
                         * that's fixed */
            pow_bits:   12,
            queries:    20,
            fri_layout: vec![3, 2],
        },)
        .is_ok());
    }

    #[test]
    fn fib_test_4096() {
        let index = 4000;
        let secret = field_element!("0f00dbabe0cafebabe");
        let value = fibonacci::get_value(index, &secret);

        let witness = fibonacci::Witness { secret };
        let claim = fibonacci::Claim { index, value };

        let actual = stark_proof(&claim, &witness, &ProofParams {
            blowup:     16,
            pow_bits:   12,
            queries:    20,
            fri_layout: vec![2, 1, 4, 2],
        });

        assert!(check_proof(actual.proof.as_slice(), &claim, &ProofParams {
            blowup:     16,
            pow_bits:   12,
            queries:    20,
            fri_layout: vec![2, 1, 4, 2],
        },)
        .is_ok());
    }

    // TODO: What are we actually testing here? Should we add these as debug_assert
    // to the main implementation? Should we break up the implementation so we
    // can test the individual steps?
    #[test]
    // TODO: Refactor this code to be cleaner.
    #[allow(non_snake_case)]
    #[allow(clippy::cognitive_complexity)]
    #[allow(clippy::too_many_lines)]
    fn fib_proof_test() {
        crate::tests::init();

        let claim = fibonacci::Claim {
            index: 1000,
            value: FieldElement::from(u256h!(
                "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
            )),
        };
        let witness = fibonacci::Witness {
            secret: FieldElement::from(u256h!(
                "00000000000000000000000000000000000000000000000000000000cafebabe"
            )),
        };
        let constraints = claim.constraints();

        let trace_len = constraints.trace_nrows();
        assert_eq!(trace_len, 1024);
        let params = ProofParams {
            blowup:     16,
            pow_bits:   12,
            queries:    20,
            fri_layout: vec![3, 2],
        };

        let omega =
            field_element!("0393a32b34832dbad650df250f673d7c5edd09f076fc314a3e5a42f0606082e1");
        let g = field_element!("0659d83946a03edd72406af6711825f5653d9e35dc125289a206c054ec89c4f1");
        let eval_domain_size = trace_len * params.blowup;
        let gen = FieldElement::GENERATOR;

        // Second check that the trace table function is working.
        let trace = witness.trace(&claim);
        assert_eq!(trace[(1000, 0)], claim.value);

        let TPn = trace.interpolate();
        // Checks that the trace table polynomial interpolation is working
        assert_eq!(TPn[0].evaluate(&g.pow(1000)), trace[(1000, 0)]);

        let LDEn = PolyLDE(
            TPn.par_iter()
                .map(|p| p.low_degree_extension(params.blowup))
                .collect::<Vec<_>>(),
        );

        // Checks that the low degree extension calculation is working
        let i = 13644_usize;
        let reverse_i = permute_index(eval_domain_size, i);
        let eval_offset_x = geometric_series(&gen, &omega)
            .take(eval_domain_size)
            .collect::<Vec<_>>();
        assert_eq!(TPn[0].evaluate(&eval_offset_x[reverse_i]), LDEn.0[0][i]);
        assert_eq!(TPn[1].evaluate(&eval_offset_x[reverse_i]), LDEn.0[1][i]);

        // Checks that the groupable trait is properly grouping for &[Vec<FieldElement>]
        assert_eq!(
            (LDEn.leaf(3243))[0].clone(),
            u256h!("01ddd9e389a326817ad1d2a5311e1bc2cf7fa734ebdc2961085b5acfa87a58ff")
        );
        assert_eq!(
            (LDEn.leaf(3243))[1].clone(),
            u256h!("03dbc6c47df0606997c2cefb20c4277caf2b76bca1d31c13432f71cdd93b3718")
        );

        let (commitment, tree) = LDEn.commit().unwrap();
        // Checks that the merklelizable implementation is working [implicit check of
        // most previous steps]
        assert_eq!(
            commitment.hash().as_bytes(),
            hex!("018dc61f748b1a6c440827876f30f63cb6c4c188000000000000000000000000")
        );

        // TODO fix naming here!
        let mut proof_seed = [(claim.index as u64).to_be_bytes()].concat();
        proof_seed.extend_from_slice(&claim.value.as_montgomery().to_bytes_be());

        let mut proof = ProverChannel::new();
        proof.initialize(&proof_seed.as_slice());
        // Checks that the channel is inited properly
        assert_eq!(
            proof.coin.digest,
            hex!("c891a11ddbc6c425fad523a7a4aeafa505d7aa1638cfffbd5b747100bc69e367")
        );
        proof.write(tree.commitment());
        // Checks that the channel allows writing of [u8; 32] properly
        assert_eq!(
            proof.coin.digest,
            hex!("b7d80385fa0c8879473cdf987ea7970bb807aec78bb91af39a1504d965ad8e92")
        );

        let constraints = claim.constraints();
        assert_eq!(constraints.len(), 4);
        let mut constraint_coefficients = Vec::with_capacity(2 * constraints.len());
        for _ in 0..constraints.len() {
            constraint_coefficients.push(proof.get_random());
            constraint_coefficients.push(proof.get_random());
        }

        let constraint_polynomials = get_constraint_polynomials(
            &tree.leaves(),
            &constraints,
            &constraint_coefficients,
            trace.num_rows(),
        );
        assert_eq!(constraint_polynomials.len(), 1);
        assert_eq!(constraint_polynomials[0].len(), 1024);
        let CC = PolyLDE(
            constraint_polynomials
                .par_iter()
                .map(|p| p.low_degree_extension(params.blowup))
                .collect::<Vec<_>>(),
        );
        // Checks that our constraints are properly calculated on the domain
        assert_eq!(
            CC.0[0][permute_index(eval_domain_size, 123)].clone(),
            field_element!("05b841208b357e29ac1fe7a654efebe1ae152104571e695f311a353d4d5cabfb")
        );

        let (commitment, c_tree) = CC.commit().unwrap();
        // Checks both that the merkle tree is working for this groupable type and that
        // the constraints are properly calculated on the domain
        assert_eq!(
            hex::encode(commitment.hash().as_bytes()),
            "e276ce1357d4030a4c84cdfdb4dd77845d3f80e9000000000000000000000000"
        );
        proof.write(&commitment);

        let CO = oods_combine(&mut proof, &TPn, &constraint_polynomials);
        // Checks that our get out of domain function call has written the right values
        // to the proof
        assert_eq!(
            hex::encode(proof.coin.digest),
            "c1b7a613149f857c524a724ebb54121352b9e720bf794ecebf2d78ee4e3f938b"
        );

        // Checks that our out of domain evaluated constraints calculated right
        let trace_generator = FieldElement::root(eval_domain_size).unwrap();
        assert_eq!(
            CO.evaluate(&(FieldElement::GENERATOR * trace_generator.pow(4321))),
            field_element!("03c6b730c58b55f44bbf3cb7ea82b2e6a0a8b23558e908b5466dfe42e821ee96")
        );

        let fri_trees =
            perform_fri_layering(CO.low_degree_extension(params.blowup), &mut proof, &params);

        // Checks that the first fri merkle tree root is right
        assert_eq!(
            hex::encode(fri_trees[0].commitment().hash().as_bytes()),
            "620a934880b6c7d893acf17a21cc9c10058a7add000000000000000000000000"
        );
        // Checks that the second fri merkle tree root is right
        assert_eq!(
            hex::encode(fri_trees[1].commitment().hash().as_bytes()),
            "effd58adf9f2dac6bfd338772d0d7750c0c6f8b2000000000000000000000000"
        );
        // Checks that the fri layering function decommited the right values.
        assert_eq!(
            hex::encode(proof.coin.digest),
            "3c6cecef72873e7d73933e73279d36ca77c5a0c7497311eba735722549238334"
        );

        let pow_seed: proof_of_work::ChallengeSeed = proof.get_random();
        let pow_challenge = pow_seed.with_difficulty(params.pow_bits);
        let pow_response = pow_challenge.solve();
        debug_assert!(pow_challenge.verify(pow_response));
        // Checks that the pow function is working [may also fail if the previous steps
        // have perturbed the channel's random]
        assert_eq!(pow_response.nonce(), 281);
        proof.write(pow_response);

        let query_indices = get_indices(
            params.queries,
            64 - eval_domain_size.leading_zeros() - 1,
            &mut proof,
        );
        // Checks that the get query_indices is working
        assert_eq!(query_indices[19], 16377);

        // Decommit trace table
        for &index in &query_indices {
            proof.write(tree.leaf(index))
        }
        proof.write(&tree.open(&query_indices).unwrap());

        // Checks that our first decommitment is successful
        assert_eq!(
            hex::encode(proof.coin.digest),
            "c0bf8d8ba4d15bd0e73892e3d6e90bd4f477f9135a7be39ba7e9471e6ac68a44"
        );

        // Decommit constraints poly
        for &index in &query_indices {
            proof.write(c_tree.leaf(index))
        }
        proof.write(&c_tree.open(&query_indices).unwrap());

        // Checks that our second decommitment is successful
        assert_eq!(
            hex::encode(proof.coin.digest),
            "f2d3e6593dc23fa32655040ad5023739e15fff1d645bb809467cfccb676d6343"
        );

        decommit_fri_layers_and_trees(fri_trees.as_slice(), query_indices.as_slice(), &mut proof);
        // Checks that our fri decommitment is successful
        assert_eq!(
            hex::encode(proof.coin.digest),
            "fcf1924f84656e5068ab9cbd44ae084b235bb990eefc0fd0183c77d5645e830e"
        );
    }
}
