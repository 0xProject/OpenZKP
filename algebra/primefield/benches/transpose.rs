#![warn(clippy::all)]
use criterion::{black_box, criterion_group, Criterion};
use zkp_criterion_utils::{log_size_bench, log_thread_bench};
use zkp_macros_decl::field_element;
use zkp_primefield::{
    fft,
    fft::{fft2_permuted, fft_cofactor_permuted},
    transpose::{reference, transpose},
    FieldElement,
};
use zkp_u256::U256;

const SIZES: [usize; 6] = [64, 1024, 16384, 262144, 4194304, 16777216];

fn bench_size(crit: &mut Criterion) {
    log_size_bench(crit, "Transpose square size", &SIZES, move |bench, size| {
        let log2 = size.trailing_zeros() as usize;
        assert_eq!(log2 % 2, 0);
        let rows = 1_usize << (log2 / 2);
        let cols = 1_usize << (log2 / 2);
        let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
        let mut dst = src.clone();
        bench.iter(|| transpose(&src, &mut dst, rows))
    });
}

fn bench_size_ref(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose reference square size",
        &SIZES,
        move |bench, size| {
            let log2 = size.trailing_zeros() as usize;
            assert_eq!(log2 % 2, 0);
            let rows = 1_usize << (log2 / 2);
            let cols = 1_usize << (log2 / 2);
            let src: Vec<_> = (0..size).map(FieldElement::from).collect();
            let mut dst = src.clone();
            bench.iter(|| reference(&src, &mut dst, rows))
        },
    );
}

fn bench_strip_size(crit: &mut Criterion) {
    log_size_bench(crit, "Transpose strip size", &SIZES, move |bench, size| {
        let rows = size / 8;
        let cols = 8;
        let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
        let mut dst = src.clone();
        bench.iter(|| transpose(&src, &mut dst, rows))
    });
}

fn bench_strip_size_ref(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose reference strip size",
        &SIZES,
        move |bench, size| {
            let rows = size / 8;
            let cols = 8;
            let src: Vec<_> = (0..size).map(FieldElement::from).collect();
            let mut dst = src.clone();
            bench.iter(|| reference(&src, &mut dst, rows))
        },
    );
}
criterion_group!(
    group,
    bench_size,
    bench_size_ref,
    bench_strip_size,
    bench_strip_size_ref
);
