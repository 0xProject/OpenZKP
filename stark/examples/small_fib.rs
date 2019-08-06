#![warn(clippy::all)]
#![deny(warnings)]
use hex_literal::*;
use primefield::FieldElement;
use stark::{
    fibonacci::{get_constraint, get_trace_table, PrivateInput, PublicInput},
    stark_proof, ProofParams,
};
use std::time::Instant;
use u256::{u256h, U256};

fn main() {
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
    let trace_table = get_trace_table(1024, &private);
    let start = Instant::now();
    let potential_proof = stark_proof(&trace_table, &get_constraint(), &public, &ProofParams {
        blowup:     16,
        pow_bits:   12,
        queries:    20,
        fri_layout: vec![3, 2],
    });
    let duration = start.elapsed();
    println!("{:?}", potential_proof.coin.digest);
    println!("Time elapsed in proof function is: {:?}", duration);
}
