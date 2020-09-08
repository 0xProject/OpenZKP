// False positive: attribute has a use
#[allow(clippy::useless_attribute)]
// False positive: Importing preludes is allowed
#[allow(clippy::wildcard_imports)]
use std::prelude::v1::*;

use crate::{
    algorithms::{adc, mac},
    arch::{divrem_nby1, divrem_nbym},
    assign_ops_from_trait, self_ops_from_trait, MulFullInline, MulInline, SquareFullInline,
    SquareInline, U256,
};
use num_traits::Pow;
use std::{
    ops::{Mul, MulAssign},
    u64,
};

// Multiplicative operations: Mul, square, mulmod, pow, etc. routines

impl SquareFullInline for U256 {
    // We shadow carry for readability
    #[allow(clippy::shadow_unrelated)]
    #[inline(always)]
    fn square_full_inline(&self) -> (Self, Self) {
        let (r1, carry) = mac(0, self.limb(0), self.limb(1), 0);
        let (r2, carry) = mac(0, self.limb(0), self.limb(2), carry);
        let (r3, r4) = mac(0, self.limb(0), self.limb(3), carry);
        let (r3, carry) = mac(r3, self.limb(1), self.limb(2), 0);
        let (r4, r5) = mac(r4, self.limb(1), self.limb(3), carry);
        let (r5, r6) = mac(r5, self.limb(2), self.limb(3), 0);
        let r7 = r6 >> 63;
        let r6 = (r6 << 1) | (r5 >> 63);
        let r5 = (r5 << 1) | (r4 >> 63);
        let r4 = (r4 << 1) | (r3 >> 63);
        let r3 = (r3 << 1) | (r2 >> 63);
        let r2 = (r2 << 1) | (r1 >> 63);
        let r1 = r1 << 1;
        let (r0, carry) = mac(0, self.limb(0), self.limb(0), 0);
        let (r1, carry) = adc(r1, 0, carry);
        let (r2, carry) = mac(r2, self.limb(1), self.limb(1), carry);
        let (r3, carry) = adc(r3, 0, carry);
        let (r4, carry) = mac(r4, self.limb(2), self.limb(2), carry);
        let (r5, carry) = adc(r5, 0, carry);
        let (r6, carry) = mac(r6, self.limb(3), self.limb(3), carry);
        let (r7, _) = adc(r7, 0, carry);
        (
            Self::from_limbs([r0, r1, r2, r3]),
            Self::from_limbs([r4, r5, r6, r7]),
        )
    }
}

impl SquareInline for U256 {
    #[inline(always)]
    fn square_inline(&self) -> Self {
        self.square_full_inline().0
    }
}

impl MulInline<u64> for U256 {
    #[inline(always)]
    fn mul_inline(&self, rhs: u64) -> Self {
        self.mul_full_inline(rhs).0
    }
}

impl MulFullInline<u64> for U256 {
    type High = u64;

    #[inline(always)]
    fn mul_full_inline(&self, rhs: u64) -> (Self, u64) {
        let (r0, carry) = mac(0, self.limb(0), rhs, 0);
        let (r1, carry) = mac(0, self.limb(1), rhs, carry);
        let (r2, carry) = mac(0, self.limb(2), rhs, carry);
        let (r3, carry) = mac(0, self.limb(3), rhs, carry);
        (Self::from_limbs([r0, r1, r2, r3]), carry)
    }
}

impl MulInline<&U256> for U256 {
    // We shadow carry for readability
    #[allow(clippy::shadow_unrelated)]
    #[inline(always)]
    fn mul_inline(&self, rhs: &Self) -> Self {
        let (r0, carry) = mac(0, self.limb(0), rhs.limb(0), 0);
        let (r1, carry) = mac(0, self.limb(0), rhs.limb(1), carry);
        let (r2, carry) = mac(0, self.limb(0), rhs.limb(2), carry);
        let (r3, _) = mac(0, self.limb(0), rhs.limb(3), carry);
        let (r1, carry) = mac(r1, self.limb(1), rhs.limb(0), 0);
        let (r2, carry) = mac(r2, self.limb(1), rhs.limb(1), carry);
        let (r3, _) = mac(r3, self.limb(1), rhs.limb(2), carry);
        let (r2, carry) = mac(r2, self.limb(2), rhs.limb(0), 0);
        let (r3, _) = mac(r3, self.limb(2), rhs.limb(1), carry);
        let (r3, _) = mac(r3, self.limb(3), rhs.limb(0), 0);
        Self::from_limbs([r0, r1, r2, r3])
    }
}

