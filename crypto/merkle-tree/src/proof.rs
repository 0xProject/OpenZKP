// False positive: attribute has a use
#[allow(clippy::useless_attribute)]
// False positive: Importing preludes is allowed
#[allow(clippy::wildcard_imports)]
use std::prelude::v1::*;

use crate::{Commitment, Error, Index, Node, Result};
use itertools::Itertools;
use std::collections::VecDeque;
use zkp_error_utils::require;
use zkp_hash::{Hash, Hashable};

// Note: we can merge and split proofs. Based on indices we can
// compute which values are redundant.
#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Proof {
    commitment: Commitment,
    indices:    Vec<usize>,
    hashes:     Vec<Hash>,
}

impl Proof {
    pub fn from_hashes(
        commitment: &Commitment,
        indices: &[usize],
        hashes: &[Hash],
    ) -> Result<Self> {
        // Validate indices using `sort_indices`
        let _ = commitment.sort_indices(indices)?;
        require!(
            hashes.len() == commitment.proof_size(indices)?,
            Error::NotEnoughHashes
        );
        Ok(Self {
            commitment: commitment.clone(),
            indices:    indices.to_vec(),
            hashes:     hashes.to_vec(),
        })
    }

    pub fn hashes(&self) -> &[Hash] {
        &self.hashes
    }

    pub fn verify<Leaf: Hashable>(&self, leafs: &[(usize, Leaf)]) -> Result<()> {
        // TODO: Pass leafs by reference?
        // TODO: Check if the indices line up.

        // Construct the leaf nodes
        let mut nodes = leafs
            .iter()
            .map(|(index, leaf)| {
                Index::from_size_offset(self.commitment.size(), *index)
                    .map(|index| (index, leaf.hash()))
            })
            .collect::<Result<Vec<_>>>()?;
        nodes.sort_unstable_by_key(|(index, _)| *index);
        // OPT: `tuple_windows` copies the hashes
        require!(
            nodes
                .iter()
                .tuple_windows()
                .all(|(a, b)| a.0 != b.0 || a.1 == b.1),
            Error::DuplicateLeafMismatch
        );
        nodes.dedup_by_key(|(index, _)| *index);
        let mut nodes: VecDeque<(Index, Hash)> = nodes.into_iter().collect();

        // Create a mutable closure to pop hashes from the list
        let mut hashes_iter = self.hashes.iter();
        let mut pop = move || hashes_iter.next().ok_or(Error::NotEnoughHashes);

        // Reconstruct the root
        while let Some((current, hash)) = nodes.pop_front() {
            if let Some(parent) = current.parent() {
                // Reconstruct the parent node
                let node = if current.is_left() {
                    if let Some((next, next_hash)) = nodes.front() {
                        // TODO: Find a better way to satisfy the borrow checker.
                        let next_hash = next_hash.clone();
                        if current.sibling().unwrap() == *next {
                            // Merge left with next
                            let _ = nodes.pop_front();
                            Node(&hash, &next_hash).hash()
                        } else {
                            // Left not merged with next
                            // TODO: Find a way to merge this branch with the next.
                            Node(&hash, pop()?).hash()
                        }
                    } else {
                        // Left not merged with next
                        Node(&hash, pop()?).hash()
                    }
                } else {
                    // Right not merged with previous (or we would have skipped)
                    Node(pop()?, &hash).hash()
                };
                // Queue the new parent node for the next iteration
                nodes.push_back((parent, node))
            } else {
                // Root node has no parent, we are done
                require!(hash == *self.commitment.hash(), Error::RootHashMismatch);
            }
        }
        Ok(())
    }
}
