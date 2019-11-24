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

mod channel;
mod constraints;
mod polynomial;
mod proof;
mod proof_of_work;
mod rational_expression;
mod traits;
mod verifier;

// Optional prover functionality. Note that prover requires std.
// TODO: Make it work without std.
#[cfg(feature = "prover")]
mod algebraic_dag;
#[cfg(feature = "prover")]
mod constraint_check;
#[cfg(feature = "prover")]
mod prover;
#[cfg(feature = "prover")]
mod trace_table;
#[cfg(feature = "prover")]
mod component;

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

// Exports for prover
#[cfg(feature = "prover")]
pub use prover::{prove, Error as ProverError};
#[cfg(feature = "prover")]
pub use trace_table::TraceTable;
#[cfg(feature = "prover")]
pub use traits::Provable;

#[cfg(test)]
mod tests {
    use env_logger;

    pub(crate) fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
}
