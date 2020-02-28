/// Cache-oblivious transposition
use std::mem::swap;

// http://fftw.org/fftw-paper-ieee.pdf
// https://wgropp.cs.illinois.edu/courses/cs598-s16/lectures/lecture08.pdf

// See: <https://cacs.usc.edu/education/cs653/Frigo-CacheOblivious-FOCS99.pdf>

pub fn transpose_base<T: Clone>(src: &[T], dst: &mut [T], row_size: usize) {
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
    debug_assert_eq!(src.len() % row_size, 0);
    let col_size = src.len() / row_size;
    transpose_rec(src, dst, row_size, 0, row_size, 0, col_size);
}

// TODO: Is there an in-place version of this algorithm?
fn transpose_rec<T: Clone>(
    src: &[T],
    dst: &mut [T],
    row_size: usize,
    row_start: usize,
    row_end: usize,
    col_start: usize,
    col_end: usize,
) {
    // Base case size
    // smaller in tests for better coverage of the recursive case.
    #[cfg(test)]
    const BASE: usize = 8;
    #[cfg(not(test))]
    const BASE: usize = 1024;

    debug_assert!(row_end >= row_start);
    debug_assert!(col_end >= col_start);
    let col_size = src.len() / row_size;
    let row_span = row_end - row_start;
    let col_span = col_end - col_start;
    debug_assert!(row_span >= 1);
    debug_assert!(col_span >= 1);
    if row_span <= BASE && col_span <= BASE {
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

    /// Reference implementation of transposition
    fn transpose_ref<T: Clone>(src: &[T], dst: &mut [T], row_size: usize) {
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

    proptest! {

        #[test]
        /// Reference transpose is it's own inverse
        fn test_ref_inv((orig, row_size) in arb_matrix()) {
            let col_size = if row_size == 0 { 0 } else { orig.len() / row_size };
            let mut transposed_1 = orig.clone();
            let mut transposed_2 = orig.clone();
            transpose_ref(&orig, &mut transposed_1, row_size);
            transpose_ref(&transposed_1, &mut transposed_2, col_size);
            prop_assert_eq!(orig, transposed_2);
        }

        #[test]
        /// Transpose matches reference
        fn test_ref((orig, row_size) in arb_matrix()) {
            let mut result = orig.clone();
            let mut reference = orig.clone();
            transpose_ref(&orig, &mut reference, row_size);
            transpose(&orig, &mut result, row_size);
            prop_assert_eq!(result, reference);
        }

    }
}
