mod component;
mod inputs;
mod pedersen_points;
mod periodic_columns;
mod starkware_example;

use crate::{
    component::pedersen_merkle,
    inputs::{Claim, Witness},
    starkware_example::starkware_example,
};
use env_logger;
use log::info;
use rand::{prelude::*, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use std::{num::ParseIntError, time::Instant};
use structopt::StructOpt;
use zkp_stark::{Provable, Verifiable};

// Need to import to active the logging allocator
#[allow(unused_imports)]
use zkp_logging_allocator;

fn parse_hex(src: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(src, 16)
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "zkp-stark pedersen_merkle example",
    about = "Example zkp-stark project verifying Pedersen Merkle trees."
)]
struct Options {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Run a specific 8192 depth example
    #[structopt(long)]
    large_example: bool,

    /// Depth of pedersen merkle proof to simulate
    #[structopt(long, default_value = "256")]
    size: usize,

    /// Random seed used for generating examples
    #[structopt(long, parse(try_from_str = parse_hex))]
    seed: Option<u32>,
}

struct Timer {
    start: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        info!("Starting timer");
        Self {
            start: Instant::now(),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        println!("Prover time {:?}", duration);
    }
}

// cargo t -p zkp-stark -- test_pedersen_merkle_small_proof --nocapture
// cargo b --release && target/release/examples/pedersen_merkle -vv
// --large-example

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

    // Run specific large example if requested
    if options.large_example {
        let _timer = Timer::default();
        starkware_example();
        return;
    }

    // Initialize a reproducible random number generator
    let seed = options.seed.unwrap_or_else(random);
    println!("Using random seed {:x}", seed);
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed.into());

    // Generate a random merkle proof instance
    info!("Generating random instance of size {}...", options.size);
    let witness = Witness {
        directions: (0..options.size).map(|_| rng.gen()).collect(),
        path:       (0..options.size).map(|_| rng.gen()).collect(),
    };
    let claim = Claim::from_leaf_witness(rng.gen(), &witness);

    info!("Constructing component...");
    let component = pedersen_merkle(&claim, &witness);
    println!(
        "Constructed {} by {} trace with {} constraints",
        component.trace.num_rows(),
        component.trace.num_columns(),
        component.constraints.len(),
    );

    info!("Constructing proof...");
    let proof = {
        let _timer = Timer::default();
        component.prove(())
    }
    .expect("failed to create proof");
    println!("Proof size is {}", proof.as_bytes().len());

    info!("Verifying proof...");
    component.verify(&proof).expect("Verification failed");
}
