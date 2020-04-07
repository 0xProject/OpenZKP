use super::{Component, Mapped, PolynomialWriter};
use crate::RationalExpression;

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Horizontal<Left, Right>
where
    Left: Component,
    Right: Component,
{
    left:  Left,
    right: Right,
}

impl<Left, Right> Horizontal<Left, Right>
where
    Left: Component,
    Right: Component,
{
    pub fn new(left: Left, right: Right) -> Self {
        Horizontal { left, right }
    }

    pub fn left(&self) -> &Left {
        &self.left
    }

    pub fn right(&self) -> &Right {
        &self.right
    }
}

impl<Left, Right> Component for Horizontal<Left, Right>
where
    Left: Component,
    Right: Component,
{
    type Claim = (<Left as Component>::Claim, <Right as Component>::Claim);
    type Witness = (<Left as Component>::Witness, <Right as Component>::Witness);

    fn num_polynomials(&self) -> usize {
        self.left().num_polynomials() + self.right().num_polynomials()
    }

    fn polynomial_size(&self) -> usize {
        let left = self.left().polynomial_size();
        let right = self.right().polynomial_size();
        assert_eq!(left, right);
        left
    }

    fn claim(&self, witness: &Self::Witness) -> Self::Claim {
        (self.left.claim(&witness.0), self.right.claim(&witness.1))
    }

    fn constraints(&self, claim: &Self::Claim) -> Vec<RationalExpression> {
        use RationalExpression::*;
        let left_polynomials = self.left().num_polynomials();
        let left = self.left().constraints(&claim.0);
        let right = self.right().constraints(&claim.1);
        let right = right
            .into_iter()
            .map(|expression| {
                expression.map(&|node| {
                    match node {
                        Trace(i, j) => Trace(i + left_polynomials, j),
                        other => other,
                    }
                })
            })
            .collect::<Vec<RationalExpression>>();
        let mut result = Vec::new();
        result.extend(left.into_iter());
        result.extend(right.into_iter());
        result
    }

    fn trace<P: PolynomialWriter>(&self, trace: &mut P, witness: &Self::Witness) {
        let mut left_trace = Mapped::new(
            trace,
            self.left.num_polynomials(),
            self.left.polynomial_size(),
            |polynomial, location| (polynomial, location),
        );
        self.left.trace(&mut left_trace, &witness.0);
        let left_polys = self.left.num_polynomials();
        let mut right_trace = Mapped::new(
            trace,
            self.right.num_polynomials(),
            self.right.polynomial_size(),
            |polynomial, location| (polynomial + left_polys, location),
        );
        self.right.trace(&mut right_trace, &witness.1)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::test::Test, *};
    use proptest::prelude::*;
    use zkp_primefield::FieldElement;

    fn component(
        rows: usize,
    ) -> impl Strategy<
        Value = (
            Test,
            <Test as Component>::Claim,
            <Test as Component>::Witness,
        ),
    > {
        (
            0_usize..10,
            any::<FieldElement>(),
            any::<FieldElement>(),
            any::<FieldElement>(),
        )
            .prop_map(move |(columns, seed, claim, witness)| {
                (
                    Test::new(rows, columns, &seed),
                    claim.clone(),
                    (claim, witness),
                )
            })
    }

    #[test]
    fn test_check() {
        // Generate two components with the same number of rows
        let components = (0_usize..10).prop_flat_map(|log_rows| {
            let rows = 1 << log_rows;
            (component(rows), component(rows))
        });
        proptest!(|(
            (a, b) in components
        )| {
            let component = Horizontal::new(a.0, b.0);
            let witness = (a.2, b.2);
            prop_assert_eq!(component.check(&witness), Ok(()));
        });
    }

    // Test `Horizontal::new(Horizontal::new(A, B), C) == Horizontal::new(A,
    // Horizontal::new(B, C))`
    #[test]
    fn test_associative() {
        // Generate three components with the same number of rows
        let components = (0_usize..10).prop_flat_map(|log_rows| {
            let rows = 1 << log_rows;
            (component(rows), component(rows), component(rows))
        });
        proptest!(|(
            (a, b, c) in components
        )| {
            let left = Horizontal::new(Horizontal::new(a.0.clone(), b.0.clone()), c.0.clone());
            let left_claim = ((a.1.clone(), b.1.clone()), c.1.clone());
            let left_witness = ((a.2.clone(), b.2.clone()), c.2.clone());
            let right = Horizontal::new(a.0, Horizontal::new(b.0, c.0));
            let right_claim = (a.1, (b.1, c.1));
            let right_witness = (a.2, (b.2, c.2));
            for (result, expected) in left.constraints(&left_claim).iter()
                .zip(right.constraints(&right_claim).iter()) {
                prop_assert!(result.equals(expected));
            }
            prop_assert_eq!(left.trace_table(&left_witness), right.trace_table(&right_witness));
        });
    }
}
