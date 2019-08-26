// TODO: #![deny(missing_docs)]
#![warn(clippy::all)]
#![deny(warnings)]
#![cfg_attr(not(feature = "std"), no_std)]
mod channel;
mod constraint;
mod fft;
pub mod fibonacci;
mod geometric_series;
mod hash;
mod hashable;
mod masked_keccak;
mod merkle;
mod polynomial;
mod proof_params;
mod utils;
mod verifier;

pub use channel::{ProverChannel, VerifierChannel};
pub use proof_params::ProofParams;
pub use verifier::check_proof;

// In no std mode, substitute no_std_compat
#[cfg(not(feature = "std"))]
#[cfg_attr(feature = "std", macro_use)]
extern crate no_std_compat as std;

// Conditionally include MmapVec. If the feature is disabled substitute Vec
// instead.
#[cfg(feature = "mmap")]
mod mmap_vec;
#[cfg(not(feature = "mmap"))]
mod mmap_vec {
    pub use std::vec::Vec as MmapVec;
}

// Prover functionality is only available if the feature is set. Currently
// requires std.
// TODO: Make it work without std.

// Optional prover functionality. Note that prover requires std.
#[cfg(feature = "prover")]
pub mod pedersen_merkle;
#[cfg(feature = "prover")]
mod proofs;
#[cfg(feature = "prover")]
mod trace_table;

// Exports for prover
#[cfg(feature = "prover")]
pub use merkle::verify;
#[cfg(feature = "prover")]
pub use proofs::stark_proof;
#[cfg(feature = "prover")]
pub use trace_table::TraceTable;

// Exports for benchmarking
// TODO: Avoid publicly exposing.
pub use fft::fft_cofactor_bit_reversed;
#[cfg(feature = "prover")]
pub use merkle::make_tree;
