use super::{Hash, Hashable};
use crate::mmap_vec::MmapVec;

// TODO: Rename to VectorCommitment
pub trait VectorCommitment
where
    Self::Leaf: Hashable,
{
    type Leaf;

    fn len(&self) -> usize;

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
