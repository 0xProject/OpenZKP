#![warn(clippy::all)]
use criterion::{black_box, criterion_group, Criterion};
use zkp_criterion_utils::{log_size_bench, log_thread_bench};
use zkp_macros_decl::field_element;
use zkp_primefield::{
    transpose::{reference, transpose, transpose_inplace},
    FieldElement,
};
use zkp_u256::U256;

const SIZES: [usize; 6] = [64, 1024, 16384, 262144, 4194304, 16777216];

fn bench_square(crit: &mut Criterion) {
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

fn bench_strip(crit: &mut Criterion) {
    log_size_bench(crit, "Transpose strip size", &SIZES, move |bench, size| {
        let rows = size / 8;
        let cols = 8;
        let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
        let mut dst = src.clone();
        bench.iter(|| transpose(&src, &mut dst, rows))
    });
}

fn bench_square_inplace(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose square in-place size",
        &SIZES,
        move |bench, size| {
            let log2 = size.trailing_zeros() as usize;
            assert_eq!(log2 % 2, 0);
            let rows = 1_usize << (log2 / 2);
            let cols = 1_usize << (log2 / 2);
            let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
            let mut dst = src.clone();
            bench.iter(|| transpose_inplace(&mut dst, rows))
        },
    );
}

fn bench_reference_square(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose reference square size",
        &SIZES,
        move |bench, size| {
            let log2 = size.trailing_zeros() as usize;
            assert_eq!(log2 % 2, 0);
            let rows = 1_usize << (log2 / 2);
            let cols = 1_usize << (log2 / 2);
            let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
            let mut dst = src.clone();
            bench.iter(|| reference(&src, &mut dst, rows))
        },
    );
}

fn bench_reference_strip(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose reference strip size",
        &SIZES,
        move |bench, size| {
            let rows = size / 8;
            let cols = 8;
            let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
            let mut dst = src.clone();
            bench.iter(|| reference(&src, &mut dst, rows))
        },
    );
}
criterion_group!(
    group,
    bench_square,
    bench_strip,
    bench_square_inplace,
    /* bench_reference_square,
     * bench_reference_strip, */
);
