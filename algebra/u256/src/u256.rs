// False positive: attribute has a use
#[allow(clippy::useless_attribute)]
// False positive: Importing preludes is allowed
#[allow(clippy::wildcard_imports)]
use std::prelude::v1::*;

#[cfg(any(test, feature = "proptest"))]
use proptest_derive::Arbitrary;
use std::{cmp::Ordering, u64};

#[cfg(feature = "parity_codec")]
use parity_scale_codec::{Decode, Encode};

#[derive(PartialEq, Eq, Clone, Default, Hash)]
#[cfg_attr(feature = "parity_codec", derive(Encode, Decode))]
// TODO: Generate a quasi-random sequence.
// See http://extremelearning.com.au/unreasonable-effectiveness-of-quasirandom-sequences/
#[cfg_attr(any(test, feature = "proptest"), derive(Arbitrary))]
#[cfg_attr(any(test, feature = "proptest"), proptest(no_params))]
pub struct U256([u64; 4]);

// TODO: impl core::iter::Step so we have ranges

impl U256 {
    pub const MAX: Self = Self::from_limbs([
        u64::max_value(),
        u64::max_value(),
        u64::max_value(),
        u64::max_value(),
    ]);
    pub const ONE: Self = Self::from_limbs([1, 0, 0, 0]);
    pub const ZERO: Self = Self::from_limbs([0, 0, 0, 0]);

    // Force inlined because it is a trivial conversion which appears in many hot
    // paths
    #[inline(always)]
    pub const fn from_limbs(limbs: [u64; 4]) -> Self {
        Self(limbs)
    }

    // Force inlined because it is a trivial conversion which appears in many hot
    // paths
    #[inline(always)]
    pub const fn as_limbs(&self) -> &[u64; 4] {
        &self.0
    }

    // It's important that this gets inlined, because `index` is nearly always
    // a compile time constant, which means the range check will get optimized
    // away.
    // TODO: Make const fn
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
}

impl PartialOrd for U256 {
    // This is a small function that appears often in hot paths.
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for U256 {
    // This is a small function that appears often in hot paths.
    #[inline(always)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use zkp_macros_decl::u256h;

    #[allow(dead_code)]
    const TEST_CONST: U256 =
        u256h!("0800000000000010ffffffffffffffffffffffffffffffffffffffffffffffff");
}
