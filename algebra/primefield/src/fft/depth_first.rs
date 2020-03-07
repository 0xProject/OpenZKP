use super::{bit_reverse::permute, small::radix_2};
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

fn depth_first_recurse<Field>(
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
        _ => {
            depth_first_recurse(values, twiddles, offset, stride * 2);
            depth_first_recurse(values, twiddles, offset + stride, stride * 2);

            for (i, twiddle) in (0..size).step_by(2).zip(twiddles) {
                // TODO: First twiddle is one
                // TODO: Use radix_2
                let i = offset + i * stride;
                let j = i + stride;
                // values[j] *= twiddle;
                // radix_2(values, offset + i * stride, stride);
                let a = values[i].clone();
                let b = twiddle * &values[j];
                values[i] = &a + &b;
                values[j] = a - b;
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
        fn fft_df_ref(orig in arb_vec()) {
            let mut reference = orig.clone();
            let mut result = orig.clone();
            ref_fft_permuted(&mut reference);
            fft_depth_first(&mut result);
            prop_assert_eq!(result, reference);
        }
    }
}
