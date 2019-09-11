use std::{
    alloc::{GlobalAlloc, Layout, System},
}
use log::{error, warn, info, trace};

struct TracingAllocator;
const INFO: usize = 1_000_000;
const WARN: usize = 10_000_000
const ERROR: usize = 100_000_000;
const REJECT: usize = 1_000_000_000;

unsafe impl GlobalAlloc for TracingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() > INFO {
            if layout.size() > ERROR {
                error!("Allocating {:?} MB on heap", layout.size() / 1_000_000);
                if layout.size() > REJECT {
                    panic!("Rejecting allocation");
                }
            } else {
                info!("Allocating {:?} MB on heap", layout.size() / 1_000_000);
            }
        }
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

#[cfg(enable)]
#[global_allocator]
static Allocator: TracingAllocator = TracingAllocator;
