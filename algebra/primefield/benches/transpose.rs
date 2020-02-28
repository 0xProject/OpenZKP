#![warn(clippy::all)]
use criterion::{black_box, criterion_group, Criterion};
use zkp_criterion_utils::{log_size_bench, log_thread_bench};
use zkp_macros_decl::field_element;
use zkp_primefield::{
    fft,
    fft::{fft2_permuted, fft_cofactor_permuted},
    transpose::transpose,
    FieldElement,
};
use zkp_u256::U256;

const SIZES: [usize; 6] = [64, 256, 1024, 4096, 16384, 65536, 262144, 1048576];

fn bench_size(crit: &mut Criterion) {
    log_size_bench(crit, "Transpose square size", &SIZES, move |bench, size| {
        let row_size = size / 2;
        let src: Vec<_> = (0..size).map(FieldElement::from).collect();
        let mut dst = src.clone();
        bench.iter(|| transpose(&src, &mut dst, row_size))
    });
}

criterion_group!(group, bench_size);
