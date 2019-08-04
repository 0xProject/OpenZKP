use crate::proofs::geometric_series;
use crate::fft::{fft, ifft};
use primefield::FieldElement;
use rayon::{iter::repeatn, prelude::*};
use std::{
    cmp::max,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct Polynomial(Vec<FieldElement>);

// TODO: create a canonical representation for polynonials based on vectors with
// power of two lengths.
impl Polynomial {
    pub fn new(coefficients: &[FieldElement]) -> Self {
        let mut coefficients = coefficients.to_vec();
        coefficients.reverse();
        Self(coefficients)
    }

    pub fn from_sparse(degrees_and_coefficients: &[(usize, FieldElement)]) -> Self {
        let mut max_degree = 0usize;
        for (degree, _) in degrees_and_coefficients.iter() {
            if max_degree < *degree {
                max_degree = *degree;
            }
        }

        let mut coefficients = vec![FieldElement::ZERO; max_degree + 1];
        for (degree, coefficient) in degrees_and_coefficients.iter() {
            coefficients[*degree] = coefficient.clone();
        }
        Self::new(&coefficients)
    }

    pub fn periodic(coefficients: &[FieldElement], repetitions: usize) -> Self {
        let mut periodic_coefficients = vec![];
        for coefficient in coefficients {
            periodic_coefficients.push(coefficient.clone());
            periodic_coefficients.extend_from_slice(&vec![FieldElement::ZERO; repetitions - 1]);
        }
        Self::new(&periodic_coefficients)
    }

    #[inline(always)]
    pub fn constant(coefficient: FieldElement) -> Self {
        Self::new(&[coefficient])
    }

    #[inline(always)]
    pub fn len(self: &Self) -> usize {
        self.0.len()
    }

    #[inline(always)]
    pub fn reverse_coefficients(self: &Self) -> Vec<FieldElement> {
        let mut reverse_coefficients = self.0.clone();
        reverse_coefficients.reverse();
        reverse_coefficients
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

    #[inline(always)]
    fn coefficients(self: &Self) -> &[FieldElement] {
        &self.0
    }

    fn is_zero(&self) -> bool {
        self.coefficients().par_iter().all(|c| c.is_zero())
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

    fn extend_to_length(&self, degree_difference: usize) -> Self {
        let mut coefficients = self.0.clone();
        coefficients.extend_from_slice(&vec![FieldElement::ZERO; degree_difference]);
        Self(coefficients)
    }

    fn divide_by_x(&mut self) {
        self.0 = self.0[..self.len() - 1].to_vec();
    }

    pub fn multiply_by_x(&mut self, degree: usize) {
        self.0.extend_from_slice(&vec![FieldElement::ZERO; degree]);
    }

    fn subtract_at(&mut self, other: &Polynomial, offset: usize, factor: &FieldElement) {
        //     for (i, coefficient) in other.coefficients().iter().enumerate() {
        //         self.0[i + offset] -= factor * coefficient;
        //     }
        self.0[offset] -= factor * &other.coefficients()[0];

        let other_length = other.len();
        self.0[offset + other_length - 1] -= factor * &other.coefficients()[other_length - 1];
    }

    fn pad(&self, length: usize) -> Self {
        let mut coefficients =
            vec![FieldElement::ZERO; (length + 1).next_power_of_two() - self.len()];
        coefficients.extend_from_slice(self.coefficients());
        Self(coefficients)
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        Polynomial::aligned_coefficients(self, other)
            .map(|(x, y)| x - y)
            .all(|c| c.is_zero())
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
        let mut result = Vec::with_capacity(max(self.len(), other.len()));
        Polynomial::aligned_coefficients(self, other)
            .map(|(x, y)| x - y)
            .collect_into_vec(&mut result);
        *self = Polynomial(result)
    }
}

impl AddAssign<&Polynomial> for Polynomial {
    fn add_assign(self: &mut Self, other: &Polynomial) {
        let mut result = Vec::with_capacity(max(self.len(), other.len()));
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
        if self.is_zero() || other.is_zero() {
            return Polynomial::new(&[]);
        }
        if other.len() < 4 {
            let mut result = Polynomial(vec![]);
            for coefficient in other.coefficients().iter() {
                result.0.push(FieldElement::ZERO);
                result += &(coefficient * &self);
            }
            return result;
        }
        let result_length = self.len() + other.len() - 1;
        let padded_self = self.pad(result_length);
        let padded_other = other.pad(result_length);

        let a = fft(padded_self.coefficients());
        let b = fft(padded_other.coefficients());

        let product: Vec<_> = a.iter().zip(b).map(|(x, y)| x * y).collect();

        let c = ifft(&product);
        Self(c[c.len() - 1 - result_length..c.len() - 1].to_vec())
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div<Polynomial> for Polynomial {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.is_zero() {
            panic!("Cannot divide by zero polynomial");
        }
        if self.is_zero() {
            return Polynomial::new(&[]);
        }
        let degree_difference = self.len() - other.len();
        let inverse_leading_term = other.0[0].inv().expect("Cannot divide by zero polynomial");
        let mut remainder = self.clone();
        // let mut padded_other = other.extend_to_length(degree_difference);
        let mut result = vec![];
        for i in 0..=degree_difference {
            let q = &remainder.0[i] * &inverse_leading_term;
            remainder.subtract_at(&other, i, &q);
            result.push(q);
        }
        debug_assert!(remainder.is_zero());
        Self(result)
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
use u256::U256;
#[cfg(test)]
impl Polynomial {
    // TODO: turn this into a macro which accepts negative values as well.
    pub fn from_dense(c: &[usize]) -> Self {
        debug_assert!(!c.is_empty());
        let coefficients: Vec<FieldElement> = c
            .iter()
            .map(|x| FieldElement::from(U256::from(*x as u64)))
            .collect();
        Self::new(&coefficients)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::*;
    use quickcheck_macros::quickcheck;
    use u256::{u256h, U256};

    #[test]
    fn sparse_dense() {
        assert_eq!(
            Polynomial::new(&[FieldElement::ONE + FieldElement::ONE, FieldElement::ZERO, FieldElement::ONE]),
            Polynomial::from_sparse(&[(0, FieldElement::ONE + FieldElement::ONE), (2, FieldElement::ONE)])
        );
    }

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
    fn example_product_0() {
        let p_1 = Polynomial::from_dense(&[1, 1]);
        let p_2 = Polynomial::from_dense(&[1, 1]);
        assert_eq!(p_1 * p_2, Polynomial::from_dense(&[1, 2, 1]))
    }

    #[test]
    fn example_product_1() {
        let p_1 = Polynomial::from_dense(&[1, 2]);
        let p_2 = Polynomial::from_dense(&[1, 3, 4]);
        assert_eq!(p_1 * p_2, Polynomial::from_dense(&[1, 5, 10, 8]))
    }

    #[test]
    fn example_division_0() {
        let numerator = Polynomial::from_dense(&[1, 3, 3, 1]);
        let denominator = Polynomial::from_dense(&[1, 1]);
        assert_eq!(numerator / denominator, Polynomial::from_dense(&[1, 2, 1]))
    }

    #[test]
    fn example_division_1() {
        let numerator = Polynomial::from_dense(&[2, 2, 2, 2]);
        let denominator = Polynomial::from_dense(&[1, 1, 1, 1]);
        assert_eq!(numerator / denominator, Polynomial::from_dense(&[2]))
    }

    #[test]
    fn example_division_2() {
        let numerator = Polynomial::from_dense(&[1, 2, 1]);
        let denominator = Polynomial::from_dense(&[1, 1]);
        assert_eq!(numerator / denominator, Polynomial::from_dense(&[1, 1]))
    }

    #[test]
    fn example_division_3() {
        let numerator = Polynomial::from_dense(&[2, 6, 4]);
        let denominator = Polynomial::from_dense(&[1, 2]);

        assert_eq!(
            Polynomial::from_dense(&[2, 2]) * denominator.clone(),
            numerator.clone()
        );
        assert_eq!(
            denominator.clone() * Polynomial::from_dense(&[2, 2]),
            numerator.clone()
        );
        assert_eq!(numerator / denominator, Polynomial::from_dense(&[2, 2]))
    }

    #[test]
    fn example_division_4() {
        let numerator = Polynomial::from_dense(&[1, 0, 2, 0 ,1]);
        let denominator = Polynomial::from_dense(&[1, 0, 1]);
        assert_eq!(numerator / denominator, Polynomial::from_dense(&[1, 0, 1]))
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
    fn sum_evaluation_equivalence(x: FieldElement, a: Polynomial, b: Polynomial) -> bool {
        a.evaluate(&x) + b.evaluate(&x) == (a + b).evaluate(&x)
    }

    #[quickcheck]
    fn shift_evaluation_equivalence(a: Polynomial, shift: FieldElement, x: FieldElement) -> bool {
        a.shift(&shift).evaluate(&x) == a.evaluate(&(x * shift))
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
    fn addition_subtration_inverse(a: Polynomial, b: Polynomial) -> bool {
        a.clone() + b.clone() - b == a
    }

    #[quickcheck]
    fn division_by_self(a: Polynomial) -> bool {
        if a.is_zero() {
            return true;
        }
        a.clone() / a == Polynomial::from_dense(&[1])
    }

    #[quickcheck]
    fn division_multiplication_inverse(a: Polynomial, b: Polynomial) -> bool {
        if b.is_zero() {
            return true;
        }
        (a.clone() * b.clone()) / b == a
    }

    #[quickcheck]
    fn hack_division(a: Polynomial, b: FieldElement, c: FieldElement, d: usize) -> bool {
        if c.is_zero() {
            return true;
        }
        let denominator = Polynomial::from_sparse(&[(0, b), (d, c)]);
        a.clone() * denominator.clone() / denominator == a
    }
}
