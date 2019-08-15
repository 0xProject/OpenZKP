use crate::fft::{fft, ifft};
use primefield::FieldElement;
use rayon::prelude::*;
use std::{
    cmp::max,
    collections::BTreeMap,
    ops::{Add, AddAssign, DivAssign, Mul, MulAssign, Sub, SubAssign},
};
use u256::{commutative_binop, noncommutative_binop};

#[derive(Debug, PartialEq, Clone)]
pub struct DensePolynomial(Vec<FieldElement>);

#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub struct SparsePolynomial(BTreeMap<usize, FieldElement>);

impl DensePolynomial {
    // Coefficents are in order of ascending degree. E.g. &[1, 2] corresponds to the
    // polynomial f(x) = 1 + 2x.
    pub fn new(coefficients: &[FieldElement]) -> Self {
        assert!(coefficients.len().is_power_of_two());
        Self(coefficients.to_vec())
    }

    // Note that the length of a polynomial is not its degree, because the leading
    // coefficient of a DensePolynomial can be zero.
    fn len(&self) -> usize {
        self.0.len()
    }

    pub fn evaluate(&self, x: &FieldElement) -> FieldElement {
        let mut result = FieldElement::ZERO;
        for coefficient in self.0.iter().rev() {
            result *= x;
            result += coefficient;
        }
        result
    }

    // Removes trailing zeros or appends them so that the length is a power of two.
    fn canonicalize(&mut self) {
        let last_nonzero_index = match self.0.iter().enumerate().rev().find(|(_, x)| !x.is_zero()) {
            Some((i, _)) => i,
            None => 0,
        };
        let new_length = (last_nonzero_index + 1).next_power_of_two();
        self.0.resize(new_length, FieldElement::ZERO);
    }
}

// OPT: Write an Add<DensePolynomial> uses the fact that it's faster to add
// shorter DensePolynomial's to longer ones, rather than vice versa.
impl AddAssign<&DensePolynomial> for DensePolynomial {
    fn add_assign(&mut self, other: &DensePolynomial) {
        self.0
            .par_iter_mut()
            .zip(&other.0)
            .map(|(c_1, c_2)| *c_1 += c_2)
            .collect::<Vec<_>>();
        if self.len() < other.len() {
            self.0.extend_from_slice(&other.0[self.len()..]);
        }
        assert!(self.len().is_power_of_two());
    }
}

impl SubAssign<&Self> for DensePolynomial {
    fn sub_assign(&mut self, other: &Self) {
        self.0
            .par_iter_mut()
            .zip(&other.0)
            .map(|(c_1, c_2)| *c_1 -= c_2)
            .collect::<Vec<_>>();
        if self.len() < other.len() {
            let original_length = self.len();
            self.0.extend_from_slice(&other.0[self.len()..]);
            self.0[original_length..]
                .par_iter_mut()
                .map(|c| c.neg_assign())
                .collect::<Vec<_>>();
        }
        assert!(self.len().is_power_of_two());
    }
}

#[allow(clippy::suspicious_op_assign_impl)] // Allow use of subtraction in this implementation.
impl MulAssign<&Self> for DensePolynomial {
    fn mul_assign(&mut self, other: &Self) {
        let result_length = 2 * max(self.len(), other.len());

        self.0
            .extend_from_slice(&vec![FieldElement::ZERO; result_length - self.len()]);
        self.0 = fft(&self.0);

        let mut other_coefficients = other.0.clone();
        other_coefficients
            .extend_from_slice(&vec![FieldElement::ZERO; result_length - other.len()]);

        self.0
            .par_iter_mut()
            .zip(&fft(&other_coefficients))
            .map(|(x, y)| *x *= y)
            .collect::<Vec<_>>();
        self.0 = ifft(&self.0);
    }
}

commutative_binop!(DensePolynomial, Add, add, AddAssign, add_assign);
commutative_binop!(DensePolynomial, Mul, mul, MulAssign, mul_assign);
noncommutative_binop!(DensePolynomial, Sub, sub, SubAssign, sub_assign);

impl Mul<&DensePolynomial> for &FieldElement {
    type Output = DensePolynomial;

    fn mul(self, other: &DensePolynomial) -> DensePolynomial {
        let mut coefficients: Vec<FieldElement> = Vec::with_capacity(other.len());
        other
            .0
            .par_iter()
            .map(|x| x * self)
            .collect_into_vec(&mut coefficients);
        DensePolynomial(coefficients)
    }
}

impl MulAssign<&FieldElement> for DensePolynomial {
    fn mul_assign(&mut self, other: &FieldElement) {
        self.0
            .par_iter_mut()
            .map(|x| *x *= other)
            .collect::<Vec<_>>();
    }
}

impl SparsePolynomial {
    pub fn new(coefficients_and_degrees: &[(FieldElement, usize)]) -> Self {
        let mut map = BTreeMap::new();
        for (coefficient, degree) in coefficients_and_degrees {
            assert!(!coefficient.is_zero());
            match map.insert(*degree, coefficient.clone()) {
                None => (),
                Some(_) => panic!("Duplicate degrees found when constructing SparsePolynomial"),
            };
        }
        assert!(!map.is_empty());
        Self(map)
    }

