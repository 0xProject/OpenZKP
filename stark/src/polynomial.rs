use crate::proofs::geometric_series;
use primefield::FieldElement;
use rayon::{iter::repeatn, prelude::*};
use std::{
    cmp::max,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};
use u256::U256;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Polynomial(Vec<FieldElement>);

// TODO: create a canonical representation for polynonials based on vectors with
// power of two lengths.
impl Polynomial {
    pub fn new(coefficients: &[FieldElement]) -> Self {
        let mut coefficients = coefficients.to_vec();
        coefficients.reverse();
        Self(coefficients)
    }

    // TODO: turn these two into macros which accepts negative values as well.
    pub fn from_dense(c: &[usize]) -> Self {
        debug_assert!(!c.is_empty());
        debug_assert!(c[c.len() - 1] != 0);

        let coefficients: Vec<FieldElement> = c
            .iter()
            .rev()
            .map(|x| FieldElement::from(U256::from(*x as u64)))
            .collect();
        Self(coefficients)
    }

    pub fn from_sparse(degrees_and_coefficients: &[(usize, usize)]) -> Self {
        let mut max_degree = 0usize;
        for (degree, _) in degrees_and_coefficients.iter() {
            if max_degree < *degree {
                max_degree = *degree;
            }
        }

        let mut coefficients = vec![0usize; max_degree + 1];
        for (degree, coefficient) in degrees_and_coefficients.iter() {
            coefficients[*degree] = *coefficient;
        }
        Polynomial::from_dense(&coefficients)
    }

    #[inline(always)]
    fn coefficients(self: &Self) -> &[FieldElement] {
        &self.0
    }

    pub fn evaluate(self: &Self, x: &FieldElement) -> FieldElement {
        let mut result = FieldElement::ZERO;
        for coefficient in self.coefficients().iter() {
            result *= x;
            result += coefficient;
        }
        result
    }

    pub fn shift(self: &Self, step: &FieldElement) -> Self {
        let mut coefficients = self.0.clone();
        let shift_factors = geometric_series(&FieldElement::ONE, step, self.0.len());
        for (coefficient, shift_factor) in coefficients.iter_mut().zip(shift_factors.iter().rev()) {
            *coefficient *= shift_factor;
        }
        Self(coefficients)
    }

    fn aligned_coefficients<'a>(
        p_1: &'a Polynomial,
        p_2: &'a Polynomial,
    ) -> impl IndexedParallelIterator<Item = (&'a FieldElement, &'a FieldElement)> {
        let mut padding_1 = 0;
        let mut padding_2 = 0;
        if p_1.0.len() < p_2.0.len() {
            padding_1 = p_2.0.len() - p_1.0.len();
        } else {
            padding_2 = p_1.0.len() - p_2.0.len();
        }
        repeatn(&FieldElement::ZERO, padding_1)
            .chain(p_1.0.par_iter())
            .zip(repeatn(&FieldElement::ZERO, padding_2).chain(p_2.0.par_iter()))
    }

    fn extend_to_length(self: &Self, degree_difference: usize) -> Self {
        let mut coefficients = self.0.clone();
        coefficients.extend_from_slice(&vec![FieldElement::ZERO; degree_difference]);
        Self(coefficients)
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = Vec::with_capacity(max(self.0.len(), other.0.len()));
        Polynomial::aligned_coefficients(&self, &other)
            .map(|(x, y)| x + y)
            .collect_into_vec(&mut result);
        Polynomial(result)
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = Vec::with_capacity(max(self.0.len(), other.0.len()));
        Polynomial::aligned_coefficients(&self, &other)
            .map(|(x, y)| x - y)
            .collect_into_vec(&mut result);
        Polynomial(result)
    }
}

impl SubAssign<&Self> for Polynomial {
    fn sub_assign(self: &mut Self, other: &Self) {
        let mut result = Vec::with_capacity(max(self.0.len(), other.0.len()));
        Polynomial::aligned_coefficients(self, other)
            .map(|(x, y)| x - y)
            .collect_into_vec(&mut result);
        *self = Polynomial(result)
    }
}

impl AddAssign<&Polynomial> for Polynomial {
    fn add_assign(self: &mut Self, other: &Polynomial) {
        let mut result = Vec::with_capacity(max(self.0.len(), other.0.len()));
        Polynomial::aligned_coefficients(self, other)
            .map(|(x, y)| x + y)
            .collect_into_vec(&mut result);
        *self = Polynomial(result)
    }
}

impl Mul<&Polynomial> for &FieldElement {
    type Output = Polynomial;

    fn mul(self, other: &Polynomial) -> Polynomial {
        let mut result = other.clone();
        let _: Vec<_> = result.0.par_iter_mut().map(|x| *x *= self).collect();
        result
    }
}

