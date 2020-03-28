use super::Component;
use crate::{RationalExpression, TraceTable};
use itertools::izip;
use zkp_primefield::{FieldElement, Root};

pub struct Vertical<Element>
where
    Element: Component,
{
    elements: Vec<Element>,
}

// TODO: Validate that element constraint systems are compatible.
impl<Element> Vertical<Element>
where
    Element: Component,
{
    pub fn new(elements: Vec<Element>) -> Self {
        assert!(elements.len().is_power_of_two());
        // TODO: Validate that all constraint systems have the same dimensions
        // and the same constraints.
        Vertical { elements }
    }

    pub fn elements(&self) -> &[Element] {
        &self.elements
    }

    // TODO: Implement Index
    pub fn element(&self, index: usize) -> &Element {
        &self.elements[0]
    }
}

impl<Element> Component for Vertical<Element>
where
    Element: Component,
{
    // TODO: Avoid `Vec<_>`
    type Claim = Vec<Element::Claim>;
    type Witness = Vec<Element::Witness>;

    fn dimensions(&self) -> (usize, usize) {
        if let Some(first) = self.elements.first() {
            let (rows, columns) = first.dimensions();
            (self.elements.len() * rows, columns)
        } else {
            (0, 0)
        }
    }

    fn constraints(&self, claim: &Self::Claim) -> Vec<RationalExpression> {
        assert_eq!(claim.len(), self.elements.len());
        let size = self.elements.len();
        if let Some(first) = self.elements.first() {
            first
                .constraints(&claim[0])
                .into_iter()
                .map(|expression| {
                    expression.map(&|node| {
                        match node {
                            X => X.pow(size),
                            other => other,
                        }
                    })
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    }

    fn trace(&self, claim: &Self::Claim, witness: &Self::Witness) -> TraceTable {
        assert_eq!(claim.len(), self.elements.len());
        assert_eq!(witness.len(), self.elements.len());
        if self.elements.is_empty() {
            return TraceTable::new(0, 0);
        }
        let (rows, columns) = self.dimensions();
        let element_rows = rows / self.elements.len();
        let mut trace = TraceTable::new(rows, columns);
        izip!(self.elements().iter(), claim.iter(), witness.iter())
            .map(|(element, claim, witness)| element.trace(claim, witness))
            .enumerate()
            .for_each(|(i, element_trace)| {
                assert_eq!(element_trace.num_rows(), element_rows);
                assert_eq!(element_trace.num_columns(), columns);
                let start = i * element_rows;
                for i in 0..element_rows {
                    for j in 0..columns {
                        trace[(start + i, j)] = element_trace[(i, j)].clone();
                    }
                }
            });
        trace
    }
}

#[cfg(test)]
mod tests {
    use super::{super::test::Test, *};
    use proptest::prelude::*;
    use zkp_u256::U256;

    #[test]
    fn test_check() {
        proptest!(|(
            log_rows in 0_usize..10,
            cols in 0_usize..10,
            seed: FieldElement,
            claim: FieldElement,
            witness: Vec<FieldElement>
        )| {
            // TODO: This different column sizes
            let rows = 1 << log_rows;
            let left = Test::new(rows, cols, &seed);
            let right = Test::new(rows, cols, &seed);
            let component = Horizontal::new(left, right);
            let claim = (claim.clone(), claim.clone());
            let witness = (witness.clone(), witness.clone());
            prop_assert_eq!(component.check(&claim, &witness), Ok(()));
        });
    }
}
