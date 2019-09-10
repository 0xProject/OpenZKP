use criterion::{
    AxisScale, Bencher, Criterion, ParameterizedBenchmark, PlotConfiguration, Throughput,
};
use lazy_static::lazy_static;
use num_cpus;
use rayon::ThreadPoolBuilder;
use std::convert::TryInto;

lazy_static! {
    // Create an exponential number of threads up to the number of cpus.
    static ref THREADS: Vec<usize> = (0..=num_cpus::get().trailing_zeros())
        .map(|log| 1usize << log)
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

/// Benchmark over the number of threads.
///
/// Produces a log-log plot.
///
/// The `size` argument is for througput computations.
pub fn log_thread_bench<F>(crit: &mut Criterion, id: &str, size: usize, mut f: F)
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
