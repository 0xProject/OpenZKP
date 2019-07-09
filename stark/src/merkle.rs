use crate::proofs::*;
use primefield::FieldElement;
use rayon::prelude::*;
use tiny_keccak::Keccak;
use u256::U256;

pub trait Hashable {
    fn hash(&self) -> [u8; 32];
}

impl Hashable for U256 {
    fn hash(&self) -> [u8; 32] {
        hash_leaf(self)
    }
}

impl Hashable for &[U256] {
    fn hash(&self) -> [u8; 32] {
        hash_leaf_list(self)
    }
}

impl Hashable for Vec<U256> {
    fn hash(&self) -> [u8; 32] {
        self.as_slice().hash()
    }
}

// Note we have presumed that we want to hash the Montgomery adjusted value
impl Hashable for FieldElement {
    fn hash(&self) -> [u8; 32] {
        self.0.hash()
    }
}

fn mask(data: &mut [u8; 32]) {
    for d in data[20..].iter_mut() {
        *d = 0 as u8;
    }
}

pub fn hash_leaf(leaf: &U256) -> [u8; 32] {
    U256::to_bytes_be(leaf)
}

pub fn hash_leaf_list(leaf: &[U256]) -> [u8; 32] {
    if (&leaf).len() == 1 {
        return hash_leaf(&leaf[0]);
    }

    let mut sha3 = Keccak::new_keccak256();
    for x in leaf.iter() {
        sha3.update(&(U256::to_bytes_be(x)));
    }
    let mut res: [u8; 32] = [0; 32];
    sha3.finalize(&mut res);
    mask(&mut res);
    res
}

pub fn hash_node(left_node: &[u8; 32], right_node: &[u8; 32]) -> [u8; 32] {
    let mut res: [u8; 32] = [0; 32];

    let mut sha3 = Keccak::new_keccak256();
    sha3.update(left_node);
    sha3.update(right_node);
    sha3.finalize(&mut res);
    mask(&mut res);

    res
}

pub fn make_tree_direct<T: Hashable>(leaves: &[T]) -> Vec<[u8; 32]> {
    let n = leaves.len();
    let depth = n.trailing_zeros(); // Log_2 of n
    let layer1_index = 2_usize.pow(depth - 1);
    let mut tree = vec![[0; 32]; n]; // Get my vector heap for end results

    for (index, pair) in leaves.chunks(2).enumerate() {
        tree[layer1_index + index] = hash_node(&pair[0_usize].hash(), &pair[1_usize].hash());
    }
    for i in (0..(2_usize.pow(depth - 1))).rev() {
        tree[i] = hash_node(&tree[2 * i], &tree[2 * i + 1]);
    }
    tree
}

pub fn make_tree<T: Hashable + std::marker::Sync>(leaves: &[T]) -> Vec<[u8; 32]> {
    if leaves.len() < 256 {
        make_tree_direct(leaves)
    } else {
        make_tree_threaded(leaves)
    }
}

pub fn make_tree_threaded<T: Hashable + std::marker::Sync>(leaves: &[T]) -> Vec<[u8; 32]> {
    let n = leaves.len();
    debug_assert!(n.is_power_of_two());
    let depth = n.trailing_zeros() as usize;

    let mut layers = Vec::with_capacity(depth);
    let mut hold = Vec::with_capacity(n / 2);
    leaves
        .into_par_iter()
        .chunks(2)
        .map(|pair| hash_node(&pair[0_usize].hash(), &pair[1_usize].hash()))
        .collect_into_vec(&mut hold);
    layers.push(hold);

    for i in 1..(depth) {
        let mut hold = Vec::with_capacity(layers[i - 1].len() / 2);
        layers[i - 1]
            .clone()
            .into_par_iter()
            .chunks(2)
            .map(|pair| hash_node(&pair[0_usize], &pair[1_usize]))
            .collect_into_vec(&mut hold);
        layers.push(hold);
    }
    layers.push(vec![[0; 32]]);

    layers.into_iter().rev().flatten().collect()
}

pub fn proof<R: Hashable, T: Groupable<R>>(
    tree: &[[u8; 32]],
    indices: &[usize],
    source: T,
) -> Vec<[u8; 32]> {
    debug_assert!(tree.len().is_power_of_two());
    let depth = tree.len().trailing_zeros();
    let num_leaves = 2_usize.pow(depth);
    let num_nodes = 2 * num_leaves;
    let mut known = vec![false; num_nodes + 1];
    let mut decommitment = Vec::new();

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
                decommitment.push(tree[2 * i + 1]);
            }
            if !left && right {
                decommitment.push(tree[2 * i]);
            }
            known[i] = left || right;
        }
    }
    decommitment
}

pub fn verify<T: Hashable>(
    root: [u8; 32],
    depth: u32,
    values: &mut [(u64, T)],
    mut decommitment: Vec<[u8; 32]>,
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
                hash_node(&data_hash, &decommitment.remove(current)),
            ));

            if current == 0 {
                current = start;
            } else {
                current -= 1;
            }
        } else if !queue.is_empty() && queue[0].0 == index - 1 {
            let (_, sibbling_hash) = queue.remove(0);
            queue.push((index / 2, hash_node(&sibbling_hash, &data_hash)));

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
                hash_node(&decommitment.remove(current), &data_hash),
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

        assert_eq!(
            tree[1],
            hex!("fd112f44bc944f33e2567f86eea202350913b11c000000000000000000000000")
        );
        let mut values = vec![
            (1, leaves[1].clone()),
            (11, leaves[11].clone()),
            (14, leaves[14].clone()),
        ];

        let indices = vec![1, 11, 14];
        let decommitment = proof(tree.as_slice(), &indices, leaves.as_slice());
        let non_root = hex!("ed112f44bc944f33e2567f86eea202350913b11c000000000000000000000000");

        assert!(verify(
            tree[1],
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
