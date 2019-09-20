use crate::{
    channel::*, constraints::Constraints, polynomial::DensePolynomial, proof_of_work,
    proof_params::ProofParams,
};
use hash::Hash;
use merkle_tree::{Commitment, Error as MerkleError, Proof};
use primefield::{fft, geometric_series::root_series, FieldElement};
#[cfg(feature = "std")]
use std::error;
use std::{collections::BTreeMap, convert::TryInto, fmt, prelude::v1::*};
use u256::U256;

// False positive, for<'a> is required.
#[allow(single_use_lifetimes)]
pub fn check_proof<Public>(
    proposed_proof: &[u8],
    constraints: &Constraints,
    public: &Public,
    params: &ProofParams,
    trace_cols: usize,
    trace_len: usize,
) -> Result<()>
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
    let lde_commitment = Commitment::from_size_hash(eval_domain_size, &low_degree_extension_root)?;
    let mut constraint_coefficients: Vec<FieldElement> = Vec::with_capacity(2 * constraints.len());
    for _ in 0..constraints.len() {
        constraint_coefficients.push(channel.get_random());
        constraint_coefficients.push(channel.get_random());
    }
    let constraint_evaluated_root = Replayable::<Hash>::replay(&mut channel);
    let constraint_commitment =
        Commitment::from_size_hash(eval_domain_size, &constraint_evaluated_root)?;

    // Get the oods information from the proof and random
    let oods_point: FieldElement = channel.get_random();
    let mut oods_values: Vec<FieldElement> = Vec::with_capacity(2 * trace_cols + 1);
    let constraints_trace_degree = constraints.trace_degree();
    for _ in 0..(2 * trace_cols + constraints_trace_degree) {
        oods_values.push(Replayable::<FieldElement>::replay(&mut channel));
    }
    let mut oods_coefficients: Vec<FieldElement> = Vec::with_capacity(2 * trace_cols + 1);
    for _ in 0..2 * trace_cols + constraints_trace_degree {
        oods_coefficients.push(channel.get_random());
    }

    let mut fri_commitments: Vec<Commitment> = Vec::with_capacity(params.fri_layout.len() + 1);
    let mut eval_points: Vec<FieldElement> = Vec::with_capacity(params.fri_layout.len() + 1);
    let mut fri_size = eval_domain_size >> params.fri_layout[0];
    // Get first fri root:
    fri_commitments.push(Commitment::from_size_hash(
        fri_size,
        &Replayable::<Hash>::replay(&mut channel),
    )?);
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
        fri_commitments.push(Commitment::from_size_hash(
            fri_size,
            &Replayable::<Hash>::replay(&mut channel),
        )?);
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
        return Err(Error::InvalidPoW);
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
    let lde_proof_length = lde_commitment.proof_size(&queries)?;
    let lde_hashes = Replayable::<Hash>::replay_many(&mut channel, lde_proof_length);
    let lde_proof = Proof::from_hashes(&lde_commitment, &queries, &lde_hashes)?;
    // Note - we could express this a merkle error instead but this adds specificity
    if lde_proof.verify(&lde_values).is_err() {
        return Err(Error::InvalidLDECommitment);
    }

    // Gets the values and checks the constraint decommitment
    let mut constraint_values = Vec::with_capacity(queries.len());
    for query_index in &queries {
        constraint_values.push((
            *query_index,
            Replayable::<FieldElement>::replay_many(&mut channel, constraints_trace_degree),
        ));
    }
    let constraint_proof_length = constraint_commitment.proof_size(&queries)?;
    let constraint_hashes: Vec<Hash> =
        Replayable::<Hash>::replay_many(&mut channel, constraint_proof_length);
    let constraint_proof =
        Proof::from_hashes(&constraint_commitment, &queries, &constraint_hashes)?;
    // Note - we could express this a merkle error instead but this adds specificity
    if constraint_proof.verify(&constraint_values).is_err() {
        return Err(Error::InvalidConstraintCommitment);
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
                        coset.push(match fri_folds.get(&n) {
                            Some(x) => x.clone(),
                            None => return Err(Error::HashMapFailure),
                        });
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
                        )?);
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

        let merkle_proof_length = commitment.proof_size(&fri_indices)?;
        let merkle_hashes = Replayable::<Hash>::replay_many(&mut channel, merkle_proof_length);
        let merkle_proof = Proof::from_hashes(commitment, &fri_indices, &merkle_hashes)?;
        fri_folds = layer_folds;

        for _ in 0..params.fri_layout[k] {
            step *= 2;
        }
        len /= coset_sizes[k];

        // Note - we could express this a merkle error instead but this adds specificity
        if merkle_proof.verify(&fri_layer_values).is_err() {
            return Err(Error::InvalidFriCommitment);
        };

        previous_indices = fri_indices.clone();
        if k + 1 < params.fri_layout.len() {
            fri_indices = fri_indices
                .iter()
                .map(|ind| ind / coset_sizes[k + 1])
                .collect();
        }
    }
    if !channel.at_end() {
        return Err(Error::ProofTooLong);
    }

    // Checks that the calculated fri folded queries are the points interpolated by
    // the decommited polynomial.
    let interp_root = match FieldElement::root(len) {
        Some(x) => x,
        None => return Err(Error::RootUnavailable),
    };
    for key in &previous_indices {
        let calculated = fri_folds[key].clone();
        let x_pow = interp_root.pow(fft::permute_index(len, *key));
        let committed = DensePolynomial::new(&last_layer_coefficient).evaluate(&x_pow);

        if committed != calculated.clone() {
            return Err(Error::OodsCalculationFailure);
        }
    }

    let (trace_values, constraint_values) = oods_values.split_at(2 * trace_cols);
    if oods_value_from_trace_values(
        &constraints,
        &constraint_coefficients,
        trace_len,
        &trace_values,
        &oods_point,
    ) != oods_value_from_constraint_values(&constraint_values, &oods_point)
    {
        return Err(Error::OodsMismatch);
    }
    Ok(())
}

