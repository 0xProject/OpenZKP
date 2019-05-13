extern crate tiny_keccak;

use tiny_keccak::*; //Gets the hash functions we need to use for this merkle libary
use crate::u256::U256;
use crate::u256h;
use hex_literal::*;

#[derive(Copy, Clone, Debug)]
pub enum HashType {
    Starkware, // SHA256 and little endian
    EVM,  // Keccack256 and big endian
    MASKED, // Keccack256 with mask
}

fn mask(data: &mut [u8; 32]){
    for i in 0..32{
        if i > 20{
            data[i] = 0 as u8;
        }
    }
}

pub fn hash_leaf(mut leaf: U256, hash: &HashType) -> Vec<u8>{
    match hash{
        HashType::Starkware => {(&mut leaf).to_le(); println!("{:?}", hash);} ,
        _ => {},
    }
    transform_U256(leaf)
}

pub fn hash_leaf_list(mut leaf: Vec<U256>, hash: &HashType) -> Vec<u8>{
    match hash{
        HashType::Starkware => {return hash_leaf(leaf[0].clone(), hash);},
        HashType::EVM  => {return hash_leaf(leaf[0].clone(), hash);},
        HashType::MASKED => {
            if (&leaf).len() == 1{ return hash_leaf(leaf[0].clone(), hash); }
            
            let mut sha3 = Keccak::new_keccak256();
            for x in leaf.iter(){
                sha3.update(&(transform_U256(x.clone())));
            }
            let mut res: [u8; 32] = [0; 32];
            sha3.finalize(&mut res);
            return [res].concat();
        }
    }
}

pub fn hash_node(left_node: [u8; 32], right_node: [u8; 32], hash: HashType) -> [u8; 32]{
    let mut res: [u8; 32] = [0; 32];
    match hash{
        HashType::Starkware => {
            let mut sha3 = Keccak::new_sha3_256();
            sha3.update(&left_node);
            sha3.update(&right_node);
            sha3.finalize(&mut res);
            },
        HashType::EVM => {
            let mut sha3 = Keccak::new_keccak256();
            sha3.update(&left_node);
            sha3.update(&right_node);
            sha3.finalize(&mut res);
        },
        HashType::MASKED => {
            let mut sha3 = Keccak::new_keccak256();
            sha3.update(&left_node);
            sha3.update(&right_node);
            sha3.finalize(&mut res);
            mask(&mut res);
        },
    }
    res
}

pub fn hash_node_vec(left_node: &Vec<u8>, right_node: &Vec<u8>, hash: &HashType) -> Vec<u8>{
    let mut res: [u8; 32] = [0; 32];
    match hash{
        HashType::Starkware => {
            let mut sha3 = Keccak::new_sha3_256();
            sha3.update(left_node);
            sha3.update(right_node);
            sha3.finalize(&mut res);
            },
        HashType::EVM => {
            let mut sha3 = Keccak::new_keccak256();
            sha3.update(left_node);
            sha3.update(right_node);
            sha3.finalize(&mut res);
        },
        HashType::MASKED => {
            let mut sha3 = Keccak::new_keccak256();
            sha3.update(&left_node);
            sha3.update(&right_node);
            sha3.finalize(&mut res);
            mask(&mut res);
        },
    }
    [res].concat()
}

pub fn make_tree(leaves: Vec<U256>, hash: HashType) -> Vec<Vec<u8>>{
    let n = leaves.len() as u64;
    let depth = 64 - n.leading_zeros()-1; //Log_2 of n


    let mut tree = vec![ vec![0; 32]; 2*n as usize];  //Get my vector heap for end results
    for i in 0..n {
        tree[(2_u64.pow(depth) + i) as usize] =  hash_leaf(leaves[(i) as usize].clone(), &hash);
    }
    //println!("{:?}", tree);
    for i in (0..(2_u64.pow(depth))).rev(){
        //println!("{:?}", hash_node_vec(&tree[(2*i) as usize], &tree[(2*i+1)  as usize], &hash));
        tree[i as usize] = hash_node_vec(&tree[(2*i) as usize], &tree[(2*i+1)  as usize], &hash);
    }
    tree
}


//Helper functions
fn transform_U256(data: U256) -> Vec<u8>{
    let ret = [transform_u64(data.c3), transform_u64(data.c2), transform_u64(data.c1), transform_u64(data.c0)];
    ret.concat()
}

fn transform_u64(x:u64) -> [u8; 8] {
    let b1 : u8 = ((x >> 56) & 0xff) as u8;
    let b2 : u8 = ((x >> 48) & 0xff) as u8;
    let b3 : u8 = ((x >> 40) & 0xff) as u8;
    let b4 : u8 = ((x >> 32) & 0xff) as u8;
    let b5 : u8 = ((x >> 24) & 0xff) as u8;
    let b6 : u8 = ((x >> 16) & 0xff) as u8;
    let b7 : u8 = ((x >> 8) & 0xff) as u8;
    let b8 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4, b5, b6, b7, b8]
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate hex;
    use hex::{encode};

    #[test]
    fn creation_test(){
        let depth = 6;
        let mut leaves = Vec::new();

        for i in 0..2_u64.pow(depth){
            leaves.push(U256::from((i+10).pow(3)));
        }
        
        let tree = make_tree(leaves, HashType::EVM);
        
        assert_eq!(encode(tree[1].clone()),"42b30fb1efc6e1a7e878be62e4ac40059e83ad61d29b2f1f9dbbf8cba339028b");
    }
}