// TODO: Move to a utils crate

// TODO: Add madvise  <http://man7.org/linux/man-pages/man2/madvise.2.html>

// TODO: Specify cache level instead of defaulting to L1.

// TODO: implement IndexPrefetch<I> for Index<I>

// Use the _mm_prefetch intrinsic from stable for now.
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{_mm_prefetch, _MM_HINT_T0};

pub trait Prefetch {
    /// Prefetch for reading
    fn prefetch_read(&self);

    /// Prefetch for writing.
    /// (We don't need `&mut` here because we are not yet changing anything)
    fn prefetch_write(&self);
}

pub trait PrefetchIndex<I>
where
    I: ?Sized,
{
    /// Prefetch an element from a collection by index
    fn prefetch_index_read(&self, index: I);

    /// Prefetch an element for writing from a collection by index
    /// (We don't need `&mut` here because we are not yet changing anything)
    fn prefetch_index_write(&self, index: I);
}

// Blanket implementation for all types
impl<T> Prefetch for T {
    #[inline(always)]
    #[cfg(target_arch = "x86_64")]
    fn prefetch_read(&self) {
        // Prefetching does not affect the semantics of the program.
        #[allow(unsafe_code)]
        unsafe {
            #[allow(trivial_casts)] // False positive
            let ptr = self as *const Self as *const i8;
            _mm_prefetch(ptr, _MM_HINT_T0);
        }
    }

    #[inline(always)]
    #[cfg(not(target_arch = "x86_64"))]
    fn prefetch_read(&self) {
        // Unsupported platform, do nothing
    }

    #[inline(always)]
    fn prefetch_write(&self) {
        // Currently no intrinsic available, so do a read prefetch instead.
        self.prefetch_read()
    }
}

// Blanket implementation for slices
impl<T> PrefetchIndex<usize> for [T] {
    #[inline(always)]
    #[cfg(target_arch = "x86_64")]
    fn prefetch_index_read(&self, index: usize) {
        // Bounds checking is not necessary for prefetches.
        // Prefetches do not change the semantics and even if the prefetch
        // causes a page fault or any other memory exception, it is silently
        // ignored by the CPU.
        #[allow(unsafe_code)]
        unsafe {
            #[allow(trivial_casts)] // False positive
            let ptr = self.get_unchecked(index) as *const Self as *const i8;
            _mm_prefetch(ptr, _MM_HINT_T0);
        }
    }

    #[inline(always)]
    #[cfg(not(target_arch = "x86_64"))]
    fn prefetch_index_read(&self, _index: usize) {
        // Unsupported platform, do nothing
    }

    #[inline(always)]
    fn prefetch_index_write(&self, index: usize) {
        // No intrinsic for write available on stable yet, do a read instead
        self.prefetch_index_read(index)
    }
}
