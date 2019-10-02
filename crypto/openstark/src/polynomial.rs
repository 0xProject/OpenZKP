// TODO: Naming?
#![allow(clippy::module_name_repetitions)]
use mmap_vec::MmapVec;
#[cfg(feature = "std")]
use primefield::fft::{fft_cofactor_permuted, permute_index};
use primefield::FieldElement;
#[cfg(feature = "std")]
use rayon::prelude::*;
use std::prelude::v1::*;
use log::info;

#[derive(PartialEq, Clone)]
pub struct DensePolynomial(MmapVec<FieldElement>);

// We normally don't want to spill thousands of coefficients in the logs.
#[cfg(feature = "std")]
impl std::fmt::Debug for DensePolynomial {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "DensePolynomial(degree = {:?})", self.degree())
    }
}

impl DensePolynomial {
    pub fn from_mmap_vec(coefficients: MmapVec<FieldElement>) -> Self {
        assert!(coefficients.len().is_power_of_two());
        Self(coefficients)
    }

    // Coefficients are in order of ascending degree. E.g. &[1, 2] corresponds to
    // the polynomial f(x) = 1 + 2x.
    pub fn new(coefficients: &[FieldElement]) -> Self {
        assert!(coefficients.len().is_power_of_two());
        let mut vec = MmapVec::with_capacity(coefficients.len());
        vec.extend_from_slice(coefficients);
        Self(vec)
    }

    pub fn zeros(size: usize) -> Self {
        assert!(size.is_power_of_two());
        let mut vec = MmapVec::with_capacity(size);
        vec.resize(size, FieldElement::ZERO);
        Self(vec)
    }

    // Note that the length of a polynomial is not its degree, because the leading
    // coefficient of a DensePolynomial can be zero.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn coefficients(&self) -> &[FieldElement] {
        &self.0
    }

    // TODO: The zero polynomial is assigned a degree of 0, but it is
    // more correctly left undefined or sometimes assigned `-1` or `-∞`.
    pub fn degree(&self) -> usize {
        let mut degree = self.len() - 1;
        while self.0[degree] == FieldElement::ZERO && degree > 0 {
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
        info!("Starting LDE extension");
        // TODO: shift polynomial by FieldElement::GENERATOR outside of this function.
        const SHIFT_FACTOR: FieldElement = FieldElement::GENERATOR;
        let length = self.len() * blowup;
        let generator =
            FieldElement::root(length).expect("No generator for extended_domain_length.");
        info!("Allocating LDE result vector");
        let mut result: MmapVec<FieldElement> = MmapVec::zero_initialized(length);

        // Compute cosets in parallel
        info!("Compute cosets in parallel");
        result
            .as_mut_slice()
            .par_chunks_mut(self.len())
            .enumerate()
            .for_each(|(i, slice)| {
                let cofactor = &SHIFT_FACTOR * generator.pow(permute_index(blowup, i));
                slice.clone_from_slice(&self.coefficients());
                fft_cofactor_permuted(&cofactor, slice);
            });
        info!("LDE extension done");
        result
    }

    /// Divide out a point and add the scaled result to target.
    ///
    /// target += c * (P(X) - P(z)) / (X - z)
    /// See: https://en.wikipedia.org/wiki/Synthetic_division
    pub fn divide_out_point_into(&self, z: &FieldElement, c: &FieldElement, target: &mut Self) {
        let mut remainder = FieldElement::ZERO;
        for (coefficient, target) in self.0.iter().rev().zip(target.0.iter_mut().rev()) {
            *target += c * &remainder;
            remainder *= z;
            remainder += coefficient;
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
        Self::new(&coefficients)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dense_polynomial(coefficients: &[isize]) -> DensePolynomial {
        DensePolynomial::new(
            &coefficients
                .iter()
                .map(|c| FieldElement::from(*c))
                .collect::<Vec<_>>(),
        )
    }

    #[test]
    fn example_evaluate() {
        let p = dense_polynomial(&[1, 0, 0, 2]);
        assert_eq!(p.evaluate(&FieldElement::from(2)), FieldElement::from(17));
    }
}
