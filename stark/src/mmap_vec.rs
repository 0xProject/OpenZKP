use memmap::{MmapMut, MmapOptions};
use std::{
    marker::PhantomData,
    mem::size_of,
    ops::{Deref, DerefMut},
    prelude::v1::*,
    slice,
};
use tempfile::tempfile;

#[allow(dead_code)]
pub struct MmapVec<T: Clone> {
    mmap:     MmapMut,
    length:   usize,
    capacity: usize,
    _t:       PhantomData<T>,
}

#[allow(dead_code)]
impl<T: Clone> MmapVec<T> {
    pub fn with_capacity(capacity: usize) -> MmapVec<T> {
        debug_assert!(capacity > 0);
        // From https://docs.rs/tempfile/3.1.0/tempfile/: tempfile() relies on
        // the OS to remove the temporary file once the last handle is closed.
        let file = tempfile().expect("cannot create temporary file");
        // TODO: Round up to nearest 4KB
        let size = capacity * size_of::<T>();
        file.set_len(size as u64)
            .expect("cannot set mmap file length");
        let mmap = unsafe { MmapOptions::new().len(size).map_mut(&file) }
            .expect("cannot access memory mapped file");

        MmapVec {
            mmap,
            length: 0,
            capacity,
            _t: PhantomData,
        }
    }

    pub fn push(&mut self, next: T) {
        if self.length == self.capacity {
            panic!("MmapVec is at capacity")
        }
        let end = self.length;
        self.length += 1;
        self[end] = next;
    }

    pub fn extend(&mut self, other: &[T]) {
        let old_length = self.length;
        let new_length = old_length + other.len();
        if new_length > self.capacity {
            panic!("MmapVec cannot be extended beyond capacity")
        }
        self.length = new_length;
        self[old_length..new_length].clone_from_slice(other);
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }
}

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
    use primefield::FieldElement;
    use u256::U256;

    #[test]
    fn test_len() {
        let mut m: MmapVec<FieldElement> = MmapVec::with_capacity(2);
        m.push(FieldElement::ONE);
        m.push(FieldElement::ZERO);
        assert_eq!(m.len(), 2);
    }

    #[test]
    fn test_slice() {
        fn slice_function<T>(_x: &[T]) {}
        let m: MmapVec<FieldElement> = MmapVec::with_capacity(1);
        slice_function(m.as_slice());
    }

    #[test]
    fn test_mut_slice() {
        fn mut_slice_function<T>(mut _x: &[T]) {}
        let mut m: MmapVec<FieldElement> = MmapVec::with_capacity(1);
        mut_slice_function(m.as_mut_slice());
    }

    #[test]
    fn field_element_mmap_vec() {
        let mut m: MmapVec<FieldElement> = MmapVec::with_capacity(10);
        let v = vec![FieldElement::ONE; 10];
        m.extend(v.as_slice());

        for (i, x) in m.iter_mut().enumerate() {
            *x += FieldElement::from(U256::from(i));
        }

        for i in 0..10u64 {
            assert_eq!(m[i as usize], FieldElement::from(U256::from(i + 1)))
        }
    }

    #[test]
    #[should_panic]
    fn test_positive_capacity_required() {
        let _: MmapVec<u64> = MmapVec::with_capacity(0);
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
        let v = vec![10u64; 2];
        m.extend(v.as_slice());
    }
}
