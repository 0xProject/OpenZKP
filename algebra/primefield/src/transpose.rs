/// Cache-oblivious transposition
use std::mem::swap;

// http://fftw.org/fftw-paper-ieee.pdf
// https://wgropp.cs.illinois.edu/courses/cs598-s16/lectures/lecture08.pdf

pub fn transpose<T>(matrix: &mut [T], row_size: usize) {
    debug_assert_eq!(matrix.len() % row_size, 0);
    transpose_rec(matrix, row_size, 0, row_size, 0, 9);
}

fn transpose_rec<T>(
    matrix: &mut [T],
    row_size: usize,
    row_start: usize,
    row_end: usize,
    col_start: usize,
    col_end: usize,
) {
    const BASE: usize = 16;
    debug_assert!(row_end >= row_start);
    debug_assert!(col_end >= col_start);
    let col_size = matrix.len() / row_size;
    let row_span = row_end - row_start;
    let col_span = col_end - col_start;
    if row_span < BASE && col_span < BASE {
        // Base case
        for row in row_start..row_end {
            for col in col_start..col_end {
                let i = col * row_size + row;
                let j = row * col_size + col;
                // TODO: Don't generate filtered values
                if i < j {
                    matrix.swap(i, j);
                }
            }
        }
    } else {
        // Divide along longest axis
        if row_span >= col_span {
            let row_mid = row_span / 2;
            transpose_rec(matrix, row_size, row_start, row_mid, col_start, col_end);
            transpose_rec(matrix, row_size, row_mid, row_end, col_start, col_end);
        } else {
            let col_mid = col_span / 2;
            transpose_rec(matrix, row_size, row_start, row_end, col_start, col_mid);
            transpose_rec(matrix, row_size, row_start, row_end, col_mid, col_end);
        }
    }
}

#[cfg(test)]
mod tests {
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
    fn transpose_ref<T: Clone>(matrix: &mut [T], row_size: usize) {
        if matrix.len() == 0 || row_size == 0 {
            return;
        }
        debug_assert_eq!(matrix.len() % row_size, 0);

        // Compute out of place using temporary
        let mut result = matrix.to_vec();
        let col_size = matrix.len() / row_size;
        for row in 0..row_size {
            for col in 0..col_size {
                let i = col * row_size + row;
                let j = row * col_size + col;
                result[j] = matrix[i].clone();
            }
        }
        for i in 0..matrix.len() {
            matrix[i] = result[i].clone();
        }
    }

    proptest! {

        #[test]
        /// Transpose is it's own inverse
        fn test_ref_inv((mut m, row_size) in arb_matrix()) {
            let col_size = if row_size == 0 { 0 } else { m.len() / row_size };
            let orig = m.clone();
            transpose_ref(&mut m, row_size);
            transpose_ref(&mut m, col_size);
            prop_assert_eq!(orig, m);
        }

    }
}
