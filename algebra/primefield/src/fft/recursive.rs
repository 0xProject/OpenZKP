use super::{
    bit_reverse::permute,
    small::{radix_2, radix_2_twiddle, radix_4, radix_8},
};
use crate::{FieldLike, Pow, RefFieldLike};
use std::prelude::v1::*;

// https://www.cs.waikato.ac.nz/~ihw/papers/13-AMB-IHW-MJC-FastFourier.pdf

/// Radix-2 depth-first in-place bit-reversed FFT.
// TODO: Radix-4?
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
    recurse(values, &twiddles, 0, 1, 1);
}

pub(crate) fn recurse<Field>(
    values: &mut [Field],
    twiddles: &[Field],
    offset: usize,
    count: usize,
    stride: usize,
) where
    Field: FieldLike + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    const MAX_LOOP: usize = 4;
    let size = values.len() / stride;
    debug_assert!(size.is_power_of_two());
    debug_assert!(offset < stride);
    debug_assert_eq!(values.len() % size, 0);
    match size {
        1 => {}
        // 2 => radix_2(values, offset, stride),
        // 4 => radix_4(values, twiddles, offset, stride),
        // 8 => radix_8(values, twiddles, offset, stride),
        _ => {
            // Inner FFT radix size/2
            if stride == count && count < MAX_LOOP {
                recurse(values, twiddles, offset, 2 * count, 2 * stride);
            } else {
                recurse(values, twiddles, offset, count, 2 * stride);
                recurse(values, twiddles, offset + stride, count, 2 * stride);
            }

            // Outer FFT radix 2
            for offset in offset..offset + count {
                radix_2(values, offset, stride);
            }
            for (offset, twiddle) in (offset..offset + size * stride)
                .step_by(2 * stride)
                .zip(twiddles)
                .skip(1)
            {
                for offset in offset..offset + count {
                    radix_2_twiddle(values, twiddle, offset, stride)
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
