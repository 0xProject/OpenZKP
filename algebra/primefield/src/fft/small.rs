use crate::{FieldLike, RefFieldLike};

// OPT: Inplace +- operation like in gcd::mat_mul.
// OPT: Use Dev's combined REDC

/// Transforms (x0, x1) to (x0 + x1, x0 - x1)
#[inline(always)]
pub fn radix_2<Field>(values: &mut [Field], offset: usize, stride: usize)
where
    Field: FieldLike + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    let i = offset;
    let j = offset + stride;
    let temp = values[i].clone();
    values[i] = &temp + &values[j];
    values[j] = temp - &values[j];
}

#[inline(always)]
pub fn radix_4<Field>(values: &mut [Field], twiddles: &[Field], offset: usize, stride: usize)
where
    Field: FieldLike + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    radix_2(values, 0, 2);
    radix_2(values, 1, 2);
    // OPT: Unchecked access
    values[offset + 3 * stride] *= &twiddles[1];
    radix_2(values, 0, 1);
    radix_2(values, 2, 1);
}

#[inline(always)]
pub fn radix_8<Field>(values: &mut [Field], twiddles: &[Field], offset: usize, stride: usize)
where
    Field: FieldLike + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    radix_4(values, twiddles, 0, 2);
    radix_4(values, twiddles, 1, 2);
    values[offset + 3 * stride] *= &twiddles[1];
    values[offset + 5 * stride] *= &twiddles[2];
    values[offset + 7 * stride] *= &twiddles[3];
    radix_2(values, 0, 1);
    radix_2(values, 2, 1);
    radix_2(values, 4, 1);
    radix_2(values, 6, 1);
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
    use crate::{traits::Root, FieldElement};
    use proptest::prelude::*;
    use zkp_macros_decl::field_element;
    use zkp_u256::U256;

    proptest! {
        #[test]
        fn test_radix_2(values in arb_vec_size(2)) {
            let mut expected = values.clone();
            ref_fft_permuted(&mut expected);
            let mut result =  values.clone();
            radix_2(&mut result, 0, 1);
            prop_assert_eq!(result, expected);
        }

        #[test]
        fn test_radix_4(values in arb_vec_size(4)) {
            let mut expected = values.clone();
            ref_fft_permuted(&mut expected);
            let mut result =  values.clone();
            radix_4(&mut result, &get_twiddles(4), 0, 1);
            prop_assert_eq!(result, expected);
        }

        #[test]
        fn test_radix_8(values in arb_vec_size(8)) {
            let mut expected = values.clone();
            ref_fft_permuted(&mut expected);
            let mut result =  values.clone();

            radix_8(&mut result, &get_twiddles(8), 0, 1);
            prop_assert_eq!(result, expected);
        }
    }
}
