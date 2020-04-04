use super::prefetch::PrefetchIndex;
use log::trace;

// TODO: Outer cache-oblivious layer for mmap-backed.
// TODO: Parallel transpose.

/// Transpose a square matrix of stretches.
///
/// The matrix is composed of `size` x `size` stretches of length `stretch`.
///
/// `stretch` can only be `1` or `2`.
pub fn transpose_square_stretch<T>(matrix: &mut [T], size: usize, stretch: usize) {
    trace!(
        "Transposing {} x {} square matrix of {} stretches",
        size,
        size,
        stretch
    );
    trace!("BEGIN Transpose");
    assert_eq!(matrix.len(), size * size * stretch);
    match stretch {
        1 => transpose_square_1(matrix, size),
        2 => transpose_square_2(matrix, size),
        _ => unimplemented!("Only stretch sizes 1 and 2 are supported"),
    }
    trace!("END Transpose");
}

// TODO: Handle odd sizes
fn transpose_square_1<T>(matrix: &mut [T], size: usize) {
    const PREFETCH_STRIDE: usize = 4;
    debug_assert_eq!(matrix.len(), size * size);
    if size % 2 != 0 {
        unimplemented!("Odd sizes are not supported");
    }

    // Iterate over upper-left triangle, working in 2x2 blocks
    // Stretches of two are useful because they span a 64B cache line when T is 32
    // bytes.
    for row in (0..size).step_by(2) {
        let i = row * size + row;
        matrix.swap(i + 1, i + size);
        for col in (row..size).step_by(2).skip(1) {
            let i = row * size + col;
            let j = col * size + row;
            if PREFETCH_STRIDE > 0 && col + PREFETCH_STRIDE * 2 < size {
                matrix.prefetch_index_write(i + PREFETCH_STRIDE * 2);
                matrix.prefetch_index_write(i + PREFETCH_STRIDE * 2 + size);
                matrix.prefetch_index_write(j + PREFETCH_STRIDE * 2 * size);
                matrix.prefetch_index_write(j + PREFETCH_STRIDE * 2 * size + size);
            }
            matrix.swap(i, j);
            matrix.swap(i + 1, j + size);
            matrix.swap(i + size, j + 1);
            matrix.swap(i + size + 1, j + size + 1);
        }
    }
}

fn transpose_square_2<T>(matrix: &mut [T], size: usize) {
    const PREFETCH_STRIDE: usize = 8;
    debug_assert_eq!(matrix.len(), 2 * size * size);

    // Iterate over upper-left triangle, working in 1x2 blocks
    for row in 0..size {
        for col in (row..size).skip(1) {
            let i = (row * size + col) * 2;
            let j = (col * size + row) * 2;
            if PREFETCH_STRIDE > 0 && col + PREFETCH_STRIDE < size {
                // Hardware prefetcher picks up on the first one.
                // matrix.prefetch_index_write(i + PREFETCH_STRIDE * 2);
                matrix.prefetch_index_write(i + PREFETCH_STRIDE * 2 * size);
            }
            matrix.swap(i, j);
            matrix.swap(i + 1, j + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn reference(matrix: &[u32], size: usize, stretch: usize) -> Vec<u32> {
        assert_eq!(matrix.len(), size * size * stretch);
        let mut result = matrix.to_vec();
        for i in 0..size {
            for j in 0..size {
                for k in 0..stretch {
                    let a = (i * size + j) * stretch + k;
                    let b = (j * size + i) * stretch + k;
                    result[b] = matrix[a];
                }
            }
        }
        result
    }

    fn arb_matrix_sized(size: usize, stretch: usize) -> impl Strategy<Value = Vec<u32>> {
        #[allow(clippy::cast_possible_truncation)]
        Just((0..size * size * stretch).map(|i| i as u32).collect())
    }

    fn arb_matrix() -> impl Strategy<Value = (Vec<u32>, usize, usize)> {
        (0_usize..=100, 1_usize..=2).prop_flat_map(|(size, stretch)| {
            (arb_matrix_sized(size, stretch), Just(size), Just(stretch))
        })
    }

    proptest! {

        /// Reference transpose is its own inverse
        #[test]
        fn reference_involutory((orig, size, stretch) in arb_matrix()) {
            let transposed = reference(&orig, size, stretch);
            let result = reference(&transposed, size ,stretch);
            prop_assert_eq!(result, orig);
        }

        /// Transpose matches reference
        #[test]
        fn compare_reference((mut matrix, size, stretch) in arb_matrix()) {
            prop_assume!(stretch == 2 || size % 2 == 0);
            let expected = reference(&matrix, size, stretch);
            transpose_square_stretch(&mut matrix, size, stretch);
            prop_assert_eq!(matrix, expected);
        }
    }
}
