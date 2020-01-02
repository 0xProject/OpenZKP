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
// TODO: Add `must_use` where relevant
#![allow(clippy::must_use_candidate)]
// All `#[inline(always)]` attributes are carefully considered and benchmarked.
// Performance is an important goal of this library.
// TODO: Provide two versions of hot functions `_inlined` and plain.
#![allow(clippy::inline_always)]

mod convert;
pub mod fft;
mod field_element;
pub mod geometric_series;
mod invert_batch;
mod ops;
mod prime_field;
mod proth_field;
#[cfg(feature = "use_rand")]
mod rand;
#[cfg(any(test, feature = "quickcheck"))]
mod test;
mod traits;
mod uint;

// Generic field implementation
pub use prime_field::{FieldParameters, PrimeField};
pub use zkp_u256::MontgomeryParameters;

// The smallest 252-bit Proth field
pub use proth_field::{FieldElement, Parameters};

pub use invert_batch::{invert_batch, invert_batch_src_dst};

// Re-exports dependencies that are part of the public interface
pub use zkp_u256 as u256;

// Export and re-export traits
// TODO: Create a prelude module that contains all the useful ones
pub use traits::{FieldLike, RefFieldLike, Root, SquareRoot};
pub use zkp_u256::{AddInline, Inv, MulInline, NegInline, One, Pow, SquareInline, SubInline, Zero};

pub use uint::UInt;

// Std/no-std imports
#[cfg(not(feature = "std"))]
extern crate no_std_compat as std;
