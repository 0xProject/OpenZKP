use super::Component;
use crate::{RationalExpression, TraceTable};
use std::collections::HashMap;
use zkp_primefield::{FieldElement, Root};

impl Component {
    /// Creates an example constraint system of given size
    ///
    /// It takes two seed values, one to make the constraints unique
    /// and one to make the witness unique.
    pub fn example(
        rows: usize,
        columns: usize,
        constraint_seed: &FieldElement,
        witness_seed: &FieldElement,
    ) -> Self {
        use RationalExpression::*;

        // Construct a sequence using the quadratic recurrence relation:
        //     x[0]   = constraint_seed     (part of constraints)
        //     x[1]   = witness_seed        (not part of constraints)
        //     x[i+2] = x[i] * x[i + 1] + constraint_seed
        let mut x0 = constraint_seed.clone();
        let mut x1 = witness_seed.clone();
        let mut next = || {
            let result = x0.clone();
            let x2 = &x0 * &x1 + constraint_seed;
            x0 = x1.clone();
            x1 = x2;
            result
        };

        // Fill in the trace table with the sequence
        // the sequence is written left-to-right, then top-to-bottom.
        let mut trace = TraceTable::new(rows, columns);
        for i in 0..(rows * columns) {
            trace[(i / columns, i % columns)] = next();
        }

        // Construct the constraint system for the sequence.
        let omega = Constant(FieldElement::root(rows).unwrap());
        let mut constraints = Vec::new();
        let mut labels = HashMap::new();
        // x[0] = start
        if rows * columns >= 1 {
            constraints.push((Trace(0, 0) - constraint_seed.into()) / (X - omega.pow(0)));
            let _ = labels.insert("start".to_owned(), (0, Trace(0, 0)));
        }
        if rows * columns >= 3 {
            let _ = labels.insert("final".to_owned(), (rows - 1, Trace(columns - 1, 0)));
            let _ = labels.insert(
                "next".to_owned(),
                (
                    rows - 1,
                    if columns == 1 {
                        Trace(0, -1) * Trace(0, 0) + constraint_seed.into()
                    } else {
                        Trace(columns - 2, 0) * Trace(columns - 1, 0) + constraint_seed.into()
                    },
                ),
            );
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
                // x[i+2] = x[i] * x[i + 1] + offset
                constraints.push(
                    (Trace(i, 0) - x0 * x1 - constraint_seed.into()) * exceptions
                        / (X.pow(rows) - 1.into()),
                )
            }
        }

        Self {
            trace,
            constraints,
            labels,
        }
    }
}
