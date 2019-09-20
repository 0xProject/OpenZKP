#![warn(clippy::all)]
use env_logger;
use log::info;
use macros_decl::field_element;
use openstark::{
    fibonacci::{PrivateInput, PublicInput},
    stark_proof, ProofParams,
};
use primefield::FieldElement;
use std::time::Instant;
use u256::U256;

fn main() {
    env_logger::init();

    info!("Constructing public input...");
    let public = PublicInput {
        index: 1000,
        value: field_element!("0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"),
    };
    info!("Public input: {:?}", public);

    info!("Constructing private input...");
    let private = PrivateInput {
        secret: field_element!("cafebabe"),
    };
    info!("Private input: {:?}", private);

    // Start timer
    let start = Instant::now();

    info!("Constructing proof...");
    let potential_proof = stark_proof(&public, &private, &ProofParams {
        blowup:     16,
        pow_bits:   12,
        queries:    20,
        fri_layout: vec![3, 2],
    });

    // Measure time
    let duration = start.elapsed();
    info!("{:?}", potential_proof.coin.digest);
    info!("Time elapsed in proof function is: {:?}", duration);
}
