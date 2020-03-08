use crate::SizeBytes;
use log::{error, info, warn};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    ptr::null_mut,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
};

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
    #[must_use]
    pub const fn new() -> Self {
        Self {
            info:              1_000_000,
            warn:              10_000_000,
            error:             100_000_000,
            reject:            1_000_000_000_000,
            allocated:         AtomicUsize::new(0),
            peak_allocated:    AtomicUsize::new(0),
            total_allocated:   AtomicUsize::new(0),
            largest_allocated: AtomicUsize::new(0),
            num_allocations:   AtomicUsize::new(0),
        }
    }

    pub fn allocated(&self) -> usize {
        self.allocated.load(Relaxed)
    }

    pub fn peak_allocated(&self) -> usize {
        self.peak_allocated.load(Relaxed)
    }

    pub fn total_allocated(&self) -> usize {
        self.total_allocated.load(Relaxed)
    }

    pub fn largest_allocated(&self) -> usize {
        self.largest_allocated.load(Relaxed)
    }

    pub fn num_allocations(&self) -> usize {
        self.num_allocations.load(Relaxed)
    }

    pub fn log_statistics(&self) {
        info!(
            "allocated {} in {} allocations. currently allocated {}, largest allocated {}",
            SizeBytes::from(self.total_allocated()),
            self.num_allocations(),
            SizeBytes::from(self.allocated()),
            SizeBytes::from(self.largest_allocated())
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
                SizeBytes::from(layout.size()),
                SizeBytes::from(self.allocated())
            );
        } else if layout.size() < self.error {
            warn!(
                "Allocating {} on heap ({} allocated)",
                SizeBytes::from(layout.size()),
                SizeBytes::from(self.allocated())
            );
        } else if layout.size() < self.reject {
            error!(
                "Allocating {} on heap ({} allocated)",
                SizeBytes::from(layout.size()),
                SizeBytes::from(self.allocated())
            );
        } else {
            error!(
                "Rejecting {} allocation on heap ({} allocated)",
                SizeBytes::from(layout.size()),
                SizeBytes::from(self.allocated())
            );
            return null_mut();
        }
        let result = System.alloc(layout);
        if !result.is_null() {
            // TODO: We are doing a lot of atomic operations here, what is
            // the performance impact?
            let allocated = self.allocated.fetch_add(layout.size(), Relaxed);
            let _ = self.total_allocated.fetch_add(layout.size(), Relaxed);
            let _ = self.num_allocations.fetch_add(1, Relaxed);
            // HACK: Using `allocated` here again is not completely fool proof
            let _ = fetch_max(&self.peak_allocated, allocated);
            let _ = fetch_max(&self.largest_allocated, layout.size());
        }
        result
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        let _ = self.allocated.fetch_sub(layout.size(), Relaxed);
    }
}

// TODO: Use `fetch_max` instead
// See https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.fetch_max
// This is pending https://github.com/rust-lang/rust/issues/48655
fn fetch_max(atom: &AtomicUsize, value: usize) -> usize {
    let mut prev = atom.load(Relaxed);
    while prev < value {
        match atom.compare_exchange_weak(prev, value, Relaxed, Relaxed) {
            Ok(_) => return value,
            Err(next_prev) => prev = next_prev,
        }
    }
    prev
}
