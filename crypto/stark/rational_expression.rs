use crate::polynomial::DensePolynomial;
use primefield::FieldElement;
use macros_decl::field_element;
use u256::U256;
use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
    hash::{Hash, Hasher},
    collections::HashMap,
    prelude::v1::*,
};

// TODO: Rename to algebraic expression
#[derive(Clone, Eq, PartialEq)]
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

impl Hash for RationalExpression {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use RationalExpression::*;
        match self {
            X => {"x".hash(state);},
            Constant(c) => {c.hash(state);},
            &Trace(i, j) => {
                "trace".hash(state);
                i.hash(state);
                j.hash(state);
            },
            Polynomial(p, a) => {
                "poly".hash(state);
                let x = field_element!("754ed488ec9208d1c552bb254c0890042078a9e1f7e36072ebff1bf4e193d11b");
                (p.evaluate(&x)).hash(state);
                a.hash(state);
            },
            Add(a, b) => {
                "add".hash(state);
                a.hash(state);
                b.hash(state);
            },
            Neg(a) => {
                "neg".hash(state);
                a.hash(state);
            },
            Mul(a, b) => {
                "mul".hash(state);
                a.hash(state);
                b.hash(state);
            },
            Inv(a) => {
                "inv".hash(state);
                a.hash(state);
            },
            Exp(a, e) => {
                "exp".hash(state);
                a.hash(state);
                e.hash(state);
            },
        }
    }
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

    pub fn soldity_encode(&self, memory_layout: &HashMap<RationalExpression, String>) -> String {
        use RationalExpression::*;

        match self {
            X => "mload(0)".to_owned(),
            Constant(_) if memory_layout.contains_key(self) => memory_layout.get(self).unwrap().clone(),
            Constant(c) => format!("0x{}", U256::from(c).to_string()),
            Trace(_, _) => memory_layout.get(self).unwrap().clone(),
            Polynomial(_, _) => memory_layout.get(self).unwrap().clone(),
            Add(a, b) => format!("addmod({}, {}, PRIME)" , a.soldity_encode(memory_layout), b.soldity_encode(memory_layout)),
            Neg(a) => format!("sub(PRIME , {})", a.soldity_encode(memory_layout)),
            Mul(a, b) => format!("mulmod({}, {}, PRIME)" , a.soldity_encode(memory_layout), b.soldity_encode(memory_layout)),
            Inv(_) => memory_layout.get(self).unwrap().clone(),
            Exp(a, e) => {
                match e {
                    0 =>  "0x01".to_owned(),
                    1 => a.soldity_encode(memory_layout),
                    _ => {
                        // TODO - Check the gas to see what the real breaking point should be
                        if *e < 10 {
                            format!("small_expmod({}, {}, PRIME)", a.soldity_encode(memory_layout), e.to_string())
                        } else {
                            format!("expmod({}, {}, PRIME)", a.soldity_encode(memory_layout), e.to_string())
                        }
                    }
                }
            },
        }
    }

    // TODO - DRY this by writing a generic search over subtypes

    pub fn trace_search(&self) -> HashMap::<RationalExpression, bool> {
        use RationalExpression::*;

        match self {
            X => HashMap::new(),
            Constant(_) => HashMap::new(),
            Trace(_, _) => [(self.clone(), true)].iter().cloned().collect(),
            Polynomial(_, a) => a.trace_search(),
            Add(a, b) => { let mut first = a.trace_search();
                            first.extend(b.trace_search());
                            first},
            Neg(a) => a.trace_search(),
            Mul(a, b) => { let mut first = a.trace_search();
                            first.extend(b.trace_search());
                            first},
            Inv(a) => a.trace_search(),
            Exp(a, _) => a.trace_search(),
        }
    }

    pub fn inv_search(&self) -> HashMap::<RationalExpression, bool> {
        use RationalExpression::*;

        match self {
            X => HashMap::new(),
            Constant(_) => HashMap::new(),
            Trace(_, _) => HashMap::new(),
            Polynomial(_, a) => a.inv_search(),
            Add(a, b) => { let mut first = a.inv_search();
                            first.extend(b.inv_search());
                            first},
            Neg(a) => a.inv_search(),
            Mul(a, b) => { let mut first = a.inv_search();
                            first.extend(b.inv_search());
                            first},
            Inv(_) => [(self.clone(), true)].iter().cloned().collect(),
            Exp(a, _) => a.inv_search(),
        }
    }

    pub fn periodic_search(&self) -> HashMap::<RationalExpression, bool> {
        use RationalExpression::*;

        match self {
            X => HashMap::new(),
            Constant(_) => HashMap::new(),
            Trace(_, _) => HashMap::new(),
            Polynomial(_, _) => [(self.clone(), true)].iter().cloned().collect(),
            Add(a, b) => { let mut first = a.periodic_search();
                            first.extend(b.periodic_search());
                            first},
            Neg(a) => a.periodic_search(),
            Mul(a, b) => { let mut first = a.periodic_search();
                            first.extend(b.periodic_search());
                            first},
            Inv(a) => a.periodic_search(),
            Exp(a, _) => a.periodic_search(),
        }
    }
}
