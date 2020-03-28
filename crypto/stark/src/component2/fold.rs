use super::Component;
use crate::{RationalExpression, TraceTable};
use itertools::izip;
use std::cmp::min;
use zkp_primefield::{FieldElement, Root};

/// Note: `Fold::new(Fold::new(A, m), n) == Fold::new(A, m + n)`
pub struct Fold<Element>
where
    Element: Component,
{
    element: Element,
    folds:   usize,
}

// TODO: Validate that element constraint systems are compatible.
impl<Element> Fold<Element>
where
    Element: Component,
{
    pub fn new(element: Element, folds: usize) -> Self {
        Fold { element, folds }
    }

    pub fn element(&self) -> &Element {
        &self.element
    }
}

impl<Element> Component for Fold<Element>
where
    Element: Component,
{
    type Claim = Element::Claim;
    type Witness = Element::Witness;

    fn dimensions(&self) -> (usize, usize) {
        let (rows, columns) = self.element.dimensions();
        (rows << self.folds, ceil_div(columns, 1 << self.folds))
    }

    fn constraints(&self, claim: &Self::Claim) -> Vec<RationalExpression> {
        unimplemented!()
    }

    fn trace(&self, claim: &Self::Claim, witness: &Self::Witness) -> TraceTable {
        unimplemented!()
    }
}

fn ceil_div(numerator: usize, denominator: usize) -> usize {
    assert!(denominator > 0);
    (numerator.wrapping_sub(1) / denominator).wrapping_add(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_ceil_div() {
        proptest!(|(numerator: usize, denominator:usize)| {
            prop_assume!(denominator > 0);
            let result = ceil_div(numerator, denominator);
            if result > 0 {
                prop_assert!((result - 1) * denominator < numerator);
                prop_assert!(numerator - (result - 1) * denominator < denominator);
            } else {
                prop_assert!(numerator == 0);
            }
        });
    }
}
