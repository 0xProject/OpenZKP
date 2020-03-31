use super::Component;
use crate::{RationalExpression, TraceTable};
use zkp_primefield::{FieldElement, Root};

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

    fn dimensions(&self) -> (usize, usize) {
        let left = self.left().dimensions();
        let right = self.right().dimensions();
        assert_eq!(left.0, right.0);
        (left.0, left.1 + right.1)
    }

    fn constraints(&self, claim: &Self::Claim) -> Vec<RationalExpression> {
        use RationalExpression::*;
        let (_rows, left_columns) = self.left().dimensions();
        let left = self.left().constraints(&claim.0);
        let right = self.right().constraints(&claim.1);
        let right = right
            .into_iter()
            .map(|expression| {
                expression.map(&|node| {
                    match node {
                        Trace(i, j) => Trace(i + left_columns, j),
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

    fn trace(&self, claim: &Self::Claim, witness: &Self::Witness) -> TraceTable {
        let left = self.left().trace(&claim.0, &witness.0);
        let right = self.right().trace(&claim.1, &witness.1);
        assert_eq!(left.num_rows(), right.num_rows());
        let mut trace = TraceTable::new(left.num_rows(), left.num_columns() + right.num_columns());
        for i in 0..trace.num_rows() {
            for j in 0..left.num_columns() {
                trace[(i, j)] = left[(i, j)].clone();
            }
            let shift = left.num_columns();
            for j in 0..right.num_columns() {
                trace[(i, j + shift)] = left[(i, j)].clone();
            }
        }
        trace
    }
}

#[cfg(test)]
mod tests {
    use super::{super::test::Test, *};
    use proptest::prelude::*;
    use zkp_u256::U256;

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
                (Test::new(rows, columns, &seed), claim, witness)
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
            let claim = (a.1, b.1);
            let witness = (a.2, b.2);
            prop_assert_eq!(component.check(&claim, &witness), Ok(()));
        });
    }

    // TODO: Test `Horizontal::new(A, Empty) == Horizontal::new(Empty, A) == A`

    // Horizontal::new(B, C))`
}
