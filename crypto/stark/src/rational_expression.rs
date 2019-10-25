use crate::polynomial::DensePolynomial;
use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
    prelude::v1::*,
};
use zkp_primefield::FieldElement;

// TODO: Rename to algebraic expression
#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum RationalExpression {
    X,
    Constant(FieldElement),
    Trace(usize, isize),
    Polynomial(DensePolynomial, Box<RationalExpression>),
    Add(Box<RationalExpression>, Box<RationalExpression>),
    Neg(Box<RationalExpression>),
    Mul(Box<RationalExpression>, Box<RationalExpression>),
    Inv(Box<RationalExpression>),
    Exp(Box<RationalExpression>, usize),
}

impl RationalExpression {
    pub fn neg(&self) -> Self {
        Self::Neg(Box::new(self.clone()))
    }

    pub fn inv(&self) -> Self {
        Self::Inv(Box::new(self.clone()))
    }

    pub fn pow(&self, exponent: usize) -> Self {
        Self::Exp(Box::new(self.clone()), exponent)
    }
}

impl From<i32> for RationalExpression {
    fn from(value: i32) -> Self {
        Self::Constant(value.into())
    }
}

impl From<&FieldElement> for RationalExpression {
    fn from(value: &FieldElement) -> Self {
        Self::Constant(value.clone())
    }
}

impl Add for RationalExpression {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::Add(Box::new(self), Box::new(other))
    }
}

// Clippy false positive
#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub for RationalExpression {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + other.neg()
    }
}

impl Mul for RationalExpression {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::Mul(Box::new(self), Box::new(other))
    }
}

// Clippy false positive
#[allow(clippy::suspicious_arithmetic_impl)]
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
        self.degree_impl(1, trace_degree)
    }

    pub fn trace_degree(&self) -> (usize, usize) {
        self.degree_impl(0, 1)
    }

    // TODO: do this with a generic function.
    fn degree_impl(&self, x_degree: usize, trace_degree: usize) -> (usize, usize) {
        use RationalExpression::*;
        match self {
            X => (x_degree, 0),
            Constant(_) => (0, 0),
            Trace(..) => (trace_degree, 0),
            Polynomial(p, a) => {
                let (n, d) = a.degree_impl(x_degree, trace_degree);
                (p.degree() * n, p.degree() * d)
            }
            Add(a, b) => {
                let (an, ad) = a.degree_impl(x_degree, trace_degree);
                let (bn, bd) = b.degree_impl(x_degree, trace_degree);
                assert!(ad == 0); // TODO: Can we handle this better?
                assert!(bd == 0);
                (std::cmp::max(an, bn), 0)
            }
            Neg(a) => a.degree_impl(x_degree, trace_degree),
            Mul(a, b) => {
                let (an, ad) = a.degree_impl(x_degree, trace_degree);
                let (bn, bd) = b.degree_impl(x_degree, trace_degree);
                (an + bn, ad + bd)
            }
            Inv(a) => {
                let (n, d) = a.degree_impl(x_degree, trace_degree);
                (d, n)
            }
            Exp(a, e) => {
                let (n, d) = a.degree_impl(x_degree, trace_degree);
                (e * n, e * d)
            }
        }
    }

    // Note - This function is incomplete in it's treatment of rational expressions
    // and may not produce the right answer when used with nested expressions
    // containing inverses
    pub fn check(
        &self,
        x: &FieldElement,
        trace: &dyn Fn(usize, isize) -> FieldElement,
    ) -> (FieldElement, bool) {
        use RationalExpression::*;

        match self {
            X => (x.clone(), true),
            Constant(c) => (c.clone(), true),
            &Trace(i, j) => (trace(i, j), true),

            Polynomial(p, a) => {
                let (res, is_ok) = a.check(x, trace);
                if is_ok {
                    (p.evaluate(&res), true)
                } else {
                    (FieldElement::ONE, false)
                }
            }
            Add(a, b) => {
                let (res_a, a_ok) = a.check(x, trace);
                let (res_b, b_ok) = b.check(x, trace);
                if a_ok && b_ok {
                    (res_a + res_b, true)
                } else {
                    (FieldElement::ONE, false)
                }
            }
            Neg(a) => {
                let (res_a, a_ok) = a.check(x, trace);
                // Note - this means a false should be either one or -one
                (-&res_a, a_ok)
            }
            Mul(a, b) => {
                let (res_a, a_ok) = a.check(x, trace);
                let (res_b, b_ok) = b.check(x, trace);

                if a_ok && b_ok {
                    (res_a * res_b, true)
                } else if a_ok && !b_ok {
                    if res_a == FieldElement::ZERO {
                        (FieldElement::ZERO, true)
                    } else {
                        (FieldElement::ONE, false)
                    }
                } else if !a_ok && b_ok {
                    if res_b == FieldElement::ZERO {
                        (FieldElement::ZERO, true)
                    } else {
                        (FieldElement::ONE, false)
                    }
                } else {
                    (FieldElement::ONE, false)
                }
            }
            // TODO - This behavior is suspect
            Inv(a) => {
                let (res_a, a_ok) = a.clone().check(x, trace);
                if a_ok {
                    if res_a == FieldElement::ZERO {
                        (FieldElement::ONE, false)
                    } else {
                        (res_a, true)
                    }
                } else {
                    match *(a.clone()) {
                        Inv(b) => b.check(x, trace),
                        // TODO - Fully enumerate all checks
                        _ => (FieldElement::ONE, false),
                    }
                }
            }
            Exp(a, e) => {
                let (res_a, a_ok) = a.check(x, trace);
                if a_ok {
                    (res_a.pow(*e), true)
                } else {
                    (FieldElement::ONE, false)
                }
            }
        }
    }

    pub fn evaluate(
        &self,
        x: &FieldElement,
        trace: &dyn Fn(usize, isize) -> FieldElement,
    ) -> FieldElement {
        use RationalExpression::*;
        match self {
            X => x.clone(),
            Constant(c) => c.clone(),
            &Trace(i, j) => trace(i, j),
            Polynomial(p, a) => p.evaluate(&a.evaluate(x, trace)),
            Add(a, b) => a.evaluate(x, trace) + b.evaluate(x, trace),
            Neg(a) => -&a.evaluate(x, trace),
            Mul(a, b) => a.evaluate(x, trace) * b.evaluate(x, trace),
            Inv(a) => a.evaluate(x, trace).inv().expect("divided by zero"),
            Exp(a, e) => a.evaluate(x, trace).pow(*e),
        }
    }
}
