use crate::{polynomial::SparsePolynomial, rational_expression::RationalExpression};
use std::prelude::v1::*;
use primefield::FieldElement;

pub struct Constraint {
    pub base:        RationalExpression,
    pub denominator: RationalExpression,
    pub numerator:   RationalExpression,
}

pub fn combine_constraints(
    constraints: &[Constraint],
    coefficients: &[FieldElement],
) -> RationalExpression {
    RationalExpression::from(1)
}
