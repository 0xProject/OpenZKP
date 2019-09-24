// HACK: This sequence needs to be repeated in each project.
//       See https://github.com/rust-lang/cargo/issues/5034
// For clippy lints see: https://rust-lang.github.io/rust-clippy/master
// For rustc lints see: https://doc.rust-lang.org/rustc/lints/index.html
#![cfg_attr(not(feature = "std"), no_std)]
// #![forbid(unsafe_code)] // Allocators are by nature unsafe
#![warn(
    // Enable sets of warnings
    clippy::all,
    clippy::pedantic,
    // TODO: clippy::cargo,
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
    // TODO: missing_docs,
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

use log::{error, info, warn};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    ptr::null_mut,
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
    sync::atomic::Ordering,
};
use std::fmt;

struct SizeBytes(usize);

impl fmt::Display for SizeBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self.0 as f32;
        if size < 1.0e3 {
            write!(f, "{} B", size)
        } else if size < 1.0e6 {
            write!(f, "{:.1$} kB", size / 1.0e3, 5 - (size.log10().floor() as usize))
        } else if size < 1.0e9 {
            write!(f, "{:.1$} MB", size / 1.0e6, 8 - (size.log10().floor() as usize))
        } else if size < 1.0e12 {
            write!(f, "{:.1$} GB", size / 1.0e9, 11 - (size.log10().floor() as usize))
        } else {
            write!(f, "{} TB", size / 1.0e12)
        }
    }
}

// TODO: Make it store a static ref to an inner allocator (defaults to System)
#[cfg_attr(feature = "std", derive(Debug))]
pub struct LoggingAllocator {
    info:              usize,
    warn:              usize,
    error:             usize,
    reject:            usize,
    allocated:         AtomicUsize,
    peak_allocated:    AtomicUsize,
    total_allocated:   AtomicUsize,
    largest_allocated: AtomicUsize,
    num_allocations:   AtomicUsize,
}

impl LoggingAllocator {
    // We need a `const fn` constructor
    // TODO: Implement `Default` when trait functions can be `const`.
    pub const fn new() -> Self {
        Self {
            info:              1_000_000,
            warn:              10_000_000,
            error:             100_000_000,
            reject:            10_000_000_000,
            allocated:         AtomicUsize::new(0),
            peak_allocated:    AtomicUsize::new(0),
            total_allocated:   AtomicUsize::new(0),
            largest_allocated: AtomicUsize::new(0),
            num_allocations:   AtomicUsize::new(0),
        }
    }

    pub fn allocated(&self) -> usize {
        self.allocated.load(SeqCst)
    }

    pub fn peak_allocated(&self) -> usize {
        self.peak_allocated.load(SeqCst)
    }

    pub fn total_allocated(&self) -> usize {
        self.total_allocated.load(SeqCst)
    }

    pub fn largest_allocated(&self) -> usize {
        self.largest_allocated.load(SeqCst)
    }

    pub fn num_allocations(&self) -> usize {
        self.num_allocations.load(SeqCst)
    }

    pub fn log_statistics(&self) {
        info!(
            "allocated {} in {} allocations. currently allocated {}, largest allocated {}",
            SizeBytes(self.total_allocated()),
            self.num_allocations(),
            SizeBytes(self.allocated()),
            SizeBytes(self.largest_allocated())
        );
    }
}

// GlobalAlloc is an unsafe trait for allocators
#[allow(unsafe_code)]
unsafe impl GlobalAlloc for LoggingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() < self.info {
            // Do nothing for small allocations
            // Note that the log messages themselves also make small
            // allocations, and we want to prevent infinite recursion.
        } else if layout.size() < self.warn {
            info!(
                "Allocating {} on heap ({} allocated)",
                SizeBytes(layout.size()),
                SizeBytes(self.allocated())
            );
        } else if layout.size() < self.error {
            warn!(
                "Allocating {} on heap ({} allocated)",
                SizeBytes(layout.size()),
                SizeBytes(self.allocated())
            );
        } else if layout.size() < self.reject {
            error!(
                "Allocating {} on heap ({} allocated)",
                SizeBytes(layout.size()),
                SizeBytes(self.allocated())
            );
        } else {
            error!(
                "Rejecting {} allocation on heap ({} allocated)",
                SizeBytes(layout.size()),
                SizeBytes(self.allocated())
            );
            return null_mut();
        }
        let result = System.alloc(layout);
        if !result.is_null() {
            // TODO: We are doing a lot of atomic operations here, what is
            // the performance impact?
            let allocated = self.allocated.fetch_add(layout.size(), SeqCst);
            let _ = self.total_allocated.fetch_add(layout.size(), SeqCst);
            let _ = self.num_allocations.fetch_add(1, SeqCst);
            // HACK: Using `allocated` here again is not completely fool proof
            let _ = max_cas_loop(&self.peak_allocated, allocated);
            let _ = max_cas_loop(&self.largest_allocated, layout.size());
        }
        result
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        let _ = self.allocated.fetch_sub(layout.size(), SeqCst);
    }
}

// TODO: Use `fetch_max` instead
// https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.fetch_max
// See also https://doc.rust-lang.org/src/core/sync/atomic.rs.html#1770-1783
fn max_cas_loop(atom: &AtomicUsize, value: usize) -> usize {
    const SET_ORDER: Ordering = SeqCst;
    const FETCH_ORDER: Ordering = SeqCst;
    let mut prev = atom.load(FETCH_ORDER);
    while prev < value {
        match atom.compare_exchange_weak(prev, value, SET_ORDER, FETCH_ORDER) {
            Ok(_) => return value,
            Err(next_prev) => prev = next_prev,
        }
    }
    prev
}

#[cfg(feature = "enable")]
#[global_allocator]
pub static ALLOCATOR: LoggingAllocator = LoggingAllocator::new();
