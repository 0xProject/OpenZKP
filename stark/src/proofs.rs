use crate::{
    channel::{ProverChannel, RandomGenerator, Writable},
    constraint::Constraint,
    fft::{fft_cofactor_bit_reversed, ifft},
    hash::Hash,
    hashable::Hashable,
    mmap_vec::MmapVec,
    polynomial::{DensePolynomial, SparsePolynomial},
    proof_params::ProofParams,
    utils::Reversible,
    MerkleTree, TraceTable,
};
use itertools::{izip, Itertools};
use primefield::FieldElement;
use rayon::prelude::*;
use std::{
    marker::{Send, Sync},
    prelude::v1::*,
    vec,
};
use u256::U256;

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

// TODO: Look into lifetime annotations here. For now ignore the hint.
#[allow(single_use_lifetimes)]
pub fn stark_proof<Public>(
    trace: &TraceTable,
    constraints: &[Constraint],
    public: &Public,
    params: &ProofParams,
) -> ProverChannel
where
    for<'a> ProverChannel: Writable<&'a Public>,
    for<'b> ProverChannel: Writable<&'b Hash>,
{
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
    let tree = MerkleTree::new(trace_lde.as_slice());
    proof.write(&tree.root());

    // 2. Constraint commitment
    //

    // Read constraint coefficients from the channel.
    let mut constraint_coefficients = Vec::with_capacity(2 * constraints.len());
    for _ in constraints {
        constraint_coefficients.push(proof.get_random());
        constraint_coefficients.push(proof.get_random());
    }

    let constraint_polynomials = get_constraint_polynomials(
        &trace_polynomials,
        constraints,
        &constraint_coefficients,
        params.constraints_degree_bound,
    );

    let constraint_lde = calculate_low_degree_extensions(&constraint_polynomials, params.blowup);
    // Construct a merkle tree over the LDE combined constraints
    // and write the root to the channel.
    let c_tree = MerkleTree::new(constraint_lde.as_slice());
    proof.write(&c_tree[1]);

    // 3. Out of domain sampling
    //

    // Read the out of domain sampling point from the channel.
    // (and do a bunch more things)
    // TODO: expand
    let (oods_point, oods_coefficients) =
        get_out_of_domain_information(&mut proof, &trace_polynomials, &constraint_polynomials);

    // Divide out the OODS points from the constraints and combine.
    let oods_polynomial = calculate_fri_polynomial(
        &trace_polynomials,
        &constraint_polynomials,
        &oods_point,
        &oods_coefficients,
    );

    // 4. FRI layers
    let (fri_layers, fri_trees) = perform_fri_layering(&oods_polynomial, &mut proof, &params);

    // 5. Proof of work
    let proof_of_work = proof.pow_find_nonce(params.pow_bits);
    debug_assert!(&proof.pow_verify(proof_of_work, params.pow_bits));
    proof.write(proof_of_work);

    // 6. Query decommitments
    //

    // Fetch query indices from channel.
    let eval_domain_size = trace.num_rows() * params.blowup;
    let query_indices = get_indices(
        params.queries,
        64 - eval_domain_size.leading_zeros() - 1,
        &mut proof,
    );

    // Decommit the trace table values.
    decommit_with_queries_and_proof(
        query_indices.as_slice(),
        &trace_lde.as_slice(),
        tree.as_slice(),
        &mut proof,
    );

    // Decommit the constraint values
    decommit_with_queries_and_proof(
        query_indices.as_slice(),
        &constraint_lde.as_slice(),
        c_tree.as_slice(),
        &mut proof,
    );

    // Decommit the FRI layer values
    decommit_fri_layers_and_trees(
        fri_layers.as_slice(),
        &fri_trees.as_slice(),
        query_indices.as_slice(),
        &params,
        &mut proof,
    );

    // Q.E.D.
    proof
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

pub fn calculate_low_degree_extensions(
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

// TODO: shift polynomial by FieldElement::GENERATOR outside of this function.
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

pub fn get_constraint_polynomials(
    trace_polynomials: &[DensePolynomial],
    constraints: &[Constraint],
    constraint_coefficients: &[FieldElement],
    constraints_degree_bound: usize,
) -> Vec<DensePolynomial> {
    let mut constraint_polynomial =
        DensePolynomial::new(&vec![FieldElement::ZERO; constraints_degree_bound]);
    let trace_length = trace_polynomials[0].len();
    for (i, constraint) in constraints.iter().enumerate() {
        let mut p = (constraint.base)(trace_polynomials);
        let mut base_length = p.len();
        if base_length > trace_length {
            base_length -= 1;
        }
        p *= constraint.numerator.clone();
        p /= constraint.denominator.clone();
        constraint_polynomial += &(&constraint_coefficients[2 * i] * &p);
        let adjustment_degree = constraints_degree_bound * trace_length - base_length
            + constraint.denominator.degree()
            - constraint.numerator.degree();
        p *= SparsePolynomial::new(&[(FieldElement::ONE, adjustment_degree)]);
        constraint_polynomial += &constraint_coefficients[2 * i + 1] * &p;
    }

    let mut constraint_polynomials: Vec<Vec<FieldElement>> = vec![vec![]; constraints_degree_bound];
    for chunk in constraint_polynomial
        .coefficients()
        .chunks_exact(constraints_degree_bound)
    {
        for (i, coefficient) in chunk.iter().enumerate() {
            constraint_polynomials[i].push(coefficient.clone());
        }
    }
    constraint_polynomials
        .iter()
        .map(|x| DensePolynomial::new(x))
        .collect()
}

fn get_out_of_domain_information(
    proof: &mut ProverChannel,
    trace_polynomials: &[DensePolynomial],
    constraint_polynomials: &[DensePolynomial],
) -> (FieldElement, Vec<FieldElement>) {
    let oods_point: FieldElement = proof.get_random();

    let g = FieldElement::root(trace_polynomials[0].len())
        .expect("No root for trace polynomial length.");
    let oods_point_g = &oods_point * &g;
    let mut oods_values = Vec::with_capacity(2 * trace_polynomials.len() + 10);
    for trace_polynomial in trace_polynomials {
        let mut evaled = trace_polynomial.evaluate(&oods_point);
        oods_values.push(evaled.clone());
        evaled = trace_polynomial.evaluate(&oods_point_g);
        oods_values.push(evaled.clone());
    }
    for constraint_polynomial in constraint_polynomials {
        oods_values
            .push(constraint_polynomial.evaluate(&oods_point.pow(constraint_polynomials.len())));
    }

    for v in &oods_values {
        proof.write(v);
    }

    let mut oods_coefficients =
        Vec::with_capacity(2 * trace_polynomials.len() + constraint_polynomials.len());
    for _ in trace_polynomials {
        oods_coefficients.push(proof.get_random());
        oods_coefficients.push(proof.get_random());
    }
    for _ in constraint_polynomials {
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
    constraint_polynomials: &[DensePolynomial],
    oods_point: &FieldElement,
    oods_coefficients: &[FieldElement],
) -> DensePolynomial {
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

    let offset = 2 * trace_polynomials.len();
    let constraints_degree_bound = constraint_polynomials.len();
    for (i, constraint_polynomial) in constraint_polynomials.iter().enumerate() {
        fri_polynomial += &oods_coefficients[offset + i]
            * &divide_out_point(
                constraint_polynomial,
                &oods_point.pow(constraints_degree_bound),
            );
    }
    fri_polynomial
}

fn fri_fold(p: &DensePolynomial, c: &FieldElement) -> DensePolynomial {
    // TODO: don't shift and unshift in this function.
    let shifted = p.shift(&FieldElement::GENERATOR);
    let coefficients: Vec<FieldElement> = shifted
        .coefficients()
        .chunks_exact(2)
        .map(|pair: &[FieldElement]| (&pair[0] + c * &pair[1]).double())
        .collect();
    DensePolynomial::new(&coefficients).shift(
        &FieldElement::GENERATOR
            .inv()
            .expect("Generator cannot be zero."),
    )
}

fn perform_fri_layering(
    fri_polynomial: &DensePolynomial,
    proof: &mut ProverChannel,
    params: &ProofParams,
) -> (Vec<Vec<FieldElement>>, Vec<Vec<Hash>>) {
    let mut fri_trees: Vec<Vec<Hash>> = Vec::with_capacity(params.fri_layout.len());
    let mut fri_layers: Vec<Vec<FieldElement>> = Vec::with_capacity(params.fri_layout.len());

    // TODO: fold fri_polynomial without cloning it first.
    let mut p = fri_polynomial.clone();
    for &n_reductions in &params.fri_layout {
        let layer = evalute_polynomial_on_domain(&p, params.blowup).to_vec();
        // FRI layout values are small.
        #[allow(clippy::cast_possible_truncation)]
        let tree = (2_usize.pow(n_reductions as u32), layer.as_slice()).merkleize();
        proof.write(&tree[1]);
        fri_trees.push(tree);
        fri_layers.push(layer);

        let mut coefficient = proof.get_random();
        for _ in 0..n_reductions {
            p = fri_fold(&p, &coefficient);
            coefficient = coefficient.square();
        }
    }
    proof.write(p.shift(&FieldElement::GENERATOR).coefficients());
    (fri_layers, fri_trees)
}

fn decommit_with_queries_and_proof<R: Hashable, T: Groupable<R>>(
    queries: &[usize],
    source: &T,
    tree: &[Hash],
    proof: &mut ProverChannel,
) where
    ProverChannel: Writable<R>,
{
    // TODO
    // for &index in queries.iter() {
    // proof.write(source.get_leaf(index));
    // }
    // decommit_proof(&merkle::proof(tree, queries, source), proof);
}

// Note - This function exists because rust gets confused by the intersection of
// the write types and the others.
fn decommit_proof(decommitment: &[Hash], proof: &mut ProverChannel) {
    // TODO
    // for x in decommitment {
    // proof.write(x);
    // }
}

fn decommit_fri_layers_and_trees(
    fri_layers: &[Vec<FieldElement>],
    fri_trees: &[MerkleTree],
    query_indices: &[usize],
    params: &ProofParams,
    proof: &mut ProverChannel,
) {
    let mut previous_indices: Vec<usize> = query_indices.to_vec();

    for (layer, tree, n_reductions) in izip!(fri_layers, fri_trees, &params.fri_layout) {
        // FRI layout usizes are small.
        #[allow(clippy::cast_possible_truncation)]
        let fri_const = 2_usize.pow(*n_reductions as u32);

        let new_indices: Vec<usize> = previous_indices
            .iter()
            .map(|x| x / fri_const)
            .dedup()
            .collect();

        for i in &new_indices {
            for j in 0..fri_const {
                let n = i * fri_const + j;
                match previous_indices.binary_search(&n) {
                    Ok(_) => (),
                    _ => proof.write(&layer[n]),
                };
            }
        }

        // Write the merkle proof
        proof.write(&tree.proof(&new_indices));

        previous_indices = new_indices;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        fibonacci::{get_fibonacci_constraints, get_trace_table, PrivateInput, PublicInput},
        geometric_series::geometric_series,
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
            actual.proof.as_slice(),
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
            actual.proof.as_slice(),
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

        // Second check that the trace table function is working.
        let trace = get_trace_table(1024, &private);
        assert_eq!(trace[(1000, 0)], public.value);

        let TPn = interpolate_trace_table(&trace);
        // Checks that the trace table polynomial interpolation is working
        assert_eq!(TPn[0].evaluate(&g.pow(1000)), trace[(1000, 0)]);

        let LDEn = calculate_low_degree_extensions(&TPn, params.blowup);

        // Checks that the low degree extension calculation is working
        let i = 13644_usize;
        let reverse_i = i.bit_reverse_at(eval_domain_size);
        let eval_offset_x = geometric_series(&gen, &omega, eval_domain_size);
        assert_eq!(TPn[0].evaluate(&eval_offset_x[reverse_i]), LDEn[0][i]);
        assert_eq!(TPn[1].evaluate(&eval_offset_x[reverse_i]), LDEn[1][i]);

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

        let constraint_polynomials = get_constraint_polynomials(
            &TPn,
            &constraints,
            &constraint_coefficients,
            params.constraints_degree_bound,
        );
        assert_eq!(constraint_polynomials.len(), 1);
        assert_eq!(constraint_polynomials[0].len(), 1024);
        let CC = calculate_low_degree_extensions(&constraint_polynomials, params.blowup);
        // Checks that our constraints are properly calculated on the domain
        assert_eq!(
            CC[0][123.bit_reverse_at(eval_domain_size)].clone(),
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
            get_out_of_domain_information(&mut proof, &TPn, &constraint_polynomials);
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
            &constraint_polynomials,
            &oods_point,
            &oods_coefficients,
        );
        // Checks that our out of domain evaluated constraints calculated right
        let trace_generator = FieldElement::root(eval_domain_size).unwrap();
        assert_eq!(
            CO.evaluate(&(FieldElement::GENERATOR * trace_generator.pow(4321))),
            field_element!("03c6b730c58b55f44bbf3cb7ea82b2e6a0a8b23558e908b5466dfe42e821ee96")
        );

        let (fri_layers, fri_trees) = perform_fri_layering(&CO, &mut proof, &params);

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
            &LDEn.as_slice(),
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
            &CC.as_slice(),
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