assign_ops_from_trait!(U256, U256, MulAssign, mul_assign, MulInline, mul_assign);
self_ops_from_trait!(U256, Mul, mul, MulInline, mul, mul_assign);

impl MulFullInline<&U256> for U256 {
    type High = Self;

    // We shadow carry for readability
    #[allow(clippy::shadow_unrelated)]
    #[inline(always)]
    fn mul_full_inline(&self, rhs: &Self) -> (Self, Self) {
        let (r0, carry) = mac(0, self.limb(0), rhs.limb(0), 0);
        let (r1, carry) = mac(0, self.limb(0), rhs.limb(1), carry);
        let (r2, carry) = mac(0, self.limb(0), rhs.limb(2), carry);
        let (r3, r4) = mac(0, self.limb(0), rhs.limb(3), carry);
        let (r1, carry) = mac(r1, self.limb(1), rhs.limb(0), 0);
        let (r2, carry) = mac(r2, self.limb(1), rhs.limb(1), carry);
        let (r3, carry) = mac(r3, self.limb(1), rhs.limb(2), carry);
        let (r4, r5) = mac(r4, self.limb(1), rhs.limb(3), carry);
        let (r2, carry) = mac(r2, self.limb(2), rhs.limb(0), 0);
        let (r3, carry) = mac(r3, self.limb(2), rhs.limb(1), carry);
        let (r4, carry) = mac(r4, self.limb(2), rhs.limb(2), carry);
        let (r5, r6) = mac(r5, self.limb(2), rhs.limb(3), carry);
        let (r3, carry) = mac(r3, self.limb(3), rhs.limb(0), 0);
        let (r4, carry) = mac(r4, self.limb(3), rhs.limb(1), carry);
        let (r5, carry) = mac(r5, self.limb(3), rhs.limb(2), carry);
        let (r6, r7) = mac(r6, self.limb(3), rhs.limb(3), carry);
        (
            Self::from_limbs([r0, r1, r2, r3]),
            Self::from_limbs([r4, r5, r6, r7]),
        )
    }
}

// TODO: U256 exponent
impl Pow<u64> for &U256 {
    type Output = U256;

    fn pow(self, rhs: u64) -> U256 {
        let mut result = U256::ONE;
        let mut remaining_exponent = rhs;
        let mut square = self.clone();
        while remaining_exponent > 0 {
            if remaining_exponent % 2 == 1 {
                result = result.mul_inline(&square);
            }
            remaining_exponent >>= 1;
            square.square_assign_inline();
        }
        result
    }
}

impl U256 {
    /// Note: if `modulus` is a constant, it is faster to use
    /// `montgomery::mulmod` with precomputed parameters.
    pub fn mulmod(&self, rhs: &Self, modulus: &Self) -> Self {
        let (lo, hi) = self.mul_full(rhs);
        let mut numerator = [
            lo.limb(0),
            lo.limb(1),
            lo.limb(2),
            lo.limb(3),
            hi.limb(0),
            hi.limb(1),
            hi.limb(2),
            hi.limb(3),
            0,
        ];
        if modulus.limb(3) > 0 {
            divrem_nbym(&mut numerator, &mut [
                modulus.limb(0),
                modulus.limb(1),
                modulus.limb(2),
                modulus.limb(3),
            ]);
            Self::from_limbs([numerator[0], numerator[1], numerator[2], numerator[3]])
        } else if modulus.limb(2) > 0 {
            divrem_nbym(&mut numerator, &mut [
                modulus.limb(0),
                modulus.limb(1),
                modulus.limb(2),
            ]);
            Self::from_limbs([numerator[0], numerator[1], numerator[2], 0])
        } else if modulus.limb(1) > 0 {
            divrem_nbym(&mut numerator, &mut [modulus.limb(0), modulus.limb(1)]);
            Self::from_limbs([numerator[0], numerator[1], 0, 0])
        } else if modulus.limb(0) > 0 {
            let remainder = divrem_nby1(&mut numerator, modulus.limb(0));
            Self::from_limbs([remainder, 0, 0, 0])
        } else {
            panic!(); // TODO: return Option<>
        }
    }
}

