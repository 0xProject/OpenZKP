// This sequence needs to be repeated in each project as a workaround.
//       See https://github.com/rust-lang/cargo/issues/5034
// For clippy lints see: https://rust-lang.github.io/rust-clippy/master
// For rustc lints see: https://doc.rust-lang.org/rustc/lints/index.html
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(
    // Enable sets of warnings
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    rust_2018_idioms,
    future_incompatible,
    unused,

    // Additional unused warnings (not included in `unused`)
    unused_lifetimes,
    unused_qualifications,
    unused_results,

    // Additional misc. warnings
    anonymous_parameters,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    // missing_docs,
    missing_doc_code_examples,
    private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    variant_size_differences
)]
#![cfg_attr(feature = "std", warn(missing_debug_implementations,))]
// rand_xoshiro v0.4.0 is required for a zkp-stark example and v0.3.1 for criterion
#![allow(clippy::multiple_crate_versions)]
// TODO - Investigate possible truncation
#![allow(clippy::cast_possible_truncation)]
// TODO: Fix false positives
#![allow(clippy::enum_glob_use)]
// TODO: False positives <https://github.com/rust-lang/rust-clippy/issues/5917>
#![allow(clippy::wildcard_imports)]

mod component;
mod inputs;
mod pedersen_points;
mod periodic_columns;
mod starkware_example;

use crate::{component::MerkleTree, inputs::Witness, starkware_example::starkware_example};
use log::{info, trace};
use rand::{prelude::*, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use std::{num::ParseIntError, time::Instant};
use structopt::StructOpt;
use zkp_stark::component::Component;

// Need to import to active the logging allocator
#[allow(unused_imports)]
#[allow(clippy::single_component_path_imports)]
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
// cargo run --release --package zkp-stark --example pedersen_merkle -- -vvv
// --large-example

fn main() {
    // Parse command line options
    let options = Options::from_args();

    // Initialize logging
    env_logger::Builder::new()
        .filter_level(match options.verbose {
            0 => log::LevelFilter::Error,
            1 => log::LevelFilter::Warn,
            2 => log::LevelFilter::Info,
            3 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        })
        .format_timestamp_micros()
        .init();

    // Run specific large example if requested
    if options.large_example {
        let _timer = Timer::default();
        starkware_example();
        return;
    }
    trace!("BEGIN Pedersen-Merkle");

    // Initialize a reproducible random number generator
    let seed = options.seed.unwrap_or_else(random);
    println!("Using random seed {:x}", seed);
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed.into());

    // Generate a random merkle proof instance
    info!("Generating random instance of size {}...", options.size);
    let leaf = rng.gen();
    let path = (0..options.size).map(|_| rng.gen()).collect::<Vec<_>>();
    let witness = Witness::new(leaf, path);
    let claim = (&witness).into();

    info!("Constructing component...");
    let component = MerkleTree::new(witness.path.len());
    println!(
        "Constructing {} x {} trace with {} constraints",
        component.num_polynomials(),
        component.polynomial_size(),
        component.constraints(&claim).len(),
    );

    info!("Constructing proof...");
    let proof = {
        let _timer = Timer::default();
        component.prove(&witness)
    }
    .expect("failed to create proof");
    println!("Proof size is {}", proof.as_bytes().len());

    info!("Verifying proof...");
    component
        .verify(&claim, &proof)
        .expect("Verification failed");

    trace!("END Pedersen-Merkle");
}
