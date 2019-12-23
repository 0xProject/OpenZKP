use std::{cmp::Ordering, prelude::v1::*, u64};

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
    use zkp_macros_decl::u256h;

    #[allow(dead_code)]
    const TEST_CONST: U256 =
        u256h!("0800000000000010ffffffffffffffffffffffffffffffffffffffffffffffff");
}
