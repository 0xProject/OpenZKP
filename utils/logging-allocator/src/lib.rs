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
};

// TODO: Make it store a static ref to an inner allocator (defaults to System)
#[cfg_attr(feature = "std", derive(Debug))]
pub struct LoggingAllocator {
    info:      usize,
    warn:      usize,
    error:     usize,
    reject:    usize,
    allocated: AtomicUsize,
}

impl LoggingAllocator {
    pub const fn new() -> Self {
        Self {
            info:      1_000_000,
            warn:      10_000_000,
            error:     100_000_000,
            reject:    1_000_000_000_000,
            allocated: AtomicUsize::new(0),
        }
    }

    pub fn allocated(&self) -> usize {
        self.allocated.load(SeqCst)
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
                "Allocating {:?} MB on heap ({:?} MB allocated)",
                layout.size() / 1_000_000,
                self.allocated() / 1_000_000
            );
        } else if layout.size() < self.error {
            warn!(
                "Allocating {:?} MB on heap ({:?} MB allocated)",
                layout.size() / 1_000_000,
                self.allocated() / 1_000_000
            );
        } else if layout.size() < self.reject {
            error!(
                "Allocating {:?} MB on heap ({:?} MB allocated)",
                layout.size() / 1_000_000,
                self.allocated() / 1_000_000
            );
        } else {
            error!(
                "Rejecting {:?} MB allocation on heap ({:?} MB allocated)",
                layout.size() / 1_000_000,
                self.allocated() / 1_000_000
            );
            return null_mut();
        }
        let result = System.alloc(layout);
        if !result.is_null() {
            let _ = self.allocated.fetch_add(layout.size(), SeqCst);
        }
        result
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        let _ = self.allocated.fetch_sub(layout.size(), SeqCst);
    }
}

#[cfg(feature = "enable")]
#[global_allocator]
pub static ALLOCATOR: LoggingAllocator = LoggingAllocator::new();
