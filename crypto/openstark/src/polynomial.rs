// TODO: Naming?
#![allow(clippy::module_name_repetitions)]
#[cfg(feature = "std")]
use mmap_vec::MmapVec;
#[cfg(feature = "std")]
use primefield::fft::{fft_cofactor_permuted, permute_index};
use primefield::{
    fft::{fft, ifft},
    FieldElement,
};
#[cfg(feature = "std")]
use rayon::prelude::*;
use std::{
    cmp::max,
    collections::BTreeMap,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    prelude::v1::*,
};
use u256::{commutative_binop, noncommutative_binop};

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct DensePolynomial(Vec<FieldElement>);

// TODO: Move into separate file or combine these into an enum.
#[derive(PartialEq, Clone, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct SparsePolynomial(BTreeMap<usize, FieldElement>);

impl DensePolynomial {
    // Coefficents are in order of ascending degree. E.g. &[1, 2] corresponds to the
    // polynomial f(x) = 1 + 2x.
    pub fn new(coefficients: &[FieldElement]) -> Self {
        assert!(coefficients.len().is_power_of_two());
        Self(coefficients.to_vec())
    }

    pub fn from_vec(coefficients: Vec<FieldElement>) -> Self {
        assert!(coefficients.len().is_power_of_two());
        Self(coefficients)
    }

    // Note that the length of a polynomial is not its degree, because the leading
    // coefficient of a DensePolynomial can be zero.
    pub fn len(&self) -> usize {
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

    #[cfg(feature = "std")]
    pub fn low_degree_extension(&self, blowup: usize) -> MmapVec<FieldElement> {
        // TODO: shift polynomial by FieldElement::GENERATOR outside of this function.
        const SHIFT_FACTOR: FieldElement = FieldElement::GENERATOR;
        let length = self.len() * blowup;
        let generator =
            FieldElement::root(length).expect("No generator for extended_domain_length.");
        let mut result: MmapVec<FieldElement> = MmapVec::with_capacity(length);

        // Initialize to zero
        // TODO: Avoid initialization
        result.resize(length, FieldElement::ZERO);

        // Compute cosets in parallel
        result
            .as_mut_slice()
            .par_chunks_mut(self.len())
            .enumerate()
            .for_each(|(i, slice)| {
                let cofactor = &SHIFT_FACTOR * generator.pow(permute_index(blowup, i));
                slice.clone_from_slice(&self.coefficients());
                fft_cofactor_permuted(&cofactor, slice);
            });
        result
    }

    pub fn divide_out_point(&self, x: &FieldElement) -> Self {
        let denominator = SparsePolynomial::new(&[(-x, 0), (FieldElement::ONE, 1)]);
        let mut result = self - SparsePolynomial::new(&[(self.evaluate(x), 0)]);
        result /= denominator;
        result
    }

    // Returns a polynomial such that f.shift(s)(x) = f(s * y).
    pub fn shift(&self, factor: &FieldElement) -> Self {
        let mut shifted_coefficients = self.0.clone();
        let mut power = FieldElement::ONE;
        for coefficient in &mut shifted_coefficients {
            *coefficient *= &power;
            power *= factor;
        }
        Self(shifted_coefficients)
    }

    pub fn next(&self) -> Self {
        // TODO: implement this without assuming that the polynomial has length equal to
        // the trace length.
        let trace_generator =
            FieldElement::root(self.len()).expect("DensePolynomial length doesn't have generator.");
        self.shift(&trace_generator)
    }

    // Removes trailing zeros or appends them so that the length is minimal and a
    // power of two.
    fn canonicalize(&mut self) {
        let last_nonzero_index = match self.0.iter().enumerate().rev().find(|(_, x)| !x.is_zero()) {
            Some((i, _)) => i,
            None => 0,
        };
        let new_length = (last_nonzero_index + 1).next_power_of_two();
        self.0.resize(new_length, FieldElement::ZERO);
    }

    pub fn coefficients(&self) -> &[FieldElement] {
        &self.0
    }

    pub fn square(&self) -> Self {
        let mut result = self.0.clone();
        result.extend_from_slice(&vec![FieldElement::ZERO; self.len()]);
        result = fft(&result);
        result.iter_mut().for_each(|x| *x = x.square());
        result = ifft(&result);
        Self(result)
    }
}

// OPT: Write an Add<DensePolynomial> uses the fact that it's faster to add
// shorter DensePolynomial's to longer ones, rather than vice versa.
impl AddAssign<&DensePolynomial> for DensePolynomial {
    fn add_assign(&mut self, other: &Self) {
        self.0
            .iter_mut()
            .zip(&other.0)
            .for_each(|(c_1, c_2)| *c_1 += c_2);
        if self.len() < other.len() {
            self.0.extend_from_slice(&other.0[self.len()..]);
        }
        self.canonicalize();
    }
}

impl SubAssign<&Self> for DensePolynomial {
    fn sub_assign(&mut self, other: &Self) {
        self.0
            .iter_mut()
            .zip(&other.0)
            .for_each(|(c_1, c_2)| *c_1 -= c_2);
        if self.len() < other.len() {
            let original_length = self.len();
            self.0.extend_from_slice(&other.0[self.len()..]);
            self.0[original_length..]
                .iter_mut()
                .for_each(FieldElement::neg_assign);
        }
        self.canonicalize();
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
            .iter_mut()
            .zip(&fft(&other_coefficients))
            .for_each(|(x, y)| *x *= y);
        self.0 = ifft(&self.0);
        self.canonicalize();
    }
}

commutative_binop!(DensePolynomial, Add, add, AddAssign, add_assign);
commutative_binop!(DensePolynomial, Mul, mul, MulAssign, mul_assign);
noncommutative_binop!(DensePolynomial, Sub, sub, SubAssign, sub_assign);

impl MulAssign<&FieldElement> for DensePolynomial {
    fn mul_assign(&mut self, other: &FieldElement) {
        self.0.iter_mut().for_each(|x| *x *= other);
    }
}

impl Mul<&FieldElement> for DensePolynomial {
    type Output = Self;

