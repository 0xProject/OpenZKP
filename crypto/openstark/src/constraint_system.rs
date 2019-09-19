#[cfg(feature = "prover")]
use crate::trace_table::TraceTable;
use crate::{
    constraint::{trace_degree, Constraint},
    rational_expression::RationalExpression,
};
use itertools::Itertools;
use primefield::FieldElement;
use std::prelude::v1::*;

#[allow(dead_code)]
pub(crate) trait ConstraintSystem {
    type PrivateInput;

    // TODO: these should return results.
    fn constraints(&self) -> Vec<Constraint>;
    #[cfg(feature = "prover")]
    fn trace(&self, private: &Self::PrivateInput) -> TraceTable;
}

pub(crate) fn combine_constraints(
    constraints: &[Constraint],
    constraint_coefficients: &[FieldElement],
    trace_length: usize,
) -> RationalExpression {
    assert_eq!(2 * constraints.len(), constraint_coefficients.len());
    let target_degree = trace_degree(constraints) * trace_length - 1;

    constraints
        .iter()
        .zip(constraint_coefficients.iter().tuples())
        .map(
            |(constraint, (coefficient_low, coefficient_high))| -> RationalExpression {
                let (num, den) = constraint.expr.degree(trace_length - 1);
                let adjustment_degree = target_degree + den - num;
                let adjustment = RationalExpression::Constant(coefficient_low.clone())
                    + RationalExpression::Constant(coefficient_high.clone())
                        * RationalExpression::X.pow(adjustment_degree);
                adjustment * constraint.expr.clone()
            },
        )
        .sum()
}
