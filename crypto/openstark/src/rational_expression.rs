use crate::polynomial::{DensePolynomial, SparsePolynomial};
use primefield::FieldElement;
use std::{
    cmp::{max, Ordering},
    collections::BTreeMap,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone, Debug)]
pub enum RationalExpression {
    X,
    Constant(FieldElement),
    Trace(usize, isize),
    PeriodicColumn(SparsePolynomial),
    Add(Box<RationalExpression>, Box<RationalExpression>),
    Sub(Box<RationalExpression>, Box<RationalExpression>),
    Mul(Box<RationalExpression>, Box<RationalExpression>),
    Div(Box<RationalExpression>, Box<RationalExpression>),
    Pow(Box<RationalExpression>, usize),
}

#[derive(Clone, Debug)]
pub struct RationalExpressionStruct {
    pub numerator:   RationalExpression, // cannot have div
    pub denominator: RationalExpression,
}

impl RationalExpressionStruct {
    pub fn pow(&self, n: usize) -> Self {
        RationalExpressionStruct {
            numerator:   self.numerator.pow(n),
            denominator: self.denominator.pow(n),
        }
    }
}

impl Add for RationalExpressionStruct {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        RationalExpressionStruct {
            numerator:   self.numerator * other.denominator.clone()
                + other.numerator * self.denominator.clone(),
            denominator: self.denominator * other.denominator,
        }
    }
}

impl Sub for RationalExpressionStruct {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        RationalExpressionStruct {
            numerator:   self.numerator * other.denominator.clone()
                - other.numerator * self.denominator.clone(),
            denominator: self.denominator * other.denominator,
        }
    }
}

impl Mul for RationalExpressionStruct {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        RationalExpressionStruct {
            numerator:   self.numerator * other.numerator,
            denominator: self.denominator * other.denominator,
        }
    }
}

impl Div for RationalExpressionStruct {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        RationalExpressionStruct {
            numerator:   self.numerator * other.denominator,
            denominator: self.denominator * other.numerator,
        }
    }
}

// Effectively a sparse polynomial!
#[derive(Clone, Debug)]
struct GroupedRationalExpression(pub BTreeMap<Vec<(usize, isize)>, RationalExpressionStruct>);

impl GroupedRationalExpression {
    pub fn new(
        key: Vec<(usize, isize)>,
        value: RationalExpressionStruct,
    ) -> GroupedRationalExpression {
        let mut map = BTreeMap::new();
        map.insert(key, value);
        GroupedRationalExpression(map)
    }

    fn pow(&self, n: usize) -> Self {
        let mut result = self.clone();
        let r: RationalExpressionStruct = self.0.get(&vec![]).unwrap().clone();
        GroupedRationalExpression::new(vec![], r.pow(n))
    }

    fn add(a: Self, b: Self) -> Self {
        let mut result = a.0.clone();
        for (indices, coefficient) in b.0 {
            let c = result.get(&indices);
            match c {
                Some(c_a) => result.insert(indices, c_a.clone() + coefficient),
                None => result.insert(indices, coefficient),
            };
        }
        GroupedRationalExpression(result)
    }

    fn sub(a: Self, b: Self) -> Self {
        let mut result = a.0.clone();
        for (indices, coefficient) in b.0 {
            let c = result.get(&indices);
            match c {
                Some(c_a) => result.insert(indices, c_a.clone() - coefficient),
                None => {
                    result.insert(
                        indices,
                        RationalExpressionStruct {
                            numerator:   0.into(),
                            denominator: 1.into(),
                        } - coefficient,
                    )
                }
            };
        }
        GroupedRationalExpression(result)
    }

    fn mul(a: Self, b: Self) -> Self {
        let mut result: BTreeMap<Vec<(usize, isize)>, RationalExpressionStruct> = BTreeMap::new();
        for (indices, coefficient) in a.0 {
            for (other_indices, other_coefficient) in b.0.clone() {
                let mut new_indices = [&indices[..], &other_indices[..]].concat();
                new_indices.sort();

                let c = result.get(&new_indices);
                match c {
                    Some(existing_coefficient) => {
                        result.insert(
                            new_indices,
                            existing_coefficient.clone()
                                + coefficient.clone() * other_coefficient.clone(),
                        )
                    }
                    None => {
                        result.insert(new_indices, coefficient.clone() * other_coefficient.clone())
                    }
                };
            }
        }
        Self(result)
    }

    fn div(numerator: Self, denominator: Self) -> Self {
        let keys: Vec<_> = denominator.0.keys().collect();
        assert_eq!(keys[0].len(), 0);
        let divisor = denominator.0.get(&vec![]).unwrap();

        let mut result: BTreeMap<Vec<(usize, isize)>, RationalExpressionStruct> = BTreeMap::new();
        for (indices, coefficient) in numerator.0 {
            result.insert(indices, coefficient / divisor.clone());
        }
        Self(result)
    }
}

