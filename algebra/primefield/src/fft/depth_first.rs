use super::{
    bit_reverse::permute,
    small::{radix_2, radix_2_twiddle, radix_4, radix_8},
};
use crate::{FieldLike, Pow, RefFieldLike};

/// Radix-2 depth-first in-place bit-reversed FFT.
// TODO: Radix-4?
pub fn fft_depth_first<Field>(values: &mut [Field])
where
    Field: FieldLike + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    let size = values.len();
    debug_assert!(size.is_power_of_two());
    let root = Field::root(values.len()).expect("No root exists");
    let mut twiddles = (0..size / 2).map(|i| root.pow(i)).collect::<Vec<_>>();
    permute(&mut twiddles);
    depth_first_recurse(values, &twiddles, 0, 1);
}

pub(crate) fn depth_first_recurse<Field>(
    values: &mut [Field],
    twiddles: &[Field],
    offset: usize,
    stride: usize,
) where
    Field: FieldLike + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    let size = values.len() / stride;
    debug_assert!(size.is_power_of_two());
    debug_assert!(offset < stride);
    debug_assert_eq!(values.len() % size, 0);
    match size {
        1 => {}
        2 => radix_2(values, offset, stride),
        4 => radix_4(values, twiddles, offset, stride),
        8 => radix_8(values, twiddles, offset, stride),
        _ => {
            depth_first_recurse(values, twiddles, offset, stride * 2);
            depth_first_recurse(values, twiddles, offset + stride, stride * 2);
            radix_2(values, offset, stride);
            (offset..offset + size * stride)
                .step_by(2 * stride)
                .zip(twiddles)
                .skip(1)
                .for_each(|(offset, twiddle)| radix_2_twiddle(values, twiddle, offset, stride));
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
        fn fft_df_ref(orig in arb_vec()) {
            let mut reference = orig.clone();
            let mut result = orig.clone();
            ref_fft_permuted(&mut reference);
            fft_depth_first(&mut result);
            prop_assert_eq!(result, reference);
        }
    }
}
