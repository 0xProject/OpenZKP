use super::{Hash, Hashable};

// TODO: Rename to VectorCommitment
pub trait Merkelizable
where
    Self::Leaf: Hashable,
{
    type Leaf;

    fn len(&self) -> usize;

    fn leaf(&self, index: usize) -> &Self::Leaf;

    fn leaf_hash(&self, index: usize) -> Hash {
        self.leaf(index).hash()
    }

    // TODO: Add `commit(&self) -> (Commitment, Tree)`
}

// TODO ExactSizeIterator + Index<usize>
