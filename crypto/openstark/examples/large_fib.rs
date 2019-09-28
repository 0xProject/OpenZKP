#![warn(clippy::all)]
use env_logger;
use log::info;
use macros_decl::field_element;
use openstark::{fibonacci, proof, verify, Provable, Verifiable};
use primefield::FieldElement;
use std::{env, time::Instant};
use u256::U256;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(args[1].parse::<usize>().expect("Invalid number supplied"))
            .build_global()
            .expect("Error building Rayon thread pool.");
    }
    info!("Starting Fibonacci benchmark...");

    let index = 1_000_000;
    let secret = field_element!("cafebabe");
    let value = fibonacci::get_value(index, &secret);

    let claim = fibonacci::Claim { index, value };
    let witness = fibonacci::Witness { secret };

    let start = Instant::now();
    let constraints = claim.constraints();
    let trace = claim.trace(&witness);
    let potential_proof = proof(&constraints, &trace);
    let duration = start.elapsed();
    println!("{:?}", potential_proof.coin.digest);
    println!("Time elapsed in proof function is: {:?}", duration);
    println!("The proof length is {}", potential_proof.proof.len());
    println!(
        "The estimated size bound is: {}",
        constraints.max_proof_size()
    );

    let verified = verify(&constraints, potential_proof.proof.as_slice());
    println!("Checking the proof resulted in: {:?}", verified);
}
