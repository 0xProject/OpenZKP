#![warn(clippy::all)]
use criterion::{black_box, criterion_group, Criterion};
use zkp_criterion_utils::{log_size_bench, log_thread_bench};
use zkp_macros_decl::field_element;
use zkp_primefield::{
    transpose::{reference, transpose, transpose_inplace},
    FieldElement,
};
use zkp_u256::U256;

const SIZES: [usize; 4] = [16384, 262144, 4194304, 16777216];
const N2N: [usize; 4] = [131072, 524288, 2097152, 8388608];

fn bench_square(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose n ⨉ n size",
        &SIZES,
        move |bench, size| {
            let log2 = size.trailing_zeros() as usize;
            let rows = 1_usize << (log2 / 2);
            let cols = 1_usize << (log2 / 2);
            assert_eq!(rows * cols, size);
            let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
            let mut dst = src.clone();
            bench.iter(|| transpose(&src, &mut dst, cols))
        },
    );
}

fn bench_strip(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose n ⨉ 8 size",
        &SIZES,
        move |bench, size| {
            let rows = size / 8;
            let cols = 8;
            assert_eq!(rows * cols, size);
            let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
            let mut dst = src.clone();
            bench.iter(|| transpose(&src, &mut dst, cols))
        },
    );
}

fn bench_square_inplace(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose in-place n ⨉ n size",
        &SIZES,
        move |bench, size| {
            let log2 = size.trailing_zeros() as usize;
            let rows = 1_usize << (log2 / 2);
            let cols = 1_usize << (log2 / 2);
            assert_eq!(rows * cols, size);
            let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
            let mut dst = src.clone();
            bench.iter(|| transpose_inplace(&mut dst, cols))
        },
    );
}

fn bench_rectangle_inplace(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose in-place n ⨉ 2n size",
        &N2N,
        move |bench, size| {
            let log2 = size.trailing_zeros() as usize;
            let rows = 1_usize << (log2 / 2);
            let cols = 2_usize << (log2 / 2);
            assert_eq!(rows * cols, size);
            let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
            let mut dst = src.clone();
            bench.iter(|| transpose_inplace(&mut dst, cols))
        },
    );
}

fn bench_rectangle2_inplace(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose in-place 2n ⨉ n size",
        &N2N,
        move |bench, size| {
            let log2 = size.trailing_zeros() as usize;
            let rows = 2_usize << (log2 / 2);
            let cols = 1_usize << (log2 / 2);
            assert_eq!(rows * cols, size);
            let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
            let mut dst = src.clone();
            bench.iter(|| transpose_inplace(&mut dst, cols))
        },
    );
}

fn bench_strip_inplace(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose in-place n ⨉ 8 size",
        &N2N,
        move |bench, size| {
            let rows = size / 8;
            let cols = 8;
            assert_eq!(rows * cols, size);
            let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
            let mut dst = src.clone();
            bench.iter(|| transpose_inplace(&mut dst, cols))
        },
    );
}

fn bench_strip2_inplace(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose in-place 8 ⨉ n size",
        &N2N,
        move |bench, size| {
            let rows = 8;
            let cols = size / 8;
            assert_eq!(rows * cols, size);
            let src: Vec<_> = (0..rows * cols).map(FieldElement::from).collect();
            let mut dst = src.clone();
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
