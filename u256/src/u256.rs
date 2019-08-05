use crate::{
    commutative_binop,
    division::{divrem_nby1, divrem_nbym},
    gcd::inv_mod,
    noncommutative_binop, u256h,
    utils::{adc, div_2_1, mac, sbb},
};
use hex_literal::*;
use std::{
    cmp::Ordering,
    fmt,
    num::Wrapping,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Shl,
        ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
    u64,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Empty,
    Overflow,
    InnerError(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(error: std::num::ParseIntError) -> Self {
        ParseError::InnerError(error)
    }
}

// We can't use `u64::from` here because it is not a const fn.
// You'd want to use this with `#[allow(clippy::cast_lossless)]` to
// surpress a clippy false positive (u8 -> u64 is always safe).
macro_rules! u64_from_bytes_be {
    ($arr:ident, $offset:expr) => {
        ($arr[$offset + 0] as u64) << 56
            | ($arr[$offset + 1] as u64) << 48
            | ($arr[$offset + 2] as u64) << 40
            | ($arr[$offset + 3] as u64) << 32
            | ($arr[$offset + 4] as u64) << 24
            | ($arr[$offset + 5] as u64) << 16
            | ($arr[$offset + 6] as u64) << 8
            | ($arr[$offset + 7] as u64)
    };
}

#[macro_export]
macro_rules! u256h {
    ($hexstr:expr) => {
        U256::from_bytes_be(&hex!($hexstr))
    };
}

#[derive(PartialEq, Eq, Clone, Default)]
pub struct U256 {
    pub c0: u64,
    pub c1: u64,
    pub c2: u64,
    pub c3: u64,
}

impl U256 {
    pub const MAX: U256 = U256::new(u64::MAX, u64::MAX, u64::MAX, u64::MAX);
    pub const ONE: U256 = U256::new(1, 0, 0, 0);
    pub const ZERO: U256 = U256::new(0, 0, 0, 0);

    #[inline(always)]
    pub const fn new(c0: u64, c1: u64, c2: u64, c3: u64) -> Self {
        Self { c0, c1, c2, c3 }
    }

    #[inline(always)]
    #[allow(clippy::cast_lossless)]
    pub const fn from_bytes_be(n: &[u8; 32]) -> Self {
        Self::new(
            u64_from_bytes_be!(n, 24),
            u64_from_bytes_be!(n, 16),
            u64_from_bytes_be!(n, 8),
            u64_from_bytes_be!(n, 0),
        )
    }

    pub fn to_bytes_be(&self) -> [u8; 32] {
        let mut r = [0; 32];
        let mut n = self.clone();
        for i in (0..32).rev() {
            r[i] = n.c0 as u8;
            n >>= 8;
        }
        r
    }

    #[inline(always)]
    #[allow(clippy::cast_lossless)]
    pub const fn from_slice(limbs: &[u32; 8]) -> Self {
        // TODO: Remove
        Self::new(
            ((limbs[1] as u64) << 32) | (limbs[0] as u64),
            ((limbs[3] as u64) << 32) | (limbs[2] as u64),
            ((limbs[5] as u64) << 32) | (limbs[4] as u64),
            ((limbs[7] as u64) << 32) | (limbs[6] as u64),
        )
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == U256::ZERO
    }

    pub fn from_decimal_str(s: &str) -> Result<U256, ParseError> {
        if s.is_empty() {
            return Err(ParseError::Empty);
        }
        // ceil(2^256 / 10)
        const MAX10: U256 =
            u256h!("199999999999999999999999999999999999999999999999999999999999999a");
        // TODO: Support other radices
        // TODO: Implement as trait
        // OPT: Convert 19 digits at a time using u64.
        let mut result = U256::ZERO;
        for (i, _c) in s.chars().enumerate() {
            if result > MAX10 {
                return Err(ParseError::Overflow);
            }
            result *= U256::from(10u64);
            let digit = U256::from(u64::from_str_radix(&s[i..=i], 10)?);
            if &result + &digit < result {
                return Err(ParseError::Overflow);
            }
            result += digit;
        }
        Ok(result)
    }

    pub fn to_decimal_str(&self) -> String {
        if *self == U256::ZERO {
            return "0".to_string();
        }
        let mut result = String::new();
        let mut copy = self.clone();
        while copy > U256::ZERO {
            // OPT: Convert 19 digits at a time using u64.
            let digit = (&copy % U256::from(10u64)).c0;
            result.push_str(&digit.to_string());
            copy /= U256::from(10u64);
        }
        // Reverse digits
        // Note: Chars are safe here instead of graphemes, because all graphemes
        // are a single codepoint.
        result.chars().rev().collect()
    }

    pub fn from_hex_str(s: &str) -> U256 {
        let byte_string = format!("{:0>64}", s.trim_start_matches("0x"));
        let bytes = hex::decode(byte_string).unwrap();
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes[..32]);
        U256::from_bytes_be(&array)
    }

    #[inline(always)]
    pub const fn is_even(&self) -> bool {
        self.c0 & 1 == 0
    }

    #[inline(always)]
    pub const fn is_odd(&self) -> bool {
        self.c0 & 1 == 1
    }

    #[inline(always)]
    pub fn bits(&self) -> usize {
        256 - self.leading_zeros()
    }

    #[inline(always)]
    pub fn msb(&self) -> usize {
        255 - self.leading_zeros()
    }

    #[inline(always)]
    pub fn bit(&self, i: usize) -> bool {
        if i < 64 {
            self.c0 >> i & 1 == 1
        } else if i < 128 {
            self.c1 >> (i - 64) & 1 == 1
        } else if i < 192 {
            self.c2 >> (i - 128) & 1 == 1
        } else if i < 256 {
            self.c3 >> (i - 192) & 1 == 1
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn leading_zeros(&self) -> usize {
        if self.c3 > 0 {
            self.c3.leading_zeros() as usize
        } else if self.c2 > 0 {
            64 + self.c2.leading_zeros() as usize
        } else if self.c1 > 0 {
            128 + self.c1.leading_zeros() as usize
        } else if self.c0 > 0 {
            192 + self.c0.leading_zeros() as usize
        } else {
            256
        }
    }

    #[inline(always)]
    pub fn trailing_zeros(&self) -> usize {
        if self.c0 > 0 {
            self.c0.trailing_zeros() as usize
        } else if self.c1 > 0 {
            64 + self.c1.trailing_zeros() as usize
        } else if self.c2 > 0 {
            128 + self.c2.trailing_zeros() as usize
        } else if self.c3 > 0 {
            192 + self.c3.trailing_zeros() as usize
        } else {
            256
        }
    }

    #[inline(always)]
    pub const fn mul_full(&self, rhs: &U256) -> (U256, U256) {
        let (r0, carry) = mac(0, self.c0, rhs.c0, 0);
        let (r1, carry) = mac(0, self.c0, rhs.c1, carry);
        let (r2, carry) = mac(0, self.c0, rhs.c2, carry);
        let (r3, r4) = mac(0, self.c0, rhs.c3, carry);
        let (r1, carry) = mac(r1, self.c1, rhs.c0, 0);
        let (r2, carry) = mac(r2, self.c1, rhs.c1, carry);
        let (r3, carry) = mac(r3, self.c1, rhs.c2, carry);
        let (r4, r5) = mac(r4, self.c1, rhs.c3, carry);
        let (r2, carry) = mac(r2, self.c2, rhs.c0, 0);
        let (r3, carry) = mac(r3, self.c2, rhs.c1, carry);
        let (r4, carry) = mac(r4, self.c2, rhs.c2, carry);
        let (r5, r6) = mac(r5, self.c2, rhs.c3, carry);
        let (r3, carry) = mac(r3, self.c3, rhs.c0, 0);
        let (r4, carry) = mac(r4, self.c3, rhs.c1, carry);
        let (r5, carry) = mac(r5, self.c3, rhs.c2, carry);
        let (r6, r7) = mac(r6, self.c3, rhs.c3, carry);
        (U256::new(r0, r1, r2, r3), U256::new(r4, r5, r6, r7))
    }

    #[inline(always)]
    pub const fn sqr_full(&self) -> (U256, U256) {
        let (r1, carry) = mac(0, self.c0, self.c1, 0);
        let (r2, carry) = mac(0, self.c0, self.c2, carry);
        let (r3, r4) = mac(0, self.c0, self.c3, carry);
        let (r3, carry) = mac(r3, self.c1, self.c2, 0);
        let (r4, r5) = mac(r4, self.c1, self.c3, carry);
        let (r5, r6) = mac(r5, self.c2, self.c3, 0);
        let r7 = r6 >> 63;
        let r6 = (r6 << 1) | (r5 >> 63);
        let r5 = (r5 << 1) | (r4 >> 63);
        let r4 = (r4 << 1) | (r3 >> 63);
        let r3 = (r3 << 1) | (r2 >> 63);
        let r2 = (r2 << 1) | (r1 >> 63);
        let r1 = r1 << 1;
        let (r0, carry) = mac(0, self.c0, self.c0, 0);
        let (r1, carry) = adc(r1, 0, carry);
        let (r2, carry) = mac(r2, self.c1, self.c1, carry);
        let (r3, carry) = adc(r3, 0, carry);
        let (r4, carry) = mac(r4, self.c2, self.c2, carry);
        let (r5, carry) = adc(r5, 0, carry);
        let (r6, carry) = mac(r6, self.c3, self.c3, carry);
        let (r7, _carry) = adc(r7, 0, carry);
        (U256::new(r0, r1, r2, r3), U256::new(r4, r5, r6, r7))
    }

    // Short division
    // TODO: Can be computed in-place
    pub fn divrem_u64(&self, rhs: u64) -> Option<(U256, u64)> {
        if rhs == 0 {
            None
        } else {
            // Knuth Algorithm S
            // 4 by 1 division
            let (q3, r) = div_2_1(self.c3, 0, rhs);
            let (q2, r) = div_2_1(self.c2, r, rhs);
            let (q1, r) = div_2_1(self.c1, r, rhs);
            let (q0, r) = div_2_1(self.c0, r, rhs);
            Some((U256::new(q0, q1, q2, q3), r))
        }
    }

    // Long division
    pub fn divrem(&self, rhs: &U256) -> Option<(U256, U256)> {
        let mut numerator = [self.c0, self.c1, self.c2, self.c3, 0];
        if rhs.c3 > 0 {
            divrem_nbym(&mut numerator, &mut [rhs.c0, rhs.c1, rhs.c2, rhs.c3]);
            Some((
                U256::new(numerator[4], 0, 0, 0),
                U256::new(numerator[0], numerator[1], numerator[2], numerator[3]),
            ))
        } else if rhs.c2 > 0 {
            divrem_nbym(&mut numerator, &mut [rhs.c0, rhs.c1, rhs.c2]);
            Some((
                U256::new(numerator[3], numerator[4], 0, 0),
                U256::new(numerator[0], numerator[1], numerator[2], 0),
            ))
        } else if rhs.c1 > 0 {
            divrem_nbym(&mut numerator, &mut [rhs.c0, rhs.c1]);
            Some((
                U256::new(numerator[2], numerator[3], numerator[4], 0),
                U256::new(numerator[0], numerator[1], 0, 0),
            ))
        } else if rhs.c0 > 0 {
            let remainder = divrem_nby1(&mut numerator, rhs.c0);
            Some((
                U256::new(numerator[0], numerator[1], numerator[2], numerator[3]),
                U256::new(remainder, 0, 0, 0),
            ))
        } else {
            None
        }
    }

    pub fn mulmod(&self, rhs: &U256, modulus: &U256) -> U256 {
        let (lo, hi) = self.mul_full(rhs);
        let mut numerator = [lo.c0, lo.c1, lo.c2, lo.c3, hi.c0, hi.c1, hi.c2, hi.c3, 0];
        if modulus.c3 > 0 {
            divrem_nbym(&mut numerator, &mut [
                modulus.c0, modulus.c1, modulus.c2, modulus.c3,
            ]);
            U256::new(numerator[0], numerator[1], numerator[2], numerator[3])
        } else if modulus.c2 > 0 {
            divrem_nbym(&mut numerator, &mut [modulus.c0, modulus.c1, modulus.c2]);
            U256::new(numerator[0], numerator[1], numerator[2], 0)
        } else if modulus.c1 > 0 {
            divrem_nbym(&mut numerator, &mut [modulus.c0, modulus.c1]);
            U256::new(numerator[0], numerator[1], 0, 0)
        } else if modulus.c0 > 0 {
            let remainder = divrem_nby1(&mut numerator, modulus.c0);
            U256::new(remainder, 0, 0, 0)
        } else {
            panic!(); // TODO: return Option<>
        }
    }

    // Computes the inverse modulo 2^256
    pub fn invmod256(&self) -> Option<U256> {
        if self.is_even() {
            None
        } else {
            // Invert using Hensel lifted Newton-Rhapson itteration
            // See: https://arxiv.org/abs/1303.0328
            // r[2] = 3 * self XOR 2 mod 2^4
            // r[n+1] = r[n] * (1 - self * r[n]) mod 2^(2^n)
            let c = Wrapping(self.c0);
            let mut r: Wrapping<u64> = (Wrapping(3) * c) ^ Wrapping(2); // mod 2^4
            r *= Wrapping(2) - c * r; // mod 2^8
            r *= Wrapping(2) - c * r; // mod 2^16
            r *= Wrapping(2) - c * r; // mod 2^32
            r *= Wrapping(2) - c * r; // mod 2^64
            let mut r = Wrapping(u128::from(r.0));
            r *= Wrapping(2) - Wrapping(self.as_u128()) * r; // mod 2^128
            let mut r = U256::from(r.0);
            r *= &(U256::from(2u64) - &(r.clone() * self)); // mod 2^256
            Some(r)
        }
    }

    // Computes the inverse modulo a given modulus
    pub fn invmod(&self, modulus: &U256) -> Option<U256> {
        inv_mod(modulus, self)
    }

    pub fn pow(&self, exponent: u64) -> Option<U256> {
        if self.is_zero() && (exponent == 0) {
            None
        } else {
            let mut result = U256::ONE;
            let mut remaining_exponent = exponent;
            let mut square = self.clone();
            while remaining_exponent > 0 {
                if remaining_exponent % 2 == 1 {
                    result *= &square;
                }
                remaining_exponent >>= 1;
                // OPT - eliminate .clone()
                square *= square.clone();
            }
            Some(result)
        }
    }
}

macro_rules! impl_from_uint {
    ($t:ty) => {
        impl From<$t> for U256 {
            fn from(n: $t) -> U256 {
                Self::new(n as u64, 0, 0, 0)
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
    fn from(n: u128) -> U256 {
        Self::new(n as u64, (n >> 64) as u64, 0, 0)
    }
}

macro_rules! impl_from_int {
    ($t:ty) => {
        impl From<$t> for U256 {
            fn from(n: $t) -> U256 {
                if n >= 0 {
                    Self::new(n as u64, 0, 0, 0)
                } else {
                    Self::new(
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
    fn from(n: i128) -> U256 {
        if n >= 0 {
            Self::new(n as u64, (n >> 64) as u64, 0, 0)
        } else {
            Self::new(
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
        pub fn $name(&self) -> $type {
            self.c0 as $type
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
        (self.c0 as u128) | ((self.c1 as u128) << 64)
    }

    // Clippy is afraid that casting u64 to u128 is lossy
    #[allow(clippy::cast_lossless)]
    pub fn as_i128(&self) -> i128 {
        (self.c0 as i128) | ((self.c1 as i128) << 64)
    }
}

impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:016x}{:016x}{:016x}{:016x}",
            self.c3, self.c2, self.c1, self.c0
        )
    }
}

impl fmt::Debug for U256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "u256h!(\"{:016x}{:016x}{:016x}{:016x}\")",
            self.c3, self.c2, self.c1, self.c0
        )
    }
}

impl PartialOrd for U256 {
    #[inline(always)]
    fn partial_cmp(&self, other: &U256) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for U256 {
    #[inline(always)]
    fn cmp(&self, other: &U256) -> Ordering {
        let t = self.c3.cmp(&other.c3);
        if t != Ordering::Equal {
            return t;
        }
        let t = self.c2.cmp(&other.c2);
        if t != Ordering::Equal {
            return t;
        }
        let t = self.c1.cmp(&other.c1);
        if t != Ordering::Equal {
            return t;
        }
        self.c0.cmp(&other.c0)
    }
}

// Useful for checking divisability by small powers of two
impl BitAnd<u64> for &U256 {
    type Output = u64;

    #[inline(always)]
    fn bitand(self, rhs: u64) -> u64 {
        self.c0 & rhs
    }
}

impl BitAndAssign<&U256> for U256 {
    fn bitand_assign(&mut self, rhs: &U256) {
        self.c0 &= rhs.c0;
        self.c1 &= rhs.c1;
        self.c2 &= rhs.c2;
        self.c3 &= rhs.c3;
    }
}

impl ShlAssign<usize> for U256 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: usize) {
        // Note: If RHS is a compile time constant then inlining will allow
        // the branches to be optimized away.
        // Note: Test small values first, they are expected to be more common.
        // Note: We need to handle 0, 64, 128, 192 specially because `>> 0` is
        //       illegal.
        if rhs == 0 {
        } else if rhs < 64 {
            self.c3 <<= rhs;
            self.c3 |= self.c2 >> (64 - rhs);
            self.c2 <<= rhs;
            self.c2 |= self.c1 >> (64 - rhs);
            self.c1 <<= rhs;
            self.c1 |= self.c0 >> (64 - rhs);
            self.c0 <<= rhs;
        } else if rhs == 64 {
            self.c3 = self.c2;
            self.c2 = self.c1;
            self.c1 = self.c0;
            self.c0 = 0;
        } else if rhs < 128 {
            self.c3 = self.c2 << (rhs - 64);
            self.c3 |= self.c1 >> (128 - rhs);
            self.c2 = self.c1 << (rhs - 64);
            self.c2 |= self.c0 >> (128 - rhs);
            self.c1 = self.c0 << (rhs - 64);
            self.c0 = 0;
        } else if rhs == 128 {
            self.c3 = self.c1;
            self.c2 = self.c0;
            self.c1 = 0;
            self.c0 = 0;
        } else if rhs < 192 {
            self.c3 = self.c1 << (rhs - 128);
            self.c3 |= self.c0 >> (192 - rhs);
            self.c2 = self.c0 << (rhs - 128);
            self.c1 = 0;
            self.c0 = 0;
        } else if rhs == 192 {
            self.c3 = self.c0;
            self.c2 = 0;
            self.c1 = 0;
            self.c0 = 0;
        } else if rhs < 256 {
            self.c3 = self.c0 << (rhs - 192);
            self.c2 = 0;
            self.c1 = 0;
            self.c0 = 0;
        } else {
            self.c3 = 0;
            self.c2 = 0;
            self.c1 = 0;
            self.c0 = 0;
        }
    }
}

impl Shl<usize> for U256 {
    type Output = U256;

    #[inline(always)]
    fn shl(mut self, rhs: usize) -> U256 {
        self <<= rhs;
        self
    }
}

impl ShrAssign<usize> for U256 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: usize) {
        // Note: If RHS is a compile time constant then inlining will allow
        // the branches to be optimized away.
        // TODO: Test optimizing for RHS being exactly 0, 64, 128, ...
        // Note: Test small values first, they are expected to be more common.
        if rhs == 0 {
        } else if rhs < 64 {
            self.c0 >>= rhs;
            self.c0 |= self.c1 << (64 - rhs);
            self.c1 >>= rhs;
            self.c1 |= self.c2 << (64 - rhs);
            self.c2 >>= rhs;
            self.c2 |= self.c3 << (64 - rhs);
            self.c3 >>= rhs;
        } else if rhs == 64 {
            self.c0 = self.c1;
            self.c1 = self.c2;
            self.c2 = self.c3;
            self.c3 = 0;
        } else if rhs < 128 {
            self.c0 = self.c1 >> (rhs - 64);
            self.c0 |= self.c2 << (128 - rhs);
            self.c1 = self.c2 >> (rhs - 64);
            self.c1 |= self.c3 << (128 - rhs);
            self.c2 = self.c3 >> (rhs - 64);
            self.c3 = 0;
        } else if rhs == 128 {
            self.c0 = self.c2;
            self.c1 = self.c3;
            self.c2 = 0;
            self.c3 = 0;
        } else if rhs < 192 {
            self.c0 = self.c2 >> (rhs - 128);
            self.c0 |= self.c3 << (192 - rhs);
            self.c1 = self.c3 >> (rhs - 128);
            self.c2 = 0;
            self.c3 = 0;
        } else if rhs == 192 {
            self.c0 = self.c3;
            self.c1 = 0;
            self.c2 = 0;
            self.c3 = 0;
        } else if rhs < 256 {
            self.c0 = self.c3 >> (rhs - 192);
            self.c1 = 0;
            self.c2 = 0;
            self.c3 = 0;
        } else {
            self.c0 = 0;
            self.c1 = 0;
            self.c2 = 0;
            self.c3 = 0;
        }
    }
}

impl Shr<usize> for U256 {
    type Output = U256;

    #[inline(always)]
    fn shr(mut self, rhs: usize) -> U256 {
        self >>= rhs;
        self
    }
}

impl AddAssign<&U256> for U256 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: &U256) {
        let (t, carry) = adc(self.c0, rhs.c0, 0);
        self.c0 = t;
        let (t, carry) = adc(self.c1, rhs.c1, carry);
        self.c1 = t;
        let (t, carry) = adc(self.c2, rhs.c2, carry);
        self.c2 = t;
        let (t, _carry) = adc(self.c3, rhs.c3, carry);
        self.c3 = t;
    }
}

impl SubAssign<&U256> for U256 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: &U256) {
        let (t, borrow) = sbb(self.c0, rhs.c0, 0);
        self.c0 = t;
        let (t, borrow) = sbb(self.c1, rhs.c1, borrow);
        self.c1 = t;
        let (t, borrow) = sbb(self.c2, rhs.c2, borrow);
        self.c2 = t;
        let (t, _borrow) = sbb(self.c3, rhs.c3, borrow);
        self.c3 = t;
    }
}

