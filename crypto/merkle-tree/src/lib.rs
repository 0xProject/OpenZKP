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

#[cfg(not(feature = "std"))]
extern crate no_std_compat as std;

mod commitment;
/// Implements Vector Commitments using Merkle Trees.
///
/// <https://eprint.iacr.org/2011/495.pdf>
// TODO: Spin of to it's own crate.
// TODO: Implement sparse Merkle trees.
// TODO: Generalize over hash implementations.
mod index;
mod node;
mod proof;
mod result;

#[cfg(feature = "prover")]
mod tree;

#[cfg(feature = "prover")]
mod vector_commitment;

pub use commitment::Commitment;
pub use proof::Proof;
pub use result::{Error, Result};

#[cfg(feature = "prover")]
pub use tree::Tree;

#[cfg(feature = "prover")]
pub use vector_commitment::VectorCommitment;

use index::Index;
use node::Node;
