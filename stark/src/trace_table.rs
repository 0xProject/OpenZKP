use crate::mmap_vec::MmapVec;
use primefield::FieldElement;
use std::ops::{Index, IndexMut};

pub struct TraceTable {
    trace_length: usize,
    num_columns:  usize,
    values:       MmapVec<FieldElement>,
}

impl TraceTable {
    /// Constructs a zero-initialized trace table of the given size.
    pub fn new(trace_length: usize, num_columns: usize) -> TraceTable {
        let mut values: MmapVec<FieldElement> = MmapVec::with_capacity(trace_length * num_columns);
        for _i in 0..(trace_length * num_columns) {
            values.push(FieldElement::ZERO);
        }
        TraceTable {
            trace_length,
            num_columns,
            values,
        }
    }

    pub fn num_rows(&self) -> usize {
        self.trace_length
    }

    pub fn num_columns(&self) -> usize {
        self.num_columns
    }

    #[allow(dead_code)] // TODO
    pub fn generator(&self) -> FieldElement {
        FieldElement::root(self.trace_length.into()).expect("No generator for trace table length.")
    }

    /// Extract the j-th column as a vector
    // OPT: Instead of using this function, work with strides.
    pub fn column(&self, j: usize) -> MmapVec<FieldElement> {
        let mut result: MmapVec<FieldElement> = MmapVec::with_capacity(self.trace_length);
        for v in self.values.iter().skip(j).step_by(self.num_columns) {
            result.push(v.clone());
        }
        result
    }
}

/// Returns a field
impl Index<(usize, usize)> for TraceTable {
    type Output = FieldElement;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        assert!(i < self.trace_length);
        assert!(j < self.num_columns);
        &self.values[i * self.num_columns + j]
    }
}

/// Returns a mutable field
impl IndexMut<(usize, usize)> for TraceTable {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        assert!(i < self.trace_length);
        assert!(j < self.num_columns);
        &mut self.values[i * self.num_columns + j]
    }
}

/// Returns a row as a slice
impl Index<usize> for TraceTable {
    type Output = [FieldElement];

    fn index(&self, i: usize) -> &[FieldElement] {
        assert!(i < self.trace_length);
        &self.values[i * self.num_columns..(i + 1) * self.num_columns]
    }
}

/// Returns a mutable row as a slice
impl IndexMut<usize> for TraceTable {
    fn index_mut(&mut self, i: usize) -> &mut [FieldElement] {
        assert!(i < self.trace_length);
        &mut self.values[i * self.num_columns..(i + 1) * self.num_columns]
    }
}
