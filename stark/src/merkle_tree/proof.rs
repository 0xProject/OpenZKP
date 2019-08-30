use super::{Hash, Hashable, Index, Node};

#[derive(Clone, Debug)]
pub struct Proof {
    depth:         usize,
    decommitments: Vec<Hash>,
}

impl Proof {
    pub fn root<Leaf: Hashable>(&self, leafs: &[(usize, &Leaf)]) -> Hash {
        // Construct the leaf nodes
        let mut nodes = leafs
            .iter()
            .map(|(index, leaf)| (Index::from_depth_and_offset(self.depth, index), leaf.hash()))
            .collect();
        nodes.sort_unstable_by_key(|(index, _)| index);
        // OPT: `tuple_windows` copies the hashes
        if nodes
            .iter()
            .tuple_windows()
            .any(|(a, b)| a.0 == b.0 && a.1 != b.1)
        {
            panic!("Duplicate indices without duplicate leaf hashes");
        }
        nodes.dedup_by_key(|(index, _)| index);

        // Create a mutable closure to pop decommitments from the list
        // TODO: Use an iterator
        let mut counter = 0_usize;
        let pop = move || {
            counter += 1;
            self.decommitments[counter - 1]
        };

        // Reconstruct the root
        'outer: loop {
            assert!(!nodes.is_empty());
            // TODO: We can use a single ring buffer instead of vecs
            let mut next_nodes: Vec<(Index, Hash)> = Vec::new();
            let mut iter = nodes.iter().peekable();
            while let Some((index, hash)) = iter.next() {
                if let Some(parent) = index.parent() {
                    let node = if i.0.is_left() {
                        match iter.peek() {
                            // Merge if next is right
                            Some((next_index, next_hash))
                                if index.sibling() == Some(**next_index) =>
                            {
                                // Skip the next right node
                                iter.next();
                                Node(&hash, &next_hash)
                            }
                            _ => Node(&hash, &pop()),
                        }
                    } else {
                        // Right node not merged with left
                        Node(&pop(), &hash)
                    };
                    next_nodes.push((parent, node.hash()))
                } else {
                    // Root node has no parent, we are done
                    break 'outer hash;
                }
            }
            nodes = next_nodes
        }
    }
}
