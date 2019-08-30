use super::{Hash, Index, Merkelizable, Node};
use std::ops::Index as IndexOp;

#[derive(Clone, Debug)]
pub struct Tree<'a, Container: Merkelizable> {
    depth: usize,
    nodes: Vec<Hash>,
    leafs: &'a Container,
}

impl<Container> Tree<'_, Container> {
    pub fn new(leafs: &Container) -> Self {
        let num_leafs = leafs.len();
        assert!(num_leafs.is_power_of_two());
        let depth = num_leafs.trailing_zeros() as usize;
        let mut nodes = vec![Hash::default(); 2 * num_leafs - 1];

        // Hash the tree
        for (i, leaf) in leafs.iter().enumerate() {
            let mut cursor = Index::from_depth_offset(depth, i).index();
            nodes[cursor.index()] = leaf.hash();
            while cursor.is_right() {
                cursor = cursor.parent().unwrap();
                nodes[cursor.index()] = Node(
                    &nodes[cursor.left_child().index],
                    &nodes[cursor.right_child().index],
                )
                .hash()
            }
        }

        Tree {
            depth,
            nodes,
            leafs,
        }
    }

    pub fn root(&self) -> Hash {
        &self[Index::root()]
    }

    pub fn proof(&self, indices: &[usize]) -> Proof {
        // Allocate space for decommitments
        let mut decommitments: Vec<Hash> = Vec::new();

        // Convert leaf indices to a sorted list of unique MerkleIndices.
        let mut indices: Vec<Index> = indices
            .iter()
            .map(|i| Index::from_depth_offset(self.depth, *i))
            .collect();
        indices.sort();
        indices.dedup();

        // Iterate over indices
        while !indices.is_empty() {
            // TODO: We can use a single ring buffer instead of vecs
            let mut next_indices: Vec<Index> = Vec::new();
            let mut iter = indices.iter().peekable();
            while let Some(i) = iter.next() {
                // Add parent index to the queue for the next pass
                if let Some(parent) = i.parent() {
                    if !parent.is_root() {
                        next_indices.push(parent);
                    }
                }

                // Check if we merge with the next merkle index.
                if let Some(j) = iter.peek() {
                    if i.sibling() == Some(**j) {
                        // Don't write a decommitment and skip next.
                        iter.next();
                        continue;
                    }
                }

                // Add a hash to the decommitment
                decommitments.push(if i.depth() == self.depth {
                    leafs(i.offset()).hash()
                } else {
                    self[*i].clone()
                });
            }
            indices = next_indices
        }

        Proof { decommitments }
    }
}

impl<Container> IndexOp<Index> for Tree<Container> {
    type Output = Hash;

    fn index(&self, index: Index) -> &Hash {
        &self.nodes[index.index()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use macros_decl::hex;
    use u256::U256;

    impl Groupable<U256> for &[U256] {
        fn get_leaf(&self, index: usize) -> U256 {
            self[index].clone()
        }

        fn domain_size(&self) -> usize {
            self.len()
        }
    }

    #[test]
    fn test_merkle_creation_and_proof() {
        let depth = 6;
        let mut leaves = Vec::new();

        for i in 0..2_u64.pow(depth) {
            leaves.push(U256::from((i + 10).pow(3)));
        }

        let tree = make_tree(leaves.as_slice());

        let tree2 = MerkleTree::from_iter(leaves.iter());
        assert_eq!(
            tree2.root().as_bytes(),
            hex!("fd112f44bc944f33e2567f86eea202350913b11c000000000000000000000000")
        );

        assert_eq!(
            tree[1].as_bytes(),
            hex!("fd112f44bc944f33e2567f86eea202350913b11c000000000000000000000000")
        );
        let mut values = vec![
            (1, leaves[1].clone()),
            (10, leaves[10].clone()),
            (11, leaves[11].clone()),
            (14, leaves[14].clone()),
        ];

        let indices = vec![1, 11, 14];
        let decommitment = proof(tree.as_slice(), &indices, &leaves.as_slice());
        let non_root = Hash::new(hex!(
            "ed112f44bc944f33e2567f86eea202350913b11c000000000000000000000000"
        ));

        assert!(verify(
            &tree[1],
            depth,
            values.as_mut_slice(),
            &decommitment
        ));
        assert!(!verify(
            &non_root,
            depth,
            values.as_mut_slice(),
            &decommitment
        ));
    }
}
