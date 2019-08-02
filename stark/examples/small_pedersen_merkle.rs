use hex_literal::*;
use primefield::FieldElement;
use stark::{get_constraint, get_trace_table, stark_proof, ProofParams};
use std::time::Instant;
use u256::{u256h, U256};

fn main() {
    let claim_index = 1000_usize;
    let claim_fib = FieldElement::from(u256h!(
        "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
    ));
    let witness = FieldElement::from(u256h!(
        "00000000000000000000000000000000000000000000000000000000cafebabe"
    ));
    let trace_table = get_trace_table(1024, witness.clone());

    stark_proof(
        &trace_table,
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
}
