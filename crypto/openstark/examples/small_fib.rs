#![warn(clippy::all)]
use env_logger;
use log::info;
use macros_decl::field_element;
use openstark::{fibonacci, proof, Provable, Verifiable};
use primefield::FieldElement;
use std::time::Instant;
use u256::U256;

fn main() {
    env_logger::init();

    info!("Constructing claim");
    let claim = fibonacci::Claim {
        index: 1000,
        value: field_element!("0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"),
    };
    info!("Claim: {:?}", claim);

    info!("Constructing witness");
    let witness = fibonacci::Witness {
        secret: field_element!("cafebabe"),
    };
    info!("Witness: {:?}", witness);

    // Start timer
    let start = Instant::now();

    info!("Constructing proof...");
    let constraints = claim.constraints();
    let trace = claim.trace(&witness);
    let potential_proof = proof(&constraints, &trace);

    // Measure time
    let duration = start.elapsed();
    info!("{:?}", potential_proof.coin.digest);
    info!("Time elapsed in proof function is: {:?}", duration);
}
