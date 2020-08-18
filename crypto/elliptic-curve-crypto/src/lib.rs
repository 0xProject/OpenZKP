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
// #![allow(clippy::missing_errors_doc)]
// TODO: Add `must_use` attributes
#![allow(clippy::must_use_candidate)]
// TODO: False positives <https://github.com/rust-lang/rust-clippy/issues/5917>
#![allow(clippy::wildcard_imports)]

mod private_key;
mod public_key;
mod signature;

pub use private_key::PrivateKey;
pub use public_key::PublicKey;
pub use signature::Signature;

use std::prelude::v1::*;
use zkp_elliptic_curve::{window_table_affine, Affine, GENERATOR};
#[cfg(not(feature = "std"))]
extern crate no_std_compat as std;
use lazy_static::lazy_static;

lazy_static! {
    static ref GENERATOR_TABLE: [Affine; 32] = {
        let mut naf = <[Affine; 32]>::default();
        window_table_affine(&GENERATOR, &mut naf);
        naf
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use zkp_elliptic_curve::ScalarFieldElement;

    proptest!(
        #[test]
        fn test_ecdsa(digest: ScalarFieldElement, private_key: PrivateKey) {
            let public_key = PublicKey::from(&private_key);
            let signature = private_key.sign(&digest);
            prop_assert!(public_key.verify(&digest, &signature));
        }
    );
}
