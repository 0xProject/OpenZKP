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

extern crate proc_macro;
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub fn hex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    zkp_macros_lib::hex(input.into()).into()
}

#[proc_macro_hack]
pub fn u256h(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    zkp_macros_lib::u256h(input.into()).into()
}

#[proc_macro_hack]
pub fn field_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    zkp_macros_lib::field_element(input.into()).into()
}
