#![warn(clippy::all)]
use criterion::{black_box, criterion_group, Criterion};
use rand::prelude::*;
use std::iter::repeat_with;
use zkp_criterion_utils::log_size_bench;
use zkp_primefield::{
    fft::{permute, permute_index},
    FieldElement,
};

const SIZES: [usize; 4] = [16384, 262144, 4194304, 16777216];

fn bench_permute_index(crit: &mut Criterion) {
    crit.bench_function("Permute index", move |bench| {
        let size = 1 << (random::<usize>() % 32);
        let index = random::<usize>() % size;
        bench.iter(|| black_box(permute_index(black_box(size), black_box(index))))
    });
}

fn bench_permute(crit: &mut Criterion) {
    log_size_bench(crit, "Permute n", &SIZES, move |bench, size| {
        let mut values: Vec<FieldElement> = repeat_with(random).take(size).collect();
        bench.iter(|| permute(black_box(&mut values)))
    });
}

criterion_group!(group, bench_permute_index, bench_permute);
