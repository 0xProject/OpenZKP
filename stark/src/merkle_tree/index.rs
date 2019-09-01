use std::convert::TryFrom;

/// Index into a balances binary tree
///
/// The index has two representations, as a (depth, offset) pair and as an
/// index.
///
/// ```
/// (0, 0)(1, 0)(1, 1)(2, 0)(2, 1)(2, 2)(2, 3)(3, 0)(3, 1)(3, 2)(3, 3)(3, 4)(3, 5)(3, 6)(3, 7)
/// ```
///
/// The corresponding index numbering is
///
/// ```
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
// to the nice binary representation `1 << depth | offset`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Index(usize);

enum Kind {
    Root,
    Left,
    Right,
}

impl Index {
    pub const fn root() -> Self {
        Self(1)
    }

    pub const fn from_index(index: usize) -> Self {
        Self(index + 1)
    }

    // At level `depth` there are 2^depth nodes at offsets [0..2^depth-1]
    pub fn from_depth_offset(depth: usize, offset: usize) -> Option<Self> {
        if offset >= (1_usize << depth) {
            None
        } else {
            Some(Self((1_usize << depth) | offset))
        }
    }

    pub fn as_index(&self) -> usize {
        self.0 - 1
    }

    pub fn depth(&self) -> usize {
        let next_layer = (self.0 + 1).next_power_of_two();
        // Usize should always be able to hold it's number of bits
        let next_depth = usize::try_from(next_layer.trailing_zeros()).unwrap();
        next_depth - 1
    }

    pub fn offset(&self) -> usize {
        self.0 - (self.0.next_power_of_two())
    }

    pub fn is_root(&self) -> bool {
        self.0 == 1
    }

    pub fn is_left(&self) -> bool {
        self.0 != 0 && self.0 % 2 == 0
    }

    pub fn is_right(&self) -> bool {
        self.0 % 2 == 1
    }

    pub fn is_left_most(&self) -> bool {
        (self.0 + 1).is_power_of_two()
    }

    pub fn is_right_most(&self) -> bool {
        (self.0 + 2).is_power_of_two()
    }

    pub fn parent(&self) -> Option<Self> {
        if self.is_root() {
            None
        } else {
            Some(Self((self.0 - 1) >> 1))
        }
    }

    pub fn sibling(&self) -> Option<Self> {
        if self.is_root() {
            None
        } else if self.is_left() {
            Some(Self(self.0 + 1))
        } else {
            Some(Self(self.0 - 1))
        }
    }

    pub fn left_neighbor(&self) -> Option<Self> {
        if self.is_left_most() {
            None
        } else {
            Some(Self(self.0 - 1))
        }
    }

    pub fn right_neighbor(&self) -> Option<Self> {
        if self.is_right_most() {
            None
        } else {
            Some(Self(self.0 + 1))
        }
    }

    pub fn left_child(&self) -> Self {
        Self(2 * self.0 + 1)
    }

    pub fn right_child(&self) -> Self {
        Self(2 * self.0 + 2)
    }

    pub fn ancestor_of(&self, other: Index) -> bool {
        // TODO
        unimplemented!()
    }

    pub fn descents_from(&self, other: Index) -> bool {
        // TODO
        unimplemented!()
    }

    pub fn last_common_ancestor(&self, other: Self) -> Self {
        // TODO
        unimplemented!()
    }
}

#[cfg(any(test, feature = "quickcheck"))]
use quickcheck::{Arbitrary, Gen};

#[cfg(any(test, feature = "quickcheck"))]
impl Arbitrary for Index {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Self(usize::arbitrary(g) + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    pub fn test_depth_offset_roundtrip(depth: usize, offset: usize) -> bool {
        let offset = offset % (1_usize << depth);
        let index = Index::from_depth_offset(depth, offset).unwrap();
        index.depth() == depth && index.offset() == offset
    }
}
