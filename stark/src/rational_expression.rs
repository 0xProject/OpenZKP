use primefield::FieldElement;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug)]
pub enum RationalExpression {
    X,
    Constant(FieldElement),
    Trace(usize, isize),
    Add(Box<RationalExpression>, Box<RationalExpression>),
    Neg(Box<RationalExpression>),
    Mul(Box<RationalExpression>, Box<RationalExpression>),
    Inv(Box<RationalExpression>),
    Exp(Box<RationalExpression>, usize),
}

impl RationalExpression {
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
        RationalExpression::Add(
            Box::new(self),
            Box::new(RationalExpression::Neg(Box::new(other))),
        )
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
        RationalExpression::Mul(
            Box::new(self),
            Box::new(RationalExpression::Inv(Box::new(other))),
        )
    }
}

#[allow(dead_code)] // TODO
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
        }
    }

    pub fn eval(
        &self,
        trace_table: &Fn(usize, &FieldElement) -> FieldElement,
        x: &FieldElement,
        g: &FieldElement,
    ) -> FieldElement {
        use RationalExpression::*;
        match self {
            X => x.clone(),
            Constant(value) => value.clone(),
            &Trace(i, j) => trace_table(i, &(x * g.pow(j.into()))),
            Add(a, b) => a.eval(trace_table, x, g) + b.eval(trace_table, x, g),
            Neg(a) => -&a.eval(trace_table, x, g),
            Inv(a) => {
                a.eval(trace_table, x, g)
                    .inv()
                    .expect("Division by zero while evaluating RationalExpression.")
            }
            _ => unimplemented!(),
        }
    }
}
