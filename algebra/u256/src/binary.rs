// False positive: attribute has a use
#[allow(clippy::useless_attribute)]
// False positive: Importing preludes is allowed
#[allow(clippy::wildcard_imports)]
use std::prelude::v1::*;

use crate::{commutative_binop, traits::Binary, U256};
use std::{
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
        ShrAssign,
    },
    u64,
};

impl Binary for U256 {
    #[inline(always)]
    fn num_bits() -> usize {
        256
    }

    #[cfg_attr(feature = "inline", inline(always))]
    fn bit(&self, i: usize) -> bool {
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

    #[cfg_attr(feature = "inline", inline(always))]
    fn count_ones(&self) -> usize {
        (self.limb(0).count_ones()
            + self.limb(1).count_ones()
            + self.limb(2).count_ones()
            + self.limb(3).count_ones()) as usize
    }

    #[cfg_attr(feature = "inline", inline(always))]
    fn count_zeros(&self) -> usize {
        (self.limb(0).count_zeros()
            + self.limb(1).count_zeros()
            + self.limb(2).count_zeros()
            + self.limb(3).count_zeros()) as usize
    }

    #[cfg_attr(feature = "inline", inline(always))]
    fn leading_zeros(&self) -> usize {
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

    #[cfg_attr(feature = "inline", inline(always))]
    fn trailing_zeros(&self) -> usize {
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

    #[cfg_attr(feature = "inline", inline(always))]
    fn rotate_left(&self, _n: usize) -> Self {
        todo!()
    }

    #[cfg_attr(feature = "inline", inline(always))]
    fn rotate_right(&self, _n: usize) -> Self {
        todo!()
    }
}

// Useful for checking divisability by small powers of two
impl BitAnd<u64> for &U256 {
    type Output = u64;

    #[inline(always)]
    fn bitand(self, rhs: u64) -> u64 {
        self.limb(0) & rhs
    }
}

impl Not for U256 {
    type Output = Self;

    #[cfg_attr(feature = "inline", inline(always))]
    fn not(mut self) -> Self {
        self.set_limb(0, !self.limb(0));
        self.set_limb(1, !self.limb(1));
        self.set_limb(2, !self.limb(2));
        self.set_limb(3, !self.limb(3));
        self
    }
}

impl BitAndAssign<&U256> for U256 {
    #[cfg_attr(feature = "inline", inline(always))]
    fn bitand_assign(&mut self, rhs: &Self) {
        self.set_limb(0, self.limb(0) & rhs.limb(0));
        self.set_limb(1, self.limb(1) & rhs.limb(1));
        self.set_limb(2, self.limb(2) & rhs.limb(2));
        self.set_limb(3, self.limb(3) & rhs.limb(3));
    }
}

impl BitOrAssign<&U256> for U256 {
    #[cfg_attr(feature = "inline", inline(always))]
    fn bitor_assign(&mut self, rhs: &Self) {
        self.set_limb(0, self.limb(0) | rhs.limb(0));
        self.set_limb(1, self.limb(1) | rhs.limb(1));
        self.set_limb(2, self.limb(2) | rhs.limb(2));
        self.set_limb(3, self.limb(3) | rhs.limb(3));
    }
}

impl BitXorAssign<&U256> for U256 {
    #[cfg_attr(feature = "inline", inline(always))]
    fn bitxor_assign(&mut self, rhs: &Self) {
        self.set_limb(0, self.limb(0) ^ rhs.limb(0));
        self.set_limb(1, self.limb(1) ^ rhs.limb(1));
        self.set_limb(2, self.limb(2) ^ rhs.limb(2));
        self.set_limb(3, self.limb(3) ^ rhs.limb(3));
    }
}

impl ShlAssign<usize> for U256 {
    #[cfg_attr(feature = "inline", inline(always))]
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

    #[inline(always)]
    fn shl(mut self, rhs: usize) -> Self {
        self <<= rhs;
        self
    }
}

impl ShrAssign<usize> for U256 {
    #[cfg_attr(feature = "inline", inline(always))]
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

    #[inline(always)]
    fn shr(mut self, rhs: usize) -> Self {
        self >>= rhs;
        self
    }
}

commutative_binop!(U256, BitAnd, bitand, BitAndAssign, bitand_assign);
commutative_binop!(U256, BitOr, bitor, BitOrAssign, bitor_assign);
commutative_binop!(U256, BitXor, bitxor, BitXorAssign, bitxor_assign);

// TODO: Replace literals with u256h!
#[allow(clippy::unreadable_literal)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shl() {
        let mut n = U256::from_limbs([
            0x9050e39a8638969f,
            0xd7cc21c004c428d1,
            0x9026e34ec8fb83ac,
            0x03d4679634263e15,
        ]);
        let e = U256::from_limbs([
            0xcd431c4b4f800000,
            0xe002621468c82871,
            0xa7647dc1d66be610,
            0xcb1a131f0ac81371,
        ]);
        n <<= 23;
        assert_eq!(n, e);
    }

    #[test]
    fn test_shr() {
        let mut n = U256::from_limbs([
            0xbe1897b996367829,
            0x24c4cd2cacd2e3be,
            0xa0a61c4de933a54e,
            0x059e0db9d96add73,
        ]);
        let e = U256::from_limbs([
            0xa5c77d7c312f732c,
            0x674a9c49899a5959,
            0xd5bae7414c389bd2,
            0x0000000b3c1b73b2,
        ]);
        n >>= 23;
        assert_eq!(n, e);
    }
}
