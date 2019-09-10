use crate::polynomial::DensePolynomial;
use crate::polynomial::SparsePolynomial;
use primefield::FieldElement;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug)]
pub enum RationalExpression {
    X,
    Constant(FieldElement),
    Trace(usize, isize),
    Add(Box<RationalExpression>, Box<RationalExpression>),
    Sub(Box<RationalExpression>, Box<RationalExpression>),
    Mul(Box<RationalExpression>, Box<RationalExpression>),
    Pow(Box<RationalExpression>, usize),
    Div(Box<RationalExpression>, Box<RationalExpression>),
}

impl RationalExpression {
    pub fn pow(&self, exponent: usize) -> RationalExpression {
        RationalExpression::Pow(Box::new(self.clone()), exponent)
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
        RationalExpression::Div(Box::new(self), Box::new(other))
    }
}

#[allow(dead_code)] // TODO
use RationalExpression::*;
impl RationalExpression {

    /// Numerator and denominator degree of the expression in X.
    ///
    /// Calculates an upper bound. Cancelations may occur.
    // Note: We can have trace polynomials of different degree here if we want.
    pub fn degree(&self, trace_degree: usize) -> (usize, usize) {
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
        trace_table: &dyn Fn(usize, isize) -> FieldElement,
        x: &FieldElement,
    ) -> FieldElement {
        use RationalExpression::*;
        match self {
            X => x.clone(),
            Constant(value) => value.clone(),
            &Trace(i, j) => trace_table(i, j),
            Add(a, b) => a.eval(trace_table, x) + b.eval(trace_table, x),
            Neg(a) => -&a.eval(trace_table, x),
            Inv(a) => {
                a.eval(trace_table, x)
                    .inv()
                    .expect("Division by zero while evaluating RationalExpression.")
            }
            _ => unimplemented!(),
        }
    }

    pub fn eval_on_domain(
        &self,
        trace_table: &Fn(usize, isize) -> DensePolynomial,
    ) -> DensePolynomial {
        match self {
            X => DensePolynomial::new(&[FieldElement::ZERO, FieldElement::ONE]),
            Constant(value) => DensePolynomial::new(&[value.clone()]),
            &Trace(i, j) => trace_table(i, j),
            Add(a, b) => a.eval_on_domain(trace_table) + b.eval_on_domain(trace_table),
            Neg(a) => -&a.eval_on_domain(trace_table),
            Div(a, b) => a.eval_on_domain(trace_table) / b.get_denominator(),
            _ => unimplemented!(),
        }
    }

    fn get_denominator(&self) -> SparsePolynomial {
        match self {
            Self::X => SparsePolynomial::new(&[(FieldElement::ONE,1)]),
            Self::Constant(c) => SparsePolynomial::new(&[(c.clone(), 0)]),
            Self::Add(a, b) => a.get_denominator(trace_table, x) + b.get_denominator(trace_table, x),
            Self::Neg(a) => -&a.get_denominator(trace_table, x, g),
            Self::Div => panic!(),
            Self::Trace(i, j) => panic!(),
        }
    }
}
