use super::{Hash, Hashable, Index, Node};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Proof {
    depth:         usize,
    decommitments: Vec<Hash>,
}

impl Proof {
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

        // Create a mutable closure to pop decommitments from the list
        let mut decommitments_iter = self.decommitments.iter();
        let mut pop = move || {
            decommitments_iter
                .next()
                .expect("Not enough elements in decommitment.")
        };

        // Reconstruct the root
        // On each iteration, the indices in the set are strictly decreasing
        // which means eventually they reduce to the root and we terminate
        // the loop.
        'outer: loop {
            assert!(!nodes.is_empty());
            // TODO: We can use a single ring buffer instead of vecs
            let mut next_nodes: Vec<(Index, Hash)> = Vec::new();
            let mut iter = nodes.iter().peekable();
            while let Some((index, hash)) = iter.next() {
                if let Some(parent) = index.parent() {
                    let node: Node<'_> = if index.is_left() {
                        match iter.peek() {
                            // Merge if next is right
                            Some((next_index, next_hash))
                                if index.sibling() == Some(*next_index) =>
                            {
                                // Skip the next right node
                                let _ = iter.next();
                                Node(&hash, &next_hash)
                            }
                            // Left node not merged with right
                            _ => Node(&hash, &pop()),
                        }
                    } else {
                        // Right node not merged with left
                        Node(&pop(), &hash)
                    };
                    // Queue the new parent node for the next iteration
                    next_nodes.push((parent, node.hash()))
                } else {
                    // Root node has no parent, we are done
                    break 'outer hash.clone();
                }
            }
            nodes = next_nodes
        }
    }
}
