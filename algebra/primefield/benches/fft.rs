#![warn(clippy::all)]
use criterion::{black_box, criterion_group, Criterion};
use criterion_utils::{log_size_bench, log_thread_bench};
use macros_decl::field_element;
use primefield::{fft::fft_cofactor_permuted,fft::fft2_permuted,  FieldElement};
use u256::U256;

const SIZES: [usize; 6] = [64, 256, 1024, 4096, 16384, 65536];

fn fft_size(crit: &mut Criterion) {
    log_size_bench(crit, "FFT size", &SIZES, move |bench, size| {
        let cofactor =
            field_element!("0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f");
        let leaves: Vec<_> = (0..size).map(FieldElement::from).collect();
        let mut copy = leaves.clone();
        bench.iter(|| {
            copy.clone_from_slice(&leaves);
//            fft_cofactor_permuted(black_box(&cofactor), black_box(&mut copy))
            fft2_permuted(black_box(&mut copy))
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

criterion_group!(fft, fft_size, fft_threads);
