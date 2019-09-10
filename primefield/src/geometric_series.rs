use crate::FieldElement;
use std::prelude::v1::*;

#[derive(Clone, PartialEq, Eq)]
pub struct GeometricSeries {
    current: FieldElement,
    step:    FieldElement,
    length:  usize,
}

impl GeometricSeries {
    pub fn at(&self, index: usize) -> FieldElement {
        &self.current * self.step.pow(index)
    }

    /// Transform the series
    pub fn step_by(mut self, step: usize) -> GeometricSeries {
        assert!(step > 0);
        self.step = self.step.pow(step);
        self.length /= step;
        self
    }
}

impl Iterator for GeometricSeries {
    type Item = FieldElement;

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

pub fn geometric_series(base: &FieldElement, step: &FieldElement) -> GeometricSeries {
    GeometricSeries {
        current: base.clone(),
        step:    step.clone(),
        length:  usize::max_value(),
    }
}

pub fn root_series(order: usize) -> GeometricSeries {
    let root = FieldElement::root(order).expect("No root found of given order.");
    GeometricSeries {
        current: FieldElement::ONE,
        step:    root,
        length:  order,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use macros_decl::field_element;
    use u256::U256;

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
