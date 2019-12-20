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
// rand_xoshiro v0.4.0 is required for a zkp-stark example and v0.3.1 for criterion
#![allow(clippy::multiple_crate_versions)]

mod curve;
mod jacobian;
mod wnaf;

#[cfg(not(feature = "std"))]
extern crate no_std_compat as std;

pub use curve::Affine;
pub use jacobian::Jacobian;
pub use wnaf::{base_mul, double_base_mul, double_mul, mul, window_table_affine};

use zkp_macros_decl::u256h;
use zkp_primefield::FieldElement;
use zkp_u256::U256;

// Curve parameters

// Alpha = 1
// Beta  = 0x06f21413efbe40de150e596d72f7a8c5609ad26c15c915c1f4cdfcb99cee9e89
// Order = 0x0800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f

pub const BETA: FieldElement = FieldElement::from_montgomery(u256h!(
    "013931651774247fab8a1e002a41f9476725f2237aab9006359ddd67b59a21ca"
));

pub const ORDER: U256 = u256h!("0800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f");

// x = 0x01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca
// y = 0x005668060aa49730b7be4801df46ec62de53ecd11abe43a32873000c36e8dc1f
pub const GENERATOR: Affine = Affine::Point {
    x: FieldElement::from_montgomery(u256h!(
        "033840300bf6cec10429bf5184041c7b51a9bf65d4403deac9019623cf0273dd"
    )),
    y: FieldElement::from_montgomery(u256h!(
        "05a0e71610f55329fbd89a97cf4b33ad0939e3442869bbe7569d0da34235308a"
    )),
};
