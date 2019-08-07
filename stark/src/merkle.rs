use crate::{
    hash::Hash, hashable::Hashable, masked_keccak::MaskedKeccak, merkle_index::MerkleIndex,
    proofs::*,
};
use rayon::prelude::*;
use std::{
    marker::{PhantomData, Sync},
    ops::Index,
};

#[derive(Clone, Debug)]
struct MerkleNode<'a>(&'a Hash, &'a Hash);

impl Hashable for MerkleNode<'_> {
    fn hash(&self) -> Hash {
        let mut hasher = MaskedKeccak::new();
        hasher.update(self.0.as_bytes());
        hasher.update(self.1.as_bytes());
        hasher.hash()
    }
}

// Only internal nodes are stored, not the leaves
// TODO: We could create a sparse tree where the first n layers are skipped.
#[derive(Clone, Debug)]
pub struct MerkleTree<Leaf: Hashable> {
    num_leafs: usize,
    cursor:    Option<MerkleIndex>,
    depth:     usize,
    nodes:     Vec<Hash>,
    leaf_type: PhantomData<Leaf>,
}

impl<Leaf: Hashable> Index<MerkleIndex> for MerkleTree<Leaf> {
    type Output = Hash;

    fn index(&self, index: MerkleIndex) -> &Hash {
        &self.nodes[index.index()]
    }
}

#[derive(Clone, Debug)]
pub struct MerkleProof {
    decommitments: Vec<Hash>,
}

#[allow(dead_code)] // TODO: Remove
impl<Leaf: Hashable> MerkleTree<Leaf> {
    pub fn new(size: usize) -> Self {
        assert!(size.is_power_of_two());
        let depth = size.trailing_zeros() as usize;
        MerkleTree {
            num_leafs: size,
            cursor: Some(MerkleIndex::from_depth_offset(depth, 0)),
            depth,
            nodes: vec![Hash::default(); 2 * size - 1],
            leaf_type: PhantomData,
        }
    }

    pub fn from_iter<Iter>(iter: Iter) -> Self
    where
        Iter: ExactSizeIterator<Item = Leaf>,
    {
        let mut tree = MerkleTree::new(iter.len());
        for leaf in iter {
            tree.append(&leaf);
        }
        tree
    }

    /// Incrementally compute the tree by advancing the cursor
    // OPT: Write all leaves first, then compute the higher layers in one go.
    pub fn append(&mut self, leaf: &Leaf) {
        let mut cursor = self.cursor.expect("Can not append more leafs to the tree.");
        self.nodes[cursor.index()] = leaf.hash();
        self.cursor = cursor.right_neighbor();
        while cursor.is_right() {
            cursor = cursor.parent().unwrap();
            self.nodes[cursor.index()] =
                MerkleNode(&self[cursor.left_child()], &self[cursor.right_child()]).hash()
        }
    }

    pub fn root(&self) -> &Hash {
        &self[MerkleIndex::root()]
    }

    pub fn proof<'a>(&self, indices: &[usize], leafs: &Fn(usize) -> &'a Leaf) -> MerkleProof {
        // Allocate space for decommitments
        let mut decommitments: Vec<Hash> = Vec::new();

        // Convert leaf indices to a sorted list of unique MerkleIndices.
        let mut indices: Vec<MerkleIndex> = indices
            .iter()
            .map(|i| MerkleIndex::from_depth_offset(self.depth, *i))
            .collect();
        indices.sort();
        indices.dedup();

        // Iterate over indices
        while !indices.is_empty() {
            let mut next_indices: Vec<MerkleIndex> = Vec::new();
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

        MerkleProof { decommitments }
    }

    pub fn verify<'a>(
        _root: &Hash,
        _indices: &[usize],
        _leafs: &Fn(usize) -> &'a Leaf,
        _proof: &MerkleProof,
    ) -> bool {
        unimplemented!()
    }
}

pub fn make_tree_direct<T: Hashable>(leaves: &[T]) -> Vec<Hash> {
    let n = leaves.len();
    let depth = n.trailing_zeros(); // Log_2 of n
    let layer1_index = 2_usize.pow(depth - 1);
    let mut tree = vec![Hash::default(); n]; // Get my vector heap for end results

    for (index, pair) in leaves.chunks(2).enumerate() {
        tree[layer1_index + index] =
            MerkleNode(&pair[0_usize].hash(), &pair[1_usize].hash()).hash();
    }
    for i in (0..(2_usize.pow(depth - 1))).rev() {
        tree[i] = MerkleNode(&tree[2 * i], &tree[2 * i + 1]).hash();
    }
    tree
}

pub fn make_tree<T: Hashable + Sync>(leaves: &[T]) -> Vec<Hash> {
    if leaves.len() < 256 {
        make_tree_direct(leaves)
    } else {
        make_tree_threaded(leaves)
    }
}

