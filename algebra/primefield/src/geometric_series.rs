// False positives, see <https://github.com/rust-lang/rust/issues/55058>
#![allow(single_use_lifetimes)]

// False positive: attribute has a use
#[allow(clippy::useless_attribute)]
// False positive: Importing preludes is allowed
#[allow(clippy::wildcard_imports)]
use std::prelude::v1::*;

use crate::{FieldLike, Pow, RefFieldLike};
use std::cmp::min;

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct GeometricIter<Field>
where
    Field: FieldLike,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    current: Field,
    step:    Field,
    length:  usize,
}

impl<Field> GeometricIter<Field>
where
    Field: FieldLike,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    pub fn at(&self, index: usize) -> Field {
        &self.current * self.step.pow(index)
    }

    pub fn skip(mut self, n: usize) -> Self {
        self.current *= self.step.pow(n);
        self.length -= n;
        self
    }

    pub fn take(mut self, n: usize) -> Self {
        self.length = min(self.length, n);
        self
    }

    /// Transform the series
    pub fn step_by(mut self, step: usize) -> Self {
        assert!(step > 0);
        self.step = self.step.pow(step);
        self.length /= step;
        self
    }
}

impl<Field> Iterator for GeometricIter<Field>
where
    Field: FieldLike,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    type Item = Field;

    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            None
        } else {
            // OPT: Make clone free and return reference.
            let item = self.current.clone();
            self.current *= &self.step;
            self.length -= 1;
            Some(item)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.length, Some(self.length))
    }
}

// TODO: Implement multiplication for GeometricSeries x GeometricSeries and
// GeometricSeries x FieldElement.

pub fn geometric_series<Field>(base: &Field, step: &Field) -> GeometricIter<Field>
where
    Field: FieldLike,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    GeometricIter {
        current: base.clone(),
        step:    step.clone(),
        length:  usize::max_value(),
    }
}

pub fn root_series<Field>(order: usize) -> GeometricIter<Field>
where
    Field: FieldLike,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    let root = Field::root(order).expect("No root found of given order.");
    geometric_series(&Field::one(), &root).take(order)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FieldElement;
    use zkp_macros_decl::field_element;
    use zkp_u256::U256;

    #[test]
    fn geometric_series_test() {
        let base =
            field_element!("0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f");
        let step =
            field_element!("00000000000000000000000000000000000000000000000f00dbabe0cafebabe");

        let mut domain = geometric_series(&base, &step);
        assert_eq!(domain.next(), Some(base.clone()));
        assert_eq!(domain.next(), Some(&base * &step));
        assert_eq!(domain.next(), Some(&base * &step * &step));
        assert_eq!(domain.next(), Some(&base * &step * &step * &step));
    }
}
