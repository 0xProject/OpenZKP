#![warn(clippy::all)]
use criterion::{criterion_group, Criterion};
use zkp_criterion_utils::{log_size_bench, log_thread_bench};
use zkp_primefield::{
    fft::{fft2_inplace, fft_depth_first, fft_permuted_root},
    FieldElement, Root,
};

const SMALL: [usize; 7] = [4, 16, 64, 256, 1024, 4096, 16384];
const LARGE: [usize; 4] = [1048576, 4194304, 8388608, 16777216];

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

criterion_group!(group, fft_small, fft_df_small, fft_large, fft_threads);
