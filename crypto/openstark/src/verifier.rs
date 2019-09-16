use crate::{
    channel::*,
    constraint::{combine_constraints, Constraint},
    polynomial::DensePolynomial,
    proof_of_work,
    proof_params::ProofParams,
};
use hash::Hash;
use merkle_tree::{Commitment, Proof};
use primefield::{
    fft::{self, ifft},
    geometric_series::root_series,
    FieldElement,
};
use std::{collections::BTreeMap, prelude::v1::*};
use u256::U256;

// False positive, for<'a> is required.
#[allow(single_use_lifetimes)]
pub fn check_proof<Public>(
    proposed_proof: &[u8],
    constraints: &[Constraint],
    public: &Public,
    params: &ProofParams,
    trace_cols: usize,
    trace_len: usize,
) -> bool
where
    for<'a> &'a Public: Into<Vec<u8>>,
{
    let eval_domain_size = trace_len * params.blowup;
    let eval_x = root_series(eval_domain_size).collect::<Vec<_>>();

    let mut channel = VerifierChannel::new(proposed_proof.to_vec());
    let bytes: Vec<u8> = public.into();
    channel.initialize(&bytes);

    // Get the low degree root commitment, and constraint root commitment
    // TODO: Make it work as channel.read()
    let low_degree_extension_root = Replayable::<Hash>::replay(&mut channel);
    let lde_commitment =
        Commitment::from_size_hash(eval_domain_size, &low_degree_extension_root).unwrap();
    let mut constraint_coefficients: Vec<FieldElement> = Vec::with_capacity(2 * constraints.len());
    for _ in constraints {
        constraint_coefficients.push(channel.get_random());
        constraint_coefficients.push(channel.get_random());
    }
    let constraint_evaluated_root = Replayable::<Hash>::replay(&mut channel);
    let constraint_commitment =
        Commitment::from_size_hash(eval_domain_size, &constraint_evaluated_root).unwrap();

    // Get the oods information from the proof and random
    let oods_point: FieldElement = channel.get_random();
    let mut oods_values: Vec<FieldElement> = Vec::with_capacity(2 * trace_cols + 1);
    for _ in 0..(2 * trace_cols + params.constraints_degree_bound) {
        oods_values.push(Replayable::<FieldElement>::replay(&mut channel));
    }
    let mut oods_coefficients: Vec<FieldElement> = Vec::with_capacity(2 * trace_cols + 1);
    for _ in 0..=2 * trace_cols {
        oods_coefficients.push(channel.get_random());
    }

    let mut fri_commitments: Vec<Commitment> = Vec::with_capacity(params.fri_layout.len() + 1);
    let mut eval_points: Vec<FieldElement> = Vec::with_capacity(params.fri_layout.len() + 1);
    let mut fri_size = eval_domain_size >> params.fri_layout[0];
    // Get first fri root:
    fri_commitments.push(
        Commitment::from_size_hash(fri_size, &Replayable::<Hash>::replay(&mut channel)).unwrap(),
    );
    // Get fri roots and eval points from the channel random
    for &x in params.fri_layout.iter().skip(1) {
        fri_size >>= x;
        // TODO: When is x equal to zero?
        let eval_point = if x == 0 {
            FieldElement::ONE
        } else {
            channel.get_random()
        };
        eval_points.push(eval_point);
        fri_commitments.push(
            Commitment::from_size_hash(fri_size, &Replayable::<Hash>::replay(&mut channel))
                .unwrap(),
        );
    }
    // Gets the last layer and the polynomial coefficients
    eval_points.push(channel.get_random());
    let last_layer_coefficient: Vec<FieldElement> =
        Replayable::<FieldElement>::replay_many(&mut channel, fri_size / params.blowup);

    // Gets the proof of work from the proof.
    let pow_seed: proof_of_work::ChallengeSeed = channel.get_random();
    let pow_challenge = pow_seed.with_difficulty(params.pow_bits);
    let pow_response = Replayable::<proof_of_work::Response>::replay(&mut channel);
    if !pow_challenge.verify(pow_response) {
        return false;
    }

    // Gets queries from channel
    let queries = get_indices(
        params.queries,
        eval_domain_size.trailing_zeros(),
        &mut channel,
    );

    // Get values and check decommitment of low degree extension
    let lde_values: Vec<(usize, Vec<U256>)> = queries
        .iter()
        .map(|&index| {
            let held = Replayable::<U256>::replay_many(&mut channel, trace_cols);
            (index, held)
        })
        .collect();
    let lde_proof_length = lde_commitment.proof_size(&queries).unwrap();
    let lde_hashes = Replayable::<Hash>::replay_many(&mut channel, lde_proof_length);
    let lde_proof = Proof::from_hashes(&lde_commitment, &queries, &lde_hashes).unwrap();
    if lde_proof.verify(&lde_values).is_err() {
        // TODO: Return Error
        return false;
    }

    // Gets the values and checks the constraint decommitment
    let constraint_values: Vec<(usize, U256)> = queries
        .iter()
        .map({ |&index| (index, Replayable::<U256>::replay(&mut channel)) })
        .collect();
    let constraint_proof_length = constraint_commitment.proof_size(&queries).unwrap();
    let constraint_hashes: Vec<Hash> =
        Replayable::<Hash>::replay_many(&mut channel, constraint_proof_length);
    let constraint_proof =
        Proof::from_hashes(&constraint_commitment, &queries, &constraint_hashes).unwrap();
    if constraint_proof.verify(&constraint_values).is_err() {
        // TODO: Return Error
        return false;
    }

    let coset_sizes = params
        .fri_layout
        .iter()
        .map(|k| 1_usize << k)
        .collect::<Vec<_>>();
    let mut fri_indices: Vec<usize> = queries
        .to_vec()
        .iter()
        .map(|x| x / coset_sizes[0])
        .collect();

    // Folded fri values from the previous layer
    let mut fri_folds: BTreeMap<usize, FieldElement> = BTreeMap::new();

    let mut previous_indices = queries.to_vec().clone();
    let mut step = 1;
    let mut len = eval_domain_size;
    for (k, commitment) in fri_commitments.iter().enumerate() {
        let mut fri_layer_values = Vec::new();

        fri_indices.dedup();
        for i in &fri_indices {
            let mut coset: Vec<FieldElement> = Vec::new();
            for j in 0..coset_sizes[k] {
                let n = i * coset_sizes[k] + j;
                if let Ok(z) = previous_indices.binary_search(&n) {
                    if k > 0 {
                        coset.push(fri_folds.get(&n).unwrap().clone());
                    } else {
                        let z_reverse = fft::permute_index(eval_domain_size, queries[z]);
                        coset.push(out_of_domain_element(
                            lde_values[z].1.as_slice(),
                            &constraint_values[z].1,
                            &eval_x[z_reverse],
                            &oods_point,
                            oods_values.as_slice(),
                            oods_coefficients.as_slice(),
                            eval_domain_size,
                            params.blowup,
                        ));
                    }
                } else {
                    coset.push(Replayable::<FieldElement>::replay(&mut channel));
                }
            }
            fri_layer_values.push((*i, coset));
        }
        // Fold and record foldings
        let mut layer_folds = BTreeMap::new();
        for (i, coset) in &fri_layer_values {
            let _old_value = layer_folds.insert(
                *i,
                fri_fold(
                    coset.as_slice(),
                    &eval_points[k],
                    step,
                    (coset_sizes[k] / 2) * i,
                    len,
                    eval_x.as_slice(),
                ),
            );
        }

        let merkle_proof_length = commitment.proof_size(&fri_indices).unwrap();
        let merkle_hashes = Replayable::<Hash>::replay_many(&mut channel, merkle_proof_length);
        let merkle_proof = Proof::from_hashes(commitment, &fri_indices, &merkle_hashes).unwrap();
        fri_folds = layer_folds;

        for _ in 0..params.fri_layout[k] {
            step *= 2;
        }
        len /= coset_sizes[k];

        merkle_proof.verify(&fri_layer_values).unwrap();
        if merkle_proof.verify(&fri_layer_values).is_err() {
            return false;
        }

        previous_indices = fri_indices.clone();
        if k + 1 < params.fri_layout.len() {
            fri_indices = fri_indices
                .iter()
                .map(|ind| ind / coset_sizes[k + 1])
                .collect();
        }
    }
    if !channel.at_end() {
        return false;
    }

    // Checks that the calculated fri folded queries are the points interpolated by
    // the decommited polynomial.
    let interp_root = FieldElement::root(len).unwrap();
    for key in &previous_indices {
        let calculated = fri_folds[key].clone();
        let x_pow = interp_root.pow(fft::permute_index(len, *key));
        let committed = DensePolynomial::new(&last_layer_coefficient).evaluate(&x_pow);

        if committed != calculated.clone() {
            return false;
        }
    }

    // Checks that the oods point calculation matches the constraint calculation
    let mut trace_values: BTreeMap<(usize, isize), FieldElement> = BTreeMap::new();
    for i in 0..trace_cols {
        trace_values.insert((i, 0), oods_values[2 * i].clone());
        trace_values.insert((i, 1), oods_values[2 * i + 1].clone());
    }
    let trace_getter = |i, j| trace_values.get(&(i, j)).unwrap().clone();

    let claimed_oods_value = combine_constraints(&constraints, &constraint_coefficients, trace_len)
        .eval(&trace_getter, &oods_point);

    claimed_oods_value == oods_values[2 * trace_cols]
}