    pub fn degree(&self) -> usize {
        *self
            .0
            .iter()
            .next_back()
            .expect("SparsePolynomial cannot be empty")
            .0
    }

    fn leading_coefficient(&self) -> &FieldElement {
        self.0
            .iter()
            .next_back()
            .expect("SparsePolynomial cannot be empty")
            .1
    }
}

impl MulAssign<SparsePolynomial> for DensePolynomial {
    fn mul_assign(&mut self, other: SparsePolynomial) {
        let mut result = vec![FieldElement::ZERO; self.len() + other.degree()];
        for (degree, other_coefficient) in other.0.iter() {
            for (i, self_coefficient) in self.0.iter().enumerate() {
                result[i + degree] += self_coefficient * other_coefficient
            }
        }
        self.0 = result;
        self.canonicalize();
    }
}

// This assumes that the sparse polynomial exactly divides the dense one, and
// will panic if that is not the case.
impl DivAssign<SparsePolynomial> for DensePolynomial {
    fn div_assign(&mut self, denominator: SparsePolynomial) {
        if self.len() == 1 && self.0[0].is_zero() {
            return;
        }
        let inverse_leading_coefficient = denominator
            .leading_coefficient()
            .inv()
            .expect("SparsePolynomial has zero leading coefficient");
        let denominator_degree = denominator.degree();
        for i in (0..self.len()).rev() {
            if i >= denominator_degree {
                let quotient_coefficient = &self.0[i] * &inverse_leading_coefficient;
                for (degree, coefficient) in denominator.0.iter() {
                    self.0[i - denominator_degree + degree] -= &quotient_coefficient * coefficient;
                }
                self.0[i] = quotient_coefficient;
            } else {
                assert!(self.0[i].is_zero());
            }
        }
        self.0.drain(0..denominator_degree);
        self.canonicalize();
    }
}

