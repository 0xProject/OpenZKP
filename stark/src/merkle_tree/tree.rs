use super::{Commitment, Error, Hash, Hashable, Index, Node, Proof, Result, VectorCommitment};
use crate::{mmap_vec::MmapVec, require};
use std::collections::VecDeque;

/// Merkle tree
///
/// The tree will become the owner of the `Container`. This is necessary because
/// when low layer-omission is implemented we need immutable access to the
/// leaves. If shared ownership is required the `Container` can be an `Rc<_>`.
// OPT: Do not store leaf hashes but re-create.
// OPT: Allow up to `n` lower layers to be skipped.
// TODO: Make hash depend on type.
pub struct Tree<Container: VectorCommitment> {
    commitment: Commitment,
    nodes:      MmapVec<Hash>,
    leaves:     Container,
}

impl<Container: VectorCommitment> Tree<Container> {
    pub fn from_leaves(leaves: Container) -> Result<Self> {
        Self::from_leaves_skip_layers(leaves, 1)
    }

    pub fn from_leaves_skip_layers(leaves: Container, skip_layers: usize) -> Result<Self> {
        let size = leaves.len();
        if size == 0 {
            return Ok(Self {
                // TODO: Ideally give the empty tree a unique flag value.
                // Size zero commitment always exists
                commitment: Commitment::from_size_hash(size, &Hash::default()).unwrap(),
                nodes: MmapVec::with_capacity(0),
                leaves,
            });
        }
        // TODO: Support non power of two sizes
        require!(size.is_power_of_two(), Error::NumLeavesNotPowerOfTwo);
        require!(size <= Index::max_size(), Error::TreeToLarge);

        // Allocate result
        let leaf_depth = Index::depth_for_size(size);
        let nodes = if leaf_depth >= skip_layers {
            // The array size is the largest index + 1
            let depth = leaf_depth - skip_layers;
            let max_index = Index::from_depth_offset(depth, Index::size_at_depth(depth) - 1)
                .unwrap()
                .as_index();
            let mut nodes = MmapVec::with_capacity(max_index + 1);
            for _ in 0..=max_index {
                nodes.push(Hash::default());
            }
            nodes
        } else {
            MmapVec::with_capacity(0)
        };
        let mut result = Self {
            commitment: Commitment::default(),
            nodes,
            leaves,
        };

        // Hash the tree
        // OPT: Parallel implementation.
        if leaf_depth >= skip_layers {
            let depth = leaf_depth - skip_layers;
            for i in Index::iter_layer(depth) {
                result.nodes[i.as_index()] = result.compute_hash(i);
            }
            for depth in (0..depth).rev() {
                for i in Index::iter_layer(depth) {
                    result.nodes[i.as_index()] = Node(
                        &result.nodes[i.left_child().as_index()],
                        &result.nodes[i.right_child().as_index()],
                    )
                    .hash()
                }
            }
        }

        result.commitment =
            Commitment::from_size_hash(size, &result.node_hash(Index::root())).unwrap();
        Ok(result)
    }

    pub fn commitment(&self) -> &Commitment {
        &self.commitment
    }

    pub fn leaf_depth(&self) -> usize {
        Index::depth_for_size(self.leaves().len())
    }

    pub fn leaves(&self) -> &Container {
        &self.leaves
    }

    pub fn leaf(&self, index: usize) -> Container::Leaf {
        self.leaves.leaf(index)
    }

    fn compute_hash(&self, index: Index) -> Hash {
        assert!(index.depth() <= self.leaf_depth());
        if index.depth() == self.leaf_depth() {
            self.leaves.leaf_hash(index.offset())
        } else {
            Node(
                &self.compute_hash(index.left_child()),
                &self.compute_hash(index.right_child()),
            )
            .hash()
        }
    }

    pub fn node_hash(&self, index: Index) -> Hash {
        if index.as_index() < self.nodes.len() {
            self.nodes[index.as_index()].clone()
        } else {
            self.compute_hash(index)
        }
    }

    pub fn open(&self, indices: &[usize]) -> Result<Proof> {
        let indices = self.commitment().sort_indices(indices)?;
        let proof_indices: Vec<usize> = indices.iter().map(|i| i.offset()).collect();
        let mut indices: VecDeque<Index> = indices.into_iter().collect();
        let mut hashes: Vec<Hash> = Vec::new();

        while let Some(current) = indices.pop_front() {
            // Root node has no parent and means we are done
            if let Some(parent) = current.parent() {
                // Add parent index to the queue for the next pass
                indices.push_back(parent);

                // Since we have a parent, we must have a sibling
                let sibling = current.sibling().unwrap();

                // Check if we merge with the next merkle index.
                if let Some(&next) = indices.front() {
                    if next == sibling {
                        // Skip next and don't write a decommitment for either
                        let _ = indices.pop_front();
                        continue;
                    }
                }

                // Add a sibling hash to the decommitment
                hashes.push(self.node_hash(sibling));
            }
        }
        Proof::from_hashes(self.commitment(), &proof_indices, &hashes)
    }
}

