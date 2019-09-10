#![warn(clippy::all)]
use log::info;
use macros_decl::u256h;
use primefield::FieldElement;
use stark::{
    fibonacci::{get_fibonacci_constraints, get_trace_table, PrivateInput, PublicInput},
    stark_proof, ProofParams,
};
use std::time::Instant;
use u256::U256;

fn main() {
    env_logger::init();

    info!("Constructing public input...");
    let public = PublicInput {
        index: 1000,
        value: FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        )),
    };
    info!("Public input: {:?}", public);

    info!("Constructing private input...");
    let private = PrivateInput {
        secret: FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000000000000cafebabe"
        )),
    };
    info!("Private input: {:?}", private);

    info!("Constructing trace table...");
    let trace_table = get_trace_table(1024, &private);

    // Start timer
    let start = Instant::now();

    info!("Constructing constraint system...");
    let constraints = get_fibonacci_constraints(&public);

    info!("Constructing proof...");
    let potential_proof = stark_proof(&trace_table, &constraints, &public, &ProofParams {
        blowup:                   16,
        pow_bits:                 12,
        queries:                  20,
        fri_layout:               vec![3, 2],
        constraints_degree_bound: 1,
    });

    // Measure time
    let duration = start.elapsed();
    info!("{:?}", potential_proof.coin.digest);
    info!("Time elapsed in proof function is: {:?}", duration);
}