    fn mul(mut self, other: &FieldElement) -> Self {
        self *= other;
        self
    }
}

impl Mul<DensePolynomial> for &FieldElement {
    type Output = DensePolynomial;

    fn mul(self, other: DensePolynomial) -> DensePolynomial {
        other.mul(self)
    }
}

impl Mul<&DensePolynomial> for &FieldElement {
    type Output = DensePolynomial;

    fn mul(self, other: &DensePolynomial) -> DensePolynomial {
        DensePolynomial(other.0.iter().map(|x| x * self).collect())
    }
}

impl SparsePolynomial {
    pub fn new(coefficients_and_degrees: &[(FieldElement, usize)]) -> Self {
        let mut map = BTreeMap::new();
        for (coefficient, degree) in coefficients_and_degrees {
            match map.insert(*degree, coefficient.clone()) {
                None => (),
                Some(_) => panic!("Duplicate degrees found when constructing SparsePolynomial"),
            };
        }
        assert!(!map.is_empty());
        Self(map)
    }

    pub fn periodic(coefficients: &[FieldElement], power: usize) -> Self {
        let mut map = BTreeMap::new();
        for (i, coefficient) in coefficients.iter().enumerate() {
            assert!(!coefficient.is_zero());
            match map.insert(i * power, coefficient.clone()) {
                None => (),
                Some(_) => panic!("Duplicate degrees found when constructing SparsePolynomial"),
            };
        }
        assert!(!map.is_empty());
        Self(map)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn degree(&self) -> usize {
        *self
            .0
            .iter()
            .next_back()
            .expect("SparsePolynomial cannot be empty")
            .0
    }

    pub fn evaluate(&self, x: &FieldElement) -> FieldElement {
        let mut result = FieldElement::ZERO;
        for (degree, coefficient) in &self.0 {
            result += coefficient * x.pow(*degree);
        }
        result
    }

    fn leading_coefficient(&self) -> &FieldElement {
        self.0
            .iter()
            .next_back()
            .expect("SparsePolynomial cannot be empty")
            .1
    }

    pub fn pow(&self, n: usize) -> Self {
        match self.len() {
            1 => {
                let (degree, coefficient) = self.0.iter().next().unwrap();
                Self::new(&[(coefficient.pow(n), n * degree)])
            }
            _ => panic!(),
        }
    }
}

impl From<SparsePolynomial> for DensePolynomial {
    fn from(s: SparsePolynomial) -> Self {
        let mut result = vec![FieldElement::ZERO; (s.degree() + 1).next_power_of_two()];
        for (degree, coefficient) in s.0 {
            result[degree] = coefficient.clone();
        }
        DensePolynomial::from_vec(result)
    }
}

impl AddAssign<&Self> for SparsePolynomial {
    fn add_assign(&mut self, other: &Self) {
        for (degree, coefficient) in &other.0 {
            *self.0.entry(*degree).or_insert(FieldElement::ZERO) += coefficient;
        }
    }
}

impl SubAssign<&Self> for SparsePolynomial {
    fn sub_assign(&mut self, other: &Self) {
        for (degree, coefficient) in &other.0 {
            *self.0.entry(*degree).or_insert(FieldElement::ZERO) -= coefficient;
        }
    }
}

impl MulAssign<&Self> for SparsePolynomial {
    fn mul_assign(&mut self, other: &Self) {
        let mut result = BTreeMap::new();
        for (degree, coefficient) in &self.0 {
            for (other_degree, other_coefficient) in &other.0 {
                *result
                    .entry(degree + other_degree)
                    .or_insert(FieldElement::ZERO) += coefficient * other_coefficient;
            }
        }
        *self = Self(result);
    }
}

commutative_binop!(SparsePolynomial, Add, add, AddAssign, add_assign);
commutative_binop!(SparsePolynomial, Mul, mul, MulAssign, mul_assign);
noncommutative_binop!(SparsePolynomial, Sub, sub, SubAssign, sub_assign);

#[allow(clippy::suspicious_op_assign_impl)] // Allows us to use `+` here.
impl MulAssign<SparsePolynomial> for DensePolynomial {
    fn mul_assign(&mut self, other: SparsePolynomial) {
        let mut result = vec![FieldElement::ZERO; self.len() + other.degree()];
        for (degree, other_coefficient) in &other.0 {
            for (i, self_coefficient) in self.0.iter().enumerate() {
                result[i + degree] += self_coefficient * other_coefficient
            }
        }
        self.0 = result;
        self.canonicalize();
    }
}

impl Mul<SparsePolynomial> for DensePolynomial {
    type Output = DensePolynomial;

