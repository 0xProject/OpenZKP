use crate::rational_expression::RationalExpression;
use std::prelude::v1::*;

pub struct Constraint {
    pub expr: RationalExpression,
}

pub(crate) fn trace_degree(constraints: &[Constraint]) -> usize {
    constraints
        .iter()
        .map(|c| {
            let (numerator_degree, denominator_degree) = c.expr.trace_degree();
            numerator_degree - denominator_degree
        })
        .max()
        .expect("constraints is empty")
}

// TODO: Show expression
#[cfg(feature = "std")]
impl std::fmt::Debug for Constraint {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Constraint(...)")
    }
}
