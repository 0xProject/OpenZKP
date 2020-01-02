// We want these functions to be called `invert_batch`
#![allow(clippy::module_name_repetitions)]
// Many false positives from trait bounds
#![allow(single_use_lifetimes)]
use crate::{FieldLike, Inv, RefFieldLike};
use std::prelude::v1::*;

pub fn invert_batch_src_dst<Field>(source: &[Field], destination: &mut [Field])
where
    Field: FieldLike + From<usize> + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    assert_eq!(source.len(), destination.len());
    let mut accumulator = Field::one();
    for (src, dst) in source.iter().zip(destination.iter_mut()) {
        *dst = accumulator.clone();
        accumulator *= &*src;
    }
    accumulator = accumulator.inv().expect("Division by zero in batch invert");
    for (src, dst) in source.iter().zip(destination.iter_mut()).rev() {
        *dst *= &accumulator;
        accumulator *= &*src;
    }
    // OPT: We can avoid the last multiplication
}

pub fn invert_batch<Field>(source: &[Field]) -> Vec<Field>
where
    Field: FieldLike + From<usize> + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    let mut result = vec![Field::zero(); source.len()];
    invert_batch_src_dst(source, &mut result);
    result
}

// Quickcheck needs pass by value
#[allow(clippy::needless_pass_by_value)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FieldElement, Zero};
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn test_batch_inv(x: Vec<FieldElement>) -> bool {
        if x.iter().any(FieldElement::is_zero) {
            true
        } else {
            invert_batch(x.as_slice())
                .iter()
                .zip(x.iter())
                .all(|(a_inv, a)| *a_inv == a.inv().unwrap())
        }
    }
}
