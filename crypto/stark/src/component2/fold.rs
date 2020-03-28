use super::Component;
use crate::{RationalExpression, TraceTable};
use itertools::izip;
use std::cmp::min;
use zkp_primefield::{fft::permute_index, FieldElement, Root};

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
        assert_ne!(1_usize << folds, 0);
        Fold { element, folds }
    }

    pub fn element(&self) -> &Element {
        &self.element
    }

    /// Map from element coordinated to self coordinates
    pub fn map_up(&self, row: usize, column: usize) -> (usize, usize) {
        let reduction = 1 << self.folds;
        let column_folded = permute_index(reduction, column % reduction);
        let column = column / reduction;
        let row = row * reduction + column_folded;
        (row, column)
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
        use RationalExpression::*;
        let reduction = 1 << self.folds;
        self.element
            .constraints(claim)
            .into_iter()
            .map(|expression| {
                expression.map(&|node| {
                    match node {
                        Trace(column, row_offset) => {
                            let column_offset = permute_index(reduction, column);
                            Trace(
                                column / reduction,
                                (reduction as isize) * row_offset + (column_offset as isize),
                            )
                        }
                        other => other,
                    }
                })
            })
            .collect::<Vec<_>>()
    }

    fn trace(&self, claim: &Self::Claim, witness: &Self::Witness) -> TraceTable {
        let element_trace = self.element.trace(claim, witness);
        let (rows, columns) = self.dimensions();
        let mut trace = TraceTable::new(rows, columns);
        for i in 0..element_trace.num_rows() {
            for j in 0..element_trace.num_columns() {
                trace[self.map_up(i, j)] = element_trace[(i, j)].clone();
            }
        }
        trace
    }
}

fn ceil_div(numerator: usize, denominator: usize) -> usize {
    assert!(denominator > 0);
    (numerator.wrapping_sub(1) / denominator).wrapping_add(1)
}

#[cfg(test)]
mod tests {
    use super::{super::test::Test, *};
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

    #[test]
    fn test_check() {
        proptest!(|(
            log_rows in 0_usize..10,
            cols in 0_usize..20,
            folds in 0_usize..5,
            seed: FieldElement,
            claim: FieldElement,
            witness: FieldElement
        )| {
            let rows = 1 << log_rows;
            let element = Test::new(rows, cols, &seed);
            let component = Fold::new(element, folds);
            prop_assert_eq!(component.check(&claim, &witness), Ok(()));
        });
    }

    // TODO: Test `Fold::new(A, 0) == A`
    // TODO: Test `Fold::new(Fold::new(A, m), n) == Fold::new(A, m + n)`
}
