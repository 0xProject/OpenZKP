use crate::{constraint::Constraint, trace_table::TraceTable};
use primefield::FieldElement;
use itertools::Itertools;
use crate::rational_expression::RationalExpression;

#[allow(dead_code)]
pub trait ConstraintSystem {
    type PrivateInput;

    // TODO: these should return results.
    fn constraints(&self) -> Vec<Constraints>;
    fn trace(&self, private: &Self::PrivateInput) -> TraceTable;
}

pub fn combine_constraints(
    constraints: &[Constraint],
    constraint_coefficients: &[FieldElement],
) -> RationalExpression {
    assert_eq!(2 * constraints.len(), constraint_coefficients.len());
    let target_degree = 

    constraints
        .iter()
        .enumerate()
        .zip(constraint_coefficients.iter().tuples())
        .map(
            |((i, constraint), (coefficient_low, coefficient_high))| -> RationalExpression {
                let (num, den) = constraint.expr.degree(trace_degree);
                let adjustment_degree = target_degree + den - num;
                let adjustment = RationalExpression::Constant(coefficient_low.clone())
                    + RationalExpression::Constant(coefficient_high.clone())
                        * RationalExpression::X.pow(adjustment_degree);
                adjustment * constraint.expr.clone()
            },
        )
        .sum()
}
