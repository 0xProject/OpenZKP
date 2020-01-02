use crate::{FieldParameters, PrimeField, UInt as FieldUInt};
use num_traits::ToPrimitive;
use std::{convert::From, ops::Neg};
use zkp_u256::U256;

// TODO: Move upstream to `num_traits`
pub trait MaybeSigned {
    /// True if negative
    fn as_sign_abs(self) -> (bool, Self);
}

macro_rules! maybe_unsigned {
    ($type:ident) => {
        impl MaybeSigned for $type {
            #[inline(always)]
            fn as_sign_abs(self) -> (bool, Self) {
                (false, self)
            }
        }
    };
}

macro_rules! maybe_signed {
    ($type:ident) => {
        impl MaybeSigned for $type {
            #[cfg_attr(feature = "inline", inline(always))]
            fn as_sign_abs(self) -> (bool, Self) {
                if self >= 0 {
                    (false, self)
                } else {
                    (true, -self)
                }
            }
        }
    };
}

maybe_unsigned!(u8);
maybe_unsigned!(u16);
maybe_unsigned!(u32);
maybe_unsigned!(u64);
maybe_unsigned!(u128);
maybe_unsigned!(U256);
maybe_unsigned!(usize);

maybe_signed!(i8);
maybe_signed!(i16);
maybe_signed!(i32);
maybe_signed!(i64);
maybe_signed!(i128);
maybe_signed!(isize);

// HACK: Ideally we implement two generic traits based on the `Signed` and
// `Unsigned` traits. but this leads to conflicting implementations and is
// currently unsupported by Rust. We solve this using the `MaybeSigned` trait.

impl<UInt, Parameters, Other> From<Other> for PrimeField<UInt, Parameters>
where
    UInt: FieldUInt + From<Other>,
    Parameters: FieldParameters<UInt>,
    Other: MaybeSigned,
{
    #[cfg_attr(feature = "inline", inline(always))]
    fn from(other: Other) -> Self {
        let (sign, abs) = other.as_sign_abs();
        if sign {
            -Self::from_uint_reduce(&abs.into())
        } else {
            Self::from_uint_reduce(&abs.into())
        }
    }
}

impl<UInt, Parameters> ToPrimitive for PrimeField<UInt, Parameters>
where
    UInt: FieldUInt + ToPrimitive + std::ops::Shr<usize, Output = UInt>,
    Parameters: FieldParameters<UInt>,
{
    fn to_u128(&self) -> Option<u128> {
        self.to_uint().to_u128()
    }

    fn to_i128(&self) -> Option<i128> {
        let mut val = self.to_uint();
        if val < (Parameters::MODULUS >> 1) {
            val.to_i128()
        } else {
            let val = Parameters::MODULUS.sub(&val);
            val.to_i128().and_then(i128::checked_neg)
        }
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_u128().as_ref().and_then(ToPrimitive::to_u64)
    }

    fn to_i64(&self) -> Option<i64> {
        self.to_i128().as_ref().and_then(ToPrimitive::to_i64)
    }
}
