use super::{
    bit_reverse::permute_index, depth_first::depth_first_recurse, get_twiddles,
    iterative::fft_permuted_root, transpose::transpose_inplace,
};
use crate::{FieldLike, Pow, RefFieldLike};
use log::trace;
use rayon::prelude::*;
use std::cmp::max;

/// In-place FFT with permuted output.
///
/// Implement's the four step FFT in a cache-oblivious manner.
///
/// There is also a six-step version that outputs the result in normal order,
/// for this see <http://wwwa.pikara.ne.jp/okojisan/otfft-en/sixstepfft.html>.
// TODO: Bit-reversed order
pub(super) fn radix_sqrt<Field>(values: &mut [Field], root: &Field)
where
    Field: FieldLike + std::fmt::Debug + From<usize> + Send + Sync,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    // Recurse by splitting along the square root
    let length = values.len();
    let outer = 1_usize << (length.trailing_zeros() / 2);
    let inner = length / outer;
    debug_assert!(outer == inner || inner == 2 * outer);
    debug_assert_eq!(outer * inner, length);
    let _inner_root = root.pow(outer);
    let _outer_root = root.pow(inner);
    let twiddles = get_twiddles(max(outer, inner));
    parallel_recurse_inplace_permuted(
        values,
        root,
        outer,
        inner,
        |row| depth_first_recurse(row, &twiddles, 0, 1),
        |row| depth_first_recurse(row, &twiddles, 0, 1),
    );
}

/// Generic recursive six-point FFT.
fn recurse_inplace_inorder<Field, F, G>(
    values: &mut [Field],
    root: &Field,
    outer: usize,
    inner: usize,
    inner_fft: F,
    outer_fft: G,
) where
    Field: FieldLike,
    for<'a> &'a Field: RefFieldLike<Field>,
    F: Fn(&mut [Field]),
    G: Fn(&mut [Field]),
{
    let length = values.len();
    debug_assert!(root.pow(length).is_one());
    debug_assert_eq!(outer * inner, length);

    // 1 Transpose inner * outer sized matrix
    transpose_inplace(values, outer);

    // 2 Apply inner FFTs continguously
    // 3 Apply twiddle factors
    values
        .chunks_exact_mut(inner)
        .enumerate()
        .for_each(|(j, row)| {
            inner_fft(row);
            if j > 0 {
                let outer_twiddle = root.pow(j);
                let mut inner_twiddle = outer_twiddle.clone();
                for x in row.iter_mut().skip(1) {
                    *x *= &inner_twiddle;
                    inner_twiddle *= &outer_twiddle;
                }
            }
        });

    // 4 Transpose outer * inner sized matrix
    transpose_inplace(values, inner);

    // 5 Apply outer FFTs contiguously
    values
        .chunks_exact_mut(outer)
        .for_each(|row| outer_fft(row));

    // 6 Transpose back to get results in output order
    transpose_inplace(values, outer);
}

/// Generic parallel recursive six-point FFT.
fn parallel_recurse_inplace_inorder<Field, F, G>(
    values: &mut [Field],
    root: &Field,
    outer: usize,
    inner: usize,
    inner_fft: F,
    outer_fft: G,
) where
    Field: FieldLike + Send + Sync,
    for<'a> &'a Field: RefFieldLike<Field>,
    F: Fn(&mut [Field]) + Sync,
    G: Fn(&mut [Field]) + Sync,
{
    let length = values.len();
    debug_assert!(root.pow(length).is_one());
    debug_assert_eq!(outer * inner, length);

    // 1 Transpose inner * outer sized matrix
    transpose_inplace(values, outer);

    // 2 Apply inner FFTs continguously
    // 3 Apply twiddle factors
    trace!("Inner FFTs  {} times size {} (with twiddles)", outer, inner);
    values
        .par_chunks_mut(inner)
        .enumerate()
        .for_each(|(j, row)| {
            inner_fft(row);
            if j > 0 {
                let outer_twiddle = root.pow(j);
                let mut inner_twiddle = outer_twiddle.clone();
                for x in row.iter_mut().skip(1) {
                    *x *= &inner_twiddle;
                    inner_twiddle *= &outer_twiddle;
                }
            }
        });

    // 4 Transpose outer * inner sized matrix
    transpose_inplace(values, inner);

    // 5 Apply outer FFTs contiguously
    trace!("Outer FFTs  {} times size {}", outer, inner);
    values.par_chunks_mut(outer).for_each(|row| outer_fft(row));

    // 6 Transpose back to get results in output order
    transpose_inplace(values, outer);
}

