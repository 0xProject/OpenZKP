use crate::{FieldParameters, MontgomeryParameters, PrimeField};
use std::marker::PhantomData;
use zkp_macros_decl::u256h;
use zkp_u256::{to_montgomery_const, U256};

// TODO: Fix naming
#[allow(clippy::module_name_repetitions)]
pub type FieldElement = PrimeField<U256, Parameters>;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Parameters();

impl MontgomeryParameters<U256> for Parameters {
    const M64: u64 = 0xffff_ffff_ffff_ffff;
    const MODULUS: U256 =
        u256h!("0800000000000011000000000000000000000000000000000000000000000001");
    const R1: U256 = u256h!("07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1");
    const R2: U256 = u256h!("07ffd4ab5e008810ffffffffff6f800000000001330ffffffffffd737e000401");
    const R3: U256 = u256h!("038e5f79873c0a6df47d84f8363000187545706677ffcc06cc7177d1406df18e");
}

impl FieldParameters<U256> for Parameters {
    /// 3, in montgomery form.
    const GENERATOR: U256 =
        u256h!("07fffffffffff9b0ffffffffffffffffffffffffffffffffffffffffffffffa1");
    ///
    const ORDER: U256 = u256h!("0800000000000011000000000000000000000000000000000000000000000000");
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
        let uint = to_montgomery_const(n, &Parameters::MODULUS, Parameters::M64, &Parameters::R2);
        Self {
            uint,
            _parameters: PhantomData,
        }
    }
}

// TODO: Find a way to create generic implementations of these
impl From<FieldElement> for U256 {
    #[inline(always)]
    fn from(other: FieldElement) -> Self {
        other.to_uint()
    }
}

impl From<&FieldElement> for U256 {
    #[inline(always)]
    fn from(other: &FieldElement) -> Self {
        other.to_uint()
    }
}
