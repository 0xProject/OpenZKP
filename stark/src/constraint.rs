use crate::{
    polynomial::SparsePolynomial,
    rational_expression::RationalExpression::{self, Constant},
};
use primefield::FieldElement;
use std::prelude::v1::*;

pub struct Constraint {
    pub base:        RationalExpression,
    pub denominator: RationalExpression,
    pub numerator:   RationalExpression,
}

impl Constraint {
    pub fn degree(&self, trace_length: usize) -> usize {
        self.base.degree(trace_length) + self.numerator.degree(trace_length)
            - self.denominator.degree(trace_length)
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
    let target_degree = max_degree.next_power_of_two();
    for (i, constraint) in constraints.iter().enumerate() {
        // these look good
        // println!("numerator: {:?}", constraint.numerator.get_denominator());
        // println!("denominator {:?}", constraint.denominator.get_denominator());

        let x =
            constraint.base.clone() * constraint.numerator.clone() / constraint.denominator.clone();
        result = result + Constant(coefficients[2 * i].clone()) * x.clone();
        let degree_adjustment =
            RationalExpression::X.pow(target_degree - result.degree(trace_length));
        // assert_eq!(target_degree, 1024);
        // assert_eq!(result.degree(trace_length), 1);
        result = result + Constant(coefficients[2 * i + 1].clone()) * x * degree_adjustment;
    }
    // let mason: usize = result.degree(trace_length);
    // assert_eq!(mason, trace_length);
    result
}
