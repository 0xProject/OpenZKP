#![warn(clippy::all)]
#![deny(warnings)]
use macros_decl::u256h;
use primefield::FieldElement;
use stark::{
    fibonacci::{get_constraint, get_trace_table, PrivateInput, PublicInput},
    stark_proof, ProofParams,
};
use std::{env, time::Instant};
use u256::U256;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(args[1].parse::<usize>().expect("Invalid number supplied"))
            .build_global()
            .expect("Error building Rayon thread pool.");
    }

    let mut public = PublicInput {
        index: 1_000_000,
        value: FieldElement::ZERO, // To be overwritten with the correct value.
    };
    let private = PrivateInput {
        secret: FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000000000000cafebabe"
        )),
    };
    let trace_table = get_trace_table(1_048_576, &private);
    public.value = trace_table[(public.index, 0)].clone();
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