#[cfg(test)]
use u256::U256;
#[cfg(test)]
impl SparsePolynomial {
    pub fn evaluate(&self, x: &FieldElement) -> FieldElement {
        let mut result = FieldElement::ZERO;
        for (degree, coefficient) in self.0.iter() {
            result += coefficient * x.pow(U256::from(*degree));
        }
        result
    }
}

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
#[cfg(test)]
impl Arbitrary for DensePolynomial {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let mut coefficients = Vec::<FieldElement>::arbitrary(g);
        let length = coefficients.len();
        coefficients.extend_from_slice(&vec![
            FieldElement::ZERO;
            length.next_power_of_two() - length
        ]);
        assert!(coefficients.len().is_power_of_two());
        Self(coefficients)
    }
}
#[cfg(test)]
impl Arbitrary for SparsePolynomial {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let coefficients_and_degrees = Vec::<(FieldElement, usize)>::arbitrary(g);
        let mut map = BTreeMap::new();
        for (coefficient, degree) in coefficients_and_degrees {
            if coefficient.is_zero() {
                map.insert(degree, FieldElement::ONE);
            } else {
                map.insert(degree, coefficient);
            }
        }
        if map.is_empty() {
            map.insert(0, FieldElement::ONE);
        }
        Self(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    fn dense_polynomial(coefficients: &[isize]) -> DensePolynomial {
        DensePolynomial(
            coefficients
                .iter()
                .map(|c| FieldElement::from(*c))
                .collect(),
        )
    }

    fn sparse_polynomial(coefficients_and_degrees: &[(isize, usize)]) -> SparsePolynomial {
        let v: Vec<_> = coefficients_and_degrees
            .iter()
            .map(|(c, d)| (FieldElement::from(*c), *d))
            .collect();
        SparsePolynomial::new(&v)
    }

    #[test]
    fn example_evaluate() {
        let p = dense_polynomial(&[1, 0, 0, 2]);
        assert_eq!(p.evaluate(&FieldElement::from(2)), FieldElement::from(17));
    }

    #[test]
    fn example_sparse_evaluate() {
        let p = sparse_polynomial(&[(1, 0), (1, 1)]);
        assert_eq!(p.evaluate(&FieldElement::from(1)), FieldElement::from(2));
    }

    #[test]
    fn example_sum() {
        let p_1 = dense_polynomial(&[1, 2]);
        let p_2 = dense_polynomial(&[1, 2, 6, 7]);
        assert_eq!(p_1 + p_2, dense_polynomial(&[2, 4, 6, 7]));
    }

    #[test]
    fn example_difference() {
        let p_1 = dense_polynomial(&[1, 2, 5, 7]);
        let p_2 = dense_polynomial(&[1, 2, 6, 7]);
        assert_eq!(p_1 - p_2, dense_polynomial(&[0, 0, -1, 0]));
    }

    #[test]
    fn example_multiplication() {
        let p_1 = dense_polynomial(&[1, 2]);
        let p_2 = dense_polynomial(&[1, 2, 3, 4]);
        assert_eq!(p_1 * p_2, dense_polynomial(&[1, 4, 7, 10, 8, 0, 0, 0]));
    }

    #[test]
    fn example_sparse_multiplication() {
        let mut p = dense_polynomial(&[1, 2, 2, 0]);
        p *= sparse_polynomial(&[(-1, 0), (1, 1)]);
        assert_eq!(p, dense_polynomial(&[-1, -1, 0, 2]));
    }

    #[test]
    fn example_division() {
        let mut p = dense_polynomial(&[1, 3, 3, 1]);
        p /= sparse_polynomial(&[(1, 0), (1, 1)]);
        assert_eq!(p, dense_polynomial(&[1, 2, 1, 0]));
    }

    #[test]
    fn example_division_1() {
        let mut p = dense_polynomial(&[1, 1]);
        p /= sparse_polynomial(&[(1, 0), (1, 1)]);
        assert_eq!(p, dense_polynomial(&[1]));
    }

    #[test]
    fn example_division_2() {
        let mut p = dense_polynomial(&[-1, -1, 0, 2]);
        p /= sparse_polynomial(&[(-1, 0), (1, 1)]);
        assert_eq!(p, dense_polynomial(&[1, 2, 2, 0]));
    }

    #[test]
    fn example_division_3() {
        let mut p = dense_polynomial(&[1, 3, 3, 1]);
        p /= sparse_polynomial(&[(1, 0), (1, 1)]);
        assert_eq!(p, dense_polynomial(&[1, 2, 1, 0]));
    }

    #[test]
    fn example_scalar_multiplication() {
        let c = FieldElement::from(-2);
        let p = dense_polynomial(&[1, 2, 5, -7]);
        assert_eq!(&c * &p, dense_polynomial(&[-2, -4, -10, 14]));
    }

    #[quickcheck]
    fn sum_evaluation_equivalence(a: DensePolynomial, b: DensePolynomial, x: FieldElement) -> bool {
        a.evaluate(&x) + b.evaluate(&x) == (a + b).evaluate(&x)
    }

    #[quickcheck]
    fn difference_evaluation_equivalence(
        a: DensePolynomial,
        b: DensePolynomial,
        x: FieldElement,
    ) -> bool {
        a.evaluate(&x) - b.evaluate(&x) == (a - b).evaluate(&x)
    }

    #[quickcheck]
    fn product_evaluation_equivalence(
        a: DensePolynomial,
        b: DensePolynomial,
        x: FieldElement,
    ) -> bool {
        a.evaluate(&x) * b.evaluate(&x) == (a * b).evaluate(&x)
    }

    #[quickcheck]
    fn sparse_product_evaluation_equivalence(
        a: DensePolynomial,
        b: SparsePolynomial,
        x: FieldElement,
    ) -> bool {
        let evaluate_first = a.evaluate(&x) * b.evaluate(&x);

        let mut product = a.clone();
        product *= b;
        let evaluate_last = product.evaluate(&x);

        evaluate_first == evaluate_last
    }

    #[quickcheck]
    fn scalar_multiplication_evaluation_equivalence(
        a: DensePolynomial,
        c: FieldElement,
        x: FieldElement,
    ) -> bool {
        &c * a.evaluate(&x) == (&c * &a).evaluate(&x)
    }

    #[quickcheck]
    fn sum_commutivity(a: DensePolynomial, b: DensePolynomial) -> bool {
        &a + &b == b + a
    }

    #[quickcheck]
    fn difference_anticommutivity(a: DensePolynomial, b: DensePolynomial) -> bool {
        let x = &a - &b;
        let y = &FieldElement::from(-1) * &(&b - &a);
        assert_eq!(x.len(), y.len());
        for i in 0..x.len() {
            assert_eq!(x.0[i], y.0[i]);
        }
        &a - &b == &FieldElement::from(-1) * &(b - a)
    }

    #[quickcheck]
    fn product_commutivity(a: DensePolynomial, b: DensePolynomial) -> bool {
        &a * &b == b * a
    }

    #[quickcheck]
    fn addition_subtraction_inverse(
        a: DensePolynomial,
        b: DensePolynomial,
        x: FieldElement,
    ) -> bool {
        // We cannot directly check for equality of the two sides because adding and
        // subtracting b can change the length of a.
        (&a + &b - b).evaluate(&x) == a.evaluate(&x)
    }

    #[quickcheck]
    fn distributivity(a: DensePolynomial, b: DensePolynomial, c: DensePolynomial) -> bool {
        &a * &b + &a * &c == a * (b + c)
    }

    #[quickcheck]
    fn division_multiplication_inverse(a: DensePolynomial, b: SparsePolynomial) -> bool {
        let mut test = a.clone();
        test *= b.clone();
        test /= b;
        assert_eq!(a, test);
        a == test
    }
}
