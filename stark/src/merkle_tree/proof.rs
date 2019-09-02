use super::{Commitment, Hash, Hashable, Index, Node};
use itertools::Itertools;
use std::collections::VecDeque;

// Note: we can merge and split proofs. Based on indices we can
// compute which values are redundant.
#[derive(Clone, Debug)]
pub struct Proof {
    commitment: Commitment,
    indices:    Vec<usize>,
    hashes:     Vec<Hash>,
}

impl Proof {
    // TODO: Result<(), Error> instead of panic.
    pub fn from_hashes(commitment: &Commitment, indices: &[usize], hashes: &[Hash]) -> Self {
        assert!(indices.iter().all(|&i| i < commitment.num_leaves()));
        assert!(indices.iter().tuple_windows().all(|(a, b)| a < b));
        assert_eq!(hashes.len(), commitment.proof_size(indices));

        // TODO: Validate decommitment size.
        Self {
            commitment: commitment.clone(),
            indices:    indices.to_vec(),
            hashes:     hashes.to_vec(),
        }
    }

    pub fn hashes(&self) -> &[Hash] {
        &self.hashes
    }

    // TODO: Result<(), Error> instead of panic.
    pub fn verify<Leaf: Hashable>(&self, leafs: &[(usize, &Leaf)]) {
        // Construct the leaf nodes
        let mut nodes: Vec<_> = leafs
            .iter()
            .map(|(index, leaf)| {
                (
                    Index::from_depth_offset(self.commitment.depth(), *index)
                        .expect("Index out of range."),
                    leaf.hash(),
                )
            })
            .collect();
        nodes.sort_unstable_by_key(|(index, _)| *index);
        // OPT: `tuple_windows` copies the hashes
        if nodes
            .iter()
            .tuple_windows()
            .any(|(a, b)| a.0 == b.0 && a.1 != b.1)
        {
            panic!("Duplicate indices without duplicate leaf hashes");
        }
        nodes.dedup_by_key(|(index, _)| *index);
        let mut nodes: VecDeque<(Index, Hash)> = nodes.into_iter().collect();

        // Create a mutable closure to pop hashes from the list
        let mut hashes_iter = self.hashes.iter();
        let mut pop = move || {
            hashes_iter
                .next()
                .expect("Not enough elements in decommitment.")
        };

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
                            Node(&hash, &pop()).hash()
                        }
                    } else {
                        // Left not merged with next
                        Node(&hash, &pop()).hash()
                    }
                } else {
                    // Right not merged with previous (or we would have skipped)
                    Node(&pop(), &hash).hash()
                };
                // Queue the new parent node for the next iteration
                nodes.push_back((parent, node))
            } else {
                // Root node has no parent, we are done
                if hash != *self.commitment.hash() {
                    panic!("Root hashes do not match.");
                }
            }
        }
    }
}
