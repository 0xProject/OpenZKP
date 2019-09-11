use crate::{polynomial::SparsePolynomial, rational_expression::RationalExpression};
use primefield::FieldElement;
use std::prelude::v1::*;
use crate::rational_expression::RationalExpression::Constant;

pub struct Constraint {
    pub base:        RationalExpression,
    pub denominator: RationalExpression,
    pub numerator:   RationalExpression,
}

impl Constraint {
    pub fn degree(&self, trace_length: usize) -> usize {
        self.base.degree(trace_length) + self.denominator.degree(trace_length)
            - self.numerator.degree(trace_length)
    }
}

pub fn combine_constraints(
    constraints: &[Constraint],
    coefficients: &[FieldElement],
    trace_length: usize,
) -> RationalExpression {
    let mut result = RationalExpression::from(0);
    let max_degree: usize = constraints
        .iter()
        .map(|c| c.degree(trace_length))
        .max()
        .unwrap();
    for (i, constraint) in constraints.iter().enumerate() {
        let x = constraint.base.clone() * constraint.numerator.clone() / constraint.denominator.clone();
        result = result + Constant(coefficients[2 * i].clone()) * x.clone();
        let degree_adjustment = RationalExpression::X.pow(10);
        result = result + Constant(coefficients[2 * i + 1].clone()) * x * degree_adjustment;
    }
    result
}
