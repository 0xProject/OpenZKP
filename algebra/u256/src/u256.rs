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

// TODO: impl core::iter::Step so we have ranges

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
