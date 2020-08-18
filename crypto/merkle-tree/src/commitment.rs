// False positive: attribute has a use
#[allow(clippy::useless_attribute)]
// False positive: Importing preludes is allowed
#[allow(clippy::wildcard_imports)]
use std::prelude::v1::*;

use crate::{Error, Index, Result};
use itertools::Itertools;
use zkp_error_utils::require;
use zkp_hash::Hash;

#[derive(Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Commitment {
    size: usize,
    hash: Hash,
}

impl Commitment {
    pub fn from_size_hash(size: usize, hash: &Hash) -> Result<Self> {
        require!(
            size == 0 || size.is_power_of_two(),
            Error::NumLeavesNotPowerOfTwo
        );
        Ok(Self {
            size,
            hash: hash.clone(),
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    /// Convert leaf indices to a sorted list of unique `Index`s and validates
    /// their range.
    pub fn sort_indices(&self, indices: &[usize]) -> Result<Vec<Index>> {
        let mut indices = indices
            .iter()
            .map(|&i| Index::from_size_offset(self.size, i))
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
        let depth = self.size.trailing_zeros() as usize;
        let mut size = depth * indices.len();
        for (&current, &next) in indices.iter().tuple_windows() {
            let ancestor = current.last_common_ancestor(next);
            size -= ancestor.depth() + 2;
        }
        Ok(size)
    }
}
