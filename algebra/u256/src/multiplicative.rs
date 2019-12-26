use crate::{
    algorithms::{
        divrem_nby1, divrem_nbym,
        limb_operations::{adc, mac},
    },
    commutative_binop, U256,
};
use std::{
    ops::{Mul, MulAssign},
    prelude::v1::*,
    u64,
};

// Multiplicative operations: Mul, square, mulmod, pow, etc. routines

impl U256 {
    #[cfg_attr(feature = "inline", inline(always))]
    pub fn sqr(&self) -> Self {
        self.sqr_inline()
    }

    #[inline(always)]
    pub fn sqr_inline(&self) -> Self {
        let (lo, _hi) = self.sqr_full_inline();
        lo
    }
    
    /*
    #[cfg_attr(feature = "inline", inline(always))]
    pub fn mul(&self, rhs: &Self) -> Self {
        self.mul_inline(rhs)
    }

    #[inline(always)]
    pub fn mul_inline(&self, rhs: &Self) -> Self {
        todo!()
    }
    */

    #[cfg_attr(feature = "inline", inline(always))]
    pub fn mul_full(&self, rhs: &Self) -> (Self, Self) {
        self.mul_full_inline(rhs)
    }

    // We shadow carry for readability
    #[allow(clippy::shadow_unrelated)]
    #[inline(always)]
    pub fn mul_full_inline(&self, rhs: &Self) -> (Self, Self) {
        crate::algorithms::assembly::full_mul_asm(&self, rhs)
        /*
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
        */
    }

    #[cfg_attr(feature = "inline", inline(always))]
    pub fn sqr_full(&self) -> (Self, Self) {
        self.sqr_full_inline()
    }

    // We shadow carry for readability
    #[allow(clippy::shadow_unrelated)]
    #[inline(always)]
    pub fn sqr_full_inline(&self) -> (Self, Self) {
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

    pub fn pow(&self, exponent: u64) -> Option<Self> {
        if self.is_zero() && (exponent == 0) {
            None
        } else {
            let mut result = Self::ONE;
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

impl MulAssign<&U256> for U256 {
    // We shadow carry for readability
    #[allow(clippy::shadow_unrelated)]
    #[cfg_attr(feature = "inline", inline(always))]
    fn mul_assign(&mut self, rhs: &Self) {
        let (r0, carry) = mac(0, self.limb(0), rhs.limb(0), 0);
        let (r1, carry) = mac(0, self.limb(0), rhs.limb(1), carry);
        let (r2, carry) = mac(0, self.limb(0), rhs.limb(2), carry);
        let (r3, _) = mac(0, self.limb(0), rhs.limb(3), carry);
        self.set_limb(0, r0);
        let (r1, carry) = mac(r1, self.limb(1), rhs.limb(0), 0);
        let (r2, carry) = mac(r2, self.limb(1), rhs.limb(1), carry);
        let (r3, _) = mac(r3, self.limb(1), rhs.limb(2), carry);
        self.set_limb(1, r1);
        let (r2, carry) = mac(r2, self.limb(2), rhs.limb(0), 0);
        let (r3, _) = mac(r3, self.limb(2), rhs.limb(1), carry);
        self.set_limb(2, r2);
        let (r3, _) = mac(r3, self.limb(3), rhs.limb(0), 0);
        self.set_limb(3, r3);
    }
}

commutative_binop!(U256, Mul, mul, MulAssign, mul_assign);

impl MulAssign<u64> for U256 {
    #[cfg_attr(feature = "inline", inline(always))]
    fn mul_assign(&mut self, rhs: u64) {
        let (r0, carry) = mac(0, self.limb(0), rhs, 0);
        let (r1, carry) = mac(0, self.limb(1), rhs, carry);
        let (r2, carry) = mac(0, self.limb(2), rhs, carry);
        let (r3, _) = mac(0, self.limb(3), rhs, carry);
        self.set_limb(0, r0);
        self.set_limb(1, r1);
        self.set_limb(2, r2);
        self.set_limb(3, r3);
    }
}

impl Mul<u64> for U256 {
    type Output = Self;

    #[inline(always)]
    fn mul(mut self, rhs: u64) -> Self {
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
    // Carry gets re-used for readability
    #[allow(clippy::shadow_unrelated)]
    #[cfg_attr(feature = "inline", inline(always))]
    fn mul_assign(&mut self, rhs: u128) {
        // We want the truncation here
        #[allow(clippy::cast_possible_truncation)]
        let (lo, hi) = (rhs as u64, (rhs >> 64) as u64);
        let (r0, carry) = mac(0, self.limb(0), lo, 0);
        let (r1, carry) = mac(0, self.limb(1), lo, carry);
        let (r2, carry) = mac(0, self.limb(2), lo, carry);
        let (r3, _) = mac(0, self.limb(3), lo, carry);
        let (r1, carry) = mac(r1, self.limb(0), hi, 0);
        let (r2, carry) = mac(r2, self.limb(1), hi, carry);
        let (r3, _) = mac(r3, self.limb(2), hi, carry);
        self.set_limb(0, r0);
        self.set_limb(1, r1);
        self.set_limb(2, r2);
        self.set_limb(3, r3);
    }
}

impl Mul<u128> for U256 {
    type Output = Self;

    #[inline(always)]
    fn mul(mut self, rhs: u128) -> Self {
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

impl core::iter::Product for U256 {
    fn product<I: Iterator<Item = U256>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
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

    #[quickcheck]
    fn mul_full_lo(a: U256, b: U256) -> bool {
        let r = a.clone() * &b;
        let (lo, _hi) = a.mul_full(&b);
        r == lo
    }

    #[quickcheck]
    fn square(a: U256) -> bool {
        a.sqr_full() == a.mul_full(&a)
    }
}
