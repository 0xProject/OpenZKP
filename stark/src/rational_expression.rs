use primefield::FieldElement;
use std::ops::{Add, Div, Index, Mul, Sub};

#[derive(Clone, Debug)]
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

impl RationalExpression {
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
        }
    }

    pub fn eval(
        &self,
        trace_table: &Fn(usize, &FieldElement) -> FieldElement,
        x: &FieldElement,
        g: &FieldElement,
    ) -> FieldElement {
        use RationalExpression::*;
        match self {
            X => x.clone(),
            Constant(value) => value.clone(),
            Trace(&i, &j) => trace_table(i, x * g.pow(j)),
            Add(a, b) => a.eval(trace_table, x, g) + b.eval(trace_table, x, g),
            Neg(a) => -&a.eval(trace_table, x, g),
            Inv(a) => {
                a.eval(trace_table, x, g)
                    .inv()
                    .expect("Division by zero while evaluating RationalExpression.")
            }
            _ => unimplemented!(),
        }
    }
}

// Constraints

struct Constraints {
    trace_degree: usize,
    num_columns:  usize,
    constraints:  Vec<RationalExpression>,
}

impl Constraints {
    pub fn combine(&mut self, coefficients: &[FieldElement]) {
        assert_eq!(coefficients.len(), 2 * self.constraints.len());
    }
}

// Trace table

struct TraceTable {
    trace_length: usize,
    num_columns:  usize,
    values:       Vec<FieldElement>,
}

impl TraceTable {
    pub fn new(trace_length: usize, num_columns: usize) -> TraceTable {
        TraceTable {
            trace_length,
            num_columns,
            values: vec![FieldElement::ZERO; trace_length * num_columns],
        }
    }
}

impl Index<usize> for TraceTable {
    type Output = [FieldElement];

    fn index(&self, i: usize) -> &[FieldElement] {
        self.values[i * self.num_columns..(i + 1) * self.num_columns]
    }
}

impl Index<(usize, usize)> for TraceTable {
    type Output = FieldElement;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        self.values[i * self.num_columns + j]
    }
}

// Constraint System

pub trait ConstraintSystem {
    type PublicInput;
    type PrivateInput;

    fn constraints(public_input: &Self::PublicInput) -> Constraints;

    fn trace(public_inputs: &Self::PublicInput, private_input: &Self::PrivateInput) -> TraceTable;
}
/// The fibonacci constraint system.
///
/// The public inputs are an index and value in the fibonacci sequence.
struct Fibonacci;

impl ConstraintSystem for Fibonacci {
    type PrivateInput = FieldElement;
    type PublicInput = (usize, FieldElement);

    fn constraints((&index, value): &Self::PublicInput) -> Constraints {
        use RationalExpression::*;

        // Trace table generation
        let trace_degree = 1024usize;
        let num_columns = 2usize;
        let g = Constant(
            FieldElement::root(trace_degree.into()).expect("No root of unity for trace degree."),
        );
        assert!(index < trace_degree);

        // Constraint repetitions
        let first_row = RationalExpression::from(1) / (X - 1.into());
        let target_row = RationalExpression::from(1) / (X - g.pow(index));
        let every_row = (X - g.pow(trace_degree)) / (X.pow(trace_degree) - 1.into());

        // The system
        Constraints {
            trace_degree,
            num_columns,
            constraints: vec![
                (Trace(0, 0) - 1.into()) * first_row,
                (Trace(1, 0) - value.into()) * target_row,
                (Trace(0, 1) - Trace(1, 0)) * every_row,
                (Trace(1, 1) - Trace(0, 0) - Trace(1, 0)) * every_row,
            ],
        }
    }

    fn trace(public_inputs: &Self::PublicInput, private_input: &FieldElement) -> TraceTable {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

}
