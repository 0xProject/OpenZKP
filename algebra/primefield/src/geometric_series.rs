use crate::{FieldElement, FieldLike, One, Pow, Root};
use std::prelude::v1::*;

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct GeometricIter<Field>
where
    Field: FieldLike,
    for<'a> &'a Field: Pow<usize, Output = Field>,
    for<'a> &'a Field: std::ops::Mul<Field, Output = Field>,
    for<'a, 'b> &'a Field: std::ops::Mul<&'b Field, Output = Field>,
{
    current: Field,
    step:    Field,
    length:  usize,
}

impl<Field> GeometricIter<Field>
where
    Field: FieldLike,
    for<'a> &'a Field: Pow<usize, Output = Field>,
    for<'a> &'a Field: std::ops::Mul<Field, Output = Field>,
    for<'a, 'b> &'a Field: std::ops::Mul<&'b Field, Output = Field>,
{
    pub fn at(&self, index: usize) -> Field {
        &self.current * self.step.pow(index)
    }

    pub fn skip(mut self, n: usize) -> Self {
        self.current *= self.step.pow(n);
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
    for<'a> &'a Field: Pow<usize, Output = Field>,
    for<'a> &'a Field: std::ops::Mul<Field, Output = Field>,
    for<'a, 'b> &'a Field: std::ops::Mul<&'b Field, Output = Field>,
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
    for<'a> &'a Field: Pow<usize, Output = Field>,
    for<'a> &'a Field: std::ops::Mul<Field, Output = Field>,
    for<'a, 'b> &'a Field: std::ops::Mul<&'b Field, Output = Field>,
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
    Field: Root<usize>,
    for<'a> &'a Field: Pow<usize, Output = Field>,
    for<'a> &'a Field: std::ops::Mul<Field, Output = Field>,
    for<'a, 'b> &'a Field: std::ops::Mul<&'b Field, Output = Field>,
{
    let root = Field::root(order).expect("No root found of given order.");
    geometric_series(&Field::one(), &root)
}

#[cfg(test)]
mod tests {
    use super::*;
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