// TODO: Clean up
#[allow(clippy::cast_possible_truncation)]
fn get_indices(num: usize, bits: u32, proof: &mut VerifierChannel) -> Vec<usize> {
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

fn fri_fold(
    coset: &[FieldElement],
    eval_point: &FieldElement,
    mut step: usize,
    mut index: usize,
    mut len: usize,
    eval_x: &[FieldElement],
) -> FieldElement {
    let mut mutable_eval_copy = eval_point.clone();
    let mut coset_full: Vec<FieldElement> = coset.to_vec();
    while coset_full.len() > 1 {
        let mut next_coset = Vec::with_capacity(coset.len() / 2);

        for (k, pair) in coset_full.chunks(2).enumerate() {
            let x = &eval_x[fft::permute_index(len / 2, index + k) * step];
            next_coset.push(fri_single_fold(&pair[0], &pair[1], x, &mutable_eval_copy));
        }
        len /= 2;
        index /= 2;
        step *= 2;
        mutable_eval_copy = mutable_eval_copy.square();
        coset_full = next_coset;
    }
    coset_full[0].clone()
}

fn fri_single_fold(
    poly_at_x: &FieldElement,
    poly_at_neg_x: &FieldElement,
    x: &FieldElement,
    eval_point: &FieldElement,
) -> FieldElement {
    (poly_at_x + poly_at_neg_x) + eval_point / x * (poly_at_x - poly_at_neg_x)
}

// TODO - Make sure this is general
#[allow(clippy::too_many_arguments)]
fn out_of_domain_element(
    poly_points_u: &[U256],
    constraint_point_u: &U256,
    x_cord: &FieldElement,
    oods_point: &FieldElement,
    oods_values: &[FieldElement],
    oods_coefficients: &[FieldElement],
    eval_domain_size: usize,
    blowup: usize,
) -> FieldElement {
    let poly_points: Vec<FieldElement> = poly_points_u
        .iter()
        .map(|i| FieldElement::from_montgomery(i.clone()))
        .collect();
    let constraint_point = FieldElement::from_montgomery(constraint_point_u.clone());
    let x_transform = x_cord * FieldElement::GENERATOR;
    let omega = FieldElement::root(eval_domain_size).unwrap();
    let g = omega.pow(blowup);
    let mut r = FieldElement::ZERO;

    for x in 0..poly_points.len() {
        r += &oods_coefficients[2 * x] * (&poly_points[x] - &oods_values[2 * x])
            / (&x_transform - oods_point);
        r += &oods_coefficients[2 * x + 1] * (&poly_points[x] - &oods_values[2 * x + 1])
            / (&x_transform - &g * oods_point);
    }
    r += &oods_coefficients[oods_coefficients.len() - 1]
        * (constraint_point - &oods_values[oods_coefficients.len() - 1])
        / (&x_transform - oods_point);

    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{fibonacci::*, proofs::stark_proof};
    use macros_decl::u256h;

    #[test]
    fn verifier_fib_test() {
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
        let constraints = &get_fibonacci_constraints(&public);
        let actual = stark_proof(
            &get_trace_table(1024, &private),
            &constraints,
            &public,
            &ProofParams {
                blowup:                   16,
                pow_bits:                 12,
                queries:                  20,
                fri_layout:               vec![3, 2],
                constraints_degree_bound: 1,
            },
        );

        assert!(check_proof(
            actual.proof.as_slice(),
            &constraints,
            &public,
            &ProofParams {
                blowup:                   16,
                pow_bits:                 12,
                queries:                  20,
                fri_layout:               vec![3, 2],
                constraints_degree_bound: 1,
            },
            2,
            1024
        ));
    }
}
