#![warn(clippy::all)]
use criterion::Criterion;
use zkp_criterion_utils::log_size_bench;
use zkp_primefield::{fft::transpose_square_stretch, FieldElement};

#[cfg(not(test))]
const SIZES: [usize; 4] = [16_384, 262_144, 4_194_304, 16_777_216];

#[cfg(not(test))]
const N2N: [usize; 4] = [131_072, 524_288, 2_097_152, 8_388_608];

#[cfg(test)]
const SIZES: [usize; 1] = [16_384];

#[cfg(test)]
const N2N: [usize; 1] = [131_072];

fn bench_square_1(crit: &mut Criterion) {
    log_size_bench(crit, "Transpose n x n size", &SIZES, move |bench, size| {
        let log2 = size.trailing_zeros() as usize;
        let rows = 1_usize << (log2 / 2);
        let stretch = 1;
        assert_eq!(rows * rows * stretch, size);
        let mut matrix: Vec<_> = (0..size).map(FieldElement::from).collect();
        bench.iter(|| transpose_square_stretch(&mut matrix, rows, stretch))
    });
}

fn bench_square_2(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose n x n x 2 size",
        &N2N,
        move |bench, size| {
            let log2 = size.trailing_zeros() as usize;
            let rows = 1_usize << (log2 / 2);
            let stretch = 2;
            assert_eq!(rows * rows * stretch, size);
            let mut matrix: Vec<_> = (0..size).map(FieldElement::from).collect();
            bench.iter(|| transpose_square_stretch(&mut matrix, rows, stretch))
        },
    );
}

pub fn group(crit: &mut Criterion) {
    bench_square_1(crit);
    bench_square_2(crit);
}
