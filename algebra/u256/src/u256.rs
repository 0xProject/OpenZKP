use crate::{
    commutative_binop,
    division::{divrem_nby1, divrem_nbym},
    gcd::inv_mod,
    noncommutative_binop,
    utils::{adc, div_2_1, mac, sbb},
};
use std::{
    cmp::Ordering,
    num::Wrapping,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Shl,
        ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
    prelude::v1::*,
    u64,
};

#[cfg(feature = "std")]
use std::{fmt, format};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Empty,
    Overflow,
    InnerError(core::num::ParseIntError),
}

impl From<core::num::ParseIntError> for ParseError {
    fn from(error: core::num::ParseIntError) -> Self {
        Self::InnerError(error)
    }
}

#[derive(PartialEq, Eq, Clone, Default, Hash)]
pub struct U256([u64; 4]);

impl U256 {
    pub const MAX: Self = Self::from_limbs(
        u64::max_value(),
        u64::max_value(),
        u64::max_value(),
        u64::max_value(),
    );
    pub const ONE: Self = Self::from_limbs(1, 0, 0, 0);
    pub const ZERO: Self = Self::from_limbs(0, 0, 0, 0);

    pub const fn from_limbs(c0: u64, c1: u64, c2: u64, c3: u64) -> Self {
        Self([c0, c1, c2, c3])
    }

    // It's important that this gets inlined, because `index` is nearly always
    // a compile time constant, which means the range check will get optimized
    // away.
    #[inline(always)]
    pub fn limb(&self, index: usize) -> u64 {
        self.0.get(index).cloned().unwrap_or_default()
    }

    // It's important that this gets inlined, because `index` is nearly always
    // a compile time constant, which means the range check will get optimized
    // away.
    #[inline(always)]
    pub fn set_limb(&mut self, index: usize, value: u64) {
        if let Some(elem) = self.0.get_mut(index) {
            *elem = value;
        } else {
            panic!("Limb out of range.")
        }
    }

