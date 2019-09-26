use crate::rational_expression::RationalExpression;
use itertools::Itertools;
use primefield::FieldElement;
use std::prelude::v1::*;

#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Constraints(Vec<RationalExpression>);

impl Constraints {
    pub fn new(constraints: Vec<RationalExpression>) -> Self {
        Self(constraints)
    }

    pub fn trace_degree(&self) -> usize {
        self.0
            .iter()
            .map(|c| {
                let (numerator_degree, denominator_degree) = c.trace_degree();
                numerator_degree - denominator_degree
            })
            .max()
            .expect("constraints is empty")
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn combine(
        &self,
        constraint_coefficients: &[FieldElement],
        trace_length: usize,
    ) -> RationalExpression {
        assert_eq!(2 * self.0.len(), constraint_coefficients.len());
        let target_degree = self.trace_degree() * trace_length - 1;

        self.0
            .iter()
            .zip(constraint_coefficients.iter().tuples())
            .map(
                |(constraint, (coefficient_low, coefficient_high))| -> RationalExpression {
                    let (num, den) = constraint.degree(trace_length - 1);
                    let adjustment_degree = target_degree + den - num;
                    let adjustment = RationalExpression::Constant(coefficient_low.clone())
                        + RationalExpression::Constant(coefficient_high.clone())
                            * RationalExpression::X.pow(adjustment_degree);
                    adjustment * constraint.clone()
                },
            )
            .sum()
    }
}
