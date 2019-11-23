use crate::polynomial::DensePolynomial;
use rayon::prelude::*;
use std::{
    ops::{Index, IndexMut},
    prelude::v1::*,
};
use zkp_mmap_vec::MmapVec;
use zkp_primefield::{
    fft::{ifft_permuted, permute},
    FieldElement,
};

#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct TraceTable {
    trace_length: usize,
    num_columns:  usize,
    values:       MmapVec<FieldElement>,
}

impl TraceTable {
    /// Constructs a zero-initialized trace table of the given size.
    pub fn new(trace_length: usize, num_columns: usize) -> Self {
        let mut values: MmapVec<FieldElement> = MmapVec::with_capacity(trace_length * num_columns);
        for _ in 0..(trace_length * num_columns) {
            values.push(FieldElement::ZERO);
        }
        Self {
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

    pub fn generator(&self) -> FieldElement {
        FieldElement::root(self.trace_length).expect("No generator for trace table length.")
    }

    pub fn iter_row(&self, i: usize) -> impl Iterator<Item = &FieldElement> {
        // Delegate to Index<usize> which returns a row slice.
        self[i].iter()
    }

    pub fn iter_column(&self, j: usize) -> impl Iterator<Item = &FieldElement> {
        self.values[j..].iter().step_by(self.num_columns)
    }

    /// Extract the j-th column as a vector
    ///
    /// It allocates a potentially large new vector. Where possible, use
    /// the index accessors or the column iterator instead. It is unfortunately
    /// not possible to get a slice of a column (since the representation is
    /// row first.)
    // TODO: Use strides
    pub fn column_to_mmapvec(&self, j: usize) -> MmapVec<FieldElement> {
        let mut result: MmapVec<FieldElement> = MmapVec::with_capacity(self.trace_length);
        for v in self.iter_column(j) {
            result.push(v.clone());
        }
        result
    }

    pub fn interpolate(&self) -> Vec<DensePolynomial> {
        (0..self.num_columns())
            .into_par_iter()
            // OPT: Use and FFT that can transform the entire table in one pass,
            // working on whole rows at a time. That is, it is vectorized over rows.
            // OPT: Use an in-place FFT. We don't need the trace table after this,
            // so it can be replaced by a matrix of coefficients.
            .map(|j| {
                // Copy column to vec
                let mut vec = MmapVec::with_capacity(self.num_rows());
                for v in self.iter_column(j) {
                    vec.push(v.clone());
                }

                // Transform to coefficients
                ifft_permuted(&mut vec);
                permute(&mut vec);
                DensePolynomial::from_mmap_vec(vec)
            })
            .collect::<Vec<DensePolynomial>>()
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