    pub fn from_bytes_be(n: &[u8; 32]) -> Self {
        Self::from_limbs(
            u64::from_be_bytes([n[24], n[25], n[26], n[27], n[28], n[29], n[30], n[31]]),
            u64::from_be_bytes([n[16], n[17], n[18], n[19], n[20], n[21], n[22], n[23]]),
            u64::from_be_bytes([n[8], n[9], n[10], n[11], n[12], n[13], n[14], n[15]]),
            u64::from_be_bytes([n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7]]),
        )
    }

    pub fn to_bytes_be(&self) -> [u8; 32] {
        let mut r = [0; 32];
        let mut n = self.clone();
        // We want truncation here
        #[allow(clippy::cast_possible_truncation)]
        for i in (0..32).rev() {
            r[i] = n.limb(0) as u8;
            n >>= 8;
        }
        r
    }

    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    // Can not use Self inside the macro
    #[allow(clippy::use_self)]
    pub fn from_decimal_str(s: &str) -> Result<Self, ParseError> {
        // ceil(2^256 / 10)
        let max10: Self = Self::from_limbs(
            0x9999_9999_9999_999a_u64,
            0x9999_9999_9999_9999_u64,
            0x9999_9999_9999_9999_u64,
            0x1999_9999_9999_9999_u64,
        );
        if s.is_empty() {
            return Err(ParseError::Empty);
        }
        // TODO: Support other radices
        // TODO: Implement as trait
        // OPT: Convert 19 digits at a time using u64.
        let mut result = Self::ZERO;
        for (i, _c) in s.chars().enumerate() {
            if result > max10 {
                return Err(ParseError::Overflow);
            }
            result *= Self::from(10_u64);
            let digit = Self::from(u64::from_str_radix(&s[i..=i], 10)?);
            if &result + &digit < result {
                return Err(ParseError::Overflow);
            }
            result += digit;
        }
        Ok(result)
    }

    pub fn to_decimal_str(&self) -> String {
        if *self == Self::ZERO {
            return "0".to_string();
        }
        let mut result = String::new();
        let mut copy = self.clone();
        while copy > Self::ZERO {
            // OPT: Convert 19 digits at a time using u64.
            let digit = (&copy % Self::from(10_u64)).limb(0);
            result.push_str(&digit.to_string());
            copy /= Self::from(10_u64);
        }
        // Reverse digits
        // Note: Chars are safe here instead of graphemes, because all graphemes
        // are a single codepoint.
        result.chars().rev().collect()
    }

    #[cfg(feature = "std")]
    pub fn from_hex_str(s: &str) -> Self {
        let byte_string = format!("{:0>64}", s.trim_start_matches("0x"));
        let bytes = hex::decode(byte_string).unwrap();
        let mut array = [0_u8; 32];
        array.copy_from_slice(&bytes[..32]);
        Self::from_bytes_be(&array)
    }

    pub fn is_even(&self) -> bool {
        self.limb(0) & 1 == 0
    }

    pub fn is_odd(&self) -> bool {
        self.limb(0) & 1 == 1
    }

    pub fn bits(&self) -> usize {
        256 - self.leading_zeros()
    }

    pub fn msb(&self) -> usize {
        255 - self.leading_zeros()
    }

    pub fn bit(&self, i: usize) -> bool {
        if i < 64 {
            self.limb(0) >> i & 1 == 1
        } else if i < 128 {
            self.limb(1) >> (i - 64) & 1 == 1
        } else if i < 192 {
            self.limb(2) >> (i - 128) & 1 == 1
        } else if i < 256 {
            self.limb(3) >> (i - 192) & 1 == 1
        } else {
            false
        }
    }

    pub fn leading_zeros(&self) -> usize {
        if self.limb(3) > 0 {
            self.limb(3).leading_zeros() as usize
        } else if self.limb(2) > 0 {
            64 + self.limb(2).leading_zeros() as usize
        } else if self.limb(1) > 0 {
            128 + self.limb(1).leading_zeros() as usize
        } else if self.limb(0) > 0 {
            192 + self.limb(0).leading_zeros() as usize
        } else {
            256
        }
    }

    pub fn trailing_zeros(&self) -> usize {
        if self.limb(0) > 0 {
            self.limb(0).trailing_zeros() as usize
        } else if self.limb(1) > 0 {
            64 + self.limb(1).trailing_zeros() as usize
        } else if self.limb(2) > 0 {
            128 + self.limb(2).trailing_zeros() as usize
        } else if self.limb(3) > 0 {
            192 + self.limb(3).trailing_zeros() as usize
        } else {
            256
        }
    }

    // Short division
    // TODO: Can be computed in-place
    pub fn divrem_u64(&self, rhs: u64) -> Option<(Self, u64)> {
        if rhs == 0 {
            None
        } else {
            // Knuth Algorithm S
            // 4 by 1 division
            let (q3, r) = div_2_1(self.limb(3), 0, rhs);
            let (q2, r) = div_2_1(self.limb(2), r, rhs);
            let (q1, r) = div_2_1(self.limb(1), r, rhs);
            let (q0, r) = div_2_1(self.limb(0), r, rhs);
            Some((Self::from_limbs(q0, q1, q2, q3), r))
        }
    }

    // Long division
    pub fn divrem(&self, rhs: &Self) -> Option<(Self, Self)> {
        let mut numerator = [self.limb(0), self.limb(1), self.limb(2), self.limb(3), 0];
        if rhs.limb(3) > 0 {
            // divrem_nby4
            divrem_nbym(&mut numerator, &mut [
                rhs.limb(0),
                rhs.limb(1),
                rhs.limb(2),
                rhs.limb(3),
            ]);
            Some((
                Self::from_limbs(numerator[4], 0, 0, 0),
                Self::from_limbs(numerator[0], numerator[1], numerator[2], numerator[3]),
            ))
        } else if rhs.limb(2) > 0 {
            // divrem_nby3
            divrem_nbym(&mut numerator, &mut [rhs.limb(0), rhs.limb(1), rhs.limb(2)]);
            Some((
                Self::from_limbs(numerator[3], numerator[4], 0, 0),
                Self::from_limbs(numerator[0], numerator[1], numerator[2], 0),
            ))
        } else if rhs.limb(1) > 0 {
            // divrem_nby2
            divrem_nbym(&mut numerator, &mut [rhs.limb(0), rhs.limb(1)]);
            Some((
                Self::from_limbs(numerator[2], numerator[3], numerator[4], 0),
                Self::from_limbs(numerator[0], numerator[1], 0, 0),
            ))
        } else if rhs.limb(0) > 0 {
            let remainder = divrem_nby1(&mut numerator, rhs.limb(0));
            Some((
                Self::from_limbs(numerator[0], numerator[1], numerator[2], numerator[3]),
                Self::from_limbs(remainder, 0, 0, 0),
            ))
        } else {
            None
        }
    }

    // Computes the inverse modulo 2^256
    pub fn invmod256(&self) -> Option<Self> {
        if self.is_even() {
            None
        } else {
            // Invert using Hensel lifted Newton-Rhapson itteration
            // See: https://arxiv.org/abs/1303.0328
            // r[2] = 3 * self XOR 2 mod 2^4
            // r[n+1] = r[n] * (1 - self * r[n]) mod 2^(2^n)
            let c = Wrapping(self.limb(0));
            let mut r: Wrapping<u64> = (Wrapping(3) * c) ^ Wrapping(2); // mod 2^4
            r *= Wrapping(2) - c * r; // mod 2^8
            r *= Wrapping(2) - c * r; // mod 2^16
            r *= Wrapping(2) - c * r; // mod 2^32
            r *= Wrapping(2) - c * r; // mod 2^64
            let mut r = Wrapping(u128::from(r.0));
            r *= Wrapping(2) - Wrapping(self.as_u128()) * r; // mod 2^128
            let mut r = Self::from(r.0);
            r *= &(Self::from(2_u64) - &(r.clone() * self)); // mod 2^256
            Some(r)
        }
    }

    // Computes the inverse modulo a given modulus
    pub fn invmod(&self, modulus: &Self) -> Option<Self> {
        inv_mod(modulus, self)
    }
}

