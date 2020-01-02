use crate::{FieldParameters, PrimeField, UInt as FieldUInt};
use num_traits::{Unsigned, Zero};
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