impl MulAssign<&U256> for U256 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: &U256) {
        let (r0, carry) = mac(0, self.c0, rhs.c0, 0);
        let (r1, carry) = mac(0, self.c0, rhs.c1, carry);
        let (r2, carry) = mac(0, self.c0, rhs.c2, carry);
        let (r3, _carry) = mac(0, self.c0, rhs.c3, carry);
        self.c0 = r0;
        let (r1, carry) = mac(r1, self.c1, rhs.c0, 0);
        let (r2, carry) = mac(r2, self.c1, rhs.c1, carry);
        let (r3, _carry) = mac(r3, self.c1, rhs.c2, carry);
        self.c1 = r1;
        let (r2, carry) = mac(r2, self.c2, rhs.c0, 0);
        let (r3, _carry) = mac(r3, self.c2, rhs.c1, carry);
        self.c2 = r2;
        let (r3, _carry) = mac(r3, self.c3, rhs.c0, 0);
        self.c3 = r3;
    }
}

impl DivAssign<&U256> for U256 {
    fn div_assign(&mut self, rhs: &U256) {
        let (q, _r) = self.divrem(rhs).unwrap();
        *self = q;
    }
}

impl RemAssign<&U256> for U256 {
    fn rem_assign(&mut self, rhs: &U256) {
        let (_q, r) = self.divrem(rhs).unwrap();
        *self = r;
    }
}