macro_rules! impl_from_uint {
    ($type:ty) => {
        impl From<$type> for U256 {
            // $type could be u64, which triggers the lint.
            #[allow(trivial_numeric_casts)]
            fn from(n: $type) -> Self {
                Self::from_limbs(n as u64, 0, 0, 0)
            }
        }
    };
}

impl_from_uint!(u8);
impl_from_uint!(u16);
impl_from_uint!(u32);
impl_from_uint!(u64);
impl_from_uint!(usize);

impl From<u128> for U256 {
    fn from(n: u128) -> Self {
        // We want truncation here
        #[allow(clippy::cast_possible_truncation)]
        Self::from_limbs(n as u64, (n >> 64) as u64, 0, 0)
    }
}

macro_rules! impl_from_int {
    ($t:ty) => {
        impl From<$t> for U256 {
            // We want twos-complement casting
            #[allow(clippy::cast_sign_loss)]
            // We want truncation here
            #[allow(clippy::cast_possible_truncation)]
            fn from(n: $t) -> Self {
                if n >= 0 {
                    Self::from_limbs(n as u64, 0, 0, 0)
                } else {
                    Self::from_limbs(
                        n as u64,
                        u64::max_value(),
                        u64::max_value(),
                        u64::max_value(),
                    )
                }
            }
        }
    };
}

impl_from_int!(i8);
impl_from_int!(i16);
impl_from_int!(i32);
impl_from_int!(i64);
impl_from_int!(isize);

impl From<i128> for U256 {
    // We want twos-complement casting
    #[allow(clippy::cast_sign_loss)]
    // We want truncation here
    #[allow(clippy::cast_possible_truncation)]
    fn from(n: i128) -> Self {
        if n >= 0 {
            Self::from_limbs(n as u64, (n >> 64) as u64, 0, 0)
        } else {
            Self::from_limbs(
                n as u64,
                (n >> 64) as u64,
                u64::max_value(),
                u64::max_value(),
            )
        }
    }
}

