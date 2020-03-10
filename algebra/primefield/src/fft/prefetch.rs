#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{_mm_prefetch, _MM_HINT_T0};

// TODO: Use crate or builtins.

// TODO: Slice version
// TODO: MADV_HUGEPAGE

// TODO: Add madvise  <http://man7.org/linux/man-pages/man2/madvise.2.html>

#[cfg(target_arch = "x86_64")]
extern "C" {
    /// See <http://llvm.org/docs/LangRef.html#llvm-prefetch-intrinsic>
    #[link_name = "llvm.prefetch"]
    fn llvm_prefetch(a: *const i8, b: i32, c: i32, d: i32) -> ();
}

pub trait Prefetch {
    fn prefetch_write(&self);
}

impl<T> Prefetch for &T {
    #[inline(always)]
    fn prefetch_write(&self) {
        let ptr = *self as *const T;
        #[cfg(target_arch = "x86_64")]
        unsafe {
            // Prefetch Write, L1, Data
            llvm_prefetch(ptr as *const i8, 1, 3, 1);
        }
    }
}
