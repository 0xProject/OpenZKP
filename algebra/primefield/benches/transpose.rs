#![warn(clippy::all)]
use criterion::{criterion_group, Criterion};
use zkp_criterion_utils::log_size_bench;
use zkp_primefield::{
    fft::{transpose, transpose_inplace},
    FieldElement,
};

const SIZES: [usize; 4] = [16_384, 262_144, 4_194_304, 16_777_216];
const N2N: [usize; 4] = [131_072, 524_288, 2_097_152, 8_388_608];

fn bench_square(crit: &mut Criterion) {
    log_size_bench(crit, "Transpose n x n size", &SIZES, move |bench, size| {
        let log2 = size.trailing_zeros() as usize;
        let rows = 1_usize << (log2 / 2);
        let cols = 1_usize << (log2 / 2);
        assert_eq!(rows * cols, size);
        let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
        let mut dst = src.clone();
        bench.iter(|| transpose(&src, &mut dst, cols))
    });
}

fn bench_strip(crit: &mut Criterion) {
    log_size_bench(crit, "Transpose n x 8 size", &SIZES, move |bench, size| {
        let rows = size / 8;
        let cols = 8;
        assert_eq!(rows * cols, size);
        let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
        let mut dst = src.clone();
        bench.iter(|| transpose(&src, &mut dst, cols))
    });
}

fn bench_square_inplace(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose in-place n x n size",
        &SIZES,
        move |bench, size| {
            let log2 = size.trailing_zeros() as usize;
            let rows = 1_usize << (log2 / 2);
            let cols = 1_usize << (log2 / 2);
            assert_eq!(rows * cols, size);
            let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
            let mut dst = src;
            bench.iter(|| transpose_inplace(&mut dst, cols))
        },
    );
}

fn bench_rectangle_inplace(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose in-place n x 2n size",
        &N2N,
        move |bench, size| {
            let log2 = size.trailing_zeros() as usize;
            let rows = 1_usize << (log2 / 2);
            let cols = 2_usize << (log2 / 2);
            assert_eq!(rows * cols, size);
            let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
            let mut dst = src;
            bench.iter(|| transpose_inplace(&mut dst, cols))
        },
    );
}

fn bench_rectangle2_inplace(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose in-place 2n x n size",
        &N2N,
        move |bench, size| {
            let log2 = size.trailing_zeros() as usize;
            let rows = 2_usize << (log2 / 2);
            let cols = 1_usize << (log2 / 2);
            assert_eq!(rows * cols, size);
            let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
            let mut dst = src;
            bench.iter(|| transpose_inplace(&mut dst, cols))
        },
    );
}

fn bench_strip_inplace(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose in-place n x 8 size",
        &N2N,
        move |bench, size| {
            let rows = size / 8;
            let cols = 8;
            assert_eq!(rows * cols, size);
            let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
            let mut dst = src;
            bench.iter(|| transpose_inplace(&mut dst, cols))
        },
    );
}

fn bench_strip2_inplace(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose in-place 8 x n size",
        &N2N,
        move |bench, size| {
            let rows = 8;
            let cols = size / 8;
            assert_eq!(rows * cols, size);
            let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
            let mut dst = src;
            bench.iter(|| transpose_inplace(&mut dst, cols))
        },
    );
}

criterion_group!(
    group,
    bench_square,
    bench_strip,
    bench_square_inplace,
    bench_rectangle_inplace,
    bench_rectangle2_inplace,
    bench_strip_inplace,
    bench_strip2_inplace,
);