// TODO: use fft for Mul and Div if appropriate.
// https://stackoverflow.com/questions/44770632/fft-division-for-fast-polynomial-division
#[allow(clippy::suspicious_arithmetic_impl)]
impl Mul<Polynomial> for Polynomial {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.0.is_empty() || other.0.is_empty() {
            return Polynomial(vec![]);
        }
        let mut result = Polynomial(vec![]);
        for coefficient in other.coefficients().iter() {
            result.0.push(FieldElement::ZERO);
            result += &(coefficient * &self);
        }
        result
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div<Polynomial> for Polynomial {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let degree_difference = self.0.len() - other.0.len();
        let inverse_leading_term = other.0[0].inv().expect("Cannot divide by zero polynomial");
        let mut remainder = self.clone();
        let mut other = other.extend_to_length(degree_difference);
        let mut result = vec![];
        for i in 0..=degree_difference {
            let q = &remainder.0[i] * &inverse_leading_term;
            remainder -= &(&q * &other);
            result.push(q);
            other.0.pop();
        }
        // TODO: panic if remainder is not zero?
        Polynomial(result)
    }
}

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
#[cfg(test)]
impl Arbitrary for Polynomial {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Polynomial(Vec::<FieldElement>::arbitrary(g))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::*;
    use quickcheck_macros::quickcheck;
    use u256::{u256h, U256};

    #[test]
    fn example_evaluate() {
        let x = FieldElement::from(u256h!(
            "04d59eebac89518453d226545efb550870f641831aaf0ed1fa2ec54499eb2183"
        ));
        let mut coef = Vec::with_capacity(100);
        for i in 0..100 {
            coef.push(FieldElement::from(U256::from(123_456_u64 + i)));
        }
        let res = Polynomial::new(&coef).evaluate(&x);
        assert_eq!(
            U256::from(res),
            u256h!("00e2d3ab8631086d0680da2c28d48b4b5248c0484eae8a04f39c646483d09f09")
        );
    }

    #[test]
    fn example_sum() {
        let p_1 = Polynomial::from_dense(&[1, 2, 1]);
        let p_2 = Polynomial::from_dense(&[1, 2, 6, 7]);
        assert_eq!(p_1 + p_2, Polynomial::from_dense(&[2, 4, 7, 7]))
    }

    #[test]
    fn example_difference() {
        let p_1 = Polynomial::from_dense(&[1, 2, 10, 7]);
        let p_2 = Polynomial::from_dense(&[1, 2, 6]);
        assert_eq!(p_1 - p_2, Polynomial::from_dense(&[0, 0, 4, 7]))
    }

    #[test]
    fn example_product() {
        let p_1 = Polynomial::from_dense(&[1, 1]);
        let p_2 = Polynomial::from_dense(&[1, 1]);
        assert_eq!(p_1 * p_2, Polynomial::from_dense(&[1, 2, 1]))
    }

    #[test]
    fn example_division() {
        let numerator = Polynomial::from_dense(&[1, 3, 3, 1]);
        let denominator = Polynomial::from_dense(&[1, 1]);
        assert_eq!(numerator / denominator, Polynomial::from_dense(&[1, 2, 1]))
    }

    #[test]
    fn example_shift() {
        let p = Polynomial::from_dense(&[3, 2, 1]);
        assert_eq!(
            p.shift(&(FieldElement::from(U256::from(2u64)))),
            Polynomial::from_dense(&[3, 4, 4])
        )
    }

    #[quickcheck]
    fn product_evaluation_equivalence(x: FieldElement, a: Polynomial, b: Polynomial) -> bool {
        a.evaluate(&x) * b.evaluate(&x) == (a * b).evaluate(&x)
    }

    #[quickcheck]
    fn sum_commutivity(a: Polynomial, b: Polynomial) -> bool {
        a.clone() + b.clone() == b + a
    }

    #[quickcheck]
    fn product_commutivity(a: Polynomial, b: Polynomial) -> bool {
        a.clone() * b.clone() == b * a
    }

    #[quickcheck]
    fn distributivity(a: Polynomial, b: Polynomial, c: Polynomial) -> bool {
        a.clone() * (b.clone() + c.clone()) == a.clone() * b + a * c
    }

    #[quickcheck]
    fn shift_definition(a: Polynomial, shift: FieldElement, x: FieldElement) -> bool {
        a.shift(&shift).evaluate(&x) == a.evaluate(&(x * shift))
    }

    // TODO: fix this.
    #[quickcheck]
    fn addition_subtration_inverse(a: Polynomial, b: Polynomial) -> bool {
        a.clone() + b.clone() - b == a
    }

    #[quickcheck]
    fn division_multiplication_inverse(a: Polynomial, b: Polynomial) -> bool {
        // TODO remove these once we have a canonical representation for polynomials.
        if a.0.is_empty() || b.0.is_empty() {
            return true;
        }
        if b.0[0].is_zero() {
            return true;
        }
        a.clone() * b.clone() / b == a
    }
}