    fn mul(self, other: SparsePolynomial) -> DensePolynomial {
        let mut copy = self.clone();
        copy *= other;
        copy
    }
}

// This assumes that the sparse polynomial exactly divides the dense one, and
// will panic if that is not the case.
#[allow(clippy::suspicious_op_assign_impl)] // Allows us to use `*` here.
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
                for (degree, coefficient) in &denominator.0 {
                    self.0[i - denominator_degree + degree] -= &quotient_coefficient * coefficient;
                }
                self.0[i] = quotient_coefficient;
            } else {
                assert!(self.0[i].is_zero());
            }
        }
        let _ = self.0.drain(0..denominator_degree);
        self.canonicalize();
    }
}

impl Div<SparsePolynomial> for DensePolynomial {
    type Output = DensePolynomial;

    fn div(self, denominator: SparsePolynomial) -> DensePolynomial {
        let mut copy = self.clone();
        copy /= denominator;
        copy
    }
}

impl Add<SparsePolynomial> for &DensePolynomial {
    type Output = DensePolynomial;

    fn add(self, other: SparsePolynomial) -> DensePolynomial {
        let mut sum = self.0.clone();
        for (degree, coefficient) in &other.0 {
            sum[*degree] += coefficient;
        }
        DensePolynomial(sum)
    }
}

impl Sub<SparsePolynomial> for &DensePolynomial {
    type Output = DensePolynomial;

    fn sub(self, other: SparsePolynomial) -> DensePolynomial {
        let mut difference = self.0.clone();
        for (degree, coefficient) in &other.0 {
            difference[*degree] -= coefficient;
        }
        DensePolynomial(difference)
    }
}

impl Sub<&DensePolynomial> for SparsePolynomial {
    type Output = DensePolynomial;

    fn sub(self, other: &DensePolynomial) -> DensePolynomial {
        &FieldElement::NEGATIVE_ONE * &(other - self)
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
                let _ = map.insert(degree, FieldElement::ONE);
            } else {
                let _ = map.insert(degree, coefficient);
            }
        }
        if map.is_empty() {
            let _ = map.insert(0, FieldElement::ONE);
        }
        Self(map)
    }
}

// Qiuckcheck needs pass by value
#[allow(clippy::needless_pass_by_value)]
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    fn dense_polynomial(coefficients: &[isize]) -> DensePolynomial {
        let mut p = DensePolynomial(
            coefficients
                .iter()
                .map(|c| FieldElement::from(*c))
                .collect(),
        );
        p.canonicalize();
        p
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
    fn sparse_difference_evaluation_equivalence(
        a: SparsePolynomial,
        b: SparsePolynomial,
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
    fn sparse_sparse_product_evaluation_equivalence(
        a: SparsePolynomial,
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
    fn shift_evaluation_equivalence(a: DensePolynomial, s: FieldElement, x: FieldElement) -> bool {
        a.shift(&s).evaluate(&x) == a.evaluate(&(s * x))
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
    fn addition_subtraction_inverse(a: DensePolynomial, b: DensePolynomial) -> bool {
        &a + &b - b == a
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
