#![warn(clippy::all)]
use criterion::{black_box, criterion_group, Criterion};
use criterion_utils::{log_size_bench, log_thread_bench};
use macros_decl::field_element;
use primefield::{fft::fft_cofactor_bit_reversed, FieldElement};
use u256::U256;

const SIZES: [usize; 6] = [64, 256, 1024, 4096, 16384, 65536];

fn fft_size(crit: &mut Criterion) {
    log_size_bench(crit, "FFT size", &SIZES, move |bench, size| {
        let cofactor =
            field_element!("0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f");
        let leaves: Vec<_> = (0..size).map(FieldElement::from).collect();
        bench.iter(|| {
            black_box(fft_cofactor_bit_reversed(
                black_box(&leaves),
                black_box(&cofactor),
            ))
        })
    });
}

fn fft_threads(crit: &mut Criterion) {
    let size: usize = *SIZES.last().unwrap();
    log_thread_bench(crit, "FFT threads", size, move |bench| {
        let cofactor =
            field_element!("0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f");
        let leaves: Vec<_> = (0..size).map(FieldElement::from).collect();
        bench.iter(|| {
            black_box(fft_cofactor_bit_reversed(
                black_box(&leaves),
                black_box(&cofactor),
            ))
        })
    });
}

criterion_group!(fft, fft_size, fft_threads);
