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
    // OPT: Inplace +- operation like in gcd::mat_mul.
    // OPT: Use Dev's combined REDC

    let (left, right) = values.split_at_mut(offset + stride);
    let t = left[offset].clone();
    left[offset] += &right[0];
    // OPT: sub_from_assign
    right[0] -= t;
    right[0].neg_assign();
}

// See https://math.stackexchange.com/questions/1626897/whats-the-formulation-of-n-point-radix-n-for-ntt/1627247
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
pub fn radix_8<Field>(values: &mut [Field], offset: usize, stride: usize)
where
    Field: FieldLike + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    unimplemented!()
    // let omega = Field::root(4).expect("No root of order 4 found");
    // radix_4(values, 0, 2);
    // radix_4(values, 1, 2);
    // values[offset + 3 * stride] *= omega;
    // radix_2(values, 0, 1);
    // radix_2(values, 2, 1);
    // radix_2(values, 2, 1);
    // radix_2(values, 2, 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FieldElement;
    use zkp_macros_decl::field_element;
    use zkp_u256::U256;

    #[test]
    fn test_radix_2() {
        let mut x = [
            field_element!("0234287dcbaffe7f969c748655fca9e58fa8120b6d56eb0c1080d17957ebe47b"),
            field_element!("06c81c707ecc44b5f60297ec08d2d585513c1ba022dd93af66a1dbacb162a3f3"),
        ];
        radix_2(&mut x, 0, 1);
        assert_eq!(x, [
            field_element!("00fc44ee4a7c43248c9f0c725ecf7f6ae0e42dab90347ebb7722ad26094e886d"),
            field_element!("036c0c0d4ce3b9daa099dc9a4d29d4603e6bf66b4a79575ca9def5cca6894089")
        ]);
    }
}