impl From<RationalExpression> for GroupedRationalExpression {
    fn from(value: RationalExpression) -> Self {
        use RationalExpression::*;
        match value {
            X => {
                GroupedRationalExpression::new(vec![], RationalExpressionStruct {
                    numerator:   X,
                    denominator: 1.into(),
                })
            }
            Constant(c) => {
                GroupedRationalExpression::new(vec![], RationalExpressionStruct {
                    numerator:   Constant(c),
                    denominator: 1.into(),
                })
            }
            Trace(i, j) => {
                GroupedRationalExpression::new(vec![(i, j)], RationalExpressionStruct {
                    numerator:   1.into(),
                    denominator: 1.into(),
                })
            }
            PeriodicColumn(p) => {
                GroupedRationalExpression::new(vec![], RationalExpressionStruct {
                    numerator:   PeriodicColumn(p),
                    denominator: 1.into(),
                })
            }
            Add(a, b) => Self::add((*a).into(), (*b).into()),
            Sub(a, b) => Self::sub((*a).into(), (*b).into()),
            Mul(a, b) => Self::mul((*a).into(), (*b).into()),
            Div(a, b) => Self::div((*a).into(), (*b).into()), // should this be being called twice?
            Pow(a, n) => GroupedRationalExpression::from(*a).pow(n),
        }
    }
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

impl Div for RationalExpression {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        RationalExpression::Div(Box::new(self), Box::new(other))
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
            PeriodicColumn(p) => p.degree(),
            Add(a, b) => max(a.degree(trace_degree), b.degree(trace_degree)),
            Sub(a, b) => max(a.degree(trace_degree), b.degree(trace_degree)),
            Mul(a, b) => a.degree(trace_degree) + b.degree(trace_degree),
            Pow(a, n) => n * a.degree(trace_degree),
            Div(a, b) => a.degree(trace_degree) - b.degree(trace_degree),
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
            PeriodicColumn(p) => p.evaluate(x),
            Add(a, b) => a.eval(trace_table, x) + b.eval(trace_table, x),
            Sub(a, b) => a.eval(trace_table, x) - b.eval(trace_table, x),
            Mul(a, b) => a.eval(trace_table, x) * b.eval(trace_table, x),
            Div(a, b) => a.eval(trace_table, x) / b.eval(trace_table, x),
            Pow(a, n) => a.eval(trace_table, x).pow(*n),
        }
    }

    pub fn eval_on_domain(
        &self,
        trace_table: &dyn Fn(usize, isize) -> DensePolynomial,
    ) -> DensePolynomial {
        let grouped = GroupedRationalExpression::from(self.clone());
        let mut result = DensePolynomial::new(&[FieldElement::ZERO]);
        for (indices, coefficients) in grouped.0 {
            // assert!(indices.len() <= 1);
            let product = indices
                .iter()
                .fold(DensePolynomial::new(&[FieldElement::ONE]), |x, (i, j)| {
                    x * trace_table(*i, *j)
                });
            println!("indices: {:?}", indices);
            println!("numerator: {:?}", coefficients.numerator.get_denominator());
            let x: DensePolynomial =
                product.clone() * coefficients.numerator.clone().get_denominator();
            println!(
                "denominator {:?}",
                coefficients.denominator.get_denominator()
            );
            let y = x / coefficients.denominator.get_denominator();
            result += product * coefficients.numerator.clone().get_denominator()
                / coefficients.denominator.get_denominator();
        }
        result
    }

    pub fn get_denominator(&self) -> SparsePolynomial {
        match self {
            Self::X => SparsePolynomial::new(&[(FieldElement::ONE, 1)]),
            Self::Constant(c) => SparsePolynomial::new(&[(c.clone(), 0)]),
            Self::PeriodicColumn(p) => p.clone(),
            Self::Add(a, b) => a.get_denominator() + b.get_denominator(),
            Self::Sub(a, b) => a.get_denominator() - b.get_denominator(),
            Self::Mul(a, b) => a.get_denominator() * b.get_denominator(),
            Self::Pow(a, n) => {
                match &**a {
                    // lol i can't believe this works
                    Self::X => SparsePolynomial::new(&[(FieldElement::ONE, *n)]),
                    Self::Constant(c) => SparsePolynomial::new(&[(c.pow(*n), 0)]),
                    _ => panic!(),
                }
            }
            Self::Div(..) => panic!(),
            Self::Trace(..) => panic!(),
        }
    }
}
