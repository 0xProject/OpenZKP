#![warn(clippy::all)]
use env_logger;
use log::info;
use macros_decl::field_element;
use openstark::{check_proof, vfd_matter::vdf::Claim, stark_proof, ProofParams};
use primefield::FieldElement;
use std::{env, time::Instant};
use u256::U256;

fn main() {
    env_logger::init();
    info!("Starting Fibonacci benchmark...");

    let c0_start =
        field_element!("00a74f2a70da4ea3723cabd2acc55d03f9ff6d0e7acef0fc63263b12c10dd837");
    let c1_start =
        field_element!("02ba0d3dfeb1ee83889c5ad8534ba15723a42b306e2f44d5eee10bfa939ae756");
    let c0_end =
        field_element!("02c190f26be11bc330401087c92214777ca6e2d25183303d0b0ec4feb7277f64");
    let c1_end =
        field_element!("05e1e4162ab76832cc21610cc20c25b998ecbf53d0825b9ccd7f80037c532856");
    let input = Claim { c0_start, c1_start, c0_end, c1_end};
    let params = ProofParams::suggested(1048576);
    let start = Instant::now();
    let potential_proof = stark_proof(&input, &(), &params);
    let duration = start.elapsed();
    println!("{:?}", potential_proof.coin.digest);
    println!("Time elapsed in proof function is: {:?}", duration);
    println!("The proof length is {}", potential_proof.proof.len());

    let verified = check_proof(potential_proof.proof.as_slice(), &input, &params);
    println!("Checking the proof resulted in: {:?}", verified);
}
