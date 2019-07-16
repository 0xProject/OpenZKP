// TODO: #![deny(missing_docs)]
#![warn(clippy::all)]
#![deny(warnings)]
mod channel;
mod fft;
mod fibonacci;
mod merkle;
mod mmap_vec;
mod pedersen_merkle;
mod polynomial;
mod proofs;
mod utils;

pub use merkle::verify;
pub use proofs::{stark_proof, ProofParams};

// Example system
pub use fibonacci::{get_constraint, get_trace_table};

// Exports for benchmarking
// TODO: Avoid publicly exposing.
pub use fft::fft_cofactor;
pub use merkle::make_tree;
