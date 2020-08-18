// This module abstracts low-level `unsafe` behaviour
#![allow(unsafe_code)]

// False positive: attribute has a use
#[allow(clippy::useless_attribute)]
// False positive: Importing preludes is allowed
#[allow(clippy::wildcard_imports)]
use std::prelude::v1::*;

use log::trace;
use memmap::{MmapMut, MmapOptions};
use std::{
    cmp::max,
    marker::PhantomData,
    mem::size_of,
    ops::{Deref, DerefMut},
    ptr::drop_in_place,
    slice,
};

// TODO: Variant of MmapVec where it switched between Vec and Mmap after
//       a treshold size.

#[derive(Debug)] // TODO: Custom implementation
pub struct MmapVec<T: Clone> {
    mmap:     MmapMut,
    length:   usize,
    capacity: usize,
    _t:       PhantomData<T>,
}

impl<T: Clone> MmapVec<T> {
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        // TODO: Round up to nearest 4KB
        // Note: mmaped files can not be empty, so we use at leas one byte.
        let size = max(1, capacity * size_of::<T>());
        trace!("Allocating {} MB in anonymous mmap", size / 1_000_000);
        let mmap = MmapOptions::new()
            .len(size)
            .map_anon()
            .expect("cannot access memory mapped file");
        Self {
            mmap,
            length: 0,
            capacity,
            _t: PhantomData,
        }
    }

    /// # Safety
    /// This function returns an array of size `len` that is initialized
    /// with all bits set to zero. This is only safe if all-zeros is a valid
    /// and safe bit-pattern for type `T`. This is for example the case for
    /// integer types, but is not safe for types containing references.
    // TODO: Maybe we should do something like a Zeroed trait?
    // See https://github.com/rust-lang/rfcs/issues/2626
    #[must_use]
    pub unsafe fn zero_initialized(len: usize) -> Self {
        let mut result = Self::with_capacity(len);
        result.length = len;
        result
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, next: T) {
        if self.length == self.capacity {
            panic!("MmapVec is at capacity")
        }
        let end = self.length;
        self.length += 1;
        self[end] = next;
    }

    pub fn clear(&mut self) {
        self.truncate(0)
    }

    pub fn truncate(&mut self, length: usize) {
        if length >= self.length {
            return;
        }
        #[allow(unsafe_code)]
        unsafe {
            // Modified from std::vec::Vec::truncate, which has this comment:
            // This is safe because:
            //
            // * the slice passed to `drop_in_place` is valid; the `len > self.len` case
            //   avoids creating an invalid slice, and
            // * the `len` of the vector is shrunk before calling `drop_in_place`, such that
            //   no value will be dropped twice in case `drop_in_place` were to panic once
            //   (if it panics twice, the program aborts).
            let slice_pointer: *mut [T] = &mut self.as_mut_slice()[length..];
            self.length = length;
            drop_in_place(slice_pointer);
        }
    }

    pub fn resize(&mut self, size: usize, fill: T) {
        if size > self.capacity {
            panic!("MmapVec is at capacity")
        }
        while self.length < size {
            self.push(fill.clone());
        }
        self.length = size;
    }

    pub fn extend_from_slice(&mut self, slice: &[T]) {
        if self.length + slice.len() > self.capacity {
            panic!("MmapVec would grow beyond capacity")
        }
        let start = self.length;
        self.length += slice.len();
        self.as_mut_slice()[start..].clone_from_slice(slice);
    }

    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self
    }

    #[inline]
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }
}

impl<T: Clone + PartialEq> PartialEq for MmapVec<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T: Clone> Clone for MmapVec<T> {
    fn clone(&self) -> Self {
        let mut clone = Self::with_capacity(self.capacity);
        clone.extend(self.iter());
        clone
    }
}

impl<T: Clone> Extend<T> for MmapVec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        // The function signature is for compatibility with Vec::extend.
        // OPT: Specialize for SliceIterator
        for i in iter {
            self.push(i)
        }
    }
}

impl<'a, T: 'a + Clone> Extend<&'a T> for MmapVec<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        // The function signature is for compatibility with Vec::extend.
        for i in iter {
            self.push(i.clone())
        }
    }
}

// TODO: Implement Rayon's ParallelExtend
// see <https://docs.rs/rayon/1.3.0/rayon/iter/trait.ParallelExtend.html#tymethod.par_extend>

impl<T: Clone> Deref for MmapVec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.mmap.as_ptr() as *const T, self.length) }
    }
}

impl<T: Clone> DerefMut for MmapVec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.mmap.as_mut_ptr() as *mut T, self.length) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let empty = MmapVec::<u64>::with_capacity(0);
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn test_len() {
        let mut m: MmapVec<String> = MmapVec::with_capacity(2);
        m.push("Hello".to_string());
        m.push("World".to_string());
        assert_eq!(m.len(), 2);
    }

    #[test]
    fn test_slice() {
        fn slice_function<T>(_x: &[T]) {}
        let m: MmapVec<String> = MmapVec::with_capacity(1);
        slice_function(m.as_slice());
    }

    #[test]
    fn test_mut_slice() {
        fn mut_slice_function<T>(mut _x: &[T]) {}
        let mut m: MmapVec<String> = MmapVec::with_capacity(1);
        mut_slice_function(m.as_mut_slice());
    }

    #[test]
    fn field_element_mmap_vec() {
        let mut m: MmapVec<usize> = MmapVec::with_capacity(10);
        let v = vec![42; 10];
        m.extend(v.as_slice());

        for (i, x) in m.iter_mut().enumerate() {
            *x += i;
        }

        for i in 0..10 {
            assert_eq!(m[i], 42 + i)
        }
    }

    #[test]
    #[should_panic]
    fn test_cannot_index_beyond_end() {
        let mut m: MmapVec<u64> = MmapVec::with_capacity(1);
        m[0] = 10;
    }

    #[test]
    #[should_panic]
    fn test_cannot_extend_beyond_capacity() {
        let mut m: MmapVec<u64> = MmapVec::with_capacity(1);
        let v = vec![10_u64; 2];
        m.extend(v.as_slice());
    }
}
