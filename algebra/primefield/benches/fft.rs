#![warn(clippy::all)]
use criterion::{black_box, criterion_group, Criterion, Throughput};
use rand::prelude::*;
use std::iter::repeat_with;
use zkp_criterion_utils::{log_size_bench, log_thread_bench};
use zkp_primefield::{
    fft::{
        fft2_inplace, fft_depth_first, fft_permuted_root, get_twiddles,
        small::{radix_2, radix_2_twiddle, radix_4, radix_8},
    },
    FieldElement, Root,
};

const SMALL: [usize; 7] = [4, 16, 64, 256, 1024, 4096, 16384];
const LARGE: [usize; 4] = [1048576, 4194304, 8388608, 16777216];

fn fft_base(crit: &mut Criterion) {
    let mut group = crit.benchmark_group("FFT base");
    // radix_2
    group.throughput(Throughput::Elements(2));
    group.bench_function("2", move |bench| {
        let mut values: Vec<FieldElement> = repeat_with(random).take(2).collect();
        bench.iter(|| radix_2(black_box(&mut values), black_box(0), black_box(1)))
    });
    // radix_2_twiddle
    group.throughput(Throughput::Elements(2));
    group.bench_function("2 (twiddle)", move |bench| {
        let twiddle = random();
        let mut values: Vec<FieldElement> = repeat_with(random).take(2).collect();
        bench.iter(|| {
            radix_2_twiddle(
                black_box(&mut values),
                black_box(&twiddle),
                black_box(0),
                black_box(1),
            )
        })
    });
    // radix_4
    let twiddles = get_twiddles(4);
    group.throughput(Throughput::Elements(4));
    group.bench_function("4", move |bench| {
        let mut values: Vec<FieldElement> = repeat_with(random).take(4).collect();
        bench.iter(|| {
            radix_4(
                black_box(&mut values),
                black_box(&twiddles),
                black_box(0),
                black_box(1),
            )
        })
    });
    // radix_8
    let twiddles = get_twiddles(8);
    group.throughput(Throughput::Elements(8));
    group.bench_function("8", move |bench| {
        let mut values: Vec<FieldElement> = repeat_with(random).take(8).collect();
        bench.iter(|| {
            radix_8(
                black_box(&mut values),
                black_box(&twiddles),
                black_box(0),
                black_box(1),
            )
        })
    });
}

fn fft_small(crit: &mut Criterion) {
    log_size_bench(crit, "FFT size", &SMALL, move |bench, size| {
        let root = FieldElement::root(size).expect("No root of unity for input length");
        let mut values: Vec<_> = (0..size).map(FieldElement::from).collect();
        bench.iter(|| fft_permuted_root(&root, &mut values))
    });
}

fn fft_df_small(crit: &mut Criterion) {
    log_size_bench(crit, "FFT DF size", &SMALL, move |bench, size| {
        let mut values: Vec<_> = (0..size).map(FieldElement::from).collect();
        bench.iter(|| fft_depth_first(&mut values))
    });
}

fn fft_large(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "FFT cache-oblivious size",
        &LARGE,
        move |bench, size| {
            let mut values: Vec<_> = (0..size).map(FieldElement::from).collect();
            bench.iter(|| fft2_inplace(&mut values))
        },
    );
}

fn fft_threads(crit: &mut Criterion) {
    const SIZE: usize = 4194304;
    log_thread_bench(crit, "FFT threads", SIZE, move |bench| {
        let mut values: Vec<_> = (0..SIZE).map(FieldElement::from).collect();
        bench.iter(|| fft2_inplace(&mut values))
    });
}

criterion_group!(
    group,
    fft_base,
    fft_small,
    fft_df_small,
    fft_large,
    fft_threads
);
