// TODO: #![deny(missing_docs)]
#![warn(clippy::all)]
#![deny(warnings)]
#![cfg_attr(not(feature = "std"), no_std)]
mod curve;
mod jacobian;
mod wnaf;

#[cfg(not(feature = "std"))]
extern crate no_std_compat as std;

pub use curve::Affine;
pub use jacobian::Jacobian;
pub use wnaf::{base_mul, double_base_mul, double_mul, mul, window_table_affine};

// The ECDSA functions are not implemented for security.
#[cfg(feature = "unsafe_ecdsa")]
mod ecdsa;

#[cfg(feature = "unsafe_ecdsa")]
pub use ecdsa::{private_to_public, sign, verify};

use macros_decl::u256h;
use primefield::FieldElement;
use u256::U256;

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
