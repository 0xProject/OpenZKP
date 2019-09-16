use crate::{
    constraint::polynomial_expression::PolynomialExpression,
    polynomial::{DensePolynomial, SparsePolynomial},
};
use primefield::FieldElement;
use std::{
    cmp::{max, Ord},
    collections::BTreeSet,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Clone, Debug)]
pub enum TraceExpression {
    Trace(usize, isize),
    PolynomialExpression(PolynomialExpression),
    Neg(Box<TraceExpression>),
    Add(Box<TraceExpression>, Box<TraceExpression>),
    Mul(Box<TraceExpression>, Box<TraceExpression>),
}

impl TraceExpression {
    pub fn degree(&self, trace_length: usize) -> usize {
        use TraceExpression::*;
        match self {
            Trace(..) => trace_length - 1,
            PolynomialExpression(p) => p.degree(),
            Neg(a) => a.degree(trace_length),
            Add(a, b) => max(a.degree(trace_length), b.degree(trace_length)),
            Mul(a, b) => a.degree(trace_length) + b.degree(trace_length),
        }
    }

    pub fn evaluate_for_dense(
        &self,
        trace: &dyn Fn(usize, isize) -> DensePolynomial,
    ) -> DensePolynomial {
        use TraceExpression::*;
        match self {
            &Trace(i, j) => trace(i, j),
            PolynomialExpression(p) => DensePolynomial::from(SparsePolynomial::from(p.clone())),
            Neg(a) => DensePolynomial::new(&[FieldElement::ZERO]) - a.evaluate_for_dense(trace),
            Add(a, b) => a.evaluate_for_dense(trace) + b.evaluate_for_dense(trace),
            Mul(a, b) => a.evaluate_for_dense(trace) * b.evaluate_for_dense(trace),
        }
    }

    pub fn evaluate_for_element(
        &self,
        trace: &dyn Fn(usize, isize) -> FieldElement,
        x: &FieldElement,
    ) -> FieldElement {
        use TraceExpression::*;
        match self {
            &Trace(i, j) => trace(i, j),
            PolynomialExpression(p) => SparsePolynomial::from(p.clone()).evaluate(x),
            Neg(a) => -&a.evaluate_for_element(trace, x),
            Add(a, b) => a.evaluate_for_element(trace, x) + b.evaluate_for_element(trace, x),
            Mul(a, b) => a.evaluate_for_element(trace, x) * b.evaluate_for_element(trace, x),
        }
    }
}

impl From<PolynomialExpression> for TraceExpression {
    fn from(p: PolynomialExpression) -> Self {
        Self::PolynomialExpression(p)
    }
}

impl From<FieldElement> for TraceExpression {
    fn from(x: FieldElement) -> Self {
        TraceExpression::PolynomialExpression(PolynomialExpression::Constant(x))
    }
}

impl From<isize> for TraceExpression {
    fn from(i: isize) -> Self {
        Self::from(FieldElement::from(i))
    }
}

impl<T: Into<TraceExpression>> Add<T> for TraceExpression {
    type Output = Self;

    fn add(self, other: T) -> TraceExpression {
        Self::Add(Box::new(self.clone()), Box::new(other.into()))
    }
}

impl<T: Into<TraceExpression>> Sub<T> for TraceExpression {
    type Output = Self;

    fn sub(self, other: T) -> TraceExpression {
        self + Self::Neg(Box::new(other.into()))
    }
}

impl<T: Into<TraceExpression>> Mul<T> for TraceExpression {
    type Output = Self;

    fn mul(self, other: T) -> TraceExpression {
        Self::Mul(Box::new(self.clone()), Box::new(other.into()))
    }
}

impl AddAssign<TraceExpression> for TraceExpression {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other.clone()
    }
}

impl Sub<TraceExpression> for FieldElement {
    type Output = TraceExpression;

    fn sub(self, other: TraceExpression) -> TraceExpression {
        TraceExpression::Neg(Box::new(other - self))
    }
}

impl Sub<TraceExpression> for isize {
    type Output = TraceExpression;

    fn sub(self, other: TraceExpression) -> TraceExpression {
        TraceExpression::Neg(Box::new(other - TraceExpression::from(self)))
    }
}
