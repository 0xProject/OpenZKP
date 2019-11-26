mod component;
mod constraints;
mod inputs;
mod pedersen_points;
mod periodic_columns;
mod starkware_example;
mod trace_table;

use crate::{
    component::pedersen_merkle,
    constraints::get_pedersen_merkle_constraints,
    inputs::{Claim, Witness},
    starkware_example::starkware_example,
};
use env_logger;
use log::{error, info};
use rand::{
    distributions::{Distribution, Standard},
    prelude::*,
    SeedableRng,
};
use rand_xoshiro::Xoshiro256PlusPlus;
use std::{collections::HashMap, num::ParseIntError, time::Instant};
use structopt::StructOpt;
use zkp_macros_decl::{field_element, hex};
use zkp_primefield::FieldElement;
use zkp_stark::{prove, Component, Constraints, Provable};
use zkp_u256::U256;

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
        println!("Time elapsed generating proof: {:?}", duration);
    }
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

    // Measure time elapsed (from here till it goes out of scope)
    let _timer = Timer::default();

    // Run specific large example if requested
    if options.large_example {
        starkware_example();
        return;
    }

    // Initialize a reproducible random number generator
    let seed = options.seed.unwrap_or_else(random);
    println!("Using random seed: {:x}", seed);
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed.into());

    // Generate a random merkle proof
    let witness = Witness {
        directions: (0..options.size).map(|_| rng.gen()).collect(),
        path: (0..options.size).map(|_| rng.gen()).collect(),
    };
    let claim = Claim::from_leaf_witness(rng.gen(), &witness);
    claim.verify(&witness);
    dbg!(claim, witness);

    error!("Variable size trees not implemented yet!");
}
