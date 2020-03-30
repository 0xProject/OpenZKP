use crate::{Error, Result};
#[cfg(any(test, feature = "proptest"))]
use proptest_derive::Arbitrary;
use std::{convert::TryFrom, ops::RangeInclusive};
use zkp_error_utils::require;

#[cfg(feature = "std")]
use std::fmt;

const USIZE_BITS: usize = 0_usize.count_zeros() as usize;

/// Index into a balanced binary tree
///
/// The index has two representations, as a (depth, offset) pair and as an
/// index.
///
/// ```ignore
/// (0, 0)(1, 0)(1, 1)(2, 0)(2, 1)(2, 2)(2, 3)(3, 0)(3, 1)(3, 2)(3, 3)(3, 4)(3, 5)(3, 6)(3, 7)
/// ```
///
/// The corresponding index numbering is
///
/// ```ignore
///                            0
///              1                           2
///       3             4             5             6
///     7   8         9   10       11  12         13  14
/// ```
///
/// Nodes are indexed [0...n-1], where n = 2^k-1 is the total number of leafs
/// and nodes in the tree. Nodes are indexed in breadth-first order, starting
/// with the root at 0.
// Internally, the representation is the index number offset by one. This leads
// to the nice binary representation `1 << depth | offset`. Equivalently, it is
// the path to the node, stating with `1` at the root and then `0` for left and
// `1` for right.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    any(test, feature = "proptest"),
    proptest(no_params),
    derive(Arbitrary)
)]
pub struct Index(usize);

impl Index {
    pub const fn max_size() -> usize {
        1_usize << (USIZE_BITS - 1)
    }

    pub const fn size_at_depth(depth: usize) -> usize {
        // Note: We can not do checks in a `const fn`.
        // assert!(depth < USIZE_BITS);
        1_usize << depth
    }

    pub fn depth_for_size(size: usize) -> usize {
        // TODO: Add checks
        size.next_power_of_two().trailing_zeros() as usize
    }

    pub const fn root() -> Self {
        Self(1)
    }

    pub const fn from_index(index: usize) -> Self {
        Self(index + 1)
    }

    pub fn iter_layer(depth: usize) -> LayerIter {
        LayerIter {
            size:   1_usize << depth,
            offset: 0,
        }
    }

    pub fn layer_range(depth: usize) -> RangeInclusive<usize> {
        // TODO: Overflow check
        let start = Self::from_depth_offset(depth, 0).unwrap();
        let end = Self::from_depth_offset(depth, Self::size_at_depth(depth) - 1).unwrap();
        start.as_index()..=end.as_index()
    }

    // At level `depth` there are 2^depth nodes at offsets [0..2^depth-1]
    pub fn from_size_offset(size: usize, offset: usize) -> Result<Self> {
        require!(size.is_power_of_two(), Error::NumLeavesNotPowerOfTwo);
        require!(size <= Self::max_size(), Error::TreeToLarge);
        require!(offset < size, Error::IndexOutOfRange);
        Ok(Self(size | offset))
    }

    pub fn from_depth_offset(depth: usize, offset: usize) -> Result<Self> {
        Self::from_size_offset(Self::size_at_depth(depth), offset)
    }

    pub fn as_index(self) -> usize {
        self.0 - 1
    }

    pub fn depth(self) -> usize {
        let next_layer = (self.0 + 1).next_power_of_two();
        // Usize should always be able to hold its number of bits
        let next_depth = usize::try_from(next_layer.trailing_zeros()).unwrap();
        next_depth - 1
    }

    pub fn offset(self) -> usize {
        self.0 - (1_usize << self.depth())
    }

    pub fn is_root(self) -> bool {
        self.0 == 1
    }

    pub fn is_left(self) -> bool {
        self.0 % 2 == 0
    }

    pub fn is_right(self) -> bool {
        self.0 != 1 && self.0 % 2 == 1
    }

    pub fn is_left_most(self) -> bool {
        self.0.is_power_of_two()
    }

    pub fn is_right_most(self) -> bool {
        (self.0 + 1).is_power_of_two()
    }

    pub fn parent(self) -> Option<Self> {
        if self.is_root() {
            None
        } else {
            Some(Self(self.0 >> 1))
        }
    }

    pub fn sibling(self) -> Option<Self> {
        if self.is_root() {
            None
        } else {
            Some(Self(self.0 ^ 1))
        }
    }

    pub fn left_neighbor(self) -> Option<Self> {
        if self.is_left_most() {
            None
        } else {
            Some(Self(self.0 - 1))
        }
    }

    pub fn right_neighbor(self) -> Option<Self> {
        if self.is_right_most() {
            None
        } else {
            Some(Self(self.0 + 1))
        }
    }

    pub fn left_child(self) -> Self {
        Self(self.0 << 1)
    }

    pub fn right_child(self) -> Self {
        Self((self.0 << 1) | 1)
    }

    pub fn last_common_ancestor(self, other: Self) -> Self {
        // Align their first bits all the way to the left
        let a = self.0 << self.0.leading_zeros();
        let b = other.0 << other.0.leading_zeros();

        // Extract the longest common prefix
        let prefix_length = (a ^ b).leading_zeros();
        let prefix = a >> (0_usize.count_zeros() - prefix_length);
        Self(prefix)
    }

    /// Concatenate `other`'s path to ours.
    pub fn concat(self, other: Self) -> Self {
        // TODO: Check overflow
        let other_depth = other.depth();
        let other_path = other.0 ^ 1_usize << other_depth;
        Self(self.0 << other_depth | other_path)
    }
}

#[cfg(feature = "std")]
impl fmt::Debug for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Index({:}, {:})", self.depth(), self.offset())
    }
}

#[cfg_attr(feature = "std", derive(Debug))]
pub struct LayerIter {
    size:   usize,
    offset: usize,
}

/// Iterating on an Index will go right until the end of the layer.
impl Iterator for LayerIter {
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset < self.size {
            let item = Index::from_size_offset(self.size, self.offset).unwrap();
            self.offset += 1;
            Some(item)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest!(
        #[test]
        fn test_depth_offset_roundtrip(depth: usize, offset: usize) {
            let depth = depth % (Index::max_size().trailing_zeros() as usize);
            let offset = offset % Index::size_at_depth(depth);
            let index = Index::from_size_offset(1_usize << depth, offset).unwrap();
            prop_assert_eq!(index.depth(), depth);
            prop_assert_eq!(index.offset(), offset);
        }

        #[test]
        fn test_children(parent: Index) {
            prop_assume!(parent.depth() != 63);

            let left = parent.left_child();
            let right = parent.right_child();
            prop_assert!(left.is_left());
            prop_assert!(right.is_right());
            prop_assert_eq!(left.depth(), right.depth());
            prop_assert_eq!(left.depth(), parent.depth() + 1);
            prop_assert_eq!(left.offset() + 1, right.offset());
            prop_assert_eq!(left.parent().unwrap(), parent);
            prop_assert_eq!(right.parent().unwrap(), parent);
            prop_assert_eq!(left.right_neighbor().unwrap(), right);
            prop_assert_eq!(right.left_neighbor().unwrap(), left);
            prop_assert_eq!(left.sibling().unwrap(), right);
            prop_assert_eq!(right.sibling().unwrap(), left);
        }
    );
}
