use env_logger;
use log::{info, warn};
use num_cpus;
use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::{current_num_threads, ThreadPoolBuilder};
use std::{
    iter::repeat_with,
    num::ParseIntError,
    time::{Duration, Instant},
};
use structopt::StructOpt;
use zkp_logging_allocator::ALLOCATOR;
use zkp_mmap_vec::MmapVec;
use zkp_primefield::{
    fft::{fft2_inplace, permute, transpose_inplace},
    FieldElement,
};

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

    /// Allocation strategy to use (defaults to mmap)
    /// Valid options are: heap, mmap
    #[structopt(long, default_value = "mmap")]
    allocation: String,

    /// Operation to benchmark (defaults to fft)
    /// Valid options are: fft, transpose, permute
    #[structopt(default_value = "fft")]
    operation: String,

    /// Run a benchmark of a given size
    #[structopt()]
    log_size: Option<usize>,
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
    fn random<R: Rng + ?Sized>(rng: &mut R, allocation: &str, size: usize) -> Allocation {
        match allocation {
            "heap" => {
                info!("Allocating size {} on heap", size);
                let mut vec = Vec::<FieldElement>::with_capacity(size);
                info!("Filling with random numbers");
                vec.extend(repeat_with(|| rng.gen::<FieldElement>()).take(size));
                assert_eq!(vec.len(), size);
                Allocation::Heap(vec)
            }
            "mmap" => {
                info!("Memore mapping size {} ", size);
                let mut vec = MmapVec::<FieldElement>::with_capacity(size);
                info!("Filling with random numbers");
                vec.extend(repeat_with(|| rng.gen::<FieldElement>()).take(size));
                assert_eq!(vec.len(), size);
                Allocation::Mmap(vec)
            }
            _ => unimplemented!(),
        }
    }

    fn as_mut_slice(&mut self) -> &mut [FieldElement] {
        match self {
            Allocation::Heap(vec) => vec,
            Allocation::Mmap(vec) => vec,
        }
    }
}

fn bench<R: Rng + ?Sized>(
    rng: &mut R,
    allocation: &str,
    log_size: usize,
    name: &str,
    func: &mut Box<dyn FnMut(&mut [FieldElement])>,
) -> Result<Duration, Error> {
    let size = 1 << log_size;
    info!("Benchmarking {} size 2^{} = {}", name, log_size, size);
    let mut allocation = Allocation::random(rng, allocation, size);
    info!("{}-ing", name);
    let start = Instant::now();
    func(allocation.as_mut_slice());
    let duration = start.elapsed();
    warn!("Total time {:?}", duration);
    Ok(duration)
}

fn main() -> Result<(), Error> {
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

    // Get function to benchmark
    let name = &options.operation;
    let mut func: Box<dyn FnMut(&mut [FieldElement])> = match name.as_ref() {
        "fft" => Box::new(fft2_inplace),
        "permute" => Box::new(permute),
        "transpose" => {
            Box::new(|values: &mut [FieldElement]| {
                let length = values.len();
                let rows = 1_usize << (length.trailing_zeros() / 2);
                let cols = length / rows;
                transpose_inplace(values, cols)
            })
        }
        _ => unimplemented!(),
    };

    // Run benchmark
    if let Some(log_size) = options.log_size {
        bench(&mut rng, &options.allocation, log_size, name, &mut func)?;
        // Log allocator stats
        ALLOCATOR.log_statistics();
    } else {
        for log_size in 1.. {
            let duration = bench(&mut rng, &options.allocation, log_size, name, &mut func)?;
            // Log allocator stats
            ALLOCATOR.log_statistics();
            println!("{}\t{}", log_size, duration.as_secs_f64());
        }
    }

    Ok(())
}
