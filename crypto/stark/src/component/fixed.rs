use crate::{constraint_check::check_constraints, Constraints, RationalExpression, TraceTable};
use std::collections::HashMap;
use zkp_primefield::{FieldElement, Pow, Root};

use super::{index_rotate, traits::Component};

#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct FixedComponent {
    // TODO: Make private
    pub trace:       TraceTable,
    pub constraints: Vec<RationalExpression>,
    pub labels:      HashMap<String, (usize, RationalExpression)>,
}

impl Component for FixedComponent {
    type Claim = ();
    type Witness = ();

    fn trace(
        &self,
        claim: &Self::Claim,
        witness: &Self::Witness,
    ) -> (
        Vec<RationalExpression>,
        HashMap<String, (usize, RationalExpression)>,
        TraceTable,
    ) {
        (
            (self.trace.num_rows(), self.trace.num_columns()),
            self.constraints.clone(),
            self.labels.clone(),
        )
    }
}

impl FixedComponent {
    /// Constructs an empty component of given size.
    ///
    /// This is useful in combination with composition combinators to pad out a
    /// component to a required size.
    pub fn empty(rows: usize, columns: usize) -> Self {
        Self {
            trace:       TraceTable::new(rows, columns),
            constraints: Vec::new(),
            labels:      HashMap::new(),
        }
    }

    pub fn check(&self) -> bool {
        let constraints = Constraints::from_expressions(
            (self.trace.num_rows(), self.trace.num_columns()),
            Vec::new(),
            self.constraints.clone(),
        )
        .unwrap();
        check_constraints(&constraints, &self.trace).is_ok()
    }

    pub fn generator(&self) -> FieldElement {
        FieldElement::root(self.trace.num_rows()).expect("no generator for trace length")
    }

    // TODO: Generic eval for given X that interpolates the columns

    pub fn eval_row(&self, expression: &RationalExpression, row: usize) -> FieldElement {
        assert!(row < self.trace.num_rows());
        let x = self.generator().pow(row);
        expression.evaluate(&x, &|col, offset| {
            self.trace[(index_rotate(self.trace.num_rows(), row, offset), col)].clone()
        })
    }

    pub fn eval_label(&self, label: &str) -> FieldElement {
        let (row, expression) = &self.labels[label];
        self.eval_row(expression, *row)
    }

    pub fn rename_label(&mut self, old: &str, new: &str) {
        if let Some(value) = self.labels.remove(old) {
            let _ = self.labels.insert(new.to_string(), value);
        } else {
            panic!("Label '{}' not found", old);
        }
    }

    pub fn remove_label(&mut self, label: &str) {
        if self.labels.remove(label).is_none() {
            panic!("Label '{}' not found", label);
        }
    }

    pub fn project_into(
        &self,
        target: &mut Self,
        trace_map: impl Fn(usize, usize) -> (usize, usize),
        expr_map: impl Fn(RationalExpression) -> RationalExpression,
    ) {
        // Copy over TraceTable
        for i in 0..self.trace.num_rows() {
            for j in 0..self.trace.num_columns() {
                target.trace[trace_map(i, j)] = self.trace[(i, j)].clone();
            }
        }
        // Copy over Constraints
        target.constraints.extend(
            self.constraints
                .iter()
                .map(|expr| expr.clone().map(&expr_map)),
        );
        // Copy over Labels
        // TODO: Rename colliding labels?
        // TODO: Row numbers?
        target.labels.extend(
            self.labels
                .iter()
                .map(|(label, (row, expr))| (label.clone(), (*row, expr.map(&expr_map)))),
        )
    }
}

#[cfg(feature = "test")]
impl FixedComponent {
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