macro_rules! as_int {
    ($name:ident, $type:ty) => {
        // $type could be u64, which triggers the lint.
        #[allow(trivial_numeric_casts)]
        pub fn $name(&self) -> $type {
            self.limb(0) as $type
        }
    };
}

// We don't want newlines between the macro invocations.
#[rustfmt::skip]
impl U256 {
    as_int!(as_u8, u8);
    as_int!(as_u16, u16);
    as_int!(as_u32, u32);
    as_int!(as_u64, u64);
    as_int!(as_usize, usize);
    as_int!(as_i8, i8);
    as_int!(as_i16, i16);
    as_int!(as_i32, i32);
    as_int!(as_i64, i64);
    as_int!(as_isize, isize);

    // Clippy is afraid that casting u64 to u128 is lossy
    #[allow(clippy::cast_lossless)]
    pub fn as_u128(&self) -> u128 {
        (self.limb(0) as u128) | ((self.limb(1) as u128) << 64)
    }

    // Clippy is afraid that casting u64 to u128 is lossy
    #[allow(clippy::cast_lossless)]
    pub fn as_i128(&self) -> i128 {
        (self.limb(0) as i128) | ((self.limb(1) as i128) << 64)
    }
}

#[cfg(feature = "std")]
impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:016x}{:016x}{:016x}{:016x}",
            self.limb(3),
            self.limb(2),
            self.limb(1),
            self.limb(0)
        )
    }
}

#[cfg(feature = "std")]
impl fmt::Debug for U256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "u256h!(\"{:016x}{:016x}{:016x}{:016x}\")",
            self.limb(3),
            self.limb(2),
            self.limb(1),
            self.limb(0)
        )
    }
}

impl PartialOrd for U256 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for U256 {
    fn cmp(&self, other: &Self) -> Ordering {
        let t = self.limb(3).cmp(&other.limb(3));
        if t != Ordering::Equal {
            return t;
        }
        let t = self.limb(2).cmp(&other.limb(2));
        if t != Ordering::Equal {
            return t;
        }
        let t = self.limb(1).cmp(&other.limb(1));
        if t != Ordering::Equal {
            return t;
        }
        self.limb(0).cmp(&other.limb(0))
    }
}

// Useful for checking divisability by small powers of two
impl BitAnd<u64> for &U256 {
    type Output = u64;

    fn bitand(self, rhs: u64) -> u64 {
        self.limb(0) & rhs
    }
}

impl BitAndAssign<&U256> for U256 {
    fn bitand_assign(&mut self, rhs: &Self) {
        self.set_limb(0, self.limb(0) & rhs.limb(0));
        self.set_limb(1, self.limb(1) & rhs.limb(1));
        self.set_limb(2, self.limb(2) & rhs.limb(2));
        self.set_limb(3, self.limb(3) & rhs.limb(3));
    }
}

impl ShlAssign<usize> for U256 {
    fn shl_assign(&mut self, rhs: usize) {
        // Note: If RHS is a compile time constant then inlining will allow
        // the branches to be optimized away.
        // Note: Test small values first, they are expected to be more common.
        // Note: We need to handle 0, 64, 128, 192 specially because `>> 0` is
        //       illegal.
        let mut c0 = self.limb(0);
        let mut c1 = self.limb(1);
        let mut c2 = self.limb(2);
        let mut c3 = self.limb(3);
        if rhs == 0 {
        } else if rhs < 64 {
            c3 <<= rhs;
            c3 |= c2 >> (64 - rhs);
            c2 <<= rhs;
            c2 |= c1 >> (64 - rhs);
            c1 <<= rhs;
            c1 |= c0 >> (64 - rhs);
            c0 <<= rhs;
        } else if rhs == 64 {
            c3 = c2;
            c2 = c1;
            c1 = c0;
            c0 = 0;
        } else if rhs < 128 {
            c3 = c2 << (rhs - 64);
            c3 |= c1 >> (128 - rhs);
            c2 = c1 << (rhs - 64);
            c2 |= c0 >> (128 - rhs);
            c1 = c0 << (rhs - 64);
            c0 = 0;
        } else if rhs == 128 {
            c3 = c1;
            c2 = c0;
            c1 = 0;
            c0 = 0;
        } else if rhs < 192 {
            c3 = c1 << (rhs - 128);
            c3 |= c0 >> (192 - rhs);
            c2 = c0 << (rhs - 128);
            c1 = 0;
            c0 = 0;
        } else if rhs == 192 {
            c3 = c0;
            c2 = 0;
            c1 = 0;
            c0 = 0;
        } else if rhs < 256 {
            c3 = c0 << (rhs - 192);
            c2 = 0;
            c1 = 0;
            c0 = 0;
        } else {
            c3 = 0;
            c2 = 0;
            c1 = 0;
            c0 = 0;
        }
        self.set_limb(0, c0);
        self.set_limb(1, c1);
        self.set_limb(2, c2);
        self.set_limb(3, c3);
    }
}

