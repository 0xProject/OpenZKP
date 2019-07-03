// https://phab.mercurial-scm.org/D1327

use memmap::{Mmap, MmapMut, MmapOptions};
use std::{
    fs::{File, OpenOptions},
    io::{Seek, SeekFrom, Write},
};
use crate::FieldElement;
use crate::U256;
use std::ops::Index;

pub struct MmapVec {
    size: usize,
    file: File,
    data: MmapMut,
}

impl MmapVec {
    pub fn new(size: usize) -> Self {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("test.mmap")
            .expect("Unable to open file");

        file.seek(SeekFrom::Start(size as u64)).unwrap();
        file.write_all(&[0]).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();

        let mut data: MmapMut = unsafe {
            memmap::MmapOptions::new()
                .map_mut(&file)
                .expect("Could not access data from memory mapped file")
        };
        Self { size, file, data }
    }
}

impl Index<usize> for MmapVec {
    type Output = FieldElement;

    fn index(&self, i: usize) -> &Self::Output {
        let slice_start = i * 32;
        let slice_end = slice_start + 32;
        let mut array = [0u8; 32];
        array.copy_from_slice(&self.data[slice_start..slice_end]);
        debug_assert!(slice_end < self.size as usize);
        &FieldElement(U256::from_bytes_be(&array))
    }
}

#[test]
fn memmap_test() {
    let v = MmapVec::new(1024);
    assert_eq!(v[0], FieldElement::ZERO);
    // const SIZE: u64 = 1024 * 1024;
    // let src = vec![1, 2, 3, 4];
    //
    //
    //
    // data[..src.len()].copy_from_slice(src.as_slice());
    // assert_eq!(data[3], 1);
}
