use crate::polynomial::DensePolynomial;
#[cfg(feature = "std")]
use std::{cmp::Ordering, collections::hash_map::DefaultHasher};
use std::{
    collections::BTreeSet,
    hash::{Hash, Hasher},
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
    prelude::v1::*,
};
use zkp_macros_decl::field_element;
use zkp_primefield::{FieldElement, Inv, One, Pow, Zero};
use zkp_u256::U256;

// TODO: Rename to algebraic expression
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum RationalExpression {
    X,
    Constant(FieldElement),
    Trace(usize, isize),
    Polynomial(DensePolynomial, Box<RationalExpression>),
    // TODO - Make this a struct with internally named members
    // the members are (index, degree bound, expression, name)
    ClaimPolynomial(usize, usize, Box<RationalExpression>, Option<&'static str>),
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

    /// Apply a function bottom up on the expression.
    ///
    /// **Note.** Unlike the conventional generalization of `map` to tree
    /// structures, this map also applies the function to each tree node,
    /// after it has been applied to all its descendants.
    pub fn map(&self, f: &impl Fn(Self) -> Self) -> Self {
        use RationalExpression::*;
        let e = match self {
            // Tree types are recursed first
            Polynomial(p, e) => Polynomial(p.clone(), Box::new(e.map(f))),
            ClaimPolynomial(i, n, e, name) => ClaimPolynomial(*i, *n, Box::new(e.map(f)), *name),
            Add(a, b) => Add(Box::new(a.map(f)), Box::new(b.map(f))),
            Neg(a) => Neg(Box::new(a.map(f))),
            Mul(a, b) => Mul(Box::new(a.map(f)), Box::new(b.map(f))),
            Inv(a) => Inv(Box::new(a.map(f))),
            Exp(a, e) => Exp(Box::new(a.map(f)), *e),

            // Leaf types are mapped as is.
            other => other.clone(),
        };
        f(e)
    }

    pub fn substitute_claim(&self, claim_polynomials: &[DensePolynomial]) -> Self {
        use RationalExpression::*;
        let f = |x| {
            match x {
                ClaimPolynomial(i, degree_bound, a, _) => {
                    let claim_polynomial = claim_polynomials
                        .get(i)
                        .expect("ClaimPolynomial index out of bounds")
                        .clone();
                    assert!(claim_polynomial.degree() <= degree_bound);
                    Polynomial(claim_polynomial, a)
                }
                _ => x.clone(),
            }
        };
        self.map(&f)
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

impl From<FieldElement> for RationalExpression {
    fn from(value: FieldElement) -> Self {
        Self::Constant(value)
    }
}

impl<T: Into<RationalExpression>> Add<T> for RationalExpression {
    type Output = Self;

    fn add(self, other: T) -> Self {
        Self::Add(Box::new(self), Box::new(other.into()))
    }
}

// Clippy false positive
#[allow(clippy::suspicious_arithmetic_impl)]
impl<T: Into<RationalExpression>> Sub<T> for RationalExpression {
    type Output = Self;

    fn sub(self, other: T) -> Self {
        self + other.into().neg()
    }
}

impl<T: Into<RationalExpression>> Mul<T> for RationalExpression {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self::Mul(Box::new(self), Box::new(other.into()))
    }
}

// Clippy false positive
#[allow(clippy::suspicious_arithmetic_impl)]
impl<T: Into<RationalExpression>> Div<T> for RationalExpression {
    type Output = Self;

    fn div(self, other: T) -> Self {
        self * other.into().inv()
    }
}

impl Sum<RationalExpression> for RationalExpression {
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = RationalExpression>,
    {
        iter.next()
            .map_or(0.into(), |expr| iter.fold(expr, |a, b| a + b))
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
            ClaimPolynomial(_, degree_bound, a, _) => {
                let (n, d) = a.degree_impl(x_degree, trace_degree);
                (degree_bound * n, degree_bound * d)
            }
            Add(a, b) => {
                let (a_numerator, a_denominator) = a.degree_impl(x_degree, trace_degree);
                let (b_numerator, b_denominator) = b.degree_impl(x_degree, trace_degree);
                (
                    std::cmp::max(a_numerator + b_denominator, b_numerator + a_denominator),
                    a_denominator + b_denominator,
                )
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

    // Note - This function is incomplete in its treatment of rational expressions
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
                    (FieldElement::one(), false)
                }
            }
            ClaimPolynomial(..) => panic!("ClaimPolynomial should be substituted by Polynomial"),
            Add(a, b) => {
                let (res_a, a_ok) = a.check(x, trace);
                let (res_b, b_ok) = b.check(x, trace);
                if a_ok && b_ok {
                    (res_a + res_b, true)
                } else {
                    (FieldElement::one(), false)
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
                    if res_a == FieldElement::zero() {
                        (FieldElement::zero(), true)
                    } else {
                        (FieldElement::one(), false)
                    }
                } else if !a_ok && b_ok {
                    if res_b == FieldElement::zero() {
                        (FieldElement::zero(), true)
                    } else {
                        (FieldElement::one(), false)
                    }
                } else {
                    (FieldElement::one(), false)
                }
            }
            // TODO - This behavior is suspect
            Inv(a) => {
                let (res_a, a_ok) = a.clone().check(x, trace);
                if a_ok {
                    if res_a == FieldElement::zero() {
                        (FieldElement::one(), false)
                    } else {
                        (res_a, true)
                    }
                } else {
                    match *(a.clone()) {
                        Inv(b) => b.check(x, trace),
                        // TODO - Fully enumerate all checks
                        _ => (FieldElement::one(), false),
                    }
                }
            }
            Exp(a, e) => {
                let (res_a, a_ok) = a.check(x, trace);
                if a_ok {
                    (res_a.pow(*e), true)
                } else {
                    (FieldElement::one(), false)
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
            Polynomial(p, a) => {
                let inner = a.evaluate(x, trace);
                p.evaluate(&inner)
            }
            ClaimPolynomial(..) => panic!("ClaimPolynomial should be substituted by Polynomial"),
            Add(a, b) => a.evaluate(x, trace) + b.evaluate(x, trace),
            Neg(a) => -&a.evaluate(x, trace),
            Mul(a, b) => a.evaluate(x, trace) * b.evaluate(x, trace),
            Inv(a) => a.evaluate(x, trace).inv().expect("divided by zero"),
            Exp(a, e) => a.evaluate(x, trace).pow(*e),
        }
    }

    pub fn trace_arguments(&self) -> BTreeSet<(usize, isize)> {
        let mut arguments = BTreeSet::new();
        self.trace_arguments_impl(&mut arguments);
        arguments
    }

    fn trace_arguments_impl(&self, s: &mut BTreeSet<(usize, isize)>) {
        use RationalExpression::*;
        match self {
            &Trace(i, j) => {
                let _ = s.insert((i, j));
            }
            X | Constant(_) => (),
            Polynomial(_, a) | Exp(a, _) | Neg(a) | Inv(a) => a.trace_arguments_impl(s),
            Add(a, b) | Mul(a, b) => {
                a.trace_arguments_impl(s);
                b.trace_arguments_impl(s);
            }
            ClaimPolynomial(..) => panic!("ClaimPolynomial should be substituted by Polynomial"),
        }
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for RationalExpression {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use RationalExpression::*;
        match self {
            X => {
                "x".hash(state);
            }
            Constant(c) => {
                c.hash(state);
            }
            &Trace(i, j) => {
                "trace".hash(state);
                i.hash(state);
                j.hash(state);
            }
            Polynomial(..) => {
                "poly".hash(state);
                let x = field_element!(
                    "754ed488ec9208d1c552bb254c0890042078a9e1f7e36072ebff1bf4e193d11b"
                );
                (self.evaluate(&x, &|_, _| panic!("Trace in polynomial not supported")))
                    .hash(state);
            }
            Add(a, b) => {
                "add".hash(state);
                a.hash(state);
                b.hash(state);
            }
            Neg(a) => {
                "neg".hash(state);
                a.hash(state);
            }
            Mul(a, b) => {
                "mul".hash(state);
                a.hash(state);
                b.hash(state);
            }
            Inv(a) => {
                "inv".hash(state);
                a.hash(state);
            }
            Exp(a, e) => {
                "exp".hash(state);
                a.hash(state);
                e.hash(state);
            }
            ClaimPolynomial(i, n, a, _) => {
                "claim_polynomial".hash(state);
                i.hash(state);
                n.hash(state);
                a.hash(state);
            }
        }
    }
}

#[cfg(feature = "std")]
impl PartialOrd for RationalExpression {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "std")]
fn get_hash(r: &RationalExpression) -> u64 {
    let mut hasher = DefaultHasher::new();
    r.hash(&mut hasher);
    hasher.finish()
}

#[cfg(feature = "std")]
impl Ord for RationalExpression {
    fn cmp(&self, other: &Self) -> Ordering {
        get_hash(self).cmp(&get_hash(other))
    }
}