impl Shl<usize> for U256 {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> Self {
        self <<= rhs;
        self
    }
}

impl ShrAssign<usize> for U256 {
    fn shr_assign(&mut self, rhs: usize) {
        // Note: If RHS is a compile time constant then inlining will allow
        // the branches to be optimized away.
        // TODO: Test optimizing for RHS being exactly 0, 64, 128, ...
        // Note: Test small values first, they are expected to be more common.
        let mut c0 = self.limb(0);
        let mut c1 = self.limb(1);
        let mut c2 = self.limb(2);
        let mut c3 = self.limb(3);
        if rhs == 0 {
        } else if rhs < 64 {
            c0 >>= rhs;
            c0 |= c1 << (64 - rhs);
            c1 >>= rhs;
            c1 |= c2 << (64 - rhs);
            c2 >>= rhs;
            c2 |= c3 << (64 - rhs);
            c3 >>= rhs;
        } else if rhs == 64 {
            c0 = c1;
            c1 = c2;
            c2 = c3;
            c3 = 0;
        } else if rhs < 128 {
            c0 = c1 >> (rhs - 64);
            c0 |= c2 << (128 - rhs);
            c1 = c2 >> (rhs - 64);
            c1 |= c3 << (128 - rhs);
            c2 = c3 >> (rhs - 64);
            c3 = 0;
        } else if rhs == 128 {
            c0 = c2;
            c1 = c3;
            c2 = 0;
            c3 = 0;
        } else if rhs < 192 {
            c0 = c2 >> (rhs - 128);
            c0 |= c3 << (192 - rhs);
            c1 = c3 >> (rhs - 128);
            c2 = 0;
            c3 = 0;
        } else if rhs == 192 {
            c0 = c3;
            c1 = 0;
            c2 = 0;
            c3 = 0;
        } else if rhs < 256 {
            c0 = c3 >> (rhs - 192);
            c1 = 0;
            c2 = 0;
            c3 = 0;
        } else {
            c0 = 0;
            c1 = 0;
            c2 = 0;
            c3 = 0;
        }
        self.set_limb(0, c0);
        self.set_limb(1, c1);
        self.set_limb(2, c2);
        self.set_limb(3, c3);
    }
}

impl Shr<usize> for U256 {
    type Output = Self;

    fn shr(mut self, rhs: usize) -> Self {
        self >>= rhs;
        self
    }
}

impl AddAssign<&U256> for U256 {
    fn add_assign(&mut self, rhs: &Self) {
        let (c0, carry) = adc(self.limb(0), rhs.limb(0), 0);
        let (c1, carry) = adc(self.limb(1), rhs.limb(1), carry);
        let (c2, carry) = adc(self.limb(2), rhs.limb(2), carry);
        let (c3, _) = adc(self.limb(3), rhs.limb(3), carry);
        self.set_limb(0, c0);
        self.set_limb(1, c1);
        self.set_limb(2, c2);
        self.set_limb(3, c3);
    }
}

