mod constraints;
mod inputs;
mod pedersen_points;
mod periodic_columns;
mod trace_table;
mod starkware_example;

use env_logger;
use log::info;
use std::time::Instant;
use zkp_macros_decl::{field_element, hex};
use zkp_primefield::FieldElement;
use zkp_stark::{prove, Provable, Component, Constraints};
use zkp_u256::U256;
use std::collections::HashMap;
use starkware_example::{STARKWARE_CLAIM, starkware_witness};
use structopt::StructOpt;

use constraints::get_pedersen_merkle_constraints;
use inputs::{Claim, Witness};

// Need to import to active the logging allocator
#[allow(unused_imports)]
use zkp_logging_allocator;

fn pedersen_merkle(claim: &Claim, witness: &Witness) -> Component {
    info!("Constructing constraint system...");
    let constraints = get_pedersen_merkle_constraints(&claim).expressions().to_vec();
    let trace = claim.trace(witness);
    let labels = HashMap::default();
    Component { trace, constraints, labels }
}

#[derive(StructOpt, Debug)]
struct Options {
    size: usize,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
}

fn main() {
    // Parse command line options
    let options = Options::from_args();

    // Initialize logging
    env_logger::Builder::new()
    .filter_level(match options.verbose {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Off,
    })
    .init();

    dbg!(options);
    
    info!("Starting Pederson benchmark...");
    let start = Instant::now();

    info!("Constructing claim");
    let claim = STARKWARE_CLAIM;
    info!("Claim: {:?}", claim);

    info!("Constructing witness...");
    let witness = starkware_witness();

    info!("Constructing component...");
    let component = pedersen_merkle(&claim, &witness);
    info!("Constructed {:?}x{:?} trace", component.trace.num_rows(), component.trace.num_columns());
    info!("Constructed {:?} constraints", component.constraints.len());

    info!("Constructing proof...");
    let mut constraints = Constraints::from_expressions(
        (component.trace.num_rows(), component.trace.num_columns()),
        (&claim).into(),
        component.constraints
    ).expect("Could not create Constraint object");
    constraints.blowup = 16;
    constraints.pow_bits = 28;
    constraints.num_queries = 13;
    constraints.fri_layout = vec![3, 3, 3, 3, 2];
    let proof = prove(&constraints, &component.trace).unwrap();

    info!("Spot checking proof...");
    assert_eq!(
        proof.as_bytes()[0..32],
        hex!("b00a4c7f03959e01df2504fb73d2b238a8ab08b2000000000000000000000000")
    );
    assert_eq!(
        proof.as_bytes()[32..64],
        hex!("2e821fe1f3062acdbd3a4bd0be2293f4264abc7b000000000000000000000000")
    );

    // FRI commitments
    assert_eq!(
        proof.as_bytes()[640..672],
        hex!("b5ae7a8389c7de33f08f79c7dca057e5db5c0d65000000000000000000000000")
    );
    assert_eq!(
        proof.as_bytes()[672..704],
        hex!("83f4858900e1519c1b788333f55b54762485e5d6000000000000000000000000")
    );
    assert_eq!(
        proof.as_bytes()[704..736],
        hex!("be090ca452f0affe901588d522960b7b92d8882c000000000000000000000000")
    );
    assert_eq!(
        proof.as_bytes()[736..768],
        hex!("3cc9adaad436cfab60978d57f13d5f22e6a8791f000000000000000000000000")
    );
    assert_eq!(
        proof.as_bytes()[768..800],
        hex!("8af79c56d74b9252c3c542fc2b56d4692c608c98000000000000000000000000")
    );

    let duration = start.elapsed();
    println!("Time elapsed generating proof: {:?}", duration);
}
