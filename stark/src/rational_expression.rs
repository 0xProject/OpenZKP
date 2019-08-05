use primefield::FieldElement;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
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

impl From<i32> for RationalExpression {
    fn from(value: i32) -> Self {
        RationalExpression::Constant(value.into())
    }
}

impl From<&FieldElement> for RationalExpression {
    fn from(value: FieldElement) -> Self {
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
            Trace(_i, exp) => {
                let (n, d) = exp.degree(trace_degree);
                assert!(d == 0); // TODO: Is there a meaningful use?
                (n * trace_degree, 0)
            }
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
            _ => unimplemented!(),
        }
    }

    /// Returns the number of columns.
    pub fn columns(&self) -> usize {
        unimplemented!() // TODO
    }

    pub fn eval(
        &self,
        trace_table: &Fn(usize, &FieldElement) -> FieldElement,
        x: &FieldElement,
    ) -> FieldElement {
        use RationalExpression::*;
        match self {
            X => x.clone(),
            Constant(value) => value.clone(),
            Trace(i, exp) => trace_table(*i, &exp.eval(trace_table, x)),
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
}

struct Constraints {
    trace_degree: usize,
    constraints:  Vec<RationalExpression>,
}

/// The fibonacci constraint system.
///
/// The public inputs are an index and value in the fibonacci sequence.
#[allow(dead_code)] // TODO
pub fn fibonacci(index: usize, value: &FieldElement) -> Constraints {
    use RationalExpression::*;

    // Trace table generation
    let trace_degree = 1024usize;
    let g = Constant(
        FieldElement::root(trace_degree.into()).expect("No root of unity for trace degree."),
    );
    assert!(index < trace_degree);

    // Constraint repetitions
    let first_row = RationalExpression::from(1) / (X - 1.into());
    let target_row = RationalExpression::from(1) / (X - Exp(Box::new(g), index));
    let every_row = (X - Exp(Box::new(g), trace_degree)) / (X - Exp(Box::new(g), index));

    // The system
    Constraints {
        trace_degree,
        constraints: vec![
            (Trace(0, 0) - 1.into()) * first_row,
            (Trace(1, 0) - value.into()) * target_row,
            (Trace(0, 1) - Trace(1, 0)) * every_row,
            (Trace(1, 1) - Trace(0, 0) - Trace(1, 0)) * every_row,
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn eval_fib() {
        let constraints = fibonacci(1000, &FieldElement::ONE);
    }
}