impl SubAssign<&U256> for U256 {
    fn sub_assign(&mut self, rhs: &Self) {
        let (c0, borrow) = sbb(self.limb(0), rhs.limb(0), 0);
        let (c1, borrow) = sbb(self.limb(1), rhs.limb(1), borrow);
        let (c2, borrow) = sbb(self.limb(2), rhs.limb(2), borrow);
        let (c3, _) = sbb(self.limb(3), rhs.limb(3), borrow);
        self.set_limb(0, c0);
        self.set_limb(1, c1);
        self.set_limb(2, c2);
        self.set_limb(3, c3);
    }
}

impl DivAssign<&U256> for U256 {
    fn div_assign(&mut self, rhs: &Self) {
        let (q, _r) = self.divrem(rhs).unwrap();
        *self = q;
    }
}

impl RemAssign<&U256> for U256 {
    fn rem_assign(&mut self, rhs: &Self) {
        let (_q, r) = self.divrem(rhs).unwrap();
        *self = r;
    }
}

commutative_binop!(U256, Add, add, AddAssign, add_assign);
commutative_binop!(U256, BitAnd, bitand, BitAndAssign, bitand_assign);
noncommutative_binop!(U256, Sub, sub, SubAssign, sub_assign);
noncommutative_binop!(U256, Div, div, DivAssign, div_assign);
noncommutative_binop!(U256, Rem, rem, RemAssign, rem_assign);

#[cfg(any(test, feature = "quickcheck"))]
use quickcheck::{Arbitrary, Gen};

// TODO: Generate a quasi-random sequence.
// See http://extremelearning.com.au/unreasonable-effectiveness-of-quasirandom-sequences/
#[cfg(any(test, feature = "quickcheck"))]
impl Arbitrary for U256 {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Self::from_limbs(
            u64::arbitrary(g),
            u64::arbitrary(g),
            u64::arbitrary(g),
            u64::arbitrary(g),
        )
    }
}

