// False positive: attribute has a use
#[allow(clippy::useless_attribute)]
// False positive: Importing preludes is allowed
#[allow(clippy::wildcard_imports)]
use std::prelude::v1::*;

use crate::{Commitment, Result, Tree};
use zkp_hash::{Hash, Hashable};

#[cfg(feature = "mmap")]
use crate::mmap_vec::MmapVec;

pub trait VectorCommitment
where
    Self: Sync + Sized,
    Self::Leaf: Sync + Hashable,
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

    fn commit(self) -> Result<(Commitment, Tree<Self>)> {
        let tree = Tree::from_leaves(self)?;
        let commitment = tree.commitment().clone();
        Ok((commitment, tree))
    }
}

// TODO ExactSizeIterator + Index<usize>

impl<Leaf: Hashable + Clone + Sync> VectorCommitment for Vec<Leaf> {
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
impl<Leaf: Hashable + Clone + Sync> VectorCommitment for MmapVec<Leaf> {
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
