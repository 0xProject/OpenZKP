use std::ops::Index;
use std::slice::SliceIndex;

// TODO: Move to utils crate

// TODO: Use crate or builtins.

// TODO: Add madvise  <http://man7.org/linux/man-pages/man2/madvise.2.html>
// TODO: MADV_HUGEPAGE

// TODO: Specify cache level instead of defaulting to L1.

// TODO: implement IndexPrefetch<I> for Index<I>

#[cfg(target_arch = "x86_64")]
extern "C" {
    /// See <http://llvm.org/docs/LangRef.html#llvm-prefetch-intrinsic>
    #[link_name = "llvm.prefetch"]
    fn llvm_prefetch(a: *const i8, b: i32, c: i32, d: i32) -> ();
}

const READ: i32 = 0;
const WRITE: i32 = 1;
const L1: i32 = 3;
const DATA: i32 = 1;

pub trait Prefetch {
    /// Prefetch for reading
    fn prefetch(&self);

    /// Prefetch for writing.
    /// (We don't need `&mut` here because we are not yet changing anything)
    fn prefetch_write(&self);
}

pub trait PrefetchIndex<I> where I: ?Sized {
    /// Prefetch an element from a collection by index
    fn prefetch_index(&self, index: I);

    /// Prefetch an element for writing from a collection by index
    /// (We don't need `&mut` here because we are not yet changing anything)
    fn prefetch_index_write(&self, index: I);
}

// Blanket implementation for all types
impl<T> Prefetch for T {
    #[inline(always)]
    fn prefetch(&self) {
        let ptr = self as *const T;
        #[cfg(target_arch = "x86_64")]
        // Prefetching does not affect the semantics of the program.
        #[allow(unsafe_code)]
        unsafe {
            llvm_prefetch(ptr as *const i8, READ, L1, DATA);
        }
    }

    #[inline(always)]
    fn prefetch_write(&self) {
        let ptr = self as *const T;
        #[cfg(target_arch = "x86_64")]
        // Prefetching does not affect the semantics of the program.
        #[allow(unsafe_code)]
        unsafe {
            llvm_prefetch(ptr as *const i8, WRITE, L1, DATA);
        }
    }
}

// Blanket implementation for slices
impl<T> PrefetchIndex<usize> for [T] where  {
    #[inline(always)]
    fn prefetch_index(&self, index: usize) {

        // Bounds checking is not necessary for prefetches.
        // Prefetches do not change the semantics and even if the prefetch
        // causes a page fault or any other memory exception, it is silently
        // ignored by the CPU.
        #[allow(unsafe_code)]
        unsafe {
            let ptr = self.get_unchecked(index) as *const T;
            llvm_prefetch(ptr as *const i8, READ, L1, DATA);
        }
    }

    #[inline(always)]
    fn prefetch_index_write(&self, index: usize) {

        // Bounds checking is not necessary for prefetches.
        // Prefetches do not change the semantics and even if the prefetch
        // causes a page fault or any other memory exception, it is silently
        // ignored by the CPU.
        #[allow(unsafe_code)]
        unsafe {
            let ptr = self.get_unchecked(index) as *const T;
            llvm_prefetch(ptr as *const i8, WRITE, L1, DATA);
        }
    }
}
