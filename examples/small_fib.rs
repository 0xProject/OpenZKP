use hex_literal::*;
use starkcrypto::{fibonacci::*, field::FieldElement, proofs::*, u256::*, u256h};
use std::time::Instant;

fn main() {
    let claim_index = 1000_usize;
    let claim_fib = FieldElement::from(u256h!(
        "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
    ));
    let witness = FieldElement::from(u256h!(
        "00000000000000000000000000000000000000000000000000000000cafebabe"
    ));
    let trace_table = get_trace_table(1024, witness.clone());
    let start = Instant::now();
    let potential_proof = stark_proof(
        &trace_table,
        &get_constraint(),
        claim_index,
        claim_fib,
        &ProofParams {
            beta:       16,
            pow_bits:   12,
            queries:    20,
            fri_layout: vec![0, 3, 2],
        },
    );
    let duration = start.elapsed();
    println!("{:?}", potential_proof.digest);
    println!("Time elapsed in proof function is: {:?}", duration);
}
