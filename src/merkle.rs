use crate::u256::U256;
use crate::u256h;
use tiny_keccak::Keccak;

fn mask(data: &mut [u8; 32]) {
    for i in 0..32 {
        if i > 19 {
            data[i] = 0 as u8;
        }
    }
}

pub fn hash_leaf(mut leaf: U256) -> [u8; 32] {
    U256::to_bytes_be(&leaf)
}

pub fn hash_leaf_list(mut leaf: Vec<U256>) -> [u8; 32] {
    if (&leaf).len() == 1 {
        return hash_leaf(leaf[0].clone());
    }

    let mut sha3 = Keccak::new_keccak256();
    for x in leaf.iter() {
        sha3.update(&(U256::to_bytes_be(x)));
    }
    let mut res: [u8; 32] = [0; 32];
    sha3.finalize(&mut res);
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

pub fn hash_node_vec(left_node: &Vec<u8>, right_node: &Vec<u8>) -> [u8; 32] {
    let mut res: [u8; 32] = [0; 32];
    let mut sha3 = Keccak::new_keccak256();
    sha3.update(&left_node);
    sha3.update(&right_node);
    sha3.finalize(&mut res);
    mask(&mut res);

    res
}

pub fn make_tree(leaves: Vec<U256>) -> Vec<[u8; 32]> {
    let n = leaves.len() as u64;
    let depth = 64 - n.leading_zeros() - 1; //Log_2 of n

    let mut tree = vec![[0; 32]; 2 * n as usize]; //Get my vector heap for end results
    for i in 0..n {
        tree[(2_u64.pow(depth) + i) as usize] = hash_leaf(leaves[(i) as usize].clone());
    }
    for i in (0..(2_u64.pow(depth))).rev() {
        tree[i as usize] = hash_node(&tree[(2 * i) as usize], &tree[(2 * i + 1) as usize]);
    }
    tree
}

pub fn proof(tree: Vec<[u8; 32]>, indices: Vec<usize>) -> Vec<[u8; 32]> {
    let depth = 64 - (tree.len() as u64).leading_zeros() - 2; //Log base 2 - 1
    let num_leaves = 2_u64.pow(depth);
    let num_nodes = 2 * num_leaves;
    let mut known = vec![false; num_nodes as usize];
    let mut decommitment = Vec::new();

    let fixed = 2_u64.pow(depth);
    for i in indices.iter() {
        known[(fixed as usize) + i] = true;
    }

    for i in (1..2_u64.pow(depth) - 1).rev() {
        let left = known[(2 * i) as usize];
        let right = known[(2 * i + 1) as usize];
        if left && !right {
            decommitment.push(tree[(2 * i + 1) as usize].clone());
        }
        if !left && right {
            decommitment.push(tree[(2 * i) as usize].clone());
        }
        known[i as usize] = left || right;
    }
    decommitment
}

pub fn verify(
    root: [u8; 32],
    depth: u32,
    mut values: Vec<(u64, U256)>,
    mut decommitment: Vec<[u8; 32]>,
) -> bool {
    let mut queue = Vec::with_capacity(values.len());
    values.sort_by(|a, b| b.0.cmp(&a.0)); //Sorts the list by index
    for leaf in values.iter() {
        let tree_index = 2_u64.pow(depth) + leaf.0;
        queue.push((tree_index, hash_leaf(leaf.1.clone())));
    }
    loop {
        if queue.is_empty() {
            break;
        }
        let (index, data_hash) = queue.remove(0); //Debug check that this is doing it right
        if index == 1 {
            return data_hash == root;
        } else if index % 2 == 0 {
            queue.push((index / 2, hash_node(&data_hash, &decommitment.remove(0))));
        } else if queue.len() > 0 && queue[0].0 == index - 1 {
            let (_, sibbling_hash) = queue.remove(0);
            queue.push((index / 2, hash_node(&sibbling_hash, &data_hash)));
        } else {
            queue.push((index / 2, hash_node(&decommitment.remove(0), &data_hash)));
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::*;

    #[test]
    fn test_merkle_creation_and_proof() {
        let depth = 6;
        let mut leaves = Vec::new();

        for i in 0..2_u64.pow(depth) {
            leaves.push(U256::from((i + 10).pow(3)));
        }

        let tree = make_tree(leaves.clone());

        // Value from python masked mode
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
        let mut decommitment = proof(tree.clone(), indices);
        let non_root = hex!("ed112f44bc944f33e2567f86eea202350913b11c000000000000000000000000");
        assert!(verify(
            tree[1].clone(),
            depth,
            values.clone(),
            decommitment.clone()
        ));
        assert!(!verify(non_root, depth, values, decommitment));
    }
}
