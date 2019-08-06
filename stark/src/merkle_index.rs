/// Index into a balances binary tree
///
/// Nodes are indexed [0...n-1], where n = 2^k-1 is the total number of leafs
/// and nodes in the tree. Nodes are indexed in breadth-first order, starting
/// with the root at 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MerkleIndex(usize);

#[allow(dead_code)] // TODO Remove
impl MerkleIndex {
    pub fn root() -> MerkleIndex {
        MerkleIndex(0)
    }

    pub fn from_index(index: usize) -> MerkleIndex {
        MerkleIndex(index)
    }

    pub fn from_depth_offset(depth: usize, offset: usize) -> MerkleIndex {
        // At level `depth` there are 2^depth nodes at offsets [0..2^depth-1]
        assert!(offset < 1usize << depth);
        MerkleIndex((1usize << depth) - 1 + offset)
    }

    pub fn index(&self) -> usize {
        self.0
    }

    pub fn depth(&self) -> usize {
        ((self.0 + 2).next_power_of_two().trailing_zeros() as usize) - 1
    }

    pub fn offset(&self) -> usize {
        (self.0 + 1) - (self.0 + 1).next_power_of_two()
    }

    pub fn is_root(&self) -> bool {
        self.0 == 0
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

    pub fn parent(&self) -> Option<MerkleIndex> {
        if self.is_root() {
            None
        } else {
            Some(MerkleIndex((self.0 - 1) >> 1))
        }
    }

    pub fn left_sibling(&self) -> Option<MerkleIndex> {
        if self.is_left_most() {
            None
        } else {
            Some(MerkleIndex(self.0 - 1))
        }
    }

    pub fn right_sibling(&self) -> Option<MerkleIndex> {
        if self.is_right_most() {
            None
        } else {
            Some(MerkleIndex(self.0 + 1))
        }
    }

    pub fn left_child(&self) -> MerkleIndex {
        MerkleIndex(2 * self.0 + 1)
    }

    pub fn right_child(&self) -> MerkleIndex {
        MerkleIndex(2 * self.0 + 2)
    }
}
