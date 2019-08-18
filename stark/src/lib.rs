// TODO: #![deny(missing_docs)]
#![warn(clippy::all)]
#![deny(warnings)]
#![cfg_attr(not(feature = "std"), no_std)]
mod channel;
mod fft;
pub mod fibonacci;
mod hash;
mod hashable;
mod masked_keccak;
mod merkle;
mod pedersen_merkle;
mod polynomial;
mod proofs;
mod trace_table;
mod utils;
mod verifier;

pub use merkle::verify;
pub use proofs::{stark_proof, ProofParams};
pub use trace_table::TraceTable;
pub use verifier::check_proof;

// In no std mode, substitute no_std_compat
#[cfg(not(feature = "std"))]
extern crate no_std_compat as std;

// Conditionally include MmapVec. If the feature is disabled substitute Vec
// instead.
#[cfg(feature = "mmap")]
mod mmap_vec;
#[cfg(not(feature = "mmap"))]
mod mmap_vec {
    pub use std::vec::Vec as MmapVec;
}

// Exports for benchmarking
// TODO: Avoid publicly exposing.
pub use fft::fft_cofactor_bit_reversed;
pub use merkle::make_tree;
