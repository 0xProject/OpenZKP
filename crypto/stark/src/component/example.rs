use super::traits::Component;
use crate::{RationalExpression, TraceTable};
use std::collections::HashMap;
use zkp_primefield::{FieldElement, Root};

struct Example {
    rows:    usize,
    columns: usize,
    seed:    FieldElement,
}

impl Component for Example {
    type Claim = FieldElement;
    type Witness = FieldElement;

    fn constraints(
        &self,
        claim: &Self::Claim,
    ) -> (
        (usize, usize),
        Vec<RationalExpression>,
        HashMap<String, (usize, RationalExpression)>,
    ) {
        use RationalExpression::*;

        // Construct the constraint system for the sequence.
        let omega = Constant(FieldElement::root(self.rows).unwrap());
        let mut constraints = Vec::new();
        let mut labels = HashMap::new();
        // x[0] = start
        if self.rows * self.columns >= 1 {
            constraints.push((Trace(0, 0) - (&self.seed).into()) / (X - omega.pow(0)));
            let _ = labels.insert("start".to_owned(), (0, Trace(0, 0)));
        }
        if self.rows * self.columns >= 3 {
            let _ = labels.insert(
                "final".to_owned(),
                (self.rows - 1, Trace(self.columns - 1, 0)),
            );
            let _ = labels.insert(
                "next".to_owned(),
                (
                    self.rows - 1,
                    if self.columns == 1 {
                        Trace(0, -1) * Trace(0, 0) + claim.into()
                    } else {
                        Trace(self.columns - 2, 0) * Trace(self.columns - 1, 0) + claim.into()
                    },
                ),
            );
            // For each column we need to add a constraint
            for i in 0..self.columns {
                // Find the previous two cells in the table
                let (x0, x1) = match (i, self.columns) {
                    (0, 1) => (Trace(0, -2), Trace(0, -1)),
                    (0, _) => (Trace(self.columns - 2, -1), Trace(self.columns - 1, -1)),
                    (1, _) => (Trace(self.columns - 1, -1), Trace(0, 0)),
                    (..) => (Trace(i - 2, 0), Trace(i - 1, 0)),
                };
                // Exempt the first two cells
                let exceptions = match (i, self.columns) {
                    (0, 1) => (X - omega.pow(0)) * (X - omega.pow(1)),
                    (0, _) | (1, _) => (X - omega.pow(0)),
                    (..) => 1.into(),
                };
                // x[i+2] = x[i] * x[i + 1] + offset
                constraints.push(
                    (Trace(i, 0) - x0 * x1 - claim.into()) * exceptions
                        / (X.pow(self.rows) - 1.into()),
                )
            }
        }

        ((self.rows, self.columns), constraints, labels)
    }

    fn trace(&self, claim: &Self::Claim, witness: &Self::Witness) -> TraceTable {
        // Construct a sequence using the quadratic recurrence relation:
        //     x[0]   = seed           (part of constraints)
        //     x[1]   = witness        (not part of constraints)
        //     x[i+2] = x[i] * x[i + 1] + claim
        let mut x0 = self.seed.clone();
        let mut x1 = witness.clone();
        let mut next = || {
            let result = x0.clone();
            let x2 = &x0 * &x1 + claim;
            x0 = x1.clone();
            x1 = x2;
            result
        };

        // Fill in the trace table with the sequence
        // the sequence is written left-to-right, then top-to-bottom.
        let mut trace = TraceTable::new(self.rows, self.columns);
        for i in 0..(self.rows * self.columns) {
            trace[(i / self.columns, i % self.columns)] = next();
        }
        trace
    }
}
