#![warn(clippy::all)]
#![deny(warnings)]

use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, Bencher, Criterion,
    ParameterizedBenchmark, PlotConfiguration, Throughput,
};
use hex_literal::*;
use lazy_static::lazy_static;
use primefield::FieldElement;
use rayon::ThreadPoolBuilder;
use stark::{fft_cofactor, get_constraint, get_trace_table, make_tree, stark_proof, ProofParams};
use std::{convert::TryInto, marker::Send};
use u256::{u256h, U256};

const SIZES: [usize; 6] = [64, 256, 1024, 4096, 16384, 65536];
lazy_static! {
    // Create an exponential number of threads up to the number of cpus.
    static ref THREADS: Vec<usize> = (0..=num_cpus::get().trailing_zeros())
        .map(|log| 1usize << log)
        .collect();
}

/// Utility function for log-log benchmark plots over a size parameter.
fn log_size_bench<F>(crit: &mut Criterion, id: &str, sizes: &'static [usize], mut f: F)
where
    F: FnMut(&mut Bencher, usize) + 'static,
{
    crit.bench(
        id,
        ParameterizedBenchmark::new(id, move |bench, &&size| f(bench, size), sizes)
            .sample_size(10)
            .throughput(|&&s| Throughput::Elements(s.try_into().unwrap()))
            .plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic)),
    );
}

/// Utility function for log-log benchmark plots over the number of threads in
/// the thread-pool.
fn log_thread_bench<F>(crit: &mut Criterion, id: &str, size: usize, mut f: F)
where
    F: FnMut(&mut Bencher) + 'static + Send,
{
    crit.bench(
        id,
        ParameterizedBenchmark::new(
            id,
            move |bench, &&num_threads| {
                let pool = ThreadPoolBuilder::new()
                    .num_threads(num_threads)
                    .build()
                    .expect("Building benchmark thread pool failed.");
                pool.install(|| f(bench))
            },
            THREADS.iter(),
        )
        .sample_size(10)
        .throughput(move |_| Throughput::Elements(size.try_into().unwrap()))
        .plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic)),
    );
}

fn merkle_tree_size(crit: &mut Criterion) {
    log_size_bench(crit, "Merkle tree size", &SIZES, move |bench, size| {
        let leaves: Vec<_> = (0..size).map(|i| U256::from(i as u64)).collect();
        bench.iter(|| black_box(make_tree(black_box(&leaves))))
    });
}

fn merkle_tree_threads(crit: &mut Criterion) {
    let size: usize = *SIZES.last().unwrap();
    log_thread_bench(crit, "Merkle tree threads", size, move |bench| {
        let leaves: Vec<_> = (0..size).map(|i| U256::from(i as u64)).collect();
        bench.iter(|| black_box(make_tree(black_box(&leaves))))
    });
}

fn fft_size(crit: &mut Criterion) {
    log_size_bench(crit, "FFT size", &SIZES, move |bench, size| {
        let cofactor = FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        ));
        let leaves: Vec<_> = (0..size)
            .map(|i| FieldElement::from(U256::from(i as u64)))
            .collect();
        bench.iter(|| black_box(fft_cofactor(black_box(&leaves), black_box(&cofactor))))
    });
}

fn fft_threads(crit: &mut Criterion) {
    let size: usize = *SIZES.last().unwrap();
    log_thread_bench(crit, "FFT threads", size, move |bench| {
        let cofactor = FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        ));
        let leaves: Vec<_> = (0..size)
            .map(|i| FieldElement::from(U256::from(i as u64)))
            .collect();
        bench.iter(|| black_box(fft_cofactor(black_box(&leaves), black_box(&cofactor))))
    });
}

fn abstracted_fib_proof_make(crit: &mut Criterion) {
    let claim_index = 1000_usize;
    let claim_fib = FieldElement::from(u256h!(
        "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
    ));
    let witness = FieldElement::from(u256h!(
        "00000000000000000000000000000000000000000000000000000000cafebabe"
    ));

    crit.bench_function("Making an abstracted Fibonacci proof", move |bench| {
        bench.iter(|| {
            black_box(stark_proof(
                &get_trace_table(1024, witness.clone()),
                &get_constraint(),
                claim_index,
                claim_fib.clone(),
                &ProofParams {
                    blowup:     16,
                    pow_bits:   12,
                    queries:    20,
                    fri_layout: vec![3, 2, 1],
                },
            ))
        })
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    merkle_tree_size(c);
    merkle_tree_threads(c);
    fft_size(c);
    fft_threads(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_group! {
   name = slow_benches;
   config = Criterion::default().sample_size(20);
   targets = abstracted_fib_proof_make
}
criterion_main!(benches, slow_benches);
