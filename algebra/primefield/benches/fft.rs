#![warn(clippy::all)]
use criterion::{black_box, Criterion, Throughput};
use rand::prelude::*;
use std::iter::repeat_with;
use zkp_criterion_utils::{log_size_bench, log_thread_bench};
use zkp_primefield::{
    fft::{
        fft_vec_recursive, get_twiddles,
        small::{radix_2, radix_2_twiddle, radix_4, radix_8},
    },
    Fft, FieldElement, Root,
};

const SMALL: [usize; 9] = [4, 16, 64, 256, 1024, 4_096, 16_384, 65_536, 262_144];

#[cfg(not(test))]
const LARGE: [usize; 4] = [1_048_576, 4_194_304, 8_388_608, 16_777_216];

#[cfg(test)]
const LARGE: [usize; 1] = [1_048_576];

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
    let root = FieldElement::root(4).unwrap();
    let twiddles = get_twiddles(&root, 4);
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
    let root = FieldElement::root(8).unwrap();
    let twiddles = get_twiddles(&root, 8);
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

fn fft_rec_small(crit: &mut Criterion) {
    // Exclude from unit tests (single iter is long)
    if cfg!(test) {
        return;
    }
    log_size_bench(crit, "FFT rec size", &SMALL, move |bench, size| {
        let root = FieldElement::root(size).unwrap();
        let twiddles = get_twiddles(&root, size);
        let mut values: Vec<_> = (0..size).map(FieldElement::from).collect();
        bench.iter(|| fft_vec_recursive(&mut values, &twiddles, 0, 1, 1))
    });
}

fn fft_large(crit: &mut Criterion) {
    // Exclude from unit tests (single iter is long)
    if cfg!(test) {
        return;
    }
    log_size_bench(
        crit,
        "FFT cache-oblivious size",
        &LARGE,
        move |bench, size| {
            let mut values: Vec<_> = (0..size).map(FieldElement::from).collect();
            bench.iter(|| values.fft())
        },
    );
}

fn fft_threads(crit: &mut Criterion) {
    let size = if cfg!(test) { 1_048_576 } else { 4_194_304 };
    log_thread_bench(crit, "FFT threads", size, move |bench| {
        let mut values: Vec<_> = (0..size).map(FieldElement::from).collect();
        bench.iter(|| values.fft())
    });
}

pub fn group(crit: &mut Criterion) {
    fft_base(crit);
    fft_rec_small(crit);
    fft_large(crit);
    fft_threads(crit);
}
