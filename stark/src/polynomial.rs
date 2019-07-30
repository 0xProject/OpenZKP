use primefield::FieldElement;
use rayon::{iter::repeatn, prelude::*};
use std::{
    cmp::max,
    ops::{Add, AddAssign, Mul},
};
use u256::U256;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Polynomial(Vec<FieldElement>);

impl Polynomial {
    pub fn new(coefficients: &[FieldElement]) -> Self {
        let mut coefficients = coefficients.to_vec();
        coefficients.reverse();
        Self(coefficients)
    }

    pub fn from_dense(c: &[usize]) -> Self {
        debug_assert!(c.len() > 0);
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
            coefficients[*degree] = coefficient.clone();
        }
        Polynomial::from_dense(&coefficients)
    }

    #[inline(always)]
    fn coefficients(self: &Self) -> &[FieldElement] {
        &self.0
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

    pub fn evaluate(self: &Self, x: &FieldElement) -> FieldElement {
        let mut result = FieldElement::ZERO;
        for coefficient in self.coefficients().iter() {
            result *= x;
            result += coefficient;
        }
        result
    }

    fn shift(self: &mut Self) {
        self.0.push(FieldElement::ZERO);
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

impl Mul<Polynomial> for Polynomial {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.0.len() == 0 || other.0.len() == 0 {
            return Polynomial(vec![]);
        }
        let mut result = Polynomial(vec![]);
        for coefficient in other.coefficients().iter() {
            result.shift();
            result += &(coefficient * &self);
        }
        result
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
    fn example_product() {
        let p_1 = Polynomial::from_dense(&[1, 1]);
        let p_2 = Polynomial::from_dense(&[1, 1]);
        assert_eq!(p_1 * p_2, Polynomial::from_dense(&[1, 2, 1]))
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
}