// TODO: Replace literals with u256h!
#[allow(clippy::unreadable_literal)]
// Quickcheck requires pass by value
#[allow(clippy::needless_pass_by_value)]
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    use zkp_macros_decl::u256h;

    #[allow(dead_code)]
    const TEST_CONST: U256 =
        u256h!("0800000000000010ffffffffffffffffffffffffffffffffffffffffffffffff");

    #[test]
    fn test_from_decimal_str() {
        assert_eq!(U256::from_decimal_str(""), Err(ParseError::Empty));
        assert_eq!(U256::from_decimal_str("0"), Ok(U256::ZERO));
        assert_eq!(U256::from_decimal_str("00"), Ok(U256::ZERO));
        assert_eq!(U256::from_decimal_str("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"), Ok(U256::ZERO));
        assert_eq!(U256::from_decimal_str("1"), Ok(U256::ONE));
        assert_eq!(
            U256::from_decimal_str(
                "115792089237316195423570985008687907853269984665640564039457584007913129639935"
            ),
            Ok(u256h!(
                "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
            ))
        );
        assert_eq!(
            U256::from_decimal_str(
                "115792089237316195423570985008687907853269984665640564039457584007913129639936"
            ),
            Err(ParseError::Overflow)
        );
        assert_eq!(
            U256::from_decimal_str(
                "1000000000000000000000000000000000000000000000000000000000000000000000000000000"
            ),
            Err(ParseError::Overflow)
        );
        assert!(U256::from_decimal_str("12a3").is_err());
    }

    #[quickcheck]
    fn test_decimal_to_from(n: U256) -> bool {
        let decimal = n.clone().to_decimal_str();
        let m = U256::from_decimal_str(&decimal).unwrap();
        n == m
    }

    #[test]
    fn test_shl() {
        let mut n = U256::from_limbs(
            0x9050e39a8638969f,
            0xd7cc21c004c428d1,
            0x9026e34ec8fb83ac,
            0x03d4679634263e15,
        );
        let e = U256::from_limbs(
            0xcd431c4b4f800000,
            0xe002621468c82871,
            0xa7647dc1d66be610,
            0xcb1a131f0ac81371,
        );
        n <<= 23;
        assert_eq!(n, e);
    }

    #[test]
    fn test_shr() {
        let mut n = U256::from_limbs(
            0xbe1897b996367829,
            0x24c4cd2cacd2e3be,
            0xa0a61c4de933a54e,
            0x059e0db9d96add73,
        );
        let e = U256::from_limbs(
            0xa5c77d7c312f732c,
            0x674a9c49899a5959,
            0xd5bae7414c389bd2,
            0x0000000b3c1b73b2,
        );
        n >>= 23;
        assert_eq!(n, e);
    }

    #[test]
    fn test_add() {
        let mut a = U256::from_limbs(
            0x7209a73f5af87656,
            0x99223186ad9732d3,
            0xd403de023ea32bf3,
            0x01b54cf967a0f4f0,
        );
        let b = U256::from_limbs(
            0xabe25acf4f460ee0,
            0x627c6bdf52bd869e,
            0x403390a0497c51ab,
            0x041aa3e6140810ca,
        );
        let e = U256::from_limbs(
            0x1dec020eaa3e8536,
            0xfb9e9d660054b972,
            0x14376ea2881f7d9e,
            0x05cff0df7ba905bb,
        );
        a += &b;
        assert_eq!(a, e);
    }

    #[test]
    fn test_sub() {
        let mut a = U256::from_limbs(
            0x281c7cfb32e98dd8,
            0x9018b2a04f60102b,
            0xd6e32fb1e0564153,
            0x02d005315d1af15f,
        );
        let b = U256::from_limbs(
            0x407666ddda2343ae,
            0xb4dd92954c5a0860,
            0x237cf6a1c121a335,
            0x05d6ce1edbd1908a,
        );
        let e = U256::from_limbs(
            0xe7a6161d58c64a2a,
            0xdb3b200b030607ca,
            0xb36639101f349e1d,
            0xfcf93712814960d5,
        );
        a -= &b;
        assert_eq!(a, e);
    }

    #[test]
    fn test_invmod256() {
        let a = U256::from_limbs(
            0xf80aa815a36a7e47,
            0x090be90cfa96712a,
            0xf52ec0a4083d2c14,
            0x05405dfd1d1c1a97,
        );
        let e = U256::from_limbs(
            0xf0a9a0091b3bcb77,
            0x42d3eba6084ca0de,
            0x60d848b6513392d7,
            0xdf45026654d086d6,
        );
        let r = a.invmod256().unwrap();
        assert_eq!(r, e);
    }

    #[test]
    fn test_invmod_small() {
        let n = U256::from_limbs(271, 0, 0, 0);
        let m = U256::from_limbs(383, 0, 0, 0);
        let i = U256::from_limbs(106, 0, 0, 0);
        let r = n.invmod(&m).unwrap();
        assert_eq!(i, r);
    }

    #[test]
    fn test_invmod() {
        let m = U256::from_limbs(
            0x0000000000000001,
            0x0000000000000000,
            0x0000000000000000,
            0x0800000000000011,
        );
        let n = U256::from_limbs(
            0x1717f47973471ed5,
            0xe106229070982941,
            0xd82120c54277c73e,
            0x07717a21e77894e8,
        );
        let i = U256::from_limbs(
            0xbda5eaad406f66d1,
            0xfac4d8e66130d944,
            0x97c88939cbce8317,
            0x001752ce51d19c97,
        );
        let r = n.invmod(&m).unwrap();
        assert_eq!(i, r);
    }

    #[quickcheck]
    fn commutative_add(a: U256, b: U256) -> bool {
        let mut l = a.clone();
        let mut r = b.clone();
        l += &b;
        r += &a;
        l == r
    }

    #[quickcheck]
    fn test_divrem_u64(a: U256, b: u64) -> bool {
        match a.divrem_u64(b) {
            None => b == 0,
            Some((q, r)) => r < b && q * &U256::from(b) + &U256::from(r) == a,
        }
    }

    #[quickcheck]
    fn test_divrem(a: U256, b: U256) -> bool {
        match a.divrem(&b) {
            None => b == U256::ZERO,
            Some((q, r)) => r < b && q * &b + &r == a,
        }
    }

    #[quickcheck]
    fn invmod256(a: U256) -> bool {
        match a.invmod256() {
            None => true,
            Some(i) => a * &i == U256::ONE,
        }
    }
}
