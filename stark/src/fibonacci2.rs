use crate::{
    constraint_system::ConstraintSystem, constraints::Constraints,
    rational_expression::RationalExpression, trace_table::TraceTable,
};
use primefield::FieldElement;

pub struct Fibonacci;

impl Fibonacci {
    const NUM_COLUMNS: usize = 2;
    const TRACE_LENGTH: usize = 1024;

    fn generator() -> FieldElement {
        FieldElement::root(Self::TRACE_LENGTH.into()).expect("No root of unity for trace degree.")
    }
}

impl ConstraintSystem for Fibonacci {
    type Private = FieldElement;
    type Public = (usize, FieldElement);

    fn constraints((index, value): &Self::Public) -> Constraints {
        use RationalExpression::*;
        let index = *index;
        assert!(index < Self::TRACE_LENGTH);

        // Constraint repetitions
        let g = Constant(Self::generator());
        let first_row = RationalExpression::from(1) / (X - 1.into());
        let target_row = RationalExpression::from(1) / (X - g.pow(index));
        let every_row = (X - g.pow(Self::TRACE_LENGTH)) / (X.pow(Self::TRACE_LENGTH) - 1.into());

        // The system
        Constraints {
            trace_degree: Self::TRACE_LENGTH,
            num_columns:  Self::NUM_COLUMNS,
            constraints:  vec![
                (Trace(0, 0) - 1.into()) * first_row,
                (Trace(1, 0) - value.into()) * target_row,
                (Trace(0, 1) - Trace(1, 0)) * every_row.clone(),
                (Trace(1, 1) - Trace(0, 0) - Trace(1, 0)) * every_row,
            ],
        }
    }

    fn trace((index, value): &Self::Public, secret: &FieldElement) -> TraceTable {
        assert!(*index < Self::TRACE_LENGTH);

        // Compute trace table
        let mut trace = TraceTable::new(Self::TRACE_LENGTH, Self::NUM_COLUMNS);
        trace[(0, 0)] = 1.into();
        trace[(0, 1)] = secret.clone();
        for i in 0..(Self::TRACE_LENGTH - 1) {
            trace[(i + 1, 0)] = trace[(i, 1)].clone();
            trace[(i + 1, 1)] = &trace[(i, 0)] + &trace[(i, 1)];
        }

        // Verify claim
        assert_eq!(trace[(*index, 0)], *value);
        trace
    }
}
