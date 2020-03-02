#![warn(clippy::all)]
use criterion::{black_box, criterion_group, Criterion};
use zkp_criterion_utils::{log_size_bench, log_thread_bench};
use zkp_macros_decl::field_element;
use zkp_primefield::{
    fft,
    fft::{fft2_inplace, fft_permuted_root},
    FieldElement, Root,
};
use zkp_u256::U256;

const SMALL: [usize; 7] = [4, 16, 64, 256, 1024, 4096, 16384];

const LARGE: [usize; 4] = [1048576, 4194304, 8388608, 16777216];

fn fft_small(crit: &mut Criterion) {
    log_size_bench(crit, "FFT size", &SMALL, move |bench, size| {
        let root = FieldElement::root(size).expect("No root of unity for input length");
        let mut values: Vec<_> = (0..size).map(FieldElement::from).collect();
        bench.iter(|| fft_permuted_root(&root, &mut values))
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
    const SIZE: usize = 8388608;
    log_thread_bench(crit, "FFT threads", SIZE, move |bench| {
        let mut values: Vec<_> = (0..SIZE).map(FieldElement::from).collect();
        bench.iter(|| fft2_inplace(&mut values))
    });
}

criterion_group!(group, fft_small, fft_large, fft_threads);
