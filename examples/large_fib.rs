use hex_literal::*;
use starkcrypto::{fibonacci::*, field::FieldElement, proofs::*, u256::*, u256h};
use std::{env, time::Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(args[1].parse::<usize>().expect("Invalid number supplied"))
            .build_global()
            .expect("Error building Rayon thread pool.");
    }

    let claim_index = 1_000_000_usize;
    let witness = FieldElement::from(u256h!(
        "00000000000000000000000000000000000000000000000000000000cafebabe"
    ));
    let trace_table = get_trace_table(1_048_576, witness.clone());
    let claim_fib = trace_table.elements[2_000_000].clone();
    let start = Instant::now();
    let potential_proof = stark_proof(
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
    let duration = start.elapsed();
    println!("{:?}", potential_proof.digest);
    println!("Time elapsed in proof function is: {:?}", duration);
}
