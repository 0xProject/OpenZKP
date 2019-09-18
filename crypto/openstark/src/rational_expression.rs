use crate::{polynomial::DensePolynomial, trace_table::TraceTable};
use primefield::FieldElement;
use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Neg, Sub},
};

// TODO: Rename to algebraic expression
#[derive(Clone)]
pub enum RationalExpression {
    X,
    Constant(FieldElement),
    Trace(usize, isize),
    Add(Box<RationalExpression>, Box<RationalExpression>),
    Neg(Box<RationalExpression>),
    Mul(Box<RationalExpression>, Box<RationalExpression>),
    Inv(Box<RationalExpression>),
    Exp(Box<RationalExpression>, usize),
    Poly(DensePolynomial, Box<RationalExpression>),
}

impl std::fmt::Debug for RationalExpression {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RationalExpression::*;
        match self {
            X => write!(fmt, "X"),
            Constant(a) => write!(fmt, "{:?}", a),
            Trace(i, j) => write!(fmt, "Trace({}, {})", i, j),
            Add(a, b) => write!(fmt, "({:?} + {:?})", a, b),
            Neg(a) => write!(fmt, "-{:?}", a),
            Mul(a, b) => write!(fmt, "({:?} * {:?})", a, b),
            Inv(a) => write!(fmt, "1/{:?}", a),
            Exp(a, e) => write!(fmt, "{:?}^{:?}", a, e),
            Poly(_, a) => write!(fmt, "P({:?})", a),
        }
    }
}

impl RationalExpression {
    pub fn neg(&self) -> RationalExpression {
        RationalExpression::Neg(Box::new(self.clone()))
    }

    pub fn inv(&self) -> RationalExpression {
        RationalExpression::Inv(Box::new(self.clone()))
    }

    pub fn pow(&self, exponent: usize) -> RationalExpression {
        RationalExpression::Exp(Box::new(self.clone()), exponent)
    }
}

impl From<i32> for RationalExpression {
    fn from(value: i32) -> Self {
        RationalExpression::Constant(value.into())
    }
}

impl From<&FieldElement> for RationalExpression {
    fn from(value: &FieldElement) -> Self {
        RationalExpression::Constant(value.clone())
    }
}

impl Add for RationalExpression {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        RationalExpression::Add(Box::new(self), Box::new(other))
    }
}

impl Sub for RationalExpression {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + other.neg()
    }
}

impl Mul for RationalExpression {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        RationalExpression::Mul(Box::new(self), Box::new(other))
    }
}

impl Div for RationalExpression {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}

impl Sum<RationalExpression> for RationalExpression {
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = RationalExpression>,
    {
        if let Some(expr) = iter.next() {
            iter.fold(expr, |a, b| a + b)
        } else {
            0.into()
        }
    }
}

impl RationalExpression {
    /// Numerator and denominator degree of the expression in X.
    ///
    /// Calculates an upper bound. Cancelations may occur.
    // Note: We can have trace polynomials of different degree here if we want.
    pub fn degree(&self, trace_degree: usize) -> (usize, usize) {
        use RationalExpression::*;
        match self {
            X => (1, 0),
            Constant(_) => (0, 0),
            Trace(..) => (trace_degree, 0),
            Add(a, b) => {
                let (an, ad) = a.degree(trace_degree);
                let (bn, bd) = b.degree(trace_degree);
                assert!(ad == 0); // TODO: Can we handle this better?
                assert!(bd == 0);
                (std::cmp::max(an, bn), 0)
            }
            Neg(a) => a.degree(trace_degree),
            Mul(a, b) => {
                let (an, ad) = a.degree(trace_degree);
                let (bn, bd) = b.degree(trace_degree);
                (an + bn, ad + bd)
            }
            Inv(a) => {
                let (n, d) = a.degree(trace_degree);
                (d, n)
            }
            Exp(a, e) => {
                let (n, d) = a.degree(trace_degree);
                (e * n, e * d)
            }
            Poly(p, a) => {
                let (n, d) = a.degree(trace_degree);
                (p.degree() * n, p.degree() * d)
            }
        }
    }

    /// Simplify the expression
    ///
    /// Does constant propagation and simplifies expressions like `0 + a`,
    /// `0 * a`, `1 * a`, `-(-a)`, `1/(1/a)`, `a^0` and `a^1`.
    pub fn simplify(self) -> Self {
        use RationalExpression::*;
        match self {
            Add(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                match (a, b) {
                    (a, Constant(FieldElement::ZERO)) => a,
                    (Constant(FieldElement::ZERO), b) => b,
                    (Constant(a), Constant(b)) => Constant(a + b),
                    (a, b) => a + b,
                }
            }
            Mul(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                match (a, b) {
                    (a, Constant(FieldElement::ZERO)) => Constant(FieldElement::ZERO),
                    (Constant(FieldElement::ZERO), b) => Constant(FieldElement::ZERO),
                    (a, Constant(FieldElement::ONE)) => a,
                    (Constant(FieldElement::ONE), b) => b,
                    (Constant(a), Constant(b)) => Constant(a * b),
                    (a, b) => a * b,
                }
            }
            Neg(a) => {
                let a = a.simplify();
                match a {
                    // TODO: impl std::ops::Neg for FieldElement
                    Constant(a) => Constant(FieldElement::ZERO - a),
                    Neg(a) => *a,
                    a => a.neg(),
                }
            }
            Inv(a) => {
                let a = a.simplify();
                match a {
                    Constant(a) => Constant(a.inv().expect("Division by zero.")),
                    Inv(a) => *a,
                    a => a.inv(),
                }
            }
            Exp(a, e) => {
                let a = a.simplify();
                match (a, e) {
                    (a, 0) => Constant(FieldElement::ONE),
                    (a, 1) => a,
                    (Constant(a), e) => Constant(a.pow(e)),
                    (a, e) => a.pow(e),
                }
            }
            e => e,
        }
    }
}
