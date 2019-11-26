mod constraints;
mod inputs;
mod pedersen_points;
mod periodic_columns;
mod trace_table;
mod starkware_example;
mod component;

use env_logger;
use log::{info, error};
use std::time::Instant;
use zkp_macros_decl::{field_element, hex};
use zkp_primefield::FieldElement;
use zkp_stark::{prove, Provable, Component, Constraints};
use zkp_u256::U256;
use std::collections::HashMap;
use starkware_example::{starkware_example};
use structopt::StructOpt;
use component::pedersen_merkle;

use constraints::get_pedersen_merkle_constraints;
use inputs::{Claim, Witness};

// Need to import to active the logging allocator
#[allow(unused_imports)]
use zkp_logging_allocator;


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

    /// Run a size 8192 build-in example
    #[structopt(long)]
    large_example: bool,

    /// Depth of pedersen proof to simulate
    size: usize,
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

    error!("Variable size trees not implemented yet!");
}
