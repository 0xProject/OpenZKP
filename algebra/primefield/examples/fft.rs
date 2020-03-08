use env_logger;
use log::{info, warn};
use num_cpus;
use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::{current_num_threads, prelude::*, ThreadPoolBuilder};
use std::{
    iter::repeat_with,
    num::ParseIntError,
    time::{Duration, Instant},
};
use structopt::StructOpt;
use zkp_logging_allocator::ALLOCATOR;
use zkp_mmap_vec::MmapVec;
use zkp_primefield::{fft::fft2_inplace, FieldElement};

fn parse_hex(src: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(src, 16)
}

#[derive(Debug, StructOpt)]
struct Options {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: usize,

    /// Number of threads to use (defaults to number of cores)
    #[structopt(long)]
    threads: Option<usize>,

    /// Random seed (defaults to fresh entropy)
    #[structopt(long, parse(try_from_str = parse_hex))]
    seed: Option<u32>,

    /// Use heap allocations instead of memory-mapped files
    // TODO: Xor with --mmap
    #[structopt(long)]
    heap: bool,

    /// Use memory-mapped files instead of heap allocations (default)
    #[structopt(long)]
    mmap: bool,

    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Run a benchmark of a given size
    Size {
        #[structopt()]
        log_size: usize,
    },
    /// Run benchmarks of ever larger proof sizes
    Benchmark {},
}

#[derive(Debug)]
enum Error {
    Io(std::io::Error),
    ThreadPoolBuild(rayon::ThreadPoolBuildError),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<rayon::ThreadPoolBuildError> for Error {
    fn from(err: rayon::ThreadPoolBuildError) -> Self {
        Self::ThreadPoolBuild(err)
    }
}

enum Allocation {
    Heap(Vec<FieldElement>),
    Mmap(MmapVec<FieldElement>),
}

impl Allocation {
    fn random<R: Rng + ?Sized>(rng: &mut R, mmap: bool, size: usize) -> Allocation {
        if mmap {
            info!("Memore mapping size {} ", size);
            let mut vec = MmapVec::<FieldElement>::with_capacity(size);
            info!("Filling with random numbers");
            vec.extend(repeat_with(|| rng.gen::<FieldElement>()).take(size));
            assert_eq!(vec.len(), size);
            Allocation::Mmap(vec)
        } else {
            info!("Allocating size {} on heap", size);
            let mut vec = Vec::<FieldElement>::with_capacity(size);
            info!("Filling with random numbers");
            vec.extend(repeat_with(|| rng.gen::<FieldElement>()).take(size));
            assert_eq!(vec.len(), size);
            Allocation::Heap(vec)
        }
    }

    fn as_mut_slice(&mut self) -> &mut [FieldElement] {
        match self {
            Allocation::Heap(vec) => vec,
            Allocation::Mmap(vec) => vec,
        }
    }
}

fn bench_fft<R: Rng + ?Sized>(rng: &mut R, mmap: bool, log_size: usize) -> Result<Duration, Error> {
    let size = 1 << log_size;
    info!("Benchmarking FFT of size 2^{} = {}", log_size, size);

    let mut allocation = Allocation::random(rng, mmap, size);
    info!("FFT transforming");
    let start = Instant::now();
    fft2_inplace(allocation.as_mut_slice());
    let duration = start.elapsed();
    warn!("Total time {:?}", duration);

    Ok(duration)
}

fn main() -> Result<(), Error> {
    use Command::*;
    let options = Options::from_args();

    // Initialize log output
    if options.verbose > 0 {
        // TODO: Max of RUST_LOG and command line arg
        std::env::set_var("RUST_LOG", match options.verbose {
            0 => "error",
            1 => "warn",
            2 => "info",
            3 => "debug",
            _ => "trace",
        });
    }
    env_logger::init();

    // Configure thread pool
    if let Some(threads) = options.threads {
        ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()?;
    }
    info!(
        "Using {} threads on {} cores",
        current_num_threads(),
        num_cpus::get()
    );

    // Configure random seed
    let seed = options.seed.unwrap_or_else(random);
    info!("Using random seed {:x}", seed);
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed.into());

    // Run command
    match options.command {
        Size { log_size } => {
            bench_fft(&mut rng, options.mmap, log_size)?;
        }
        Benchmark {} => {
            for log_size in 1.. {
                let duration = bench_fft(&mut rng, options.mmap, log_size)?;
                println!("{}\t{}", log_size, duration.as_secs_f64());
            }
        }
    };

    // Log allocator stats
    ALLOCATOR.log_statistics();

    Ok(())
}
