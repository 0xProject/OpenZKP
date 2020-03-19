#![warn(clippy::all)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zkp_criterion_utils::{log_size_bench, log_thread_bench};
use zkp_merkle_tree::Tree;
use zkp_u256::U256;

const SIZES: [usize; 6] = [64, 256, 1024, 4096, 16384, 65536];

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

fn criterion_benchmark(c: &mut Criterion) {
    merkle_tree_size(c);
    merkle_tree_threads(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
