// Copyright 2017 Facebook, Inc.
//
// This software may be used and distributed according to the terms of the
// GNU General Public License version 2 or any later version.

//! Simple vector based on mmap

// use errors::Result;
use memmap::{MmapMut, MmapOptions};
use std::{
    borrow::Borrow,
    fs::{File, OpenOptions},
    io::{Seek, SeekFrom},
    marker::PhantomData,
    mem::size_of,
    ops::{Deref, DerefMut, Drop, Index, IndexMut, Range, RangeFrom, RangeFull, RangeTo},
    path::PathBuf,
    ptr, slice,
};

/// A subset of Vec<T> features based on a mmap-ed file. Application should take
/// care of locking so `MVec`s backed by a same file would either have multiple
/// readers, or have a unique reader.
pub struct MmapVec<T> {
    mmap: MmapMut,
    file: File,
    len:  usize,
    cap:  usize,
    _t:   PhantomData<T>,
}

impl<T: Clone> MmapVec<T> {
    pub fn from_file(mut file: File) -> std::io::Result<Self> {
        let min_size: usize = size_of::<T>().max(4096);

        // Make sure the file is not empty and has a size of multiples of size_of::<T>()
        file.seek(SeekFrom::Start(0))?;
        let mut size = file.seek(SeekFrom::End(0))? as usize;
        let len = size / size_of::<T>();
        if size < min_size {
            file.set_len(min_size as u64)?;
            size = min_size;
        }
        let cap = size / size_of::<T>();
        size = cap * size_of::<T>();

        // mmap the file
        let mmap = unsafe { MmapOptions::new().len(size).map_mut(&file) }?;
        Ok(Self {
            mmap,
            file,
            len,
            cap,
            _t: PhantomData,
        })
    }

    pub fn from_path<P: Borrow<PathBuf>>(path: P) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.borrow())?;
        Self::from_file(file)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn extend_from_slice(&mut self, other: &[T]) {
        let len = self.len();
        let slice_len = other.len();
        self.reserve(slice_len);
        self.len += slice_len;
        self[len..(len + slice_len)].clone_from_slice(other);
    }

    pub fn resize(&mut self, new_len: usize, value: T) {
        let len = self.len;
        if new_len > len {
            let n = new_len - len;
            self.reserve(n);
            unsafe {
                let mut ptr = self.mmap.as_mut_ptr() as *mut T;
                ptr = ptr.offset(self.len as isize);
                for _ in 0..n {
                    ptr::write(ptr, value.clone());
                    ptr = ptr.offset(1);
                }
            }
        }
        self.len = new_len;
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        let required_cap = self.len + additional;
        if required_cap > self.cap {
            // Preallocate more space to reduce disk fragments. Neither too big nor too
            // small.
            let new_cap = ((required_cap + 0x40_0000) & !0x3f_ffff)
                .min(self.cap * 4)
                .max(required_cap);
            // Always align to 4KB
            let new_cap = (new_cap + 0x1000) & !0xfff;
            let size = new_cap * size_of::<T>();
            self.file.set_len(size as u64).expect("disk full");
            self.mmap = unsafe { MmapOptions::new().len(size).map_mut(&self.file) }.expect("mmap");
            self.cap = new_cap;
        }
    }

    pub fn sync(&self) -> std::io::Result<()> {
        Ok(self.mmap.flush()?)
    }
}

impl<T> Deref for MmapVec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.mmap.as_ptr() as *const T, self.len) }
    }
}

impl<T> DerefMut for MmapVec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.mmap.as_mut_ptr() as *mut T, self.len) }
    }
}

impl<T> Drop for MmapVec<T> {
    fn drop(&mut self) {
        if self.cap != self.len {
            // Truncate the file to actual size
            let size = self.len * size_of::<T>();
            self.file.set_len(size as u64).expect("truncate failed");
        }
    }
}

// Mark MmapVec as supporting Index/IndexMut interface
macro_rules! impl_index {
    ($I:ty, $T:tt, $O:tt) => {
        impl<$T: Copy> Index<$I> for MmapVec<T> {
            type Output = $O;

            #[inline]
            fn index(&self, index: $I) -> &$O {
                self.deref().index(index)
            }
        }

        impl<$T: Copy> IndexMut<$I> for MmapVec<T> {
            #[inline]
            fn index_mut(&mut self, index: $I) -> &mut $O {
                self.deref_mut().index_mut(index)
            }
        }
    };
}

impl_index!(usize, T, T);
impl_index!(Range<usize>, T, [T]);
impl_index!(RangeFrom<usize>, T, [T]);
impl_index!(RangeTo<usize>, T, [T]);
impl_index!(RangeFull, T, [T]);

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    use std::fmt::Debug;
    use tempdir::TempDir;
    use crate::FieldElement;

    fn check_with_vec<T: Copy + Eq + Default + Debug>(v: &Vec<T>) -> std::io::Result<()> {
        let tempdir = TempDir::new("mvec")?;
        let path = tempdir.path().join("mvec");
        {
            let mut m: MmapVec<T> = MmapVec::from_path(&path)?;

            assert_eq!(m.len(), 0);
            assert_ne!(m.cap(), 0);

            m.extend_from_slice(&v[..]);
            assert_eq!(m.len(), v.len());
            assert!(m.cap() >= v.len());
            assert_eq!(m[..], v[..]);
        }
        {
            // Reload the file
            let m: MmapVec<T> = MmapVec::<T>::from_path(&path)?;
            assert_eq!(m[..], v[..]);
        }
        Ok(())
    }

    #[test]
    fn check_large_buffer() {
        assert!(check_with_vec(&vec![0x12345678abcdef01u64; 10000]).is_ok());
    }

    #[test]
    fn check_resize() {
        let tempdir = TempDir::new("mvec").unwrap();
        let path = tempdir.path().join("resize");

        let mut v = Vec::<u64>::new(); // reference
        {
            let mut m: MmapVec<u64> = MmapVec::from_path(path).unwrap();
            for &(size, default) in [(10, 101), (100, 201), (1000, 301), (500, 401)].iter() {
                m.resize(size, default);
                v.resize(size, default);
                assert_eq!(m[..], v[..]);
            }
        }

        assert!(check_with_vec(&vec![0x12345678abcdef01u64; 10000]).is_ok());
    }

    #[quickcheck]
    fn test_compare_with_vec_i8(v: Vec<i8>) -> bool {
        check_with_vec(&v).is_ok()
    }

    #[quickcheck]
    fn test_compare_with_vec_u32(v: Vec<u32>) -> bool {
        check_with_vec(&v).is_ok()
    }

    #[quickcheck]
    fn test_compare_with_vec_i64(v: Vec<i64>) -> bool {
        check_with_vec(&v).is_ok()
    }

    #[test]
    fn field_element_mmap_vec() {
        let tempdir = TempDir::new("mvec").unwrap();
        let path = tempdir.path().join("resize");
        let mut m: MmapVec<FieldElement> = MmapVec::from_path(path).unwrap();
        
        // {
        //     let mut m: MmapVec<u64> = MmapVec::from_path(path).unwrap();
        //     for &(size, default) in [(10, 101), (100, 201), (1000, 301), (500, 401)].iter() {
        //         m.resize(size, default);
        //         v.resize(size, default);
        //         assert_eq!(m[..], v[..]);
        //     }
        // }
    }

}
