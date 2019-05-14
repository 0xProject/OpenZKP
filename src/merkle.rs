extern crate tiny_keccak;

use crate::u256::U256;
use crate::u256h;
use hex_literal::*;
use tiny_keccak::*; //Gets the hash functions we need to use for this merkle libary
use crypto::digest::Digest;
use crypto::sha2::*;

extern crate hex;
    use hex::encode;

#[derive(Copy, Clone, Debug)]
pub enum HashType {
    Starkware, // SHA256 and little endian
    EVM,       // Keccack256 and big endian
    MASKED,    // Keccack256 with mask
}

fn mask(data: &mut [u8; 32]) {
    for i in 0..32 {
        if i > 20 {
            data[i] = 0 as u8;
        }
    }
}

pub fn hash_leaf(mut leaf: U256, hash: &HashType) -> Vec<u8> {
    match hash {
        HashType::Starkware => {
            (&mut leaf).to_le();
        }
        _ => {}
    }
    transform_U256(leaf)
}

pub fn hash_leaf_list(mut leaf: Vec<U256>, hash: &HashType) -> Vec<u8> {
    match hash {
        HashType::Starkware => {
            return hash_leaf(leaf[0].clone(), hash);
        }
        HashType::EVM => {
            return hash_leaf(leaf[0].clone(), hash);
        }
        HashType::MASKED => {
            if (&leaf).len() == 1 {
                return hash_leaf(leaf[0].clone(), hash);
            }

            let mut sha3 = Keccak::new_keccak256();
            for x in leaf.iter() {
                sha3.update(&(transform_U256(x.clone())));
            }
            let mut res: [u8; 32] = [0; 32];
            sha3.finalize(&mut res);
            return [res].concat();
        }
    }
}

pub fn hash_node(left_node: [u8; 32], right_node: [u8; 32], hash: HashType) -> [u8; 32] {
    let mut res: [u8; 32] = [0; 32];
    match hash {
        HashType::Starkware => {
            let mut hasher = Sha256::new();
            hasher.input(&left_node);
            hasher.input(&right_node);
            hasher.result(&mut res);
        }
        HashType::EVM => {
            let mut sha3 = Keccak::new_keccak256();
            sha3.update(&left_node);
            sha3.update(&right_node);
            sha3.finalize(&mut res);
        }
        HashType::MASKED => {
            let mut sha3 = Keccak::new_keccak256();
            sha3.update(&left_node);
            sha3.update(&right_node);
            sha3.finalize(&mut res);
            mask(&mut res);
        }
    }
    res
}

pub fn hash_node_vec(left_node: &Vec<u8>, right_node: &Vec<u8>, hash: &HashType) -> Vec<u8> {
    let mut res: [u8; 32] = [0; 32];
    match hash {
        HashType::Starkware => {
            let mut hasher = Sha256::new();
            hasher.input(left_node);
            hasher.input(right_node);
            hasher.result(&mut res);
        }
        HashType::EVM => {
            let mut sha3 = Keccak::new_keccak256();
            sha3.update(left_node);
            sha3.update(right_node);
            sha3.finalize(&mut res);
        }
        HashType::MASKED => {
            let mut sha3 = Keccak::new_keccak256();
            sha3.update(&left_node);
            sha3.update(&right_node);
            sha3.finalize(&mut res);
            mask(&mut res);
        }
    }
    [res].concat()
}

pub fn make_tree(leaves: Vec<U256>, hash: HashType) -> Vec<Vec<u8>> {
    let n = leaves.len() as u64;
    let depth = 64 - n.leading_zeros() - 1; //Log_2 of n

    let mut tree = vec![vec![0; 32]; 2 * n as usize]; //Get my vector heap for end results
    for i in 0..n {
        tree[(2_u64.pow(depth) + i) as usize] = hash_leaf(leaves[(i) as usize].clone(), &hash);
    }
    for i in (0..(2_u64.pow(depth))).rev() {
        tree[i as usize] =
            hash_node_vec(&tree[(2 * i) as usize], &tree[(2 * i + 1) as usize], &hash);
    }
    tree
}

