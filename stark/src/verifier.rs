#[allow(unused_imports)]
use crate::{channel::*, merkle::*, polynomial::eval_poly, proofs::*};
use itertools::Itertools;
use primefield::FieldElement;
use u256::U256;
// use hex::*;

// TODO - Remove when all written out.
#[allow(warnings)]
pub fn check_proof(
    proposed_proof: ProverChannel,
    constraints: &Constraint,
    claim_index: usize,
    claim_value: FieldElement,
    params: &ProofParams,
    trace_cols: usize,
    trace_len: usize,
) -> bool {
    let mut public_input = [claim_index.to_be_bytes()].concat();
    public_input.extend_from_slice(&claim_value.0.to_bytes_be());
    let mut proof_check =
        VerifierChannel::new(public_input.as_slice(), proposed_proof.proof.clone());

    // Get the low degree root commitment, and constraint root commitment
    let low_degree_extension_root: [u8; 32] = proof_check.replay();
    let mut constraint_coefficients: Vec<FieldElement> =
        Vec::with_capacity(constraints.num_constraints);
    for _i in 0..constraints.num_constraints {
        constraint_coefficients.push(proof_check.get_random());
    }
    let constraint_evaluated_root: [u8; 32] = proof_check.replay();

    // Get the oods information from the proof and random
    let oods_point: FieldElement = proof_check.get_random();
    let mut oods_values: Vec<FieldElement> = Vec::with_capacity(2 * trace_cols + 1);
    for _ in 0..=2 * trace_cols {
        oods_values.push(proof_check.replay());
    }
    let mut oods_coefficients: Vec<FieldElement> = Vec::with_capacity(2 * trace_cols + 1);
    for _ in 0..=2 * trace_cols {
        oods_coefficients.push(proof_check.get_random());
    }

    let mut fri_roots: Vec<[u8; 32]> = Vec::with_capacity(params.fri_layout.len() + 1);
    let mut eval_points: Vec<FieldElement> = Vec::with_capacity(params.fri_layout.len() + 1);
    // Get first fri root:
    fri_roots.push(proof_check.replay());
    // Get fri roots and eval points from the channel random
    let mut halvings = 0;
    for &x in params.fri_layout.iter().dropping_back(1) {
        let mut eval_point = if x == 0 {
            FieldElement::ONE
        } else {
            proof_check.get_random()
        };
        eval_points.push(eval_point);
        fri_roots.push(proof_check.replay());
        halvings += x;
    }
    // Gets the last layer and the polynomial coefficients
    eval_points.push(proof_check.get_random());
    halvings += params.fri_layout[params.fri_layout.len() - 1];
    let last_layer_degree_bound = trace_len / (2_usize.pow(halvings as u32));
    let last_layer_coefficient: Vec<FieldElement> =
        proof_check.replay_many(last_layer_degree_bound);

    // Gets the proof of work from the proof, without moving the random forward.
    let mut holder = [0_u8; 8];
    holder.copy_from_slice(proof_check.read_without_replay(8));
    let proof_of_work = u64::from_be_bytes(holder);
    if !proof_check.pow_verify(proof_of_work, params.pow_bits) {
        return false;
    }
    let recorded_work: u64 = proof_check.replay();
    assert_eq!(recorded_work, proof_of_work);

    // Gets queries from channel
    let eval_domain_size = trace_len * (params.blowup as usize);
    let queries = get_indices(
        params.queries,
        eval_domain_size.trailing_zeros(),
        &mut proof_check,
    );
    let merkle_proof_length = decommitment_size(queries.as_slice(), eval_domain_size);

    // Get values and check decommitment of low degree extension
    let mut led_values: Vec<(usize, Vec<U256>)> = queries
        .iter()
        .map(|&index| {
            let held = proof_check.replay_many(trace_cols);
            (index, held)
        })
        .collect();
    let lde_decommitment: Vec<[u8; 32]> = proof_check.replay_many(merkle_proof_length);

    // TOOD - Fix merkle proof function so that it works in this case.
    assert!(verify(low_degree_extension_root, eval_domain_size.trailing_zeros(),
    led_values.as_mut_slice(), lde_decommitment));
    
    // Gets the values and checks the constraint decommitment
    let mut constraint_values: Vec<(usize, U256)> = queries
        .iter()
        .map({ |&index| (index, proof_check.replay()) })
        .collect();
    let constraint_decommitment: Vec<[u8; 32]> = proof_check.replay_many(merkle_proof_length);

    if !verify(constraint_evaluated_root, eval_domain_size.trailing_zeros(),
    constraint_values.as_mut_slice(), constraint_decommitment) {     return
    false; }

    true
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fibonacci::*;
    use hex_literal::*;
    use u256::u256h;

    #[test]
    fn verifier_fib_test() {
        let claim_index = 1000;
        let claim_value = FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        ));
        let witness = FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000000000000cafebabe"
        ));
        let actual = stark_proof(
            &get_trace_table(1024, witness),
            &get_constraint(),
            claim_index,
            claim_value.clone(),
            &ProofParams {
                blowup:     16,
                pow_bits:   12,
                queries:    20,
                fri_layout: vec![3, 2],
            },
        );

        assert!(check_proof(
            actual,
            &get_constraint(),
            claim_index,
            claim_value,
            &ProofParams {
                blowup:     16,
                pow_bits:   12,
                queries:    20,
                fri_layout: vec![3, 2],
            },
            2,
            1024
        ));
    }
}
