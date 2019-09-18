// TODO: Naming?
#![allow(clippy::module_name_repetitions)]
use itertools::Itertools;
#[cfg(feature = "std")]
use mmap_vec::MmapVec;
#[cfg(feature = "std")]
use primefield::fft::{fft_cofactor_permuted, permute_index};
use primefield::FieldElement;
#[cfg(feature = "std")]
use rayon::prelude::*;
use std::prelude::v1::*;
use u256::{commutative_binop, noncommutative_binop};

#[derive(PartialEq, Clone)]
pub struct DensePolynomial(Vec<FieldElement>);

#[cfg(feature = "std")]
impl std::fmt::Debug for DensePolynomial {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "DensePolynomial(degree = {:?})", self.degree())
    }
}

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

    pub fn coefficients(&self) -> &[FieldElement] {
        &self.0
    }

    pub fn degree(&self) -> usize {
        let mut degree = self.len() - 1;
        while self.0[degree] == FieldElement::ZERO {
            degree -= 1;
        }
        degree
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

    pub fn divide_out_point(&mut self, x: &FieldElement) {
        // P'(X) = (P(X) - P(x)) / (X - x)
        // Do a flooring division by (X - x)
        // We throw away the remainder, which is equivalent
        // to subtracting P(x).
        for (c2, c1) in self.0.iter_mut().rev().tuples() {
            *c1 -= x * &*c2;
        }
        *self.0.last_mut().unwrap() = FieldElement::ZERO;
    }

    // f'(x) = f(s * y).
    pub fn shift(&mut self, factor: &FieldElement) {
        let mut power = FieldElement::ONE;
        for coefficient in &mut self.0 {
            *coefficient *= &power;
            power *= factor;
        }
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

// Quickcheck needs pass by value
#[allow(clippy::needless_pass_by_value)]
#[cfg(test)]
mod tests {
    use super::*;
    use primefield::geometric_series::geometric_series;
    use quickcheck_macros::quickcheck;

    fn shift(factor: &FieldElement, x: &[FieldElement]) -> Vec<FieldElement> {
        let mut coefficients = ifft(x);
        for (c, power) in coefficients
            .iter_mut()
            .zip(geometric_series(&FieldElement::ONE, factor))
        {
            *c *= power;
        }
        fft(&coefficients)
    }

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

    #[test]
    fn example_evaluate() {
        let p = dense_polynomial(&[1, 0, 0, 2]);
        assert_eq!(p.evaluate(&FieldElement::from(2)), FieldElement::from(17));
    }

    #[quickcheck]
    fn shift_evaluation_equivalence(a: DensePolynomial, s: FieldElement, x: FieldElement) -> bool {
        a.shift(&s).evaluate(&x) == a.evaluate(&(s * x))
    }
}
