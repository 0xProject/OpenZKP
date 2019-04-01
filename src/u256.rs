use crate::utils::{adc, div_2_1, mac, sbb};
use hex_literal::*;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::num::Wrapping;
use std::ops::{
    Add, AddAssign, BitAnd, Mul, MulAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

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

/// Hex litterals
#[macro_export]
macro_rules! u256h {
    ($hexstr:expr) => {
        U256::from_bytes_be(hex!($hexstr))
    };
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct U256 {
    pub c0: u64,
    pub c1: u64,
    pub c2: u64,
    pub c3: u64,
}

impl U256 {
    pub const ZERO: U256 = U256::new(0, 0, 0, 0);
    pub const ONE: U256 = U256::new(1, 0, 0, 0);

    #[inline(always)]
    pub const fn new(c0: u64, c1: u64, c2: u64, c3: u64) -> Self {
        Self { c0, c1, c2, c3 }
    }

    #[inline(always)]
    #[allow(clippy::cast_lossless)]
    pub const fn from_bytes_be(n: [u8; 32]) -> Self {
        Self::new(
            u64_from_bytes_be!(n, 0),
            u64_from_bytes_be!(n, 8),
            u64_from_bytes_be!(n, 16),
            u64_from_bytes_be!(n, 24),
        )
    }

    #[inline(always)]
    pub const fn from_u64(n: u64) -> Self {
        Self::new(n, 0, 0, 0)
    }

    #[inline(always)]
    pub const fn from_u128(n: u128) -> Self {
        Self::new(n as u64, (n >> 64) as u64, 0, 0)
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
    pub fn leading_zeros(&self) -> u32 {
        if self.c3 > 0 {
            self.c3.leading_zeros()
        } else if self.c2 > 0 {
            64 + self.c2.leading_zeros()
        } else if self.c1 > 0 {
            128 + self.c1.leading_zeros()
        } else if self.c0 > 0 {
            196 + self.c0.leading_zeros()
        } else {
            256
        }
    }

    #[inline(always)]
    pub fn trailing_zeros(&self) -> u32 {
        if self.c0 > 0 {
            self.c0.trailing_zeros()
        } else if self.c1 > 0 {
            64 + self.c1.trailing_zeros()
        } else if self.c2 > 0 {
            128 + self.c2.trailing_zeros()
        } else if self.c3 > 0 {
            196 + self.c3.trailing_zeros()
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
        let (r4, r5) = mac(r4, self.c1, rhs.c2, carry);
        let (r2, carry) = mac(r2, self.c2, rhs.c0, 0);
        let (r3, carry) = mac(r3, self.c2, rhs.c1, carry);
        let (r4, carry) = mac(r4, self.c2, rhs.c2, carry);
        let (r5, r6) = mac(r5, self.c2, rhs.c2, carry);
        let (r3, carry) = mac(r3, self.c3, rhs.c0, 0);
        let (r4, carry) = mac(r4, self.c3, rhs.c1, carry);
        let (r5, carry) = mac(r5, self.c3, rhs.c2, carry);
        let (r6, r7) = mac(r6, self.c3, rhs.c2, carry);
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

    pub fn mulmod(&self, rhs: &U256, modulus: &U256) -> U256 {
        unimplemented!() // TODO
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
            r *= Wrapping(2) - Wrapping(u128::from(self)) * r; // mod 2^128
            let mut r = U256::from(r.0);
            r *= &(U256::from(2u64) - &(r.clone() * self)); // mod 2^256
            Some(r)
        }
    }

    // Computes the inverse modulo a given modulus
    pub fn invmod(&self, modulus: &U256) -> Option<U256> {
        // Handbook of Applied Cryptography Algorithm 14.61:
        // Binary Extended GCD
        // See note 14.64 on application to modular inverse.
        // The algorithm is modified to work with non-negative numbers.
        // TODO: Constant time algorithm. (Based on fermat's little theorem or
        // constant time Euclids.)
        // TODO: Lehmer's algorithm or other fast GCD
        // TODO: Reduce number of limbs on u and v as computation progresses
        let mut u = self.clone();
        let mut v = modulus.clone();
        let mut a = U256::ONE;
        let mut c = U256::ZERO;
        while u != U256::ZERO {
            while u.is_even() {
                u >>= 1;
                if a.is_odd() {
                    a += modulus;
                }
                a >>= 1;
            }
            while v.is_even() {
                v >>= 1;
                if c.is_odd() {
                    c += modulus;
                }
                c >>= 1;
            }
            if u >= v {
                if a < c {
                    a += modulus;
                }
                u -= &v;
                a -= &c;
            } else {
                if c < a {
                    c += modulus;
                }
                v -= &u;
                c -= &a;
            }
        }
        if v == U256::ONE {
            Some(c)
        } else {
            None
        }
    }
}

impl From<&U256> for u64 {
    fn from(n: &U256) -> u64 {
        n.c0
    }
}

impl From<&U256> for u128 {
    fn from(n: &U256) -> u128 {
        u128::from(n.c0) + (u128::from(n.c1) << 64)
    }
}

impl From<u64> for U256 {
    fn from(n: u64) -> U256 {
        U256::from_u64(n)
    }
}

impl From<u128> for U256 {
    fn from(n: u128) -> U256 {
        U256::from_u128(n)
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

impl ShlAssign<usize> for U256 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: usize) {
        // Note: If RHS is a compile time constant then inlining will allow
        // the branches to be optimized away.
        // TODO: Test optimizing for RHS being exactly 0, 64, 128, ...
        // Note: Test small values first, they are expected to be more common.
        if rhs < 64 {
        self.c3 <<= rhs;
        self.c3 |= self.c2 >> (64 - rhs);
        self.c2 <<= rhs;
        self.c2 |= self.c1 >> (64 - rhs);
        self.c1 <<= rhs;
        self.c1 |= self.c0 >> (64 - rhs);
        self.c0 <<= rhs;
        } else if rhs < 128 {
            self.c3 = self.c2 << (rhs - 64);
            self.c3 |= self.c2 >> (128 - rhs);
            self.c2 = self.c1 << (rhs - 64);
            self.c2 |= self.c1 >> (128 - rhs);
            self.c1 = self.c0 << (rhs - 64);
            self.c0 = 0;
        } else if rhs < 196 {
            self.c3 = self.c1 << (rhs - 128);
            self.c3 |= self.c0 >> (196 - rhs);
            self.c2 = self.c0 << (rhs - 128);
            self.c1 = 0;
            self.c0 = 0;
        } else if rhs < 256 {
            self.c3 = self.c0 << (rhs - 196);
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
        if rhs < 64 {
        self.c0 >>= rhs;
        self.c0 |= self.c1 << (64 - rhs);
        self.c1 >>= rhs;
        self.c1 |= self.c2 << (64 - rhs);
        self.c2 >>= rhs;
        self.c2 |= self.c3 << (64 - rhs);
        self.c3 >>= rhs;
        } else if rhs < 128 {
            self.c0 = self.c1 >> (rhs - 64);
            self.c0 |= self.c2 << (128 - rhs);
            self.c1 = self.c2 >> (rhs - 64);
            self.c1 |= self.c3 << (128 - rhs);
            self.c2 = self.c3 >> (rhs - 64);
            self.c3 = 0;
        } else if rhs < 196 {
            self.c0 = self.c2 >> (rhs - 128);
            self.c0 |= self.c3 << (196 - rhs);
            self.c1 = self.c3 >> (rhs - 128);
            self.c2 = 0;
            self.c3 = 0;
        } else if rhs < 256 {
            self.c0 = self.c3 >> (rhs - 196);
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

impl Add<&U256> for U256 {
    type Output = U256;
    #[inline(always)]
    fn add(mut self, rhs: &U256) -> U256 {
        self += rhs;
        self
    }
}

impl Add<U256> for &U256 {
    type Output = U256;
    #[inline(always)]
    fn add(self, rhs: U256) -> U256 {
        rhs + self
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

impl Sub<&U256> for U256 {
    type Output = U256;
    #[inline(always)]
    fn sub(mut self, rhs: &U256) -> U256 {
        self -= rhs;
        self
    }
}

impl Sub<U256> for &U256 {
    type Output = U256;
    #[inline(always)]
    fn sub(self, rhs: U256) -> U256 {
        self.clone() - &rhs // TODO: inplace algorithm
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

impl Mul<&U256> for U256 {
    type Output = U256;
    #[inline(always)]
    fn mul(mut self, rhs: &U256) -> U256 {
        self *= rhs;
        self
    }
}

impl Mul<U256> for &U256 {
    type Output = U256;
    #[inline(always)]
    fn mul(self, rhs: U256) -> U256 {
        rhs * self
    }
}

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[cfg(test)]
use rand::Rng;

#[cfg(test)]
impl Arbitrary for U256 {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        U256::new(g.gen(), g.gen(), g.gen(), g.gen())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[allow(dead_code)]
    pub const TEST_CONST: U256 =
        u256h!("0800000000000010ffffffffffffffffffffffffffffffffffffffffffffffff");

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

    #[quickcheck]
    #[test]
    fn commutative_add(a: U256, b: U256) -> bool {
        let mut l = a.clone();
        let mut r = b.clone();
        l += &b;
        r += &a;
        l == r
    }

    #[quickcheck]
    #[test]
    fn mul_full_lo(a: U256, b: U256) -> bool {
        let r = a.clone() * &b;
        let (lo, _hi) = a.mul_full(&b);
        r == lo
    }

    #[quickcheck]
    #[test]
    fn test_divrem_u64(a: U256, b: u64) -> bool {
        match a.divrem_u64(b) {
            None => b == 0,
            Some((q, r)) => r < b && q * &U256::from(b) + &U256::from(r) == a,
        }
    }

    #[quickcheck]
    #[test]
    fn invmod256(a: U256) -> bool {
        match a.invmod256() {
            None => true,
            Some(i) => a * &i == U256::ONE,
        }
    }
}
