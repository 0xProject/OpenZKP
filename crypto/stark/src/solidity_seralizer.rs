use crate::{
    channel::{RandomGenerator, Replayable, VerifierChannel},
    constraints::Constraints,
    proof_of_work, Proof,
};
use hex::encode;
use std::{collections::BTreeMap, prelude::v1::*};
use zkp_hash::Hash;
use zkp_merkle_tree::{Commitment, Error as MerkleError};
use zkp_primefield::FieldElement;
use zkp_u256::U256;

// This trait is a simple json encoding trait which exports the values
// in the format expected by the ethereum call, the hand made version
// is used instead of serde because it differ in important ways
trait SoliditySeralize {
    fn sol_encode(&self) -> String;
}

impl SoliditySeralize for FieldElement {
    fn sol_encode(&self) -> String {
        format!("\"0x{}\"", encode(self.as_montgomery().to_bytes_be()))
    }
}

impl SoliditySeralize for Hash {
    fn sol_encode(&self) -> String {
        format!("\"0x{}\"", encode(self.as_bytes()))
    }
}

impl SoliditySeralize for Commitment {
    fn sol_encode(&self) -> String {
        self.hash().sol_encode()
    }
}

// This recursive call makes a javascript array, for any type which can be
// serialized
impl<T: SoliditySeralize> SoliditySeralize for Vec<T> {
    fn sol_encode(&self) -> String {
        let mut res: String = "".to_string();
        if self.is_empty() {
            return res;
        }

        let (last, rest) = self.split_last().unwrap();
        res.push_str(&"[\n");
        for data in rest.iter() {
            res.push_str(&format!("{},\n", data.sol_encode()));
        }
        res.push_str(&format!("{}\n", last.sol_encode()));
        res.push_str("]");
        res
    }
}

