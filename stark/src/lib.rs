// TODO: #![deny(missing_docs)]
#![warn(clippy::all)]
#![deny(warnings)]
mod channel;
mod constraint_system;
mod constraints;
mod fft;
mod fibonacci;
mod fibonacci2;
mod merkle;
mod mmap_vec;
mod polynomial;
mod proofs;
mod rational_expression;
mod trace_table;
mod utils;

pub use merkle::verify;
pub use proofs::{stark_proof, ProofParams};

// Example system
pub use fibonacci::{get_constraint, get_trace_table};

// Exports for benchmarking
// TODO: Avoid publicly exposing.
pub use fft::fft_cofactor_bit_reversed;
pub use merkle::make_tree;
