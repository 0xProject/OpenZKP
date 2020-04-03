use super::Component;
use crate::{RationalExpression, TraceTable};

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Vertical<Element>
where
    Element: Component,
{
    element: Element,
    size:    usize,
}

// TODO: Validate that element constraint systems are compatible.
impl<Element> Vertical<Element>
where
    Element: Component,
{
    pub fn new(element: Element, size: usize) -> Self {
        assert!(size.is_power_of_two());
        Vertical { element, size }
    }

    pub fn element(&self) -> &Element {
        &self.element
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<Element> Component for Vertical<Element>
where
    Element: Component,
{
    // TODO: Avoid `Vec<_>`, maybe `IntoIter<_>`?
    type Claim = Vec<Element::Claim>;
    type Witness = Vec<Element::Witness>;

    fn dimensions(&self) -> (usize, usize) {
        let (rows, columns) = self.element.dimensions();
        (self.size * rows, columns)
    }

    // Note: Element can not have constraints depend on the claim!
    // TODO: Vectorize the claim? Encode claim in a lookup polynomial?
    fn constraints(&self, claim: &Self::Claim) -> Vec<RationalExpression> {
        use RationalExpression::*;
        self.element
            // TODO: Avoid `unwrap`
            .constraints(claim.first().unwrap())
            .into_iter()
            .map(|expression| {
                expression.map(&|node| {
                    match node {
                        X => X.pow(self.size),
                        other => other,
                    }
                })
            })
            .collect::<Vec<_>>()
    }

    fn trace(&self, claim: &Self::Claim, witness: &Self::Witness) -> TraceTable {
        assert_eq!(claim.len(), self.size);
        assert_eq!(witness.len(), self.size);
        let (element_rows, columns) = self.element.dimensions();
        let rows = element_rows * self.size;
        let mut trace = TraceTable::new(rows, columns);
        claim
            .iter()
            .zip(witness.iter())
            .map(|(claim, witness)| self.element.trace(claim, witness))
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
    use proptest::{collection::vec, prelude::*};
    use zkp_primefield::FieldElement;

    #[test]
    fn test_check() {
        let witness =
            (0_usize..5).prop_flat_map(|log_size| vec(any::<FieldElement>(), 1 << log_size));
        proptest!(|(
            log_rows in 0_usize..5,
            cols in 0_usize..10,
            seed: FieldElement,
            claim: FieldElement,
            witness in witness,
        )| {
            let size = witness.len();
            let element_rows = 1 << log_rows;
            let element = Test::new(element_rows, cols, &seed);
            let component = Vertical::new(element, size);
            let claim = vec![claim; size];
            prop_assert_eq!(component.check(&claim, &witness), Ok(()));
        });
    }

    // Test `Vertical::new(A, 1) == A`
    #[test]
    fn test_one() {
        proptest!(|(
            log_rows in 0_usize..5,
            cols in 0_usize..10,
            seed: FieldElement,
            claim: FieldElement,
            witness: FieldElement,
        )| {
            let element_rows = 1 << log_rows;
            let element = Test::new(element_rows, cols, &seed);
            let component = Vertical::new(element.clone(), 1);
            let claim_vec = vec![claim.clone(); 1];
            let witness_vec = vec![witness.clone(); 1];
            for (result, expected) in component.constraints(&claim_vec).iter()
                .zip(element.constraints(&claim).iter()) {
                // We expect extrinsic equality, but not intrinsic.
                prop_assert!(result.equals(expected));
            }
            prop_assert_eq!(component.trace(&claim_vec, &witness_vec), element.trace(&claim, &witness));
        });
    }

    // Test `Vertical::new(Vertical::new(A, n), m) == Vertical::new(A, n * m)`
    #[test]
    fn test_compose() {
        let witness = (0_usize..4, 0_usize..4).prop_flat_map(|(log_inner_size, log_outer_size)| {
            vec(
                vec(any::<FieldElement>(), 1 << log_inner_size),
                1 << log_outer_size,
            )
        });
        proptest!(|(
            log_rows in 0_usize..5,
            cols in 0_usize..10,
            seed: FieldElement,
            claim: FieldElement,
            witness in witness,
        )| {
            let outer_size = witness.len();
            let inner_size = witness.first().unwrap().len();
            // dbg!(outer_size, inner_size);
            let element_rows = 1 << log_rows;
            let element = Test::new(element_rows, cols, &seed);
            let inner = Vertical::new(element.clone(), inner_size);
            let outer = Vertical::new(inner, outer_size);
            let component = Vertical::new(element, outer_size * inner_size);
            let claim_vec = vec![claim.clone(); outer_size * inner_size];
            let witness_vec = witness.iter().flatten().cloned().collect::<Vec<_>>();
            let claim = vec![vec![claim; inner_size]; outer_size];
            for (result, expected) in outer.constraints(&claim).iter()
                .zip(component.constraints(&claim_vec).iter()) {
                prop_assert!(result.equals(expected));
            }
            prop_assert_eq!(outer.trace(&claim, &witness), component.trace(&claim_vec, &witness_vec));
        });
    }
}