commutative_binop!(U256, Add, add, AddAssign, add_assign);
commutative_binop!(U256, Mul, mul, MulAssign, mul_assign);
commutative_binop!(U256, BitAnd, bitand, BitAndAssign, bitand_assign);
noncommutative_binop!(U256, Sub, sub, SubAssign, sub_assign);
noncommutative_binop!(U256, Div, div, DivAssign, div_assign);
noncommutative_binop!(U256, Rem, rem, RemAssign, rem_assign);

impl MulAssign<u64> for U256 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: u64) {
        let (r0, carry) = mac(0, self.c0, rhs, 0);
        let (r1, carry) = mac(0, self.c1, rhs, carry);
        let (r2, carry) = mac(0, self.c2, rhs, carry);
        let (r3, _carry) = mac(0, self.c3, rhs, carry);
        self.c0 = r0;
        self.c1 = r1;
        self.c2 = r2;
        self.c3 = r3;
    }
}

impl Mul<u64> for U256 {
    type Output = U256;

    #[inline(always)]
    fn mul(mut self, /* mut */ rhs: u64) -> U256 {
        self.mul_assign(rhs);
        self
    }
}

impl Mul<u64> for &U256 {
    type Output = U256;

    #[inline(always)]
    fn mul(self, rhs: u64) -> U256 {
        self.clone().mul(rhs)
    }
}

