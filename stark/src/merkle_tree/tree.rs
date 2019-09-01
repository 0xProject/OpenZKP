use super::{Hash, Hashable, Index, Merkelizable, Node, Proof};
use std::ops::Index as IndexOp;

#[derive(Clone, Debug)]
pub struct Tree<'a, Container: Merkelizable> {
    depth: usize,
    nodes: Vec<Hash>,
    leafs: &'a Container,
}

impl<'a, Container: Merkelizable> Tree<'a, Container> {
    pub fn new(leafs: &'a Container) -> Self {
        let num_leafs = leafs.len();
        assert!(num_leafs.is_power_of_two());
        let depth = num_leafs.trailing_zeros() as usize;
        let mut nodes = vec![Hash::default(); 2 * num_leafs - 1];

        // Hash the tree
        // TODO: leafs.iter()
        for i in 0..leafs.len() {
            let leaf = leafs.leaf(i);
            // At `depth` there should always be an `i `th leaf.
            let mut cursor = Index::from_depth_offset(depth, i).unwrap();
            nodes[cursor.as_index()] = leaf.hash();
            while cursor.is_right() {
                cursor = cursor.parent().unwrap();
                nodes[cursor.as_index()] = Node(
                    &nodes[cursor.left_child().as_index()],
                    &nodes[cursor.right_child().as_index()],
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

    pub fn root(&self) -> &Hash {
        &self[Index::root()]
    }

    pub fn proof(&self, indices: &[usize]) -> Proof {
        // Allocate space for decommitments
        let mut decommitments: Vec<Hash> = Vec::new();

        // Convert leaf indices to a sorted list of unique MerkleIndices.
        let mut indices: Vec<Index> = indices
            .iter()
            .map(|i| Index::from_depth_offset(self.depth, *i).expect("Index out of range"))
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
                decommitments.push(self[*i].clone());
            }
            indices = next_indices
        }

        // TODO
        unimplemented!()
        // Proof {
        // depth: self.depth,
        // decommitments,
        // }
    }
}

impl<Container: Merkelizable> IndexOp<Index> for Tree<'_, Container> {
    type Output = Hash;

    fn index(&self, index: Index) -> &Hash {
        &self.nodes[index.as_index()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use macros_decl::hex;
    use u256::U256;

    impl Merkelizable for [U256] {
        type Leaf = U256;

        fn len(&self) -> usize {
            self.len()
        }

        fn leaf(&self, index: usize) -> &U256 {
            &self[index]
        }
    }

    // #[test]
    // fn test_merkle_creation_and_proof() {
    // let depth = 6;
    // let mut leaves = Vec::new();
    //
    // for i in 0..2_u64.pow(depth) {
    // leaves.push(U256::from((i + 10).pow(3)));
    // }
    //
    // let tree = Tree::new(leaves.as_slice());
    //
    // assert_eq!(
    // tree.root().as_bytes(),
    // hex!("fd112f44bc944f33e2567f86eea202350913b11c000000000000000000000000")
    // );
    // let mut values = vec![
    // (1, leaves[1].clone()),
    // (10, leaves[10].clone()),
    // (11, leaves[11].clone()),
    // (14, leaves[14].clone()),
    // ];
    //
    // let indices = vec![1, 11, 14];
    // let decommitment = tree.proof(tree.as_slice(), &indices, &leaves.as_slice());
    // let non_root = Hash::new(hex!(
    // "ed112f44bc944f33e2567f86eea202350913b11c000000000000000000000000"
    // ));
    //
    // TODO
    // assert!(verify(
    // &tree[1],
    // depth,
    // values.as_mut_slice(),
    // &decommitment
    // ));
    // assert!(!verify(
    // &non_root,
    // depth,
    // values.as_mut_slice(),
    // &decommitment
    // ));
    // }
}
