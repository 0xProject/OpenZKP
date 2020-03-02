use crate::L1_CACHE_SIZE;
use std::mem::size_of;

// TODO: Bitreverse <https://arxiv.org/pdf/1708.01873.pdf>

// http://fftw.org/fftw-paper-ieee.pdf
// https://wgropp.cs.illinois.edu/courses/cs598-s16/lectures/lecture08.pdf

// See: <https://cacs.usc.edu/education/cs653/Frigo-CacheOblivious-FOCS99.pdf>

// Reference implementation for testing and benchmarking purposes
#[cfg(any(feature = "test", feature = "bench"))]
pub fn reference<T: Clone>(src: &[T], dst: &mut [T], row_size: usize) {
    assert_eq!(src.len(), dst.len());
    if src.len() == 0 || row_size == 0 {
        return;
    }
    debug_assert_eq!(src.len() % row_size, 0);
    let col_size = src.len() / row_size;
    for row in 0..row_size {
        for col in 0..col_size {
            let i = col * row_size + row;
            let j = row * col_size + col;
            dst[j] = src[i].clone();
        }
    }
}

pub fn transpose<T: Clone>(src: &[T], dst: &mut [T], row_size: usize) {
    assert_eq!(src.len(), dst.len());
    if src.len() == 0 || row_size == 0 {
        return;
    }
    assert_eq!(src.len() % row_size, 0);
    let col_size = src.len() / row_size;
    transpose_rec(src, dst, row_size, 0, row_size, 0, col_size);
}

/// In place matrix transpose.
///
/// Uses a temporary copy when `matrix` is not square.
pub fn transpose_inplace<T: Clone>(matrix: &mut [T], row_size: usize) {
    if matrix.is_empty() || row_size == 1 || row_size == matrix.len() {
        return;
    }
    debug_assert_eq!(matrix.len() % row_size, 0);
    if matrix.len() == row_size * row_size {
        transpose_inplace_rec(matrix, row_size, 0, row_size, 0, row_size);
    } else {
        // TODO: Figure out cache-oblivious in-place algorithm
        let temp = matrix.to_vec();
        crate::transpose::transpose(&temp, matrix, row_size);
    }
}

fn transpose_inplace_rec<T: Sized + Clone>(
    matrix: &mut [T],
    row_size: usize,
    row_start: usize,
    row_end: usize,
    col_start: usize,
    col_end: usize,
) {
    // Base case size
    // TODO: Figure out why size_of::<T> can not be stored in const
    // TODO: Make const when <https://github.com/rust-lang/rust/issues/49146> lands
    let base = if cfg!(test) {
        // Small in tests for better coverage of the recursive case.
        16
    } else {
        // Size base such that the sub-matrix fits in L1
        L1_CACHE_SIZE / size_of::<T>()
    };

    debug_assert!(row_end >= row_start);
    debug_assert!(col_end >= col_start);
    let row_span = row_end - row_start;
    let col_span = col_end - col_start;
    debug_assert!(row_span >= 1);
    debug_assert!(col_span >= 1);
    if row_span * col_span <= base {
        for row in row_start..row_end {
            for col in col_start..col_end {
                let i = col * row_size + row;
                let j = row * row_size + col;
                if i < j {
                    // TODO: Don't filter, just generated better indices
                    matrix.swap(i, j);
                }
            }
        }
    } else {
        // Divide along longest axis
        if row_span >= col_span {
            let row_mid = row_start + (row_span / 2);
            transpose_inplace_rec(matrix, row_size, row_start, row_mid, col_start, col_end);
            transpose_inplace_rec(matrix, row_size, row_mid, row_end, col_start, col_end);
        } else {
            let col_mid = col_start + (col_span / 2);
            transpose_inplace_rec(matrix, row_size, row_start, row_end, col_start, col_mid);
            transpose_inplace_rec(matrix, row_size, row_start, row_end, col_mid, col_end);
        }
    }
}

fn transpose_rec<T: Sized + Clone>(
    src: &[T],
    dst: &mut [T],
    row_size: usize,
    row_start: usize,
    row_end: usize,
    col_start: usize,
    col_end: usize,
) {
    // Base case size
    // TODO: Figure out why size_of::<T> can not be stored in const
    // TODO: Make const when <https://github.com/rust-lang/rust/issues/49146> lands
    let base = if cfg!(test) {
        // Small in tests for better coverage of the recursive case.
        16
    } else {
        // Size base such that src and dst sub-matrices fit in L1
        L1_CACHE_SIZE / (2 * size_of::<T>())
    };

    debug_assert!(row_end >= row_start);
    debug_assert!(col_end >= col_start);
    let col_size = src.len() / row_size;
    let row_span = row_end - row_start;
    let col_span = col_end - col_start;
    debug_assert!(row_span >= 1);
    debug_assert!(col_span >= 1);
    if row_span * col_span <= base {
        for row in row_start..row_end {
            for col in col_start..col_end {
                let i = col * row_size + row;
                let j = row * col_size + col;
                dst[j] = src[i].clone();
            }
        }
    } else {
        // Divide along longest axis
        if row_span >= col_span {
            let row_mid = row_start + (row_span / 2);
            transpose_rec(src, dst, row_size, row_start, row_mid, col_start, col_end);
            transpose_rec(src, dst, row_size, row_mid, row_end, col_start, col_end);
        } else {
            let col_mid = col_start + (col_span / 2);
            transpose_rec(src, dst, row_size, row_start, row_end, col_start, col_mid);
            transpose_rec(src, dst, row_size, row_start, row_end, col_mid, col_end);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// Generate arbitrary u32 matrices
    fn arb_matrix_sized(rows: usize, cols: usize) -> impl Strategy<Value = (Vec<u32>, usize)> {
        (
            Just((0..rows * cols).map(|i| i as u32).collect()),
            Just(rows),
        )
    }

    fn arb_matrix() -> impl Strategy<Value = (Vec<u32>, usize)> {
        (0_usize..=100, 0_usize..=100).prop_flat_map(|(rows, cols)| arb_matrix_sized(rows, cols))
    }

    fn arb_square_matrix() -> impl Strategy<Value = (Vec<u32>, usize)> {
        (0_usize..=100).prop_flat_map(|n| arb_matrix_sized(n, n))
    }

    proptest! {

        /// Reference transpose is it's own inverse
        #[test]
        fn reference_inverse((orig, row_size) in arb_matrix()) {
            let col_size = if row_size == 0 { 0 } else { orig.len() / row_size };
            let mut transposed_1 = orig.clone();
            let mut transposed_2 = orig.clone();
            reference(&orig, &mut transposed_1, row_size);
            reference(&transposed_1, &mut transposed_2, col_size);
            prop_assert_eq!(orig, transposed_2);
        }

        /// Transpose matches reference
        #[test]
        fn compare_reference((orig, row_size) in arb_matrix()) {
            let mut result = orig.clone();
            let mut expected = orig.clone();
            reference(&orig, &mut expected, row_size);
            transpose(&orig, &mut result, row_size);
            prop_assert_eq!(result, expected);
        }

        /// Transpose inplace matches reference
        #[test]
        fn inplace_compare_reference((orig, row_size) in arb_square_matrix()) {
            let mut result = orig.clone();
            let mut expected = orig.clone();
            reference(&orig, &mut expected, row_size);
            transpose_inplace(&mut result, row_size);
            prop_assert_eq!(result, expected);
        }
    }
}
