use super::{Hash, Hashable, Index, Node};
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Proof {
    depth:         usize,
    decommitments: Vec<Hash>,
}

impl Proof {
    pub fn from_depth_decommitment(depth: usize, decommitments: &[Hash]) -> Self {
        Self {
            depth,
            decommitments: decommitments.to_vec(),
        }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn decommitments(&self) -> &[Hash] {
        &self.decommitments
    }

    // TODO: Result<Hash, Error> instead of panic.
    pub fn root<Leaf: Hashable>(&self, leafs: &[(usize, &Leaf)]) -> Hash {
        // Construct the leaf nodes
        let mut nodes: Vec<_> = leafs
            .iter()
            .map(|(index, leaf)| {
                (
                    Index::from_depth_offset(self.depth, *index).expect("Index out of range."),
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
        let mut nodes: VecDeque<(Index,Hash)> = nodes.into_iter().collect();

        // Create a mutable closure to pop decommitments from the list
        let mut decommitments_iter = self.decommitments.iter();
        let mut pop = move || {
            decommitments_iter
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
                            // Merge current and next
                            let _ = nodes.pop_front();
                            Node(&hash, &next_hash).hash()
                        } else {
                            // Left not merged with next
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
                return hash.clone();
            }
        }

        // TODO: Empty indices
        unreachable!()
    }
}