// Quickcheck requires pass by value
#[allow(clippy::needless_pass_by_value)]
#[cfg(test)]
mod tests {
    use super::*;
    use macros_decl::hex;
    use quickcheck_macros::quickcheck;
    use u256::U256;

    #[test]
    fn test_explicit_values() {
        let depth = 6;
        let leaves: Vec<_> = (0..2_u64.pow(depth))
            .map(|i| U256::from((i + 10).pow(3)))
            .collect();

        // Build the tree
        let tree = Tree::from_leaves(leaves).unwrap();
        let root = tree.commitment();
        assert_eq!(
            root.hash().as_bytes(),
            hex!("fd112f44bc944f33e2567f86eea202350913b11c000000000000000000000000")
        );

        // Open indices
        let indices = vec![1, 11, 14];
        assert_eq!(root.proof_size(&indices).unwrap(), 9);
        let proof = tree.open(&indices).unwrap();
        #[rustfmt::skip]
        assert_eq!(proof.hashes(), &[
            Hash::new(hex!("00000000000000000000000000000000000000000000000000000000000003e8")),
            Hash::new(hex!("0000000000000000000000000000000000000000000000000000000000001f40")),
            Hash::new(hex!("0000000000000000000000000000000000000000000000000000000000003d09")),
            Hash::new(hex!("4ea8b9bafb11dafcfe132a26f8e343eaef0651d9000000000000000000000000")),
            Hash::new(hex!("023a7ce535cadd222093be053ac26f9b800ee476000000000000000000000000")),
            Hash::new(hex!("70b0744af2583d10e7e3236c731d37605e196e06000000000000000000000000")),
            Hash::new(hex!("221aea6e87862ba2d03543d0aa82c6bffee310ae000000000000000000000000")),
            Hash::new(hex!("68b58e5131703684edb16d41b763017dfaa24a35000000000000000000000000")),
            Hash::new(hex!("e108b7dc670810e8588c67c2fde7ec4cc00165e8000000000000000000000000")),
        ]);

        // Verify proof
        let select_leaves: Vec<_> = indices.iter().map(|&i| (i, tree.leaf(i))).collect();
        proof.verify(select_leaves.as_slice()).unwrap();

        // Verify non-root
        let non_root = Hash::new(hex!(
            "ed112f44bc944f33e2567f86eea202350913b11c000000000000000000000000"
        ));
        let non_proof = Proof::from_hashes(
            &Commitment::from_size_hash(root.size(), &non_root).unwrap(),
            &indices,
            &proof.hashes(),
        )
        .unwrap();
        assert_eq!(
            non_proof.verify(&select_leaves),
            Err(Error::RootHashMismatch)
        );
    }

    #[test]
    fn test_empty_tree() {
        let indices: Vec<usize> = vec![];
        let leaves: Vec<U256> = vec![];

        let tree = Tree::from_leaves(leaves).unwrap();
        let root = tree.commitment();

        // Open indices
        let proof = tree.open(&indices).unwrap();
        assert_eq!(root.proof_size(&indices).unwrap(), proof.hashes().len());

        // Verify proof
        let select_leaves: Vec<(usize, U256)> = vec![];
        proof.verify(&select_leaves).unwrap();
    }

    #[quickcheck]
    fn test_merkle_tree(depth: usize, skip: usize, indices: Vec<usize>, seed: U256) {
        // We want tests up to depth 8; adjust the input
        let depth = depth % 9;
        // We want to skip up to 3 layers; adjust the input
        let skip = skip % 4;
        let num_leaves = 1_usize << depth;
        let indices: Vec<_> = indices.iter().map(|&i| i % num_leaves).collect();
        let leaves: Vec<_> = (0..num_leaves)
            .map(|i| (&seed + U256::from(i)).pow(3).unwrap())
            .collect();

        // Build the tree
        let tree = Tree::from_leaves_skip_layers(leaves, skip).unwrap();
        let root = tree.commitment();

        // Open indices
        let proof = tree.open(&indices).unwrap();
        assert_eq!(root.proof_size(&indices).unwrap(), proof.hashes().len());

        // Verify proof
        let select_leaves: Vec<_> = indices.iter().map(|&i| (i, tree.leaf(i))).collect();
        proof.verify(&select_leaves).unwrap();
    }
}
