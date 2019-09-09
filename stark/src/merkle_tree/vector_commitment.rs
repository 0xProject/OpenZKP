use super::{Hash, Hashable};
use std::prelude::v1::*;

#[cfg(feature = "mmap")]
use crate::mmap_vec::MmapVec;

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
        Self::len(self)
    }

    fn leaf(&self, index: usize) -> Self::Leaf {
        self[index].clone()
    }

    fn leaf_hash(&self, index: usize) -> Hash {
        self[index].hash()
    }
}

#[cfg(feature = "mmap")]
impl<Leaf: Hashable + Clone> VectorCommitment for MmapVec<Leaf> {
    type Leaf = Leaf;

    fn len(&self) -> usize {
        Self::len(self)
    }

    fn leaf(&self, index: usize) -> Self::Leaf {
        self[index].clone()
    }

    fn leaf_hash(&self, index: usize) -> Hash {
        self[index].hash()
    }
}
