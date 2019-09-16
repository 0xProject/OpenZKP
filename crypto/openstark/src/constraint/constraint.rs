use crate::{
    constraint::{
        polynomial_expression::PolynomialExpression::{self, Constant, X},
        trace_expression::TraceExpression,
    },
    polynomial::{DensePolynomial, SparsePolynomial},
};
use primefield::FieldElement;
use std::{collections::BTreeMap, prelude::v1::*};

pub struct Constraint {
    pub base:        TraceExpression,
    pub denominator: PolynomialExpression,
    pub numerator:   PolynomialExpression,
}

impl Constraint {
    pub fn degree(&self, trace_length: usize) -> usize {
        self.base.degree(trace_length) + self.numerator.degree() - self.denominator.degree()
    }
}

pub fn combine_constraints(
    constraints: &[Constraint],
    coefficients: &[FieldElement],
    trace_length: usize,
) -> GroupedConstraints {
    let max_degree: usize = constraints
        .iter()
        .map(|c| c.degree(trace_length))
        .max()
        .unwrap();
    let result_degree = max_degree.next_power_of_two() - 1;

    let mut result = GroupedConstraints::new();
    for (i, constraint) in constraints.iter().enumerate() {
        let degree_adjustment = X.pow(
            result_degree + constraint.denominator.degree()
                - constraint.base.degree(trace_length)
                - constraint.numerator.degree(),
        );

        result.insert(
            (constraint.numerator.clone(), constraint.denominator.clone()),
            constraint.base.clone() * coefficients[2 * i].clone(),
        );
        result.insert(
            (constraint.numerator.clone(), constraint.denominator.clone()),
            constraint.base.clone() * degree_adjustment * coefficients[2 * i + 1].clone(),
        );
    }
    result
}

pub struct GroupedConstraints(
    BTreeMap<(PolynomialExpression, PolynomialExpression), TraceExpression>,
);

impl GroupedConstraints {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(
        &mut self,
        key: (PolynomialExpression, PolynomialExpression),
        value: TraceExpression,
    ) {
        *self.0.entry(key).or_insert(TraceExpression::from(0)) += value;
    }

    pub fn eval_on_domain(
        &self,
        trace_table: &dyn Fn(usize, isize) -> DensePolynomial,
    ) -> DensePolynomial {
        let mut result = DensePolynomial::new(&[FieldElement::ZERO]);
        for ((numerator, denominator), base) in &self.0 {
            let mut increment: DensePolynomial = base.evaluate_for_dense(trace_table);
            // It's possible that not all the terms are needed in this multiplication,
            // because...?
            increment *= SparsePolynomial::from(numerator.clone());
            increment /= SparsePolynomial::from(denominator.clone());
            result += increment;
        }
        result
    }

    pub fn eval(
        &self,
        trace_table: &dyn Fn(usize, isize) -> FieldElement,
        x: &FieldElement,
    ) -> FieldElement {
        let mut result = FieldElement::ZERO;
        for ((numerator, denominator), base) in &self.0 {
            let mut increment: FieldElement = base.evaluate_for_element(trace_table, x);
            increment *= SparsePolynomial::from(numerator.clone()).evaluate(x);
            increment /= SparsePolynomial::from(denominator.clone()).evaluate(x);
            result += increment;
        }
        result
    }
}

// TODO: Show expression
#[cfg(feature = "std")]
impl std::fmt::Debug for Constraint {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Constraint(...)")
    }
}
