#![warn(clippy::all)]
use criterion::{black_box, Criterion};
use zkp_criterion_utils::{log_size_bench, log_thread_bench};
use zkp_merkle_tree::Tree;
use zkp_u256::U256;

#[cfg(not(test))]
const SIZES: [usize; 6] = [64, 256, 1024, 4096, 16384, 65536];

#[cfg(test)]
const SIZES: [usize; 1] = [64];

fn merkle_tree_size(crit: &mut Criterion) {
    log_size_bench(crit, "Merkle tree size", &SIZES, move |bench, size| {
        let leaves: Vec<_> = (0..size).map(U256::from).collect();
        bench.iter(|| black_box(Tree::from_leaves(black_box(leaves.clone()))))
    });
}

fn merkle_tree_threads(crit: &mut Criterion) {
    let size: usize = *SIZES.last().unwrap();
    log_thread_bench(crit, "Merkle tree threads", size, move |bench| {
        let leaves: Vec<_> = (0..size).map(U256::from).collect();
        bench.iter(|| black_box(Tree::from_leaves(black_box(leaves.clone()))))
    });
}

fn main() {
    let crit = &mut Criterion::default().configure_from_args();
    merkle_tree_size(crit);
    merkle_tree_threads(crit);
    crit.final_summary();
}
