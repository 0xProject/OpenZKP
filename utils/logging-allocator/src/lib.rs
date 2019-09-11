use log::{error, info, trace, warn};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    ptr::null_mut,
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
};

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
            reject:    1_000_000_000,
            allocated: AtomicUsize::new(0),
        }
    }

    pub fn allocated(&self) -> usize {
        self.allocated.load(SeqCst)
    }
}

unsafe impl GlobalAlloc for LoggingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() < self.info {
            trace!(
                "Allocating {:?} MB on heap ({:?} MB allocated)",
                layout.size() / 1_000_000,
                self.allocated() / 1_000_000
            );
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
            self.allocated.fetch_add(layout.size(), SeqCst);
        }
        result
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        self.allocated.fetch_sub(layout.size(), SeqCst);
    }
}

#[cfg(enable)]
#[global_allocator]
pub static ALLOCATOR: LoggingAllocator = LoggingAllocator::new();