/// Generic recursive six-point FFT with permuted output.
///
/// Advantages:
///  * The inner and outer FFT functions can be in permuted order
///  * Only two transpositions are required instead of three.
fn recurse_inplace_permuted<Field, F, G>(
    values: &mut [Field],
    root: &Field,
    outer: usize,
    inner: usize,
    inner_fft: F,
    outer_fft: G,
) where
    Field: FieldLike + Send + Sync,
    for<'a> &'a Field: RefFieldLike<Field>,
    F: Fn(&mut [Field]) + Sync,
    G: Fn(&mut [Field]) + Sync,
{
    let length = values.len();
    debug_assert!(root.pow(length).is_one());
    debug_assert_eq!(outer * inner, length);

    // 1 Transpose inner * outer sized matrix
    transpose_inplace(values, outer);

    // 2 Apply inner FFTs continguously
    // 3 Apply twiddle factors
    trace!("Inner FFTs  {} times size {} (with twiddles)", outer, inner);
    values
        .chunks_exact_mut(inner)
        .enumerate()
        .for_each(|(j, row)| {
            inner_fft(row);
            if j > 0 {
                let outer_twiddle = root.pow(j);
                for (i, x) in row.iter_mut().enumerate() {
                    // TODO: Precompute twiddles? At leas avoid the `pow` if we can...
                    let i = permute_index(inner, i);
                    let inner_twiddle = outer_twiddle.pow(i);
                    *x *= inner_twiddle;
                }
            }
        });

    // 4 Transpose outer * inner sized matrix
    transpose_inplace(values, inner);

    // 5 Apply outer FFTs contiguously
    trace!("Outer FFTs  {} times size {}", outer, inner);
    values
        .chunks_exact_mut(outer)
        .for_each(|row| outer_fft(row));
}

/// Generic recursive six-point FFT with permuted output.
///
/// Advantages:
///  * The inner and outer FFT functions can be in permuted order
///  * Only two transpositions are required instead of three.
fn parallel_recurse_inplace_permuted<Field, F, G>(
    values: &mut [Field],
    root: &Field,
    outer: usize,
    inner: usize,
    inner_fft: F,
    outer_fft: G,
) where
    Field: FieldLike + Send + Sync,
    for<'a> &'a Field: RefFieldLike<Field>,
    F: Fn(&mut [Field]) + Sync,
    G: Fn(&mut [Field]) + Sync,
{
    let length = values.len();
    debug_assert!(root.pow(length).is_one());
    debug_assert_eq!(outer * inner, length);

    // 1 Transpose inner * outer sized matrix
    transpose_inplace(values, outer);

    // 2 Apply inner FFTs continguously
    // 3 Apply twiddle factors
    trace!(
        "Parallel {} ⨉ inner FFT size {} (with twiddles)",
        outer,
        inner
    );
    values
        .par_chunks_mut(inner)
        .enumerate()
        .for_each(|(j, row)| {
            inner_fft(row);
            if j > 0 {
                let outer_twiddle = root.pow(j);
                let mut inner_twiddle = outer_twiddle.clone();
                for i in 1..inner {
                    let i = permute_index(inner, i);
                    row[i] *= &inner_twiddle;
                    inner_twiddle *= &outer_twiddle;
                }
            }
        });

    // 4 Transpose outer * inner sized matrix
    transpose_inplace(values, inner);

    // 5 Apply outer FFTs contiguously
    trace!("Parallel {} ⨉ outer FFT size {}", outer, inner);
    values.par_chunks_mut(outer).for_each(|row| outer_fft(row));
}

#[cfg(test)]
mod tests {
    use super::{
        super::tests::{arb_vec, ref_fft_inplace, ref_fft_permuted},
        *,
    };
    use crate::{FieldElement, Root};
    use proptest::prelude::*;
    use std::cmp::{max, min};

    proptest! {

        #[test]
        fn test_recurse_inplace_inorder(values in arb_vec()) {
            // TODO: Test different splittings
            const SPLIT: usize = 4;
            let mut expected = values.clone();
            ref_fft_inplace(&mut expected);
            let root = FieldElement::root(values.len()).unwrap();
            let mut result = values.clone();
            let inner = max(1, values.len() / SPLIT);
            let outer = min(values.len(), SPLIT);
            recurse_inplace_inorder(
                &mut result,
                &root,
                outer,
                inner,
                ref_fft_inplace,
                ref_fft_inplace,
            );
            prop_assert_eq!(result, expected);
        }

        #[test]
        fn test_recurse_inplace_permuted(orig in arb_vec()) {
            // TODO: Test different splittings
            const SPLIT: usize = 4;
            let mut expected = orig.clone();
            ref_fft_permuted(&mut expected);
            let root = FieldElement::root(orig.len()).unwrap();
            let mut result = orig.clone();
            let inner = max(1, orig.len() / SPLIT);
            let outer = min(orig.len(), SPLIT);
            recurse_inplace_permuted(
                &mut result,
                &root,
                outer,
                inner,
                ref_fft_permuted,
                ref_fft_permuted,
            );
            prop_assert_eq!(result, expected);
        }

        #[test]
        fn test_parallel_recurse_inplace_inorder(values in arb_vec()) {
            // TODO: Test different splittings
            const SPLIT: usize = 4;
            let mut expected = values.clone();
            ref_fft_inplace(&mut expected);
            let root = FieldElement::root(values.len()).unwrap();
            let mut result = values.clone();
            let inner = max(1, values.len() / SPLIT);
            let outer = min(values.len(), SPLIT);
            parallel_recurse_inplace_inorder(
                &mut result,
                &root,
                outer,
                inner,
                ref_fft_inplace,
                ref_fft_inplace,
            );
            prop_assert_eq!(result, expected);
        }
    }
}