impl Mul<U256> for u64 {
    type Output = U256;

    #[inline(always)]
    fn mul(self, rhs: U256) -> U256 {
        rhs.mul(self)
    }
}

impl Mul<&U256> for u64 {
    type Output = U256;

    #[inline(always)]
    fn mul(self, rhs: &U256) -> U256 {
        rhs.mul(self)
    }
}

impl MulAssign<u128> for U256 {
    // We need `>>` to implement mul
    #[allow(clippy::suspicious_op_assign_impl)]
    #[inline(always)]
    fn mul_assign(&mut self, rhs: u128) {
        let lo = rhs as u64;
        let hi = (rhs >> 64) as u64;
        let (r0, carry) = mac(0, self.c0, lo, 0);
        let (r1, carry) = mac(0, self.c1, lo, carry);
        let (r2, carry) = mac(0, self.c2, lo, carry);
        let (r3, _carry) = mac(0, self.c3, lo, carry);
        let (r1, carry) = mac(r1, self.c0, hi, 0);
        let (r2, carry) = mac(r2, self.c1, hi, carry);
        let (r3, _carry) = mac(r3, self.c2, hi, carry);
        self.c0 = r0;
        self.c1 = r1;
        self.c2 = r2;
        self.c3 = r3;
    }
}

impl Mul<u128> for U256 {
    type Output = U256;

