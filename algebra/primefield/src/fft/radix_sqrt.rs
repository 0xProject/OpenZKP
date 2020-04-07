use super::{
    bit_reverse::permute_index, get_twiddles, recursive::fft_vec_recursive,
    transpose::transpose_square_stretch,
};
use crate::{FieldLike, Pow, RefFieldLike};
use log::trace;
use rayon::prelude::*;

/// In-place FFT with permuted output.
///
/// Implement's the six step FFT. Output is permuted, which avoids the last
/// permutations step.
pub fn radix_sqrt<Field>(values: &mut [Field], root: &Field)
where
    Field: FieldLike + Send + Sync,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    trace!("BEGIN FFT Radix SQRT");
    trace!("Radix FFT of size {}", values.len());
    if values.len() <= 1 {
        return;
    }

    // Recurse by splitting along the square root
    // Round such that outer is larger.
    let length = values.len();
    let inner = 1_usize << (length.trailing_zeros() / 2);
    let outer = length / inner;
    let stretch = outer / inner;
    debug_assert!(root.pow(values.len()).is_one());
    debug_assert!(outer == inner || outer == 2 * inner);
    debug_assert_eq!(outer * inner, length);

    // Prepare twiddles
    let twiddles = get_twiddles(&root.pow(inner), outer);

    // 1. Transpose inner x inner x stretch square matrix
    transpose_square_stretch(values, inner, stretch);

    // 2. Apply inner FFTs contiguously
    // 2. Apply twiddle factors
    trace!("Parallel {} x inner FFT size {}", outer, inner);
    trace!("BEGIN FFT Batch Recursive");
    values
        .par_chunks_mut(outer)
        .for_each(|row| fft_vec_recursive(row, &twiddles, 0, stretch, stretch));
    trace!("END FFT Batch Recursive");

    // 4. Transpose inner x inner x stretch square matrix
    transpose_square_stretch(values, inner, stretch);

    // 5 Apply outer FFTs contiguously
    trace!(
        "Parallel {} x outer FFT size {} (with twiddles)",
        outer,
        inner
    );
    trace!("BEGIN FFT Batch Recursive twiddles");
    values
        .par_chunks_mut(outer)
        .enumerate()
        .for_each(|(i, row)| {
            if i > 0 {
                let i = permute_index(inner, i);
                let inner_twiddle = root.pow(i);
                let mut outer_twiddle = inner_twiddle.clone();
                for element in row.iter_mut().skip(1) {
                    *element *= &outer_twiddle;
                    outer_twiddle *= &inner_twiddle;
                }
            }
            fft_vec_recursive(row, &twiddles, 0, 1, 1)
        });
    trace!("END FFT Batch Recursive twiddles");
    trace!("END FFT Radix SQRT");
}

#[cfg(test)]
mod tests {
    use super::{
        super::tests::{arb_vec, ref_fft_permuted},
        *,
    };
    use crate::{FieldElement, Root};
    use proptest::prelude::*;

    proptest! {

        #[test]
        fn test_radix_sqrt(values in arb_vec()) {
            let mut expected = values.clone();
            let mut result = values.clone();
            let root = FieldElement::root(values.len()).unwrap();
            ref_fft_permuted(&mut expected);
            radix_sqrt(&mut result, &root);
            prop_assert_eq!(result, expected);
        }
    }
}
