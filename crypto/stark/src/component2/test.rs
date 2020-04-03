use super::{Component, PolyWriter};
use crate::{RationalExpression, TraceTable};
use zkp_primefield::{FieldElement, Root};

/// Test constraint system
///
/// Construct a sequence using the recurrence relation:
///     x[0]   = seed        (part of constraints)
///     x[1]   = witness     (not part of constraints)
///     x[i+2] = x[i] * x[i + 1] + claim
///
/// This sequence is then layed out in row-first order
/// across the trace dimension and constraints are produced
/// to match
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Test {
    rows:    usize,
    columns: usize,
    seed:    FieldElement,
}

impl Test {
    pub fn new(rows: usize, columns: usize, seed: &FieldElement) -> Test {
        let seed = seed.clone();
        Test {
            rows,
            columns,
            seed,
        }
    }
}

impl Component for Test {
    type Claim = FieldElement;
    type Witness = (FieldElement, FieldElement);

    fn claim(&self, witness: &Self::Witness) -> Self::Claim {
        witness.0.clone()
    }

    fn dimensions2(&self) -> (usize, usize) {
        (self.columns, self.rows)
    }

    fn constraints(&self, claim: &Self::Claim) -> Vec<RationalExpression> {
        use RationalExpression::*;

        // Construct the constraint system for the sequence.
        let rows = self.rows;
        let columns = self.columns;
        let seed = Constant(self.seed.clone());
        let omega = Constant(FieldElement::root(rows).unwrap());
        let mut constraints = Vec::new();
        // x[0] = seed
        if rows * columns >= 1 {
            constraints.push((Trace(0, 0) - seed) / (X - omega.pow(0)));
        }
        if rows * columns >= 3 {
            // For each column we need to add a constraint
            for i in 0..columns {
                // Find the previous two cells in the table
                let (x0, x1) = match (i, columns) {
                    (0, 1) => (Trace(0, -2), Trace(0, -1)),
                    (0, _) => (Trace(columns - 2, -1), Trace(columns - 1, -1)),
                    (1, _) => (Trace(columns - 1, -1), Trace(0, 0)),
                    (..) => (Trace(i - 2, 0), Trace(i - 1, 0)),
                };
                // Exempt the first two cells
                let exceptions = match (i, columns) {
                    (0, 1) => (X - omega.pow(0)) * (X - omega.pow(1)),
                    (0, _) | (1, _) => (X - omega.pow(0)),
                    (..) => 1.into(),
                };
                // x[i + 2] = x[i] * x[i + 1] + claim
                constraints.push(
                    (Trace(i, 0) - x0 * x1 - claim.into()) * exceptions / (X.pow(rows) - 1.into()),
                )
            }
        }
        constraints
    }

    fn trace2<P: PolyWriter>(&self, trace: &mut P, witness: &Self::Witness) {
        debug_assert_eq!(trace.dimensions(), self.dimensions2());

        // Generator for the sequence
        let mut x0 = self.seed.clone();
        let mut x1 = witness.1.clone();
        let mut next = || {
            let result = x0.clone();
            let x2 = &x0 * &x1 + witness.0.clone();
            x0 = x1.clone();
            x1 = x2;
            result
        };

        // Fill in the trace table with the sequence
        // the sequence is written left-to-right, then top-to-bottom.
        for i in 0..(self.rows * self.columns) {
            let polynomial = i % self.columns;
            let location = i / self.columns;
            trace.write(polynomial, location, next());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_check() {
        proptest!(|(
            log_rows in 0_usize..10,
            cols in 0_usize..10,
            seed: FieldElement,
            claim: FieldElement,
            witness: FieldElement
        )| {
            let rows = 1 << log_rows;
            let witness = (claim.clone(), witness);
            let component = Test::new(rows, cols, &seed);
            prop_assert_eq!(component.check(&witness), Ok(()));
        });
    }

    #[test]
    fn test_proof_verify() {
        let config = ProptestConfig::with_cases(10);
        proptest!(config, |(
            log_rows in 1_usize..10,
            cols in 1_usize..10,
            seed: FieldElement,
            claim: FieldElement,
            witness: FieldElement
        )| {
            let rows = 1 << log_rows;
            let witness = (claim.clone(), witness);
            let component = Test::new(rows, cols, &seed);
            let proof = component.prove(&witness).unwrap();
            let result = component.verify(&claim, &proof);
            prop_assert_eq!(result, Ok(()));
        });
    }
}
