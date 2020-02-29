#![warn(clippy::all)]
use criterion::{black_box, criterion_group, Criterion};
use zkp_criterion_utils::{log_size_bench, log_thread_bench};
use zkp_macros_decl::field_element;
use zkp_primefield::{
    fft,
    fft::{fft_cofactor_permuted, fft_recurse},
    FieldElement,
};
use zkp_u256::U256;

const SIZES: [usize; 8] = [64, 256, 1024, 4096, 16384, 65536, 262144, 1048576];

fn fft_butterfly_radix_2_simple(crit: &mut Criterion) {
    let mut a = FieldElement::from(123);
    let mut b = FieldElement::from(3432);
    crit.bench_function("FFT butterfly/radix 2 simple", move |bench| {
        bench.iter(|| {
            fft::radix_2_simple(&mut a, &mut b);
        })
    });
}

fn fft_butterfly_radix_2(crit: &mut Criterion) {
    let mut values: Vec<_> = (0..2).map(FieldElement::from).collect();
    crit.bench_function("FFT butterfly/radix 2", move |bench| {
        bench.iter(|| {
            fft::radix_2(0, 1, black_box(&mut values));
        })
    });
}

fn fft_butterfly_radix_4(crit: &mut Criterion) {
    let mut values: Vec<_> = (0..4).map(FieldElement::from).collect();
    crit.bench_function("FFT butterfly/radix 4", move |bench| {
        bench.iter(|| {
            fft::radix_4(0, 1, black_box(&mut values));
        })
    });
}

fn fft_butterfly_radix_8(crit: &mut Criterion) {
    let mut values: Vec<_> = (0..8).map(FieldElement::from).collect();
    crit.bench_function("FFT butterfly/radix 8", move |bench| {
        bench.iter(|| {
            fft::radix_8(0, 1, black_box(&mut values));
        })
    });
}

fn fft_rec_size(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "FFT cache-oblivious size",
        &SIZES,
        move |bench, size| {
            let mut values: Vec<_> = (0..size).map(FieldElement::from).collect();
            bench.iter(|| fft_recurse(&mut values))
        },
    );
}

fn fft_size(crit: &mut Criterion) {
    log_size_bench(crit, "FFT size", &SIZES, move |bench, size| {
        let cofactor =
            field_element!("0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f");
        let leaves: Vec<_> = (0..size).map(FieldElement::from).collect();
        let mut copy = leaves.clone();
        bench.iter(|| {
            copy.clone_from_slice(&leaves);
            fft_cofactor_permuted(black_box(&cofactor), black_box(&mut copy))
        })
    });
}

fn fft_threads(crit: &mut Criterion) {
    let size: usize = *SIZES.last().unwrap();
    log_thread_bench(crit, "FFT threads", size, move |bench| {
        let cofactor =
            field_element!("0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f");
        let leaves: Vec<_> = (0..size).map(FieldElement::from).collect();
        let mut copy = leaves.clone();
        bench.iter(|| {
            copy.clone_from_slice(&leaves);
            fft_cofactor_permuted(black_box(&cofactor), black_box(&mut copy))
        })
    });
}

criterion_group!(
    group,
    // fft_butterfly_radix_2_simple,
    // fft_butterfly_radix_2,
    // fft_butterfly_radix_4,
    // fft_butterfly_radix_8,
    fft_size,
    fft_rec_size,
    // fft_threads
);
