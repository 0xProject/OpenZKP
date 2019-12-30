use crate::{
    adc, assign_ops_from_trait, noncommutative_self_ops_from_trait, sbb, self_ops_from_trait,
    AddFullInline, AddInline, NegInline, SubFromFullInline, SubFromInline, SubFullInline,
    SubInline, U256,
};
use std::{
    ops::{Add, AddAssign, Sub, SubAssign},
    prelude::v1::*,
};

// Additive operations: Add, Sub

impl AddFullInline<&U256> for U256 {
    type High = u64;

    #[inline(always)]
    fn add_full_inline(&self, rhs: &Self) -> (Self, Self::High) {
        let (c0, carry) = adc(self.limb(0), rhs.limb(0), 0);
        let (c1, carry) = adc(self.limb(1), rhs.limb(1), carry);
        let (c2, carry) = adc(self.limb(2), rhs.limb(2), carry);
        let (c3, carry) = adc(self.limb(3), rhs.limb(3), carry);
        (U256::from_limbs([c0, c1, c2, c3]), carry)
    }
}

impl AddInline<&U256> for U256 {
    #[inline(always)]
    fn add_inline(&self, rhs: &Self) -> Self {
        self.add_full_inline(rhs).0
    }
}

assign_ops_from_trait!(U256, U256, AddAssign, add_assign, AddInline, add_assign);
self_ops_from_trait!(U256, Add, add, AddInline, add, add_assign);

impl SubFullInline<&U256> for U256 {
    type High = u64;

    #[inline(always)]
    fn sub_full_inline(&self, rhs: &Self) -> (Self, Self::High) {
        let (c0, borrow) = sbb(self.limb(0), rhs.limb(0), 0);
        let (c1, borrow) = sbb(self.limb(1), rhs.limb(1), borrow);
        let (c2, borrow) = sbb(self.limb(2), rhs.limb(2), borrow);
        let (c3, borrow) = sbb(self.limb(3), rhs.limb(3), borrow);
        (U256::from_limbs([c0, c1, c2, c3]), borrow)
    }
}

impl SubInline<&U256> for U256 {
    #[inline(always)]
    fn sub_inline(&self, rhs: &Self) -> Self {
        self.sub_full_inline(rhs).0
    }
}

assign_ops_from_trait!(U256, U256, SubAssign, sub_assign, SubInline, sub_assign);
noncommutative_self_ops_from_trait!(U256, Sub, sub, SubInline, sub, sub_assign);

impl SubFromFullInline<&U256> for U256 {
    type High = u64;

    #[inline(always)]
    fn sub_from_full_assign_inline(&mut self, rhs: &U256) -> Self::High {
        let (lo, hi) = rhs.sub_full_inline(self);
        *self = lo;
        hi
    }
}

impl SubFromInline<&U256> for U256 {
    #[inline(always)]
    fn sub_from_assign_inline(&mut self, rhs: &U256) {
        let _hi = self.sub_from_full_assign_inline(rhs);
    }
}

impl NegInline for U256 {
    #[inline(always)]
    fn neg_inline(&self) -> Self {
        U256::ZERO.sub_inline(self)
    }
}

impl core::iter::Sum for U256 {
    fn sum<I: Iterator<Item = U256>>(iter: I) -> Self {
        iter.fold(Self::ONE, Add::add)
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

    #[test]
    fn test_add() {
        let mut a = u256h!("01b54cf967a0f4f0d403de023ea32bf399223186ad9732d37209a73f5af87656");
        let b = U256::from_limbs([
            0xabe25acf4f460ee0,
            0x627c6bdf52bd869e,
            0x403390a0497c51ab,
            0x041aa3e6140810ca,
        ]);
        let e = U256::from_limbs([
            0x1dec020eaa3e8536,
            0xfb9e9d660054b972,
            0x14376ea2881f7d9e,
            0x05cff0df7ba905bb,
        ]);
        a += &b;
        assert_eq!(a, e);
    }

    #[test]
    fn test_sub() {
        let mut a = U256::from_limbs([
            0x281c7cfb32e98dd8,
            0x9018b2a04f60102b,
            0xd6e32fb1e0564153,
            0x02d005315d1af15f,
        ]);
        let b = U256::from_limbs([
            0x407666ddda2343ae,
            0xb4dd92954c5a0860,
            0x237cf6a1c121a335,
            0x05d6ce1edbd1908a,
        ]);
        let e = U256::from_limbs([
            0xe7a6161d58c64a2a,
            0xdb3b200b030607ca,
            0xb36639101f349e1d,
            0xfcf93712814960d5,
        ]);
        a -= &b;
        assert_eq!(a, e);
    }

    #[quickcheck]
    fn commutative_add(a: U256, b: U256) -> bool {
        let mut l = a.clone();
        let mut r = b.clone();
        l += &b;
        r += &a;
        l == r
    }
}
