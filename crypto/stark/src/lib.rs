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
// TODO: Toggle based on stable/nightly
#![allow(clippy::missing_errors_doc)]
// TODO: Add `must_use` attributes
#![allow(clippy::must_use_candidate)]
// TODO: To many false positives
#![allow(clippy::enum_glob_use)]
// TODO: False positives <https://github.com/rust-lang/rust-clippy/issues/5917>
#![allow(clippy::wildcard_imports)]

mod channel;
mod constraints;
mod polynomial;
mod proof;
mod proof_of_work;
mod rational_expression;
#[cfg(feature = "std")]
mod solidity_seralizer;
#[cfg(feature = "std")]
mod solidity_verifier;
mod traits;
mod verifier;

// Optional prover functionality. Note that prover requires std.
// TODO: Make it work without std.
#[cfg(feature = "prover")]
mod algebraic_dag;
#[cfg(feature = "prover")]
pub mod component;
#[cfg(feature = "prover")]
mod constraint_check;
#[cfg(feature = "prover")]
mod prover;
#[cfg(feature = "prover")]
mod rational_equality;
#[cfg(feature = "prover")]
mod trace_table;
// TODO: Have unconditional Debug trait on all types

// In no std mode, substitute no_std_compat
#[cfg(not(feature = "std"))]
#[cfg_attr(feature = "std", macro_use)]
extern crate no_std_compat as std;

// Re-exports dependencies that are part of the public interface
pub use zkp_primefield as primefield;

// Exports for verifier
pub use constraints::{Constraints, Error as ConstraintError};
pub use polynomial::DensePolynomial;
pub use proof::Proof;
pub use rational_expression::RationalExpression;
pub use traits::Verifiable;
pub use verifier::{verify, Error as VerifierError};

// We want std for this so that we can use hex encode
#[cfg(feature = "std")]
pub use solidity_seralizer::proof_serialize;
#[cfg(feature = "std")]
pub use solidity_verifier::generate;

// Exports for prover
#[cfg(feature = "prover")]
pub use constraint_check::check_constraints;
#[cfg(feature = "prover")]
pub use prover::{prove, Error as ProverError};
#[cfg(feature = "prover")]
pub use trace_table::TraceTable;
#[cfg(feature = "prover")]
pub use traits::Provable;

#[cfg(test)]
mod tests {
    pub(crate) fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
}