// TODO - Make this function smaller
#[allow(clippy::too_many_lines)]
pub fn proof_serialize(
    constraints: &Constraints,
    proof: &Proof,
    result_string: &mut String,
) -> Result<(), MerkleError> {
    let proof = proof.as_bytes();
    let trace_length = constraints.trace_nrows();
    let trace_cols = constraints.trace_ncolumns();
    let eval_domain_size = trace_length * constraints.blowup;

    let mut channel = VerifierChannel::new(proof.to_vec());
    // TODO - Add method to seralize public input
    channel.initialize(constraints.channel_seed());

    // Get the low degree root commitment, and constraint root commitment
    // TODO: Make it work as channel.read()
    let low_degree_extension_root: Hash = channel.replay();
    result_string.push_str(&format!(
        "\"trace_commitment\": {}, \n",
        low_degree_extension_root.sol_encode()
    ));
    let lde_commitment = Commitment::from_size_hash(eval_domain_size, &low_degree_extension_root)?;
    let _ = channel.get_coefficients(2 * constraints.len());

    let constraint_evaluated_root: Hash = channel.replay();
    result_string.push_str(&format!(
        "\"constraint_commitment\": {}, \n",
        constraint_evaluated_root.sol_encode()
    ));
    let constraint_commitment =
        Commitment::from_size_hash(eval_domain_size, &constraint_evaluated_root)?;

    // Get the oods information from the proof and random
    let _: FieldElement = channel.get_random();

    // This hack around claim polynomials is awful and should be removed
    let mut parseable_constraints = constraints.clone();
    parseable_constraints.substitute();

    let trace_arguments = parseable_constraints.trace_arguments();
    let trace_values: Vec<FieldElement> = channel.replay_many(trace_arguments.len());
    result_string.push_str(&format!(
        "\"trace_oods_values\": {}, \n",
        trace_values.sol_encode()
    ));
    let claimed_trace_map: BTreeMap<(usize, isize), FieldElement> = trace_arguments
        .into_iter()
        .zip(trace_values.iter().cloned())
        .collect();

    let constraints_trace_degree = constraints.degree().next_power_of_two();
    let claimed_constraint_values: Vec<FieldElement> =
        channel.replay_many(constraints_trace_degree);
    result_string.push_str(&format!(
        "\"constraint_oods_values\": {}, \n",
        claimed_constraint_values.sol_encode()
    ));

    let _ = channel.get_coefficients(claimed_trace_map.len() + claimed_constraint_values.len());

    let mut fri_commitments: Vec<Commitment> = Vec::with_capacity(constraints.fri_layout.len() + 1);

    let mut eval_points: Vec<FieldElement> = Vec::with_capacity(constraints.fri_layout.len() + 1);
    let mut fri_size = eval_domain_size;
    // Get fri roots and eval points from the channel random
    for &num_folds in &constraints.fri_layout {
        fri_size >>= num_folds;
        fri_commitments.push(Commitment::from_size_hash(fri_size, &channel.replay())?);
        eval_points.push(channel.get_random());
    }
    result_string.push_str(&format!(
        "\"fri_commitments\": {}, \n",
        fri_commitments.sol_encode()
    ));

    // Gets the last layer coeffiencts
    let last_layer_coefficients = channel.replay_fri_layer(fri_size / constraints.blowup);
    result_string.push_str(&format!(
        "\"last_layer_coefficients\": {}, \n",
        last_layer_coefficients.sol_encode()
    ));

    // Gets the proof of work from the proof.
    let pow_response: proof_of_work::Response = channel.replay();
    result_string.push_str(&format!(
        "\"pow_nonce\": \"0x{}\",",
        encode(pow_response.nonce().to_be_bytes())
    ));

    // Gets queries from channel
    let queries = get_indices(
        constraints.num_queries,
        eval_domain_size.trailing_zeros(),
        &mut channel,
    );

    // Get values and check decommitment of low degree extension
    let lde_values: Vec<(usize, Vec<FieldElement>)> = queries
        .iter()
        .map(|&index| (index, channel.replay_fri_layer(trace_cols)))
        .collect();
    let flattened_trace_values: Vec<FieldElement> =
        lde_values.iter().flat_map(|data| data.1.clone()).collect();
    result_string.push_str(&format!(
        "\"trace_values\": {}, \n",
        flattened_trace_values.sol_encode()
    ));

    let lde_proof_length = lde_commitment.proof_size(&queries)?;
    let lde_hashes: Vec<Hash> = channel.replay_many(lde_proof_length);
    result_string.push_str(&format!(
        "\"trace_decommitment\": {}, \n",
        lde_hashes.sol_encode()
    ));

    // Gets the values and checks the constraint decommitment
    let mut constraint_values = Vec::with_capacity(queries.len());
    for query_index in &queries {
        constraint_values.push((
            *query_index,
            channel.replay_fri_layer(constraints_trace_degree),
        ));
    }
    let flattened_constraint_values: Vec<FieldElement> = constraint_values
        .iter()
        .flat_map(|data| data.1.clone())
        .collect();
    result_string.push_str(&format!(
        "\"constraint_values\": {}, \n",
        flattened_constraint_values.sol_encode()
    ));
    let constraint_proof_length = constraint_commitment.proof_size(&queries)?;
    let constraint_hashes: Vec<Hash> = channel.replay_many(constraint_proof_length);
    result_string.push_str(&format!(
        "\"constraint_decommitment\": {}, \n",
        constraint_hashes.sol_encode()
    ));

    let coset_sizes = constraints
        .fri_layout
        .iter()
        .map(|k| 1_usize << k)
        .collect::<Vec<_>>();
    let mut fri_indices: Vec<usize> = queries
        .to_vec()
        .iter()
        .map(|x| x / coset_sizes[0])
        .collect();

    let mut previous_indices = queries.to_vec();
    let mut fri_values: Vec<Vec<FieldElement>> = Vec::new();
    let mut fri_decommitments: Vec<Vec<Hash>> = Vec::new();
    for (k, commitment) in fri_commitments.iter().enumerate() {
        fri_indices.dedup();
        let mut proof_values = Vec::new();
        for i in &fri_indices {
            for j in 0..coset_sizes[k] {
                let n = i * coset_sizes[k] + j;
                if previous_indices.binary_search(&n).is_err() {
                    let held: FieldElement = channel.replay();
                    proof_values.push(held.clone());
                }
            }
        }
        fri_values.push(proof_values);

        let merkle_proof_length = commitment.proof_size(&fri_indices)?;
        let merkle_hashes: Vec<Hash> = channel.replay_many(merkle_proof_length);
        fri_decommitments.push(merkle_hashes.clone());

        previous_indices = fri_indices.clone();
        if k + 1 < constraints.fri_layout.len() {
            fri_indices = fri_indices
                .iter()
                .map(|ind| ind / coset_sizes[k + 1])
                .collect();
        }
    }

    result_string.push_str(&format!("\"fri_values\": {}, \n", fri_values.sol_encode()));
    result_string.push_str(&format!(
        "\"fri_decommitments\": {} \n",
        fri_decommitments.sol_encode()
    ));
    Ok(())
}

