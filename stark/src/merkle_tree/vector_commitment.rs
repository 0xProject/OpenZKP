use super::{Hash, Hashable};
use std::prelude::v1::*;

#[cfg(feature = "std")]
use crate::mmap_vec::MmapVec;

// TODO: Rename to VectorCommitment
pub trait VectorCommitment
where
    Self::Leaf: Hashable,
{
    type Leaf;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn leaf(&self, index: usize) -> Self::Leaf;

    fn leaf_hash(&self, index: usize) -> Hash {
        self.leaf(index).hash()
    }

    // TODO: Add `commit(&self) -> (Commitment, Tree)`
}

// TODO ExactSizeIterator + Index<usize>

impl<Leaf: Hashable + Clone> VectorCommitment for Vec<Leaf> {
    type Leaf = Leaf;

    fn len(&self) -> usize {
        Vec::<Leaf>::len(self)
    }

    fn leaf(&self, index: usize) -> Self::Leaf {
        self[index].clone()
    }

    fn leaf_hash(&self, index: usize) -> Hash {
        self[index].hash()
    }
}

#[cfg(feature = "std")]
impl<Leaf: Hashable + Clone> VectorCommitment for MmapVec<Leaf> {
    type Leaf = Leaf;

    fn len(&self) -> usize {
        MmapVec::<Leaf>::len(self)
    }

    fn leaf(&self, index: usize) -> Self::Leaf {
        self[index].clone()
    }

    fn leaf_hash(&self, index: usize) -> Hash {
        self[index].hash()
    }
}
