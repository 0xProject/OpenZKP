use super::{Error, Hash, Index, Result};
use itertools::Itertools;
use std::prelude::v1::*;

#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Commitment {
    depth: usize,
    hash:  Hash,
}

impl Commitment {
    pub fn from_depth_hash(depth: usize, hash: &Hash) -> Result<Self> {
        if depth >= (0_usize.count_zeros() as usize) {
            // The number of leaves needs to fit `usize`
            Err(Error::DepthOutOfRange)
        } else {
            Ok(Self {
                depth,
                hash: hash.clone(),
            })
        }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn num_leaves(&self) -> usize {
        1_usize << self.depth
    }

    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    /// Convert leaf indices to a sorted list of unique `Index`s.
    pub fn sort_indices(&self, indices: &[usize]) -> Result<Vec<Index>> {
        let mut indices = indices
            .iter()
            .map(|&i| Index::from_depth_offset(self.depth, i))
            .collect::<Result<Vec<_>>>()?;
        indices.sort_unstable();
        indices.dedup();
        Ok(indices)
    }

    /// The number of hashes in the proof for the given set of indices.
    pub fn proof_size(&self, indices: &[usize]) -> Result<usize> {
        let indices = self.sort_indices(indices)?;

        // Start with the full path length for the first index
        // then add the path length of each next index up to the last common
        // ancestor with the previous index.
        let mut size = self.depth() * indices.len();
        for (&current, &next) in indices.iter().tuple_windows() {
            let ancestor = current.last_common_ancestor(next);
            size -= ancestor.depth() + 2;
        }
        Ok(size)
    }
}