// TODO: Clean up
#[allow(clippy::cast_possible_truncation)]
fn get_indices(num: usize, bits: u32, proof: &mut VerifierChannel) -> Vec<usize> {
    let mut query_indices = Vec::with_capacity(num + 3);
    while query_indices.len() < num {
        let val: U256 = proof.get_random();
        query_indices
            .push(((val.clone() >> (0x100 - 0x040)).limb(0) & (2_u64.pow(bits) - 1)) as usize);
        query_indices
            .push(((val.clone() >> (0x100 - 0x080)).limb(0) & (2_u64.pow(bits) - 1)) as usize);
        query_indices
            .push(((val.clone() >> (0x100 - 0x0C0)).limb(0) & (2_u64.pow(bits) - 1)) as usize);
        query_indices.push((val.limb(0) & (2_u64.pow(bits) - 1)) as usize);
    }
    query_indices.truncate(num);
    (&mut query_indices).sort_unstable();
    query_indices
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{prove, traits::tests::Recurrance, Provable, Verifiable};
    use zkp_macros_decl::field_element;

    #[test]
    fn seralize_recurrance() {
        let r = Recurrance {
            index:         150,
            initial_value: field_element!(
                "42f70183f3ed560b81c4cd49a8d5f27fdb747c17eaa93f41570b012649b5a47b"
            ),
            exponent:      2,
        };

        let public = r.claim();
        let private = r.witness();

        let mut constraints = public.constraints();
        constraints.num_queries = 20;
        constraints.pow_bits = 10;

        let trace = public.trace(&private);

        let mut result_string = "".to_string();
        proof_serialize(
            &constraints,
            &prove(&constraints, &trace).unwrap(),
            &mut result_string,
        )
        .unwrap();
    }

    // Note this test is actually more like a binary which we want run so it
    // commented out, The Recurrance struct can't be exported to a binary or
    // example because it only lives in tests.
    // use std::{fs::File, io::prelude::*, path::Path};
    // #[test]
    // fn output_recurrances() {
    //     let mut initial_value =
    //         field_element!("
    // 42f70183f3ed560b81c4cd49a8d5f27fdb747c17eaa93f41570b012649b5a47b");
    //     let mut result_string = "[\n".to_string();
    //     let mut index = 150;
    //     for i in 0..15 {
    //         println!("{}", i);
    //         let r = Recurrance {
    //             index:         index,
    //             initial_value: initial_value.clone(),
    //             exponent:      2,
    //         };

    //         let public = r.claim();
    //         // println!("{:?}", public.value.as_montgomery());
    //         let private = r.witness();

    //         let mut constraints = public.constraints();
    //         constraints.num_queries = 20;
    //         constraints.pow_bits = 10;
    //         let trace = public.trace(&private);

    //         result_string.push_str(&format!(
    //             "{{
    //             \"public_inputs\": {{
    //                 \"index\" : {},
    //                 \"value\" : \"0x{}\"
    //             }},",
    //             r.index,
    //             public.value.as_montgomery()
    //         ));

    //         let _ = proof_serialize(
    //             &constraints,
    //             &prove(&constraints, &trace).unwrap(),
    //             &mut result_string,
    //         )
    //         .unwrap();

    //         if i == 19 {
    //             result_string.push_str("}\n");
    //         } else {
    //             result_string.push_str("},\n");
    //         }

    //         // This changes around the value in a slightly unpredictable way
    //         initial_value *= initial_value.clone();
    //         index *= 2;
    //     }
    //     result_string.push_str("]");

    //     let path = Path::new("recurrence.json");
    //     let display = path.display();
    //     let mut file = match File::create(&path) {
    //         Err(why) => panic!("couldn't create {}: {}", display,
    // why.to_string()),         Ok(file) => file,
    //     };
    //     writeln!(&mut file, "{}", result_string).unwrap();
    //     // assert!(false);
    // }
}
