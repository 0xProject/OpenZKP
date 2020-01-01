use crate::{FieldElement, Inv, One};
use std::fmt;
use zkp_u256::U256;

// TODO: Generalize all of these

pub fn invert_batch_src_dst(source: &[FieldElement], destination: &mut [FieldElement]) {
    assert_eq!(source.len(), destination.len());
    let mut accumulator = FieldElement::one();
    for i in 0..source.len() {
        destination[i] = accumulator.clone();
        accumulator *= &source[i];
    }
    accumulator = accumulator.inv().unwrap();
    for i in (0..source.len()).rev() {
        destination[i] *= &accumulator;
        accumulator *= &source[i];
    }
}

pub fn invert_batch(to_be_inverted: &[FieldElement]) -> Vec<FieldElement> {
    if to_be_inverted.is_empty() {
        return Vec::new();
    }
    let n = to_be_inverted.len();
    let mut inverses = cumulative_product(to_be_inverted);

    // TODO: Enforce check to prevent uninvertable elements.
    let mut inverse = inverses[n - 1].inv().unwrap();
    for i in (1..n).rev() {
        inverses[i] = &inverses[i - 1] * &inverse;
        inverse *= &to_be_inverted[i];
    }
    inverses[0] = inverse;
    inverses
}

fn cumulative_product(elements: &[FieldElement]) -> Vec<FieldElement> {
    elements
        .iter()
        .scan(FieldElement::one(), |running_product, x| {
            *running_product *= x;
            Some(running_product.clone())
        })
        .collect()
}

#[cfg(feature = "std")]
impl fmt::Debug for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = U256::from(self);
        write!(
            f,
            "field_element!(\"{:016x}{:016x}{:016x}{:016x}\")",
            n.limb(3),
            n.limb(2),
            n.limb(1),
            n.limb(0)
        )
    }
}

macro_rules! impl_from_uint {
    ($t:ty) => {
        impl From<$t> for FieldElement {
            #[inline(always)]
            fn from(n: $t) -> Self {
                U256::from(n).into()
            }
        }
    };
}

impl_from_uint!(u8);
impl_from_uint!(u16);
impl_from_uint!(u32);
impl_from_uint!(u64);
impl_from_uint!(u128);
impl_from_uint!(usize);

macro_rules! impl_from_int {
    ($t:ty) => {
        impl From<$t> for FieldElement {
            #[cfg_attr(feature = "inline", inline(always))]
            fn from(n: $t) -> Self {
                if n >= 0 {
                    U256::from(n).into()
                } else {
                    -Self::from(U256::from(-n))
                }
            }
        }
    };
}

impl_from_int!(i8);
impl_from_int!(i16);
impl_from_int!(i32);
impl_from_int!(i64);
impl_from_int!(i128);
impl_from_int!(isize);

// The FieldElement versions are called `to_` and not `as_` like their
// U256 counterparts. This is because a `U256::from` is performed which
// does a non-trivial `from_montgomery` conversion.
macro_rules! to_uint {
    ($fname:ident, $uname:ident, $type:ty) => {
        #[inline(always)]
        pub fn $fname(&self) -> $type {
            U256::from(self).$uname()
        }
    };
}

macro_rules! to_int {
    ($fname:ident, $uname:ident, $type:ty) => {
        #[cfg_attr(feature = "inline", inline(always))]
        pub fn $fname(&self) -> $type {
            let n = U256::from(self);
            let half = Self::MODULUS >> 1;
            if n < half {
                n.$uname()
            } else {
                (n - Self::MODULUS).$uname()
            }
        }
    };
}

// We don't want newlines between the macro invocations.
#[rustfmt::skip]
impl FieldElement {
    to_uint!(to_u8, as_u8, u8);
    to_uint!(to_u16, as_u16, u16);
    to_uint!(to_u32, as_u32, u32);
    to_uint!(to_u64, as_u64, u64);
    to_uint!(to_u128, as_u128, u128);
    to_uint!(to_usize, as_usize, usize);
    to_int!(to_i8, as_i8, i8);
    to_int!(to_i16, as_i16, i16);
    to_int!(to_i32, as_i32, i32);
    to_int!(to_i64, as_i64, i64);
    to_int!(to_i128, as_i128, i128);
    to_int!(to_isize, as_isize, isize);
}

impl From<U256> for FieldElement {
    #[inline(always)]
    fn from(n: U256) -> Self {
        (&n).into()
    }
}

impl From<&U256> for FieldElement {
    #[inline(always)]
    fn from(n: &U256) -> Self {
        Self::from_uint_reduce(n)
    }
}

impl From<FieldElement> for U256 {
    #[inline(always)]
    fn from(n: FieldElement) -> Self {
        (&n).into()
    }
}

impl From<&FieldElement> for U256 {
    #[inline(always)]
    fn from(n: &FieldElement) -> Self {
        n.to_uint()
    }
}

#[cfg(any(test, feature = "quickcheck"))]
use quickcheck::{Arbitrary, Gen};

#[cfg(any(test, feature = "quickcheck"))]
impl Arbitrary for FieldElement {
    #[inline(always)]
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        // TODO: Generate 0, 1, p/2 and -1
        Self::from_montgomery(U256::arbitrary(g) % Self::MODULUS)
    }
}

#[allow(unused_macros)]
macro_rules! field_h {
    (- $e:expr) => {
        field_h!($e).neg()
    };
}

// Quickcheck needs pass by value
#[allow(clippy::needless_pass_by_value)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FieldElement, Zero};
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn test_batch_inv(x: Vec<FieldElement>) -> bool {
        if x.iter().any(FieldElement::is_zero) {
            true
        } else {
            invert_batch(x.as_slice())
                .iter()
                .zip(x.iter())
                .all(|(a_inv, a)| *a_inv == a.inv().unwrap())
        }
    }
}