pub fn make_tree_threaded<T: Hashable + Sync>(leaves: &[T]) -> Vec<Hash> {
    let n = leaves.len();
    debug_assert!(n.is_power_of_two());
    let depth = n.trailing_zeros() as usize;

    let mut layers = Vec::with_capacity(depth);
    let mut hold = Vec::with_capacity(n / 2);
    leaves
        .into_par_iter()
        .chunks(2)
        .map(|pair| MerkleNode(&pair[0_usize].hash(), &pair[1_usize].hash()).hash())
        .collect_into_vec(&mut hold);
    layers.push(hold);

    for i in 1..(depth) {
        let mut hold = Vec::with_capacity(layers[i - 1].len() / 2);
        layers[i - 1]
            .clone()
            .into_par_iter()
            .chunks(2)
            .map(|pair| MerkleNode(&pair[0_usize], &pair[1_usize]).hash())
            .collect_into_vec(&mut hold);
        layers.push(hold);
    }
    layers.push(vec![Hash::default()]);

    layers.into_iter().rev().flatten().collect()
}

pub fn proof<R: Hashable, T: Groupable<R>>(
    tree: &[Hash],
    indices: &[usize],
    source: T,
) -> Vec<Hash> {
    debug_assert!(tree.len().is_power_of_two());
    let depth = tree.len().trailing_zeros();
    let num_leaves = 2_usize.pow(depth);
    let num_nodes = 2 * num_leaves;
    let mut known = vec![false; num_nodes + 1];
    let mut decommitment: Vec<Hash> = Vec::new();

    let mut peekable_indicies = indices.iter().peekable();
    let mut excluded_pair = false;
    for &index in indices.iter() {
        peekable_indicies.next();
        known[num_leaves + index % num_leaves] = true;

        if index % 2 == 0 {
            known[num_leaves + 1 + index % num_leaves] = true;
            let prophet = peekable_indicies.peek();
            match prophet {
                Some(x) => {
                    if **x != index + 1 {
                        decommitment.push(source.make_group(index + 1).hash());
                    } else {
                        excluded_pair = true;
                    }
                }
                None => {
                    decommitment.push(source.make_group(index + 1).hash());
                }
            }
        } else if !excluded_pair {
            known[num_leaves - 1 + index % num_leaves] = true;
            decommitment.push(source.make_group(index - 1).hash());
        } else {
            known[num_leaves - 1 + index % num_leaves] = true;
            excluded_pair = false;
        }
    }

    for i in (2_usize.pow(depth - 1))..(2_usize.pow(depth)) {
        let left = known[2 * i];
        let right = known[2 * i + 1];
        known[i] = left || right;
    }

    for d in (1..depth).rev() {
        for i in (2_usize.pow(d - 1))..(2_usize.pow(d)) {
            let left = known[2 * i];
            let right = known[2 * i + 1];
            if left && !right {
                decommitment.push(tree[2 * i + 1].clone());
            }
            if !left && right {
                decommitment.push(tree[2 * i].clone());
            }
            known[i] = left || right;
        }
    }
    decommitment
}

pub fn verify<T: Hashable>(
    root: Hash,
    depth: u32,
    values: &mut [(u64, T)],
    mut decommitment: Vec<Hash>,
) -> bool {
    let mut queue = Vec::with_capacity(values.len());
    values.sort_by(|a, b| b.0.cmp(&a.0)); // Sorts the list by index
    for leaf in values.iter() {
        let tree_index = 2_u64.pow(depth) + leaf.0;
        queue.push((tree_index, leaf.1.hash()));
    }
    let mut start = values.len() - 1;
    let mut current = start;
    loop {
        if queue.is_empty() {
            break;
        }

        let (index, data_hash) = queue.remove(0); // Debug check that this is doing it right

        if index == 1 {
            return data_hash == root;
        } else if index % 2 == 0 {
            queue.push((
                index / 2,
                MerkleNode(&data_hash, &decommitment.remove(current)).hash(),
            ));

            if current == 0 {
                current = start;
            } else {
                current -= 1;
            }
        } else if !queue.is_empty() && queue[0].0 == index - 1 {
            let (_, sibbling_hash) = queue.remove(0);
            queue.push((index / 2, MerkleNode(&sibbling_hash, &data_hash).hash()));

            if start != 0 {
                start -= 1;
            }
            if start != 0 {
                current %= start;
            } else {
                current = 0;
            }
        } else {
            queue.push((
                index / 2,
                MerkleNode(&decommitment.remove(current), &data_hash).hash(),
            ));

            if current == 0 {
                current = start;
            } else {
                current -= 1;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::*;
    use u256::U256;

    impl Groupable<U256> for &[U256] {
        fn make_group(&self, index: usize) -> U256 {
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
            (11, leaves[11].clone()),
            (14, leaves[14].clone()),
        ];

        let indices = vec![1, 11, 14];
        let decommitment = proof(tree.as_slice(), &indices, leaves.as_slice());
        let non_root = Hash::new(hex!(
            "ed112f44bc944f33e2567f86eea202350913b11c000000000000000000000000"
        ));

        assert!(verify(
            tree[1].clone(),
            depth,
            values.as_mut_slice(),
            decommitment.clone()
        ));
        assert!(!verify(
            non_root,
            depth,
            values.as_mut_slice(),
            decommitment
        ));
    }
}
