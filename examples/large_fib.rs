use hex_literal::*;
use rayon::prelude;
use starkcrypto::fibonacci::*;
use starkcrypto::field::FieldElement;
use starkcrypto::proofs::*;
use starkcrypto::u256::*;
use starkcrypto::u256h;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    rayon::ThreadPoolBuilder::new()
        .num_threads(args[1].parse::<usize>().unwrap())
        .build_global()
        .unwrap();

    let claim_index = 1000000_u64;
    let witness = FieldElement::from(u256h!(
        "00000000000000000000000000000000000000000000000000000000cafebabe"
    ));
    let trace_table = get_trace_table(1048576, witness.clone());
    let claim_fib = trace_table.elements[2000000].clone();
    let start = Instant::now();
    let potential_proof = stark_proof(
        &trace_table,
        &get_constraint(),
        claim_index,
        claim_fib,
        2_u64.pow(4),
    );
    let duration = start.elapsed();
    println!("{:?}", potential_proof.digest);
    println!("Time elapsed in proof function is: {:?}", duration);
}
