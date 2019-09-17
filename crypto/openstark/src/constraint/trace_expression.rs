use crate::{
    constraint::polynomial_expression::PolynomialExpression,
    polynomial::{DensePolynomial, SparsePolynomial},
};
use primefield::FieldElement;
use std::{
    boxed::Box,
    cmp::max,
    ops::{Add, AddAssign, Mul, Sub},
};

#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum TraceExpression {
    Trace(usize, isize),
    PolynomialExpression(Box<PolynomialExpression>),
    Add(Box<TraceExpression>, Box<TraceExpression>),
    Sub(Box<TraceExpression>, Box<TraceExpression>),
    Mul(Box<TraceExpression>, Box<TraceExpression>),
}

enum Polynomial {
    Dense(DensePolynomial),
    Sparse(SparsePolynomial),
}

impl From<SparsePolynomial> for Polynomial {
    fn from(s: SparsePolynomial) -> Self {
        Self::Sparse(s)
    }
}

impl From<DensePolynomial> for Polynomial {
    fn from(d: DensePolynomial) -> Self {
        Self::Dense(d)
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            Self::Dense(a) => {
                match other {
                    Self::Dense(b) => Self::Dense(&a + &b),
                    Self::Sparse(b) => Self::Dense(&a + b),
                }
            }
            Self::Sparse(a) => {
                match other {
                    Self::Dense(b) => Self::Dense(&b + a),
                    Self::Sparse(b) => Self::Sparse(a + b),
                }
            }
        }
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match self {
            Self::Dense(a) => {
                match other {
                    Self::Dense(b) => Self::from(&a - &b),
                    Self::Sparse(b) => Self::from(&a - b),
                }
            }
            Self::Sparse(a) => {
                match other {
                    Self::Dense(b) => Self::from(a - &b),
                    Self::Sparse(b) => Self::from(a - b),
                }
            }
        }
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match self {
            Self::Dense(a) => {
                match other {
                    Self::Dense(b) => Self::from(a * b),
                    Self::Sparse(b) => Self::from(a * b),
                }
            }
            Self::Sparse(a) => {
                match other {
                    Self::Dense(b) => Self::from(b * a),
                    Self::Sparse(b) => Self::from(a * b),
                }
            }
        }
    }
}

impl TraceExpression {
    pub fn degree(&self, trace_length: usize) -> usize {
        use TraceExpression::*;
        match self {
            Trace(..) => trace_length - 1,
            PolynomialExpression(p) => p.degree(),
            Add(a, b) => max(a.degree(trace_length), b.degree(trace_length)),
            Sub(a, b) => max(a.degree(trace_length), b.degree(trace_length)),
            Mul(a, b) => a.degree(trace_length) + b.degree(trace_length),
        }
    }

    pub fn evaluate_smarter(
        &self,
        trace: &dyn Fn(usize, isize) -> DensePolynomial,
    ) -> DensePolynomial {
        let p = self.evaluate_for_dense(trace);
        match p {
            Polynomial::Dense(d) => d,
            Polynomial::Sparse(s) => DensePolynomial::from(s),
        }
    }

    fn evaluate_for_dense(&self, trace: &dyn Fn(usize, isize) -> DensePolynomial) -> Polynomial {
        use TraceExpression::*;
        match self {
            &Trace(i, j) => Polynomial::Dense(trace(i, j)),
            PolynomialExpression(p) => Polynomial::Sparse(SparsePolynomial::from(*p.clone())),
            Add(a, b) => a.evaluate_for_dense(trace) + b.evaluate_for_dense(trace),
            Sub(a, b) => a.evaluate_for_dense(trace) - b.evaluate_for_dense(trace),
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
            PolynomialExpression(p) => SparsePolynomial::from(*p.clone()).evaluate(x),
            Add(a, b) => a.evaluate_for_element(trace, x) + b.evaluate_for_element(trace, x),
            Sub(a, b) => a.evaluate_for_element(trace, x) - b.evaluate_for_element(trace, x),
            Mul(a, b) => a.evaluate_for_element(trace, x) * b.evaluate_for_element(trace, x),
        }
    }
}

impl From<PolynomialExpression> for TraceExpression {
    fn from(p: PolynomialExpression) -> Self {
        Self::PolynomialExpression(Box::new(p))
    }
}

impl From<FieldElement> for TraceExpression {
    fn from(x: FieldElement) -> Self {
        TraceExpression::PolynomialExpression(Box::new(PolynomialExpression::Constant(x)))
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
        Self::Sub(Box::new(self.clone()), Box::new(other.into()))
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
        TraceExpression::from(self) - other
    }
}

impl Sub<TraceExpression> for isize {
    type Output = TraceExpression;

    fn sub(self, other: TraceExpression) -> TraceExpression {
        TraceExpression::from(self) - other
    }
}
