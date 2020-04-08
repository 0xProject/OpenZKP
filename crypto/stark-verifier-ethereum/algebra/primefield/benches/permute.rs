#![warn(clippy::all)]
use criterion::{black_box, Criterion};
use rand::prelude::*;
use std::iter::repeat_with;
use zkp_criterion_utils::log_size_bench;
use zkp_primefield::{
    fft::{permute, permute_index},
    FieldElement,
};

#[cfg(not(test))]
const SIZES: [usize; 4] = [16_384, 262_144, 4_194_304, 16_777_216];

#[cfg(test)]
const SIZES: [usize; 1] = [16_384];

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

pub fn group(crit: &mut Criterion) {
    bench_permute_index(crit);
    bench_permute(crit);
}
