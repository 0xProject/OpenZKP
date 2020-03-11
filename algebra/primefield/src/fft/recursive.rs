use super::small::{radix_2, radix_2_twiddle};
use crate::{FieldLike, RefFieldLike};

// TODO: Radix-4 recursion?

/// Recursive vector-FFT.
///
/// Computes several parallel FFTs
pub fn fft_vec_recursive<Field>(
    values: &mut [Field],
    twiddles: &[Field],
    offset: usize,
    count: usize,
    stride: usize,
) where
    Field: FieldLike,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    // Target loop size
    // Use smaller base case during tests to force better coverage of recursion.
    // TODO: Make const when <https://github.com/rust-lang/rust/issues/49146> lands
    let max_loop: usize = if cfg!(test) { 8 } else { 128 };
    let size = values.len() / stride;
    debug_assert!(size.is_power_of_two());
    debug_assert!(offset < stride);
    debug_assert_eq!(values.len() % size, 0);
    // Special casing small radices doesn't seem to give and advantage.
    if size > 1 {
        // Inner FFT radix size/2
        if stride == count && count < max_loop {
            fft_vec_recursive(values, twiddles, offset, 2 * count, 2 * stride);
        } else {
            // TODO: We could do parallel recursion here, if we had a way to
            // do a strided split. (Like the ndarray package provides)
            fft_vec_recursive(values, twiddles, offset, count, 2 * stride);
            fft_vec_recursive(values, twiddles, offset + stride, count, 2 * stride);
        }

        // Outer FFT radix 2
        // Lookahead about 3
        for i in offset..offset + count {
            radix_2(values, i, stride);
        }
        for (offset, twiddle) in (offset..offset + size * stride)
            .step_by(2 * stride)
            .zip(twiddles)
            .skip(1)
        {
            for i in offset..offset + count {
                radix_2_twiddle(values, twiddle, i, stride)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{
            get_twiddles,
            tests::{arb_vec, ref_fft_permuted},
        },
        *,
    };
    use crate::{FieldElement, Root};
    use proptest::prelude::*;

    proptest! {

        #[test]
        fn test_reference(values in arb_vec()) {
            let mut expected = values.clone();
            let mut result = values.clone();
            let root = FieldElement::root(values.len()).unwrap();
            let twiddles = get_twiddles(&root, values.len());
            ref_fft_permuted(&mut expected);
            fft_vec_recursive(&mut result, &twiddles, 0, 1, 1);
            prop_assert_eq!(result, expected);
        }
    }
}
