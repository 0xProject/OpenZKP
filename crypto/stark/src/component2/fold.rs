use super::{Component, Mapped, PolyWriter};
use crate::RationalExpression;
use zkp_primefield::fft::permute_index;

/// Note: `Fold::new(Fold::new(A, m), n) == Fold::new(A, m + n)`
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
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

    fn claim(&self, witness: &Self::Witness) -> Self::Claim {
        self.element.claim(witness)
    }

    fn dimensions2(&self) -> (usize, usize) {
        let (polynomials, size) = self.element.dimensions2();
        let reduction = 1 << self.folds;
        (ceil_div(polynomials, reduction), size * reduction)
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
                            let column_offset = permute_index(reduction, column % reduction);
                            // Reductions should be small enough
                            #[allow(clippy::cast_possible_wrap)]
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

    fn trace2<P: PolyWriter>(&self, trace: &mut P, witness: &Self::Witness) {
        let reduction = 1 << self.folds;
        let mut trace = Mapped::new(trace, self.element.dimensions2(), |polynomial, location| {
            let polynomial_folded = permute_index(reduction, polynomial % reduction);
            let polynomial = polynomial / reduction;
            let location = location * reduction + polynomial_folded;
            (polynomial, location)
        });
        self.element.trace2(&mut trace, witness)
    }
}

fn ceil_div(numerator: usize, denominator: usize) -> usize {
    assert!(denominator > 0);
    if numerator == 0 {
        0
    } else {
        1 + (numerator - 1) / denominator
    }
}

#[cfg(test)]
mod tests {
    use super::{super::test::Test, *};
    use proptest::prelude::*;
    use zkp_primefield::FieldElement;

    #[test]
    fn test_ceil_div() {
        // ceil(0 / a) = 0
        proptest!(|(a in 1_usize..)| {
            prop_assert_eq!(ceil_div(0, a), 0);
        });

        proptest!(|(numerator in 1_usize.., denominator in 1_usize..)| {
            let result = ceil_div(numerator, denominator);
            let floored = numerator / denominator;
            let exact = numerator % denominator == 0;
            if exact {
                prop_assert_eq!(result, floored);
            } else {
                prop_assert_eq!(result, floored + 1);
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
            let witness = (claim.clone(), witness);
            let element = Test::new(rows, cols, &seed);
            let component = Fold::new(element, folds);
            prop_assert_eq!(component.check(&witness), Ok(()));
        });
    }

    // Test `Fold::new(A, 0) == A`
    #[test]
    fn test_zero() {
        proptest!(|(
            log_rows in 0_usize..10,
            cols in 0_usize..10,
            seed: FieldElement,
            claim: FieldElement,
            witness: FieldElement
        )| {
            let rows = 1 << log_rows;
            let witness = (claim.clone(), witness);
            let element = Test::new(rows, cols, &seed);
            let component = Fold::new(element.clone(), 0);
            prop_assert_eq!(component.constraints(&claim), element.constraints(&claim));
            prop_assert_eq!(component.trace_table(&witness), element.trace_table(&witness));
        });
    }

    // Test `Fold::new(Fold::new(A, m), n) == Fold::new(A, m + n)`
    #[test]
    fn test_compose() {
        proptest!(|(
            log_rows in 0_usize..10,
            cols in 0_usize..20,
            inner_folds in 0_usize..4,
            outer_folds in 0_usize..4,
            seed: FieldElement,
            claim: FieldElement,
            witness: FieldElement
        )| {
            let rows = 1 << log_rows;
            let witness = (claim.clone(), witness);
            let element = Test::new(rows, cols, &seed);
            let inner = Fold::new(element.clone(), inner_folds);
            let outer = Fold::new(inner, outer_folds);
            let combined = Fold::new(element, inner_folds + outer_folds);
            prop_assert_eq!(outer.constraints(&claim), combined.constraints(&claim));
            prop_assert_eq!(outer.trace_table(&witness), combined.trace_table(&witness));
        });
    }
}