pub fn proof(tree: Vec<Vec<u8>>, indices: Vec<usize>) -> Vec<Vec<u8>> {
    let depth = 64 - (tree.len() as u64).leading_zeros() - 2; //Log base 2 - 1
    let num_leaves = 2_u64.pow(depth);
    let num_nodes = 2 * num_leaves;
    let mut known = vec![false; num_nodes as usize];
    let mut decommitment = Vec::new();

    let fixed = 2_u64.pow(depth);
    for i in indices.iter() {
        known[(fixed as usize) + i] = true;
    }

    for i in (1..2_u64.pow(depth)-1).rev(){
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
    root: Vec<u8>,
    depth: u32,
    mut values: Vec<(u64, U256)>,
    mut decommitment: Vec<Vec<u8>>,
    hash: HashType,
) -> bool {
    let mut queue = Vec::with_capacity(values.len());
    values.sort_by(|a, b| b.0.cmp(&a.0)); //Sorts the list by index
    for leaf in values.iter() {
        let tree_index = 2_u64.pow(depth) + leaf.0;
        queue.push((tree_index, hash_leaf(leaf.1.clone(), &hash)));
    }
    loop {
        if queue.len() == 0 {
            break;
        }
        let (index, data_hash) = queue.remove(0); //Debug check that this is doing it right
        if index == 1 {
            return data_hash == root;
        } else if index % 2 == 0 {
            queue.push((
                index / 2,
                hash_node_vec(&data_hash, &decommitment.remove(0), &hash),
            ));
        } else if queue.len() > 0 && queue[0].0 == index - 1 {
            let (_, sibbling_hash) = queue.remove(0);
            queue.push((index / 2, hash_node_vec( &sibbling_hash, &data_hash, &hash)));
        } else {
            queue.push((
                index / 2,
                hash_node_vec(&decommitment.remove(0), &data_hash, &hash),
            ));
        }
    }

    false
}

//Helper functions
fn transform_U256(data: U256) -> Vec<u8> {
    let ret = [
        transform_u64(data.c3),
        transform_u64(data.c2),
        transform_u64(data.c1),
        transform_u64(data.c0),
    ];
    ret.concat()
}

fn transform_u64(x: u64) -> [u8; 8] {
    let b1: u8 = ((x >> 56) & 0xff) as u8;
    let b2: u8 = ((x >> 48) & 0xff) as u8;
    let b3: u8 = ((x >> 40) & 0xff) as u8;
    let b4: u8 = ((x >> 32) & 0xff) as u8;
    let b5: u8 = ((x >> 24) & 0xff) as u8;
    let b6: u8 = ((x >> 16) & 0xff) as u8;
    let b7: u8 = ((x >> 8) & 0xff) as u8;
    let b8: u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4, b5, b6, b7, b8];
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate hex;
    use hex::encode;

    #[test]
    fn test_merkle_creation_and_proof() {
        let depth = 6;
        let mut leaves = Vec::new();

        for i in 0..2_u64.pow(depth) {
            leaves.push(U256::from((i + 10).pow(3)));
        }

        let tree = make_tree(leaves.clone(), HashType::EVM);

        assert_eq!(
            encode(tree[1].clone()),
            "42b30fb1efc6e1a7e878be62e4ac40059e83ad61d29b2f1f9dbbf8cba339028b"
        );
        let mut values = vec![(1, leaves[1].clone()), (11, leaves[11].clone()), (14, leaves[14].clone())];
        let mut indices = vec![1, 11, 14];
        let mut decommitment = proof(tree.clone(), indices);
        assert!(verify(tree[1].clone(), depth, values, decommitment, HashType::EVM));
    }
    #[test]
    fn test_starkware_merkle(){
        let root = transform_U256(u256h!("ada70851a3af058545ab2d46228872463d5ce9919aa9027814595f2eaf6dd727").to_le_ret());
        let mut values = vec![(1, u256h!("cb6c2ee3e7caa624e41f5c92dc4d4aabcaec3db4472e17e632af643614459225")),
                               (14, u256h!("b37d6b9227094c545e03ac6102b0c5912b0a3b093a2f0b430cfcf927369e8d17"))];
        let mut decommitment = vec![transform_U256(u256h!("1d33eac298dda2eec370f2f4e3a19f93596476620aaf0720c70f51696460dd89").to_le_ret()),
                                    transform_U256(u256h!("70c8670e12c79d8d8804b07be9270820e323668396d612b55914c4095a2cf008").to_le_ret()),
                                    transform_U256(u256h!("3d7ae1a4a258d19692679d03d45f72710af44f6894aeb5f7f35298e4262c9e11").to_le_ret()),
                                    transform_U256(u256h!("2780c6c3921b597954a55980793c20d84ed094ca30cb099d96ec8d4f78a640f8").to_le_ret()),
                                    transform_U256(u256h!("f0ad716e2ec9ed540b259c5eee0327d13421c8940dcf24e93c6ddc74c39184e3").to_le_ret()),
                                    transform_U256(u256h!("91023ba61a85b2c3fe1835d2c2d2a48a11f626da2ef6af781510fcf4dcf0f122").to_le_ret())
                                    ];
        assert!(verify(root, 4, values, decommitment, HashType::Starkware));         
    }
}
