#![warn(clippy::all)]
use env_logger;
use log::info;
use macros_decl::field_element;
use openstark::{
    check_proof, decommitment_size_upper_bound,
    fibonacci::{get_value, PrivateInput, PublicInput},
    stark_proof, ProofParams,
};
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
    let value = get_value(index, &secret);

    let public = PublicInput { index, value };
    let private = PrivateInput { secret };

    let fri_layout = vec![3, 3, 3, 3, 2];
    let start = Instant::now();
    let potential_proof = stark_proof(&public, &private, &ProofParams {
        blowup:     16,
        pow_bits:   12,
        queries:    20,
        fri_layout: fri_layout.clone(),
    });
    let duration = start.elapsed();
    println!("{:?}", potential_proof.coin.digest);
    println!("Time elapsed in proof function is: {:?}", duration);
    println!("The proof length is {}", potential_proof.proof.len());
    println!(
        "The estimated size bound is: {}",
        decommitment_size_upper_bound(20, 2, fri_layout.clone(), 20)
    );

    let verified = check_proof(potential_proof.proof.as_slice(), &public, &ProofParams {
        blowup: 16,
        pow_bits: 12,
        queries: 20,
        fri_layout,
    });
    println!("Checking the proof resulted in: {:?}", verified);
}
