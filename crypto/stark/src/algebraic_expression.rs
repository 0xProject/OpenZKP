use zkp_primefield::FieldElement;
use crate::polynomial::DensePolynomial;
use std::fmt::Debug;

pub trait Proof: IntoPolynomials {
    type Witness: IntoPolynomials + Debug + Copy + Eq + PartialEq;
    type Claim: IntoPolynomials + Debug  + Copy + Eq + PartialEq + From<Self::Claim>;
}

pub trait IntoPolynomials {
    type Item: Eq + Copy + Debug;

    fn get_polynomial(&self, i: Self::Item) -> DensePolynomial;
}

#[derive(Clone, Eq, PartialEq, Copy)]
pub enum Polynomial<T: Proof> {
    Fixed(T::Item),
    Public(<T::Claim as IntoPolynomials>::Item),
    Private(<T::Witness as IntoPolynomials>::Item),
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AlgebraicExpression<'a, 'b, T: Proof> {
    X,
    Polynomial(Polynomial<T>, &'a Self),
    Add(&'a Self, &'b Self),
    Neg(&'a Self),
    Mul(&'a Self, &'b Self),
    Inv(&'a Self),
    Exp(&'a Self, usize),
}

struct AClaim(FieldElement);
struct AWitness(FieldElement);
//
// #[derive(Clone, Eq, PartialEq, Copy)]
// #[cfg_attr(feature = "std", derive(Debug))]
// pub enum VerifierTerm<T: Proof> {
//     X,
//     Constant(u32),
//     Trace(usize, isize),
//     FixedPolynomial(T::FixedPolynomial),
//     Public(T::ClaimPolynomial),
// }