impl MulAssign<u64> for U256 {
    #[cfg_attr(feature = "inline", inline(always))]
    fn mul_assign(&mut self, rhs: u64) {
        let result = self.mul_inline(rhs);
        *self = result;
    }
}

impl Mul<u64> for U256 {
    type Output = Self;

    #[cfg_attr(feature = "inline", inline(always))]
    fn mul(self, rhs: u64) -> Self {
        self.mul_inline(rhs)
    }
}

impl Mul<u64> for &U256 {
    type Output = U256;

    #[cfg_attr(feature = "inline", inline(always))]
    fn mul(self, rhs: u64) -> U256 {
        self.mul_inline(rhs)
    }
}

impl Mul<U256> for u64 {
    type Output = U256;

    #[cfg_attr(feature = "inline", inline(always))]
    fn mul(self, rhs: U256) -> U256 {
        rhs.mul_inline(self)
    }
}

impl Mul<&U256> for u64 {
    type Output = U256;

    #[cfg_attr(feature = "inline", inline(always))]
    fn mul(self, rhs: &U256) -> U256 {
        rhs.mul_inline(self)
    }
}

impl core::iter::Product for U256 {
    fn product<I: Iterator<Item = U256>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
    }
}

// TODO: Replace literals with u256h!
#[allow(clippy::unreadable_literal)]
// TODO: Better names
#[allow(clippy::similar_names)]
#[allow(clippy::clippy::many_single_char_names)]
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_mul() {
        let mut a = U256::from_limbs([
            0x11daab4a80b1cf9a,
            0x147ac29a5c5db4d4,
            0xb378f759c80c1d3a,
            0x02a2b5155bee10dc,
        ]);
        let b = U256::from_limbs([
            0x81aa26a88e9edd46,
            0xadb0ffe4dfb4a10f,
            0xc3a61b547a1f01ad,
            0x0554a84aa321a31c,
        ]);
        let e = U256::from_limbs([
            0x02cd4f6e3de2b61c,
            0x364935c057086115,
            0xb912b5cf544f5866,
            0x507ca4a96b4a328a,
        ]);
        a *= &b;
        assert_eq!(a, e);
    }

    #[test]
    fn test_mul_full() {
        let a = U256::from_limbs([
            0xcef29c5de9ccefc1,
            0x1f0363af6e0e89e0,
            0x2edfffcc3ce19c1c,
            0x0533aefb3249d52d,
        ]);
        let b = U256::from_limbs([
            0x7aedeade9e192566,
            0xbde10917fae93c03,
            0x3419d1ecf392f766,
            0x03027f1aaf32c3fe,
        ]);
        let elo = U256::from_limbs([
            0xc34784904e276be6,
            0x19f527745e55f913,
            0x1b805a30c8f277c6,
            0x360d66c911328f7a,
        ]);
        let ehi = U256::from_limbs([
            0x41f3f98d2b4a4d5c,
            0x2fdba3d97ab78ebe,
            0x5b3854220ea8f86c,
            0x000fa8097e2b023a,
        ]);
        let (rlo, rhi) = a.mul_full(&b);
        assert_eq!(rlo, elo);
        assert_eq!(rhi, ehi);
    }

    #[test]
    fn test_mulmod() {
        let a = U256::from_limbs([
            0xb7eb3137d7271553,
            0xf44101622499c849,
            0x6364b9150f381299,
            0x0487868a9c0b15bb,
        ]);
        let b = U256::from_limbs([
            0xee5c3e0c95ea3606,
            0xb5d23720247b076a,
            0x125d5c1cc549a496,
            0x02fa68e3d326247a,
        ]);
        let m = U256::from_limbs([
            0x04893c41700b0160,
            0x9ba854d08388861e,
            0x834be37ce5dd881f,
            0x0000000425a6a188,
        ]);
        let e = U256::from_limbs([
            0x14527949a28bfa32,
            0xa388ec81a8763eae,
            0x35b22ffb468ed013,
            0x000000032b77bd60,
        ]);
        let r = a.mulmod(&b, &m);
        assert_eq!(r, e);
    }

    proptest!(
        #[test]
        fn mul_full_lo(a: U256, b: U256) {
            let r = a.clone() * &b;
            let (lo, _hi) = a.mul_full(&b);
            prop_assert_eq!(r, lo);
        }

        #[test]
        fn square(a: U256) {
            prop_assert_eq!(a.square_full(), a.mul_full(&a));
        }
    );
}
