use super::{
    bit_reverse::permute,
    small::{radix_2, radix_2_twiddle, radix_4, radix_8},
    PrefetchIndex,
};
use crate::{FieldLike, Pow, RefFieldLike};
use std::prelude::v1::*;

// TODO: Radix-4 recursion?

/// Radix-2 depth-first in-place bit-reversed FFT.
pub fn fft_recursive<Field>(values: &mut [Field])
where
    Field: FieldLike + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    let size = values.len();
    debug_assert!(size.is_power_of_two());
    let root = Field::root(values.len()).expect("No root exists");
    let mut twiddles = (0..size / 2).map(|i| root.pow(i)).collect::<Vec<_>>();
    permute(&mut twiddles);
    fft_vec_recursive(values, &twiddles, 0, 1, 1);
}

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
    Field: FieldLike + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    const PREFETCH_STRIDE: usize = 4;
    // Target loop size
    // Use smaller base case during tests to force better coverage of recursion.
    // TODO: Make const when <https://github.com/rust-lang/rust/issues/49146> lands
    let max_loop: usize = if cfg!(test) { 8 } else { 128 };
    let size = values.len() / stride;
    debug_assert!(size.is_power_of_two());
    debug_assert!(offset < stride);
    debug_assert_eq!(values.len() % size, 0);
    match size {
        1 => {}
        // Special casing for small sizes doesn't seem to give an advantage.
        _ => {
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
                if PREFETCH_STRIDE > 0 {
                    if i + PREFETCH_STRIDE < offset + count {
                        values.prefetch_index_write(i + PREFETCH_STRIDE);
                        values.prefetch_index_write(i + PREFETCH_STRIDE + stride);
                    } else {
                        values.prefetch_index_write(i + PREFETCH_STRIDE - count + 2 * stride);
                        values.prefetch_index_write(i + PREFETCH_STRIDE - count + 3 * stride);
                    }
                }
                radix_2(values, i, stride);
            }
            for (offset, twiddle) in (offset..offset + size * stride)
                .step_by(2 * stride)
                .zip(twiddles)
                .skip(1)
            {
                for i in offset..offset + count {
                    if PREFETCH_STRIDE > 0 {
                        if i + PREFETCH_STRIDE < offset + count {
                            values.prefetch_index_write(i + PREFETCH_STRIDE);
                            values.prefetch_index_write(i + PREFETCH_STRIDE + stride);
                        } else {
                            values.prefetch_index_write(i + PREFETCH_STRIDE - count + 2 * stride);
                            values.prefetch_index_write(i + PREFETCH_STRIDE - count + 3 * stride);
                        }
                    }
                    radix_2_twiddle(values, twiddle, i, stride)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::tests::{arb_vec, ref_fft_permuted},
        *,
    };
    use proptest::prelude::*;

    proptest! {

        #[test]
        fn fft_rec_ref(orig in arb_vec()) {
            let mut reference = orig.clone();
            let mut result = orig;
            ref_fft_permuted(&mut reference);
            fft_recursive(&mut result);
            prop_assert_eq!(result, reference);
        }
    }
}
