use crate::u256::U256;
use rayon::prelude::*;
use tiny_keccak::Keccak;

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
    let n = leaves.len() as u64;
    let depth = 64 - n.leading_zeros() - 1; //Log_2 of n

    let mut tree = vec![[0; 32]; 2 * n as usize]; //Get my vector heap for end results
    for i in 0..n {
        tree[(2_u64.pow(depth) + i) as usize] = leaves[(i) as usize].hash();
    }
    for i in (0..(2_u64.pow(depth))).rev() {
        tree[i as usize] = hash_node(&tree[(2 * i) as usize], &tree[(2 * i + 1) as usize]);
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

pub const THREADS_MAX: usize = 16; //TODO - Figure out how many threads is opt

pub fn make_tree_threaded<T: Hashable + std::marker::Sync>(leaves: &[T]) -> Vec<[u8; 32]> {
    let threads = THREADS_MAX;

    let n = leaves.len();
    let depth = 64 - n.leading_zeros() - 1;

    let mut layers = Vec::with_capacity(depth as usize);
    let base_layer: Vec<[u8; 32]> = (0..threads)
        .into_par_iter()
        .map(|i| {
            let ret: Vec<[u8; 32]> = leaves[i * (n / threads)..(i * (n / threads) + (n / threads))]
                .iter()
                .map(|element| element.hash())
                .collect();
            ret
        })
        .flatten()
        .collect();
    layers.push(base_layer);

    for i in 1..((depth + 1) as usize) {
        let mut hold = Vec::with_capacity(layers[0].len() / 2);
        layers[i - 1]
            .clone()
            .into_par_iter()
            .chunks(2)
            .map(|pair| hash_node(&pair[0_usize], &pair[1_usize]))
            .collect_into_vec(&mut hold);
        layers.push(hold);
    }

    layers.push(vec![[0; 32]]); //TODO - This logic puts the root at the top, but the previous didn't, either add another hash comp or change the rest of the code to match

    layers.into_iter().rev().flatten().collect()
}

pub fn proof(tree: &[[u8; 32]], indices: &[usize]) -> Vec<[u8; 32]> {
    let depth = 64 - (tree.len() as u64).leading_zeros() - 1; //Log base 2 - 1
    let num_leaves = 2_u64.pow(depth);
    let num_nodes = 2 * num_leaves;
    let mut known = vec![false; (num_nodes + 1) as usize];
    let mut decommitment = Vec::new();

    let fixed = 2_u64.pow(depth - 1);
    for i in indices.iter() {
        known[(fixed + (*i as u64) % num_leaves) as usize] = true;
    }

    for d in (1..depth).rev() {
        for i in (2_u64.pow(d - 1))..(2_u64.pow(d)) {
            let left = known[(2 * i) as usize];
            let right = known[(2 * i + 1) as usize];
            if left && !right {
                decommitment.push(tree[(2 * i + 1) as usize]);
            }
            if !left && right {
                decommitment.push(tree[(2 * i) as usize]);
            }
            known[i as usize] = left || right;
        }
    }
    decommitment
}

pub fn verify(
    root: [u8; 32],
    depth: u32,
    values: &mut [(u64, U256)],
    mut decommitment: Vec<[u8; 32]>,
) -> bool {
    let mut queue = Vec::with_capacity(values.len());
    values.sort_by(|a, b| b.0.cmp(&a.0)); //Sorts the list by index
    for leaf in values.iter() {
        let tree_index = 2_u64.pow(depth) + leaf.0;
        queue.push((tree_index, hash_leaf(&leaf.1)));
    }
    let mut start = values.len() - 1;
    let mut current = start;
    loop {
        if queue.is_empty() {
            break;
        }

        let (index, data_hash) = queue.remove(0); //Debug check that this is doing it right

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
    use hex::*;
    use hex_literal::*;

    #[test]
    fn test_merkle_creation_and_proof() {
        let depth = 6;
        let mut leaves = Vec::new();

        for i in 0..2_u64.pow(depth) {
            leaves.push(U256::from((i + 10).pow(3)));
        }

        let tree = make_tree_threaded(leaves.as_slice());
        let dirrect_tree = make_tree(leaves.as_slice());

        assert_eq!(
            tree[1].clone(),
            hex!("fd112f44bc944f33e2567f86eea202350913b11c000000000000000000000000")
        );
        let mut values = vec![
            (1, leaves[1].clone()),
            (11, leaves[11].clone()),
            (14, leaves[14].clone()),
        ];
        let mut indices = vec![1, 11, 14];
        let mut decommitment = proof(tree.as_slice(), &indices);
        let non_root = hex!("ed112f44bc944f33e2567f86eea202350913b11c000000000000000000000000");

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
