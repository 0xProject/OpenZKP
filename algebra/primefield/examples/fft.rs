use env_logger;
use log::{info, warn};
use num_cpus;
use num_traits::pow::Pow;
use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use raw_cpuid::{CacheType, CpuId};
use rayon::{current_num_threads, ThreadPoolBuilder};
use std::{
    iter::repeat_with,
    num::ParseIntError,
    time::{Duration, Instant},
};
use structopt::StructOpt;
use zkp_logging_allocator::ALLOCATOR;
use zkp_macros_decl::u256h;
use zkp_mmap_vec::MmapVec;
use zkp_primefield::{
    fft::{fft_vec_recursive, get_twiddles, permute, radix_sqrt, transpose_square_stretch},
    Fft, FieldElement, Parameters, PrimeField, Root,
};
use zkp_u256::U256;

fn parse_hex(src: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(src, 16)
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Bls12_381_Fr();

impl Parameters for Bls12_381_Fr {
    type UInt = U256;

    /// 3, in montgomery form.
    const GENERATOR: U256 =
        u256h!("351332208fc5a8c4ff9c57876f8457b017e363d300189c0f0000000efffffff1");
    const M64: u64 = 0xffff_fffe_ffff_ffff;
    const MODULUS: U256 =
        u256h!("73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001");
    ///
    const ORDER: U256 = u256h!("0800000000000011000000000000000000000000000000000000000000000000");
    const R1: U256 = u256h!("1824b159acc5056f998c4fefecbc4ff55884b7fa0003480200000001fffffffe");
    const R2: U256 = u256h!("0748d9d99f59ff1105d314967254398f2b6cedcb87925c23c999e990f3f29c6d");
    const R3: U256 = u256h!("6e2a5bb9c8db33e973d13c71c7b5f4181b3e0d188cf06990c62c1807439b73af");
}

type Field = PrimeField<Bls12_381_Fr>;

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
    /// Valid options are: fft, fft_sqrt, fft_recursive, transpose,
    /// permute
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
    Heap(Vec<Field>),
    Mmap(MmapVec<Field>),
}

impl Allocation {
    fn random<R: Rng + ?Sized>(rng: &mut R, allocation: &str, size: usize) -> Allocation {
        match allocation {
            "heap" => {
                info!("Allocating size {} on heap", size);
                let mut vec = Vec::<Field>::with_capacity(size);
                info!("Filling with random numbers");
                vec.extend(repeat_with(|| rng.gen::<Field>()).take(size));
                assert_eq!(vec.len(), size);
                Allocation::Heap(vec)
            }
            "mmap" => {
                info!("Memore mapping size {} ", size);
                let mut vec = MmapVec::<Field>::with_capacity(size);
                info!("Filling with random numbers");
                vec.extend(repeat_with(|| rng.gen::<Field>()).take(size));
                assert_eq!(vec.len(), size);
                Allocation::Mmap(vec)
            }
            _ => unimplemented!(),
        }
    }

    fn as_mut_slice(&mut self) -> &mut [Field] {
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
    func: &mut dyn FnMut(&mut [Field]),
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

// TODO: Log topology and frequency, see raw-cpuid examples.
// TODO: Log ram size, page size, disk type
fn log_sys() {
    let cpuid = CpuId::new();
    info!(
        "CPU Model is: {}",
        cpuid.get_extended_function_info().as_ref().map_or_else(
            || "n/a",
            |extfuninfo| extfuninfo.processor_brand_string().unwrap_or("unreadable"),
        )
    );
    cpuid.get_cache_parameters().map_or_else(
        || info!("No cache parameter information available"),
        |cparams| {
            for cache in cparams {
                let size = cache.associativity()
                    * cache.physical_line_partitions()
                    * cache.coherency_line_size()
                    * cache.sets();

                let typ = match cache.cache_type() {
                    CacheType::Data => "Instruction-Cache",
                    CacheType::Instruction => "Data-Cache",
                    CacheType::Unified => "Unified-Cache",
                    _ => "Unknown cache type",
                };

                let associativity = if cache.is_fully_associative() {
                    "fully associative".to_string()
                } else {
                    format!("{}-way associativity", cache.associativity())
                };

                let size_repr = if size > 1024 * 1024 {
                    format!("{} MiB", size / (1024 * 1024))
                } else {
                    format!("{} KiB", size / 1024)
                };

                let mapping = if cache.has_complex_indexing() {
                    "hash-based-mapping"
                } else {
                    "direct-mapped"
                };

                info!(
                    "Cache L{} {}: ({}, {} B block, {}, {})",
                    cache.level(),
                    typ,
                    size_repr,
                    cache.coherency_line_size(),
                    associativity,
                    mapping,
                );
            }
        },
    );
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

    // Log system info
    log_sys();

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
    let mut func: Box<dyn FnMut(&mut [Field])> = match name.as_ref() {
        "fft" => Box::new(Fft::fft),
        "fft_sqrt" => {
            Box::new(|values| {
                let root = Field::root(values.len()).unwrap();
                radix_sqrt(values, &root);
            })
        }
        "fft_recursive" => {
            Box::new(|values| {
                let root = Field::root(values.len()).unwrap();
                let twiddles = get_twiddles(&root, values.len());
                fft_vec_recursive(values, &twiddles, 0, 1, 1);
            })
        }
        "permute" => Box::new(permute),
        "transpose" => {
            Box::new(|values: &mut [Field]| {
                let length = values.len();
                let size = 1_usize << (length.trailing_zeros() / 2);
                let stretch = length / (size * size);
                transpose_square_stretch(values, size, stretch)
            })
        }
        _ => unimplemented!(),
    };

    // Run benchmark
    if let Some(log_size) = options.log_size {
        let duration = bench(&mut rng, &options.allocation, log_size, name, &mut func)?;
        // Log allocator stats
        ALLOCATOR.log_statistics();
        println!("{}\t{}", log_size, duration.as_secs_f64());
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
