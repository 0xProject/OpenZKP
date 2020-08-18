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
// TODO: Document errors
#![allow(clippy::missing_errors_doc)]
// Some routines have assembly optimized versions available for some
// architectures
// TODO: No asm on stable
// See <https://github.com/rust-lang/rust/issues/29722>
#![cfg_attr(feature = "asm", feature(asm))]
// TODO: Port over to new asm syntax.
#![cfg_attr(feature = "asm", feature(llvm_asm))]

mod additive;
pub(crate) mod algorithms;
mod arch;
mod binary;
mod conversion;
mod division;
mod encoding;
mod functions;
mod multiplicative;
#[cfg(feature = "rand")]
mod rand;
mod traits;
mod u256;
mod u256_traits;

// TODO: Create a BinaryRing trait that represents numbers modulo some power of
// two.

pub use u256::U256;

pub use algorithms::{adc, div_2_1, mac, msb, sbb, to_montgomery_const};
// pub use arch::{divrem_nby1, divrem_nbym};
pub use num_traits::{Bounded, Inv, MulAdd, MulAddAssign, One, Pow, Zero};
pub use traits::{
    AddFullInline, AddInline, Binary, BinaryAssignRef, BinaryOps, BinaryRing, DivRem, InvMod,
    Montgomery, MontgomeryParameters, MulFullInline, MulInline, NegInline, SquareFullInline,
    SquareInline, SubFromFullInline, SubFromInline, SubFullInline, SubInline, GCD,
};

#[cfg(not(feature = "std"))]
extern crate no_std_compat as std;
