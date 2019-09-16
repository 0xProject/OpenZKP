use crate::polynomial::{DensePolynomial, SparsePolynomial};
use core::cmp::Ordering;
use lazy_static::*;
use primefield::FieldElement;
use std::{
    cmp::{max, Ord},
    collections::BTreeSet,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};
use u256::{commutative_binop, noncommutative_binop};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PolynomialExpression {
    X,
    Constant(FieldElement),
    PeriodicColumn(SparsePolynomial),
    Pow(Box<PolynomialExpression>, usize),
    Neg(Box<PolynomialExpression>),
    Add(Box<PolynomialExpression>, Box<PolynomialExpression>),
    Mul(Box<PolynomialExpression>, Box<PolynomialExpression>),
}

impl PolynomialExpression {
    pub fn pow(&self, exponent: usize) -> Self {
        Self::Pow(Box::new(self.clone()), exponent)
    }

    pub fn degree(&self) -> usize {
        SparsePolynomial::from(self.clone()).degree()
    }
}

impl From<PolynomialExpression> for SparsePolynomial {
    fn from(p: PolynomialExpression) -> Self {
        use PolynomialExpression::*;
        match p {
            X => SparsePolynomial::new(&[(FieldElement::ONE, 1)]),
            Constant(c) => SparsePolynomial::new(&[(c, 0)]),
            PeriodicColumn(p) => p,
            Pow(a, n) => SparsePolynomial::from(*a).pow(n),
            Neg(a) => {
                SparsePolynomial::new(&[(FieldElement::ZERO, 0)]) - SparsePolynomial::from(*a)
            }
            Add(a, b) => SparsePolynomial::from(*a) + SparsePolynomial::from(*b),
            Mul(a, b) => SparsePolynomial::from(*a) * SparsePolynomial::from(*b),
        }
    }
}

impl AddAssign<&PolynomialExpression> for PolynomialExpression {
    fn add_assign(&mut self, other: &Self) {
        *self = Self::Add(Box::new(self.clone()), Box::new(other.clone()));
    }
}

impl SubAssign<&PolynomialExpression> for PolynomialExpression {
    fn sub_assign(&mut self, other: &Self) {
        *self += Self::Neg(Box::new(other.clone()));
    }
}

impl MulAssign<&PolynomialExpression> for PolynomialExpression {
    fn mul_assign(&mut self, other: &Self) {
        *self = Self::Mul(Box::new(self.clone()), Box::new(other.clone()));
    }
}

commutative_binop!(PolynomialExpression, Add, add, AddAssign, add_assign);
commutative_binop!(PolynomialExpression, Mul, mul, MulAssign, mul_assign);
noncommutative_binop!(PolynomialExpression, Sub, sub, SubAssign, sub_assign);

impl Sub<FieldElement> for PolynomialExpression {
    type Output = Self;

    fn sub(self, other: FieldElement) -> Self {
        self - Self::Constant(other)
    }
}

impl Sub<isize> for PolynomialExpression {
    type Output = Self;

    fn sub(self, other: isize) -> Self {
        self - Self::Constant(other.into())
    }
}
