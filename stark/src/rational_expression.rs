use crate::polynomial::{DensePolynomial, SparsePolynomial};
use primefield::FieldElement;
use std::{
    cmp::max,
    ops::{Add, Mul, Sub},
};

#[derive(Clone, Debug)]
pub enum RationalExpression {
    X,
    Constant(FieldElement),
    Trace(usize, isize),
    Add(Box<RationalExpression>, Box<RationalExpression>),
    Sub(Box<RationalExpression>, Box<RationalExpression>),
    Mul(Box<RationalExpression>, Box<RationalExpression>),
    Pow(Box<RationalExpression>, usize),
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
        RationalExpression::Sub(Box::new(self), Box::new(other))
    }
}

impl Mul for RationalExpression {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        RationalExpression::Mul(Box::new(self), Box::new(other))
    }
}

impl RationalExpression {
    /// Numerator and denominator degree of the expression in X.
    ///
    /// Calculates an upper bound. Cancelations may occur.
    // Note: We can have trace polynomials of different degree here if we want.
    pub fn degree(&self, trace_degree: usize) -> usize {
        use RationalExpression::*;
        match self {
            X => 1,
            Constant(_) => 0,
            Trace(..) => trace_degree,
            Add(a, b) => max(a.degree(trace_degree), b.degree(trace_degree)),
            Sub(a, b) => max(a.degree(trace_degree), b.degree(trace_degree)),
            Mul(a, b) => a.degree(trace_degree) + b.degree(trace_degree),
            Pow(a, n) => n * a.degree(trace_degree),
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
            Sub(a, b) => a.eval(trace_table, x) - b.eval(trace_table, x),
            _ => unimplemented!(),
        }
    }

    pub fn eval_on_domain(
        &self,
        trace_table: &dyn Fn(usize, isize) -> DensePolynomial,
    ) -> DensePolynomial {
        match self {
            Self::X => DensePolynomial::new(&[FieldElement::ZERO, FieldElement::ONE]),
            Self::Constant(value) => DensePolynomial::new(&[value.clone()]),
            &Self::Trace(i, j) => trace_table(i, j),
            Self::Add(a, b) => a.eval_on_domain(trace_table) + b.eval_on_domain(trace_table),
            Self::Sub(a, b) => a.eval_on_domain(trace_table) - b.eval_on_domain(trace_table),
            Self::Mul(a, b) => a.eval_on_domain(trace_table) * b.eval_on_domain(trace_table),
            _ => unimplemented!(),
        }
    }

    // fn get_denominator(&self) -> SparsePolynomial {
    //     match self {
    //         Self::X => SparsePolynomial::new(&[(FieldElement::ONE, 1)]),
    //         Self::Constant(c) => SparsePolynomial::new(&[(c.clone(), 0)]),
    //         Self::Add(a, b) => a.get_denominator() + b.get_denominator(),
    //         Self::Sub(a, b) => a.get_denominator() - b.get_denominator(),
    //         Self::Trace(..) => panic!(),
    //         Self::Mul(a, b) => a.get_denominator() * b.get_denominator(),
    //         Self::Pow(a, n) => panic!(),
    //     }
    // }
}
