use crate::{
    constraint::{trace_degree, Constraint},
    rational_expression::RationalExpression,
    trace_table::TraceTable,
};
use itertools::Itertools;
use primefield::FieldElement;

#[allow(dead_code)]
pub trait ConstraintSystem {
    type PrivateInput;

    // TODO: these should return results.
    fn constraints(&self) -> Vec<Constraint>;
    fn trace(&self, private: &Self::PrivateInput) -> TraceTable;
}

pub fn combine_constraints(
    constraints: &[Constraint],
    constraint_coefficients: &[FieldElement],
    trace_length: usize,
) -> RationalExpression {
    assert_eq!(2 * constraints.len(), constraint_coefficients.len());
    let target_degree = trace_degree(constraints) * trace_length - 1;
    dbg!(target_degree);

    constraints
        .iter()
        .enumerate()
        .zip(constraint_coefficients.iter().tuples())
        .map(
            |((i, constraint), (coefficient_low, coefficient_high))| -> RationalExpression {
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
