use crate::rational_expression::RationalExpression;
use itertools::Itertools;
use primefield::FieldElement;
use std::prelude::v1::*;

#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Error {
    InvalidTraceLength,
}

#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Constraints {
    trace_length: usize,
    num_columns: usize,
    generator: FieldElement,
    expressions: Vec<RationalExpression>,
}

impl Constraints {
    pub fn from_expressions((trace_length, num_columns): (usize, usize), expressions: Vec<RationalExpression>) -> Result<Self, Error> {
        // TODO: Validate expressions
        Ok(Self {
            trace_length,
            num_columns,
            generator: FieldElement::root(trace_length).ok_or(Error::InvalidTraceLength)?,
            expressions
        })
    }

    pub fn for_size(trace_length: usize, num_columns: usize) -> Result<Self, Error> {
        Ok(Self {
            trace_length,
            num_columns,
            generator: FieldElement::root(trace_length).ok_or(Error::InvalidTraceLength)?,
            expressions: Vec::new()
        })
    }

    pub fn generator(&self) -> &FieldElement {
        &self.generator
    }

    pub fn trace_degree(&self) -> usize {
        self.expressions
            .iter()
            .map(|c| {
                let (numerator_degree, denominator_degree) = c.trace_degree();
                numerator_degree - denominator_degree
            })
            .max()
            .expect("constraints is empty")
    }

    pub(crate) fn len(&self) -> usize {
        self.expressions.len()
    }

    pub(crate) fn combine(
        &self,
        constraint_coefficients: &[FieldElement],
        trace_length: usize,
    ) -> RationalExpression {
        assert_eq!(2 * self.len(), constraint_coefficients.len());
        let target_degree = self.trace_degree() * trace_length - 1;

        self.expressions
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
