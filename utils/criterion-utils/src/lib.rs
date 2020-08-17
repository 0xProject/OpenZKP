// This sequence needs to be repeated in each project as a workaround.
//       See https://github.com/rust-lang/cargo/issues/5034
// For clippy lints see: https://rust-lang.github.io/rust-clippy/master
// For rustc lints see: https://doc.rust-lang.org/rustc/lints/index.html
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(
    // Enable sets of warnings
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    rust_2018_idioms,
    future_incompatible,
    unused,

    // Additional unused warnings (not included in `unused`)
    unused_lifetimes,
    unused_qualifications,
    unused_results,

    // Additional misc. warnings
    anonymous_parameters,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    // missing_docs,
    missing_doc_code_examples,
    private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    variant_size_differences
)]
#![cfg_attr(feature = "std", warn(missing_debug_implementations,))]
// rand_xoshiro v0.4.0 is required for a zkp-stark example and v0.3.1 for criterion
#![allow(clippy::multiple_crate_versions)]

use criterion::{
    AxisScale, Bencher, Criterion, ParameterizedBenchmark, PlotConfiguration, Throughput,
};
use lazy_static::lazy_static;
use rayon::ThreadPoolBuilder;
use std::convert::TryInto;

lazy_static! {
    // Create an exponential number of threads up to the number of cpus.
    static ref THREADS: Vec<usize> = (0..=num_cpus::get().trailing_zeros())
        .map(|log| 1_usize << log)
        .collect();
}

/// Benchmark over a size parameter.
///
/// Produces a log-log plot.
///
/// ```ignore
/// const SIZES: [usize; 6] = [64, 256, 1024, 4096, 16384, 65536];
///
/// log_size_bench(crit, "FFT size", &SIZES, move |bench, size| {
///     let leaves: Vec<_> = (0..size).map(FieldElement::from).collect();
///     bench.iter(|| {
///         black_box(fft_bit_reversed(
///             black_box(&leaves),
///         ))
///     })
/// });
/// ```
pub fn log_size_bench<F>(crit: &mut Criterion, id: &str, sizes: &'static [usize], mut f: F)
where
    F: FnMut(&mut Bencher<'_>, usize) + 'static,
{
    let _ = crit.bench(
        id,
        ParameterizedBenchmark::new(id, move |bench, &&size| f(bench, size), sizes)
            .sample_size(10)
            .throughput(|&&s| Throughput::Elements(s.try_into().unwrap()))
            .plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic)),
    );
}

/// Benchmark over the number of threads.
///
/// Produces a log-log plot.
///
/// The `size` argument is for througput computations.
pub fn log_thread_bench<F>(crit: &mut Criterion, id: &str, size: usize, mut f: F)
where
    F: FnMut(&mut Bencher<'_>) + 'static + Send,
{
    let _ = crit.bench(
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
