use crate::polynomial::DensePolynomial;
#[cfg(feature = "std")]
use std::collections::HashMap;
use std::{
    collections::BTreeSet,
    hash::{Hash, Hasher},
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
    prelude::v1::*,
};
use zkp_macros_decl::field_element;
use zkp_primefield::FieldElement;
use zkp_u256::U256;

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
        }
    }

    #[cfg(feature = "std")]
    pub fn soldity_encode(&self, memory_layout: &HashMap<Self, String>) -> String {
        use RationalExpression::*;

        #[allow(clippy::match_same_arms)]
        match self {
            X => "mload(0)".to_owned(),
            Constant(_) if memory_layout.contains_key(self) => {
                memory_layout.get(self).unwrap().clone()
            }
            Constant(c) => format!("0x{}", U256::from(c).to_string()),
            Trace(..) | Polynomial(..) => memory_layout.get(self).unwrap().clone(),
            Add(a, b) => {
                format!(
                    "addmod({}, {}, PRIME)",
                    a.soldity_encode(memory_layout),
                    b.soldity_encode(memory_layout)
                )
            }
            Neg(a) => format!("sub(PRIME , {})", a.soldity_encode(memory_layout)),
            Mul(a, b) => {
                format!(
                    "mulmod({}, {}, PRIME)",
                    a.soldity_encode(memory_layout),
                    b.soldity_encode(memory_layout)
                )
            }
            Inv(_) => memory_layout.get(self).unwrap().clone(),
            Exp(a, e) => {
                match e {
                    0 => "0x01".to_owned(),
                    1 => a.soldity_encode(memory_layout),
                    _ => {
                        // TODO - Check the gas to see what the real breaking point should be
                        if *e < 10 {
                            format!(
                                "small_expmod({}, {}, PRIME)",
                                a.soldity_encode(memory_layout),
                                e.to_string()
                            )
                        } else {
                            format!(
                                "expmod({}, {}, PRIME)",
                                a.soldity_encode(memory_layout),
                                e.to_string()
                            )
                        }
                    }
                }
            }
        }
    }

    // TODO - DRY this by writing a generic search over subtypes
    #[cfg(feature = "std")]
    pub fn trace_search(&self) -> HashMap<Self, bool> {
        use RationalExpression::*;

        match self {
            X | Constant(..) => HashMap::new(),
            Trace(..) => [(self.clone(), true)].iter().cloned().collect(),
            Add(a, b) | Mul(a, b) => {
                let mut first = a.trace_search();
                first.extend(b.trace_search());
                first
            }
            Polynomial(_, a) | Inv(a) | Exp(a, _) | Neg(a) => a.trace_search(),
        }
    }

    #[cfg(feature = "std")]
    pub fn inv_search(&self) -> HashMap<Self, bool> {
        use RationalExpression::*;

        match self {
            X | Constant(_) | Trace(..) => HashMap::new(),
            Add(a, b) | Mul(a, b) => {
                let mut first = a.inv_search();
                first.extend(b.inv_search());
                first
            }
            Inv(_) => [(self.clone(), true)].iter().cloned().collect(),
            Polynomial(_, a) | Exp(a, _) | Neg(a) => a.inv_search(),
        }
    }

    #[cfg(feature = "std")]
    pub fn periodic_search(&self) -> HashMap<Self, bool> {
        use RationalExpression::*;

        match self {
            X | Constant(_) | Trace(..) => HashMap::new(),
            Polynomial(..) => [(self.clone(), true)].iter().cloned().collect(),
            Add(a, b) | Mul(a, b) => {
                let mut first = a.periodic_search();
                first.extend(b.periodic_search());
                first
            }
            Inv(a) | Exp(a, _) | Neg(a) => a.periodic_search(),
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
            Polynomial(p, a) => {
                "poly".hash(state);
                let x = field_element!(
                    "754ed488ec9208d1c552bb254c0890042078a9e1f7e36072ebff1bf4e193d11b"
                );
                (p.evaluate(&x)).hash(state);
                a.hash(state);
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
        }
    }
}
