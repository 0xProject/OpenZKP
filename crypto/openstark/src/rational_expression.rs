use crate::{polynomial::DensePolynomial, trace_table::TraceTable};
use primefield::FieldElement;
use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone)]
pub enum RationalExpression {
    X,
    Constant(FieldElement),
    Trace(usize, isize),
    Add(Box<RationalExpression>, Box<RationalExpression>),
    Neg(Box<RationalExpression>),
    Mul(Box<RationalExpression>, Box<RationalExpression>),
    Inv(Box<RationalExpression>),
    Exp(Box<RationalExpression>, usize),
    Poly(DensePolynomial, Box<RationalExpression>),

    // TODO: Non-static lifetime.
    // TODO: Include evaluation domain info in lookup.
    Lookup(Box<RationalExpression>, &'static [FieldElement]),
}

impl std::fmt::Debug for RationalExpression {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RationalExpression::*;
        match self {
            X => write!(fmt, "X"),
            Constant(a) => write!(fmt, "{:?}", a),
            Trace(i, j) => write!(fmt, "Trace({}, {})", i, j),
            Add(a, b) => write!(fmt, "({:?} + {:?}", a, b),
            Neg(a) => write!(fmt, "-{:?}", a),
            Mul(a, b) => write!(fmt, "({:?} * {:?})", a, b),
            Inv(a) => write!(fmt, "1/{:?}", a),
            Exp(a, e) => write!(fmt, "{:?}^{:?}", a, e),
            Poly(_, a) => write!(fmt, "P({:?})", a),
            Lookup(a, _) => write!(fmt, "Lookup({:?})", a),
        }
    }
}

impl RationalExpression {
    pub fn neg(&self) -> RationalExpression {
        RationalExpression::Neg(Box::new(self.clone()))
    }

    pub fn inv(&self) -> RationalExpression {
        RationalExpression::Inv(Box::new(self.clone()))
    }

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
        self + other.neg()
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
            Poly(p, a) => {
                let (n, d) = a.degree(trace_degree);
                (p.degree() * n, p.degree() * d)
            }
            Lookup(a, _) => a.degree(trace_degree),
        }
    }

    /// Constant propagatate
    pub fn constant_propagate(self) -> Self {
        use RationalExpression::*;
        match self {
            Add(a, b) => {
                let a = a.constant_propagate();
                let b = b.constant_propagate();
                match (a, b) {
                    (Constant(a), Constant(b)) => Constant(a + b),
                    (a, b) => a + b,
                }
            }
            Mul(a, b) => {
                let a = a.constant_propagate();
                let b = b.constant_propagate();
                match (a, b) {
                    (Constant(a), Constant(b)) => Constant(a * b),
                    (a, b) => a * b,
                }
            }
            Neg(a) => {
                let a = a.constant_propagate();
                match a {
                    // TODO: impl std::ops::Neg for FieldElement
                    Constant(a) => Constant(FieldElement::ZERO - a),
                    a => a.neg(),
                }
            }
            Inv(a) => {
                let a = a.constant_propagate();
                match a {
                    Constant(a) => Constant(a.inv().expect("Division by zero.")),
                    a => a.inv(),
                }
            }
            Exp(a, e) => {
                let a = a.constant_propagate();
                match a {
                    Constant(a) => Constant(a.pow(e)),
                    a => a.pow(e),
                }
            }
            e => e,
        }
    }

    /// If the expression depends only on x, return the value for some x
    pub fn eval_x(&self, x: &FieldElement) -> Option<FieldElement> {
        // TODO
        unimplemented!()
    }

    // TODO: Simplify: constant propagation, 0 + a, 0 * a, 1 * a, neg(neg(a)), a^0,
    // a^1 inv(inv(a)). Maybe even 2*a => a+a, though for this we'd like graphs so
    // we can do CSE.

    // TODO: Factor out parts that depend only on X (periodic columns) and
    // pre-compute them. Observe that denominators tend to depend only on X, so
    // we avoid a lot of inversions this way. Note that lookups are not cheap
    // though, and sometimes evaluating X may be cheaper than a lookup. ->
    // Benchmark.

    pub fn eval(
        &self,
        trace_table: &TraceTable,
        row: (usize, usize),
        x: &FieldElement,
    ) -> FieldElement {
        use RationalExpression::*;
        match self {
            X => x.clone(),
            Constant(value) => value.clone(),
            Trace(i, o) => {
                let n = trace_table.num_rows() as isize;
                // OPT: Instead of the row.0 factor we can pass a non-oversampled
                // trace table. Multiple cosets are completely indpendent from
                // RationalExpression's perspective. This should give better
                // cache locality. Lookup will need to be changed though.
                let row = ((n + (row.1 as isize) + (row.0 as isize) * *o) % n) as usize;
                trace_table[(row, *i)].clone()
            }
            Add(a, b) => a.eval(trace_table, row, x) + b.eval(trace_table, row, x),
            Neg(a) => -&a.eval(trace_table, row, x),
            Mul(a, b) => a.eval(trace_table, row, x) * b.eval(trace_table, row, x),
            Inv(a) => {
                a.eval(trace_table, row, x)
                    .inv()
                    .expect("Division by zero while evaluating RationalExpression.")
            }
            Exp(a, i) => a.eval(trace_table, row, x).pow(*i),
            Poly(p, a) => p.evaluate(&a.eval(trace_table, row, x)),
            Lookup(_, t) => t[row.1].clone(),
        }
    }
}