    #[inline(always)]
    fn mul(mut self, rhs: u128) -> U256 {
        self.mul_assign(rhs);
        self
    }
}

impl Mul<u128> for &U256 {
    type Output = U256;

    #[inline(always)]
    fn mul(self, rhs: u128) -> U256 {
        self.clone().mul(rhs)
    }
}

impl Mul<U256> for u128 {
    type Output = U256;

    #[inline(always)]
    fn mul(self, rhs: U256) -> U256 {
        rhs.mul(self)
    }
}

impl Mul<&U256> for u128 {
    type Output = U256;

    #[inline(always)]
    fn mul(self, rhs: &U256) -> U256 {
        rhs.mul(self)
    }
}

#[cfg(any(test, feature = "quickcheck"))]
use quickcheck::{Arbitrary, Gen};

#[cfg(any(test, feature = "quickcheck"))]
impl Arbitrary for U256 {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        U256::new(
            u64::arbitrary(g),
            u64::arbitrary(g),
            u64::arbitrary(g),
            u64::arbitrary(g),
        )
    }
}

// TODO: Replace literals with u256h!
#[allow(clippy::unreadable_literal)]
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[allow(dead_code)]
    pub const TEST_CONST: U256 =
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
        let mut n = U256::new(
            0x9050e39a8638969f,
            0xd7cc21c004c428d1,
            0x9026e34ec8fb83ac,
            0x03d4679634263e15,
        );
        let e = U256::new(
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
        let mut n = U256::new(
            0xbe1897b996367829,
            0x24c4cd2cacd2e3be,
            0xa0a61c4de933a54e,
            0x059e0db9d96add73,
        );
        let e = U256::new(
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
        let mut a = U256::new(
            0x7209a73f5af87656,
            0x99223186ad9732d3,
            0xd403de023ea32bf3,
            0x01b54cf967a0f4f0,
        );
        let b = U256::new(
            0xabe25acf4f460ee0,
            0x627c6bdf52bd869e,
            0x403390a0497c51ab,
            0x041aa3e6140810ca,
        );
        let e = U256::new(
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
        let mut a = U256::new(
            0x281c7cfb32e98dd8,
            0x9018b2a04f60102b,
            0xd6e32fb1e0564153,
            0x02d005315d1af15f,
        );
        let b = U256::new(
            0x407666ddda2343ae,
            0xb4dd92954c5a0860,
            0x237cf6a1c121a335,
            0x05d6ce1edbd1908a,
        );
        let e = U256::new(
            0xe7a6161d58c64a2a,
            0xdb3b200b030607ca,
            0xb36639101f349e1d,
            0xfcf93712814960d5,
        );
        a -= &b;
        assert_eq!(a, e);
    }

    #[test]
    fn test_mul() {
        let mut a = U256::new(
            0x11daab4a80b1cf9a,
            0x147ac29a5c5db4d4,
            0xb378f759c80c1d3a,
            0x02a2b5155bee10dc,
        );
        let b = U256::new(
            0x81aa26a88e9edd46,
            0xadb0ffe4dfb4a10f,
            0xc3a61b547a1f01ad,
            0x0554a84aa321a31c,
        );
        let e = U256::new(
            0x02cd4f6e3de2b61c,
            0x364935c057086115,
            0xb912b5cf544f5866,
            0x507ca4a96b4a328a,
        );
        a *= &b;
        assert_eq!(a, e);
    }

    #[test]
    fn test_mul_full() {
        let a = U256::new(
            0xcef29c5de9ccefc1,
            0x1f0363af6e0e89e0,
            0x2edfffcc3ce19c1c,
            0x0533aefb3249d52d,
        );
        let b = U256::new(
            0x7aedeade9e192566,
            0xbde10917fae93c03,
            0x3419d1ecf392f766,
            0x03027f1aaf32c3fe,
        );
        let elo = U256::new(
            0xc34784904e276be6,
            0x19f527745e55f913,
            0x1b805a30c8f277c6,
            0x360d66c911328f7a,
        );
        let ehi = U256::new(
            0x41f3f98d2b4a4d5c,
            0x2fdba3d97ab78ebe,
            0x5b3854220ea8f86c,
            0x000fa8097e2b023a,
        );
        let (rlo, rhi) = a.mul_full(&b);
        assert_eq!(rlo, elo);
        assert_eq!(rhi, ehi);
    }

    #[test]
    fn test_invmod256() {
        let a = U256::new(
            0xf80aa815a36a7e47,
            0x090be90cfa96712a,
            0xf52ec0a4083d2c14,
            0x05405dfd1d1c1a97,
        );
        let e = U256::new(
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
        let n = U256::new(271, 0, 0, 0);
        let m = U256::new(383, 0, 0, 0);
        let i = U256::new(106, 0, 0, 0);
        let r = n.invmod(&m).unwrap();
        assert_eq!(i, r);
    }

    #[test]
    fn test_invmod() {
        let m = U256::new(
            0x0000000000000001,
            0x0000000000000000,
            0x0000000000000000,
            0x0800000000000011,
        );
        let n = U256::new(
            0x1717f47973471ed5,
            0xe106229070982941,
            0xd82120c54277c73e,
            0x07717a21e77894e8,
        );
        let i = U256::new(
            0xbda5eaad406f66d1,
            0xfac4d8e66130d944,
            0x97c88939cbce8317,
            0x001752ce51d19c97,
        );
        let r = n.invmod(&m).unwrap();
        assert_eq!(i, r);
    }

    #[test]
    fn test_mulmod() {
        let a = U256::new(
            0xb7eb3137d7271553,
            0xf44101622499c849,
            0x6364b9150f381299,
            0x0487868a9c0b15bb,
        );
        let b = U256::new(
            0xee5c3e0c95ea3606,
            0xb5d23720247b076a,
            0x125d5c1cc549a496,
            0x02fa68e3d326247a,
        );
        let m = U256::new(
            0x04893c41700b0160,
            0x9ba854d08388861e,
            0x834be37ce5dd881f,
            0x0000000425a6a188,
        );
        let e = U256::new(
            0x14527949a28bfa32,
            0xa388ec81a8763eae,
            0x35b22ffb468ed013,
            0x000000032b77bd60,
        );
        let r = a.mulmod(&b, &m);
        assert_eq!(r, e);
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
    fn mul_full_lo(a: U256, b: U256) -> bool {
        let r = a.clone() * &b;
        let (lo, _hi) = a.mul_full(&b);
        r == lo
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

    #[quickcheck]
    fn square(a: U256) -> bool {
        a.sqr_full() == a.mul_full(&a)
    }
}
