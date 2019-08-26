use crate::polynomial::{DensePolynomial, SparsePolynomial};
use std::prelude::v1::*;

pub struct Constraint {
    pub base:        Box<dyn Fn(&[DensePolynomial]) -> DensePolynomial>,
    pub denominator: SparsePolynomial,
    pub numerator:   SparsePolynomial,
}
