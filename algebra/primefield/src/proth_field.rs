use crate::{Parameters, PrimeField};
#[cfg(feature = "parity_codec")]
use parity_scale_codec::{Decode, Encode};
use std::marker::PhantomData;
use zkp_macros_decl::u256h;
use zkp_u256::{to_montgomery_const, U256};

// TODO: Fix naming
#[allow(clippy::module_name_repetitions)]
pub type FieldElement = PrimeField<Proth>;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
#[cfg_attr(feature = "parity_codec", derive(Encode, Decode))]
pub struct Proth();

impl Parameters for Proth {
    type UInt = U256;

    /// 3, in montgomery form.
    const GENERATOR: U256 =
        u256h!("07fffffffffff9b0ffffffffffffffffffffffffffffffffffffffffffffffa1");
    const M64: u64 = 0xffff_ffff_ffff_ffff;
    const MODULUS: U256 =
        u256h!("0800000000000011000000000000000000000000000000000000000000000001");
    ///
    const ORDER: U256 = u256h!("0800000000000011000000000000000000000000000000000000000000000000");
    const R1: U256 = u256h!("07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1");
    const R2: U256 = u256h!("07ffd4ab5e008810ffffffffff6f800000000001330ffffffffffd737e000401");
    const R3: U256 = u256h!("038e5f79873c0a6df47d84f8363000187545706677ffcc06cc7177d1406df18e");
}

impl FieldElement {
    /// Creates a constant value from a `U256` constant in Montgomery form.
    // TODO: Make member of `Field` after <https://github.com/rust-lang/rust/issues/57563>
    pub const fn from_montgomery_const(uint: U256) -> Self {
        Self {
            uint,
            _parameters: PhantomData,
        }
    }

    /// Creates a constant value from a `U256` constant.
    ///
    /// It does compile-time conversion to Montgomery form.
    // TODO: Make member of `Field` after <https://github.com/rust-lang/rust/issues/57563>
    pub const fn from_uint_const(n: &U256) -> Self {
        let uint = to_montgomery_const(n, &Proth::MODULUS, Proth::M64, &Proth::R2);
        Self {
            uint,
            _parameters: PhantomData,
        }
    }
}