fn oods_value_from_trace_values(
    constraints: &Constraints,
    coefficients: &[FieldElement],
    trace_length: usize,
    trace_values: &[FieldElement],
    oods_point: &FieldElement,
) -> FieldElement {
    let trace = |i: usize, j: isize| {
        let j: usize = j.try_into().unwrap();
        assert!(j == 0 || j == 1);
        trace_values[2 * i + j].clone()
    };
    constraints
        .combine(coefficients, trace_length)
        .evaluate(oods_point, &trace)
}

fn oods_value_from_constraint_values(
    constraint_values: &[FieldElement],
    oods_point: &FieldElement,
) -> FieldElement {
    // TODO - Check if this is 100% unreachable, if so remove if not error.
    assert!(constraint_values.len().is_power_of_two());

    let mut result = FieldElement::ZERO;
    let mut power = FieldElement::ONE;
    for value in constraint_values {
        result += value * &power;
        power *= oods_point;
    }
    result
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
    constraint_oods_values: &[FieldElement],
    x_cord: &FieldElement,
    oods_point: &FieldElement,
    oods_values: &[FieldElement],
    oods_coefficients: &[FieldElement],
    eval_domain_size: usize,
    blowup: usize,
) -> Result<FieldElement> {
    let poly_points: Vec<FieldElement> = poly_points_u
        .iter()
        .map(|i| FieldElement::from_montgomery(i.clone()))
        .collect();
    let x_transform = x_cord * FieldElement::GENERATOR;
    let omega = match FieldElement::root(eval_domain_size) {
        Some(x) => x,
        None => return Err(Error::RootUnavailable),
    };
    let g = omega.pow(blowup);
    let mut r = FieldElement::ZERO;

    for x in 0..poly_points.len() {
        r += &oods_coefficients[2 * x] * (&poly_points[x] - &oods_values[2 * x])
            / (&x_transform - oods_point);
        r += &oods_coefficients[2 * x + 1] * (&poly_points[x] - &oods_values[2 * x + 1])
            / (&x_transform - &g * oods_point);
    }
    for (i, constraint_oods_value) in constraint_oods_values.iter().enumerate() {
        r += &oods_coefficients[2 * poly_points.len() + i]
            * (constraint_oods_value - &oods_values[poly_points.len() * 2 + i])
            / (&x_transform - oods_point.pow(constraint_oods_values.len()));
    }
    Ok(r)
}

type Result<T> = std::result::Result<T, Error>;

// TODO - We could parametrize root unavailable with the size asked for and fri
// error with which layer failed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error {
    RootUnavailable,
    InvalidPoW,
    InvalidLDECommitment,
    InvalidConstraintCommitment,
    InvalidFriCommitment,
    HashMapFailure,
    ProofTooLong,
    OodsCalculationFailure,
    OodsMismatch,
    FriCalculationFailure,
    Merkle(MerkleError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::RootUnavailable => {
                write!(f, "The prime field doesn't have a root of this order")
            }
            Error::InvalidPoW => write!(f, "The suggested proof of work failed to verify"),
            Error::InvalidLDECommitment => write!(f, "The LDE merkle proof is incorrect"),
            Error::InvalidConstraintCommitment => {
                write!(f, "The constraint merkle proof is incorrect")
            }
            Error::InvalidFriCommitment => write!(f, "A FRI layer commitment is incorrect"),
            Error::HashMapFailure => {
                write!(
                    f,
                    "Verifier attempted to look up an empty entry in the hash map"
                )
            }
            Error::ProofTooLong => write!(f, "The proof length doesn't match the specification"),
            Error::OodsCalculationFailure => {
                write!(
                    f,
                    "The calculated odds value doesn't match the committed one"
                )
            }
            Error::FriCalculationFailure => {
                write!(
                    f,
                    "The final FRI calculation suggests the committed polynomial isn't low degree"
                )
            }
            Error::OodsMismatch => {
                write!(f, "Calculated oods value doesn't match the committed one")
            }
            // This is a wrapper, so defer to the underlying types' implementation of `fmt`.
            Error::Merkle(ref e) => std::fmt::Display::fmt(e, f),
        }
    }
}

impl From<MerkleError> for Error {
    fn from(err: MerkleError) -> Self {
        Error::Merkle(err)
    }
}

#[cfg(feature = "std")]
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Merkle(ref e) => Some(e),
            _ => None,
        }
    }
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
                blowup:     16,
                pow_bits:   12,
                queries:    20,
                fri_layout: vec![3, 2],
            },
        );

        assert!(check_proof(
            actual.proof.as_slice(),
            &constraints,
            &public,
            &ProofParams {
                blowup:     16,
                pow_bits:   12,
                queries:    20,
                fri_layout: vec![3, 2],
            },
            2,
            1024
        )
        .is_ok());
    }
}
