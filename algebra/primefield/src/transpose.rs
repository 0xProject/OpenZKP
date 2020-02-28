/// Cache-oblivious transposition
use std::mem::swap;

// http://fftw.org/fftw-paper-ieee.pdf
// https://wgropp.cs.illinois.edu/courses/cs598-s16/lectures/lecture08.pdf

pub fn transpose<T>(matrix: &mut [T], row_size: usize) {
    debug_assert_eq!(matrix.len() % row_size, 0);
    transpose_rec(&mut matrix, row_size, 0, row_size);
}

fn transpose_rec<T>(matrix: &mut [T], n: usize, x: usize, xn: usize, y: usize, yn: usize) {
    const BASE: usize = 16;
    if xn < BASE && yn < BASE {
        // Base case
        for x in x..xn {
            for y in y..yn {
                let i = y * n + x;
                let j = x * n + y;
                if i < j {
                    // TODO swap(&mut matrix[i], &mut matrix[j]);
                }
            }
        }
    } else {
        // Divide along longest axis
        if xn >= yn {
            let xmid = xn / 2;
            transpose(matrix, n, x, xmid, y, yn);
            transpose(matrix, n, x + xmid, xn - xmid, y, yn);
        } else {
            let ymid = yn / 2;
            transpose(matrix, n, x, xn, y, ymid);
            transpose(matrix, n, x, xn, y + ymid, yn - ymid);
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest;

    fn arb_matrix(rows: usize, cols: usize) -> impl Strategy<Value = Vec<u32>> {
        proptest::collection::vec(u32::arb())
    }

    fn transpose_ref<T>(matrix: &mut [T], row_size: usize) {
        debug_assert_eq!(matrix.len() % row_size, 0);
        let col_size = matrix.len() / row_size;
        for i in 0..row_size {
            for j in 0..col_size {
                matrix.swap(i, j);
            }
        }
    }
}
