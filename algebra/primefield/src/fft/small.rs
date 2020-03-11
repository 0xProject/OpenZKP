use crate::{FieldLike, RefFieldLike};

// OPT: Unchecked slice access
// OPT: Inplace +- operation like in gcd::mat_mul.
// OPT: Use Dev's combined REDC

/// Transforms (x0, x1) to (x0 + x1, x0 - x1)
#[inline(always)]
pub fn radix_2<Field>(values: &mut [Field], offset: usize, stride: usize)
where
    Field: FieldLike,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    let i = offset;
    let j = offset + stride;
    let temp = values[i].clone();
    values[i] = &temp + &values[j];
    values[j] = &temp - &values[j];
}

/// Transforms (x0, x1) to (x0 + x1, x0 - x1)
#[inline(always)]
pub fn radix_2_twiddle<Field>(values: &mut [Field], twiddle: &Field, offset: usize, stride: usize)
where
    Field: FieldLike,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    let i = offset;
    let j = offset + stride;
    let temp = values[i].clone();
    values[j] *= twiddle;
    values[i] = &temp + &values[j];
    values[j] = &temp - &values[j];
}

#[inline(always)]
pub fn radix_4<Field>(values: &mut [Field], twiddles: &[Field], offset: usize, stride: usize)
where
    Field: FieldLike,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    radix_2(values, offset, 2 * stride);
    radix_2(values, offset + stride, 2 * stride);
    values[offset + 3 * stride] *= &twiddles[1];
    radix_2(values, offset, stride);
    radix_2(values, offset + 2 * stride, stride);
}

#[inline(always)]
pub fn radix_8<Field>(values: &mut [Field], twiddles: &[Field], offset: usize, stride: usize)
where
    Field: FieldLike,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    radix_4(values, twiddles, offset, 2 * stride);
    radix_4(values, twiddles, offset + stride, 2 * stride);
    values[offset + 3 * stride] *= &twiddles[1];
    values[offset + 5 * stride] *= &twiddles[2];
    values[offset + 7 * stride] *= &twiddles[3];
    radix_2(values, offset, stride);
    radix_2(values, offset + 2 * stride, stride);
    radix_2(values, offset + 4 * stride, stride);
    radix_2(values, offset + 6 * stride, stride);
}

#[cfg(test)]
mod tests {
    use super::{
        super::{
            get_twiddles,
            tests::{arb_vec_size, ref_fft_permuted},
        },
        *,
    };
    use crate::{FieldElement, Root};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_radix_2(values in arb_vec_size(2)) {
            let mut expected = values.clone();
            ref_fft_permuted(&mut expected);
            let mut result =  values;
            radix_2(&mut result, 0, 1);
            prop_assert_eq!(result, expected);
        }

        #[test]
        fn test_radix_4(values in arb_vec_size(4)) {
            let mut expected = values.clone();
            ref_fft_permuted(&mut expected);
            let mut result =  values;
            let root = FieldElement::root(4).unwrap();
            radix_4(&mut result, &get_twiddles(&root, 4), 0, 1);
            prop_assert_eq!(result, expected);
        }

        #[test]
        fn test_radix_8(values in arb_vec_size(8)) {
            let mut expected = values.clone();
            ref_fft_permuted(&mut expected);
            let mut result =  values;
            let root = FieldElement::root(8).unwrap();
            radix_8(&mut result, &get_twiddles(&root, 8), 0, 1);
            prop_assert_eq!(result, expected);
        }
    }
}
