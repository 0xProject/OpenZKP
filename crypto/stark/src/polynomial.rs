// TODO: Naming?
#![allow(clippy::module_name_repetitions)]
#[cfg(feature = "std")]
use log::trace;
use std::prelude::v1::*;
use zkp_macros_decl::field_element;
use zkp_mmap_vec::MmapVec;
#[cfg(feature = "std")]
use zkp_primefield::{fft::permute_index, Fft, Pow, Root};
use zkp_primefield::{FieldElement, Zero};
use zkp_u256::U256;

#[derive(Clone)]
pub struct DensePolynomial(MmapVec<FieldElement>);

// We normally don't want to spill thousands of coefficients in the logs.
#[cfg(feature = "std")]
impl std::fmt::Debug for DensePolynomial {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "DensePolynomial(degree = {:?})", self.degree())
    }
}

impl PartialEq for DensePolynomial {
    fn eq(&self, other: &Self) -> bool {
        // Check equality with evaluation
        let x = field_element!("754ed488ec9208d1b552bb254c0890042078a9e1f7e36072ebff1bf4e193d11b");
        self.evaluate(&x) == other.evaluate(&x)
    }
}
impl Eq for DensePolynomial {}

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
        vec.resize(size, FieldElement::zero());
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
        while self.0[degree] == FieldElement::zero() && degree > 0 {
            degree -= 1;
        }
        degree
    }

    pub fn evaluate(&self, x: &FieldElement) -> FieldElement {
        let mut result = FieldElement::zero();
        for coefficient in self.0.iter().rev() {
            result *= x;
            result += coefficient;
        }
        result
    }

    #[cfg(feature = "std")]
    pub fn low_degree_extension(&self, blowup: usize) -> MmapVec<FieldElement> {
        trace!("BEGIN Low degree extension");
        // TODO: shift polynomial by FieldElement::generator() outside of this function.
        // TODO: Parameterize cofactor
        let shift_factor: FieldElement = FieldElement::generator();
        let length = self.len() * blowup;
        let generator =
            FieldElement::root(length).expect("No generator for extended_domain_length.");

        // FieldElement is safe to initialize zero (which maps to zero)
        #[allow(unsafe_code)]
        let mut result: MmapVec<FieldElement> = unsafe { MmapVec::zero_initialized(length) };

        // Compute cosets
        result
            .as_mut_slice()
            .chunks_mut(self.len())
            .enumerate()
            .for_each(|(i, slice)| {
                let cofactor = &shift_factor * generator.pow(permute_index(blowup, i));
                slice.clone_shifted(&self.coefficients(), &cofactor);
                slice.fft();
            });
        trace!("END Low degree extension");
        result
    }

    /// Divide out a point and add the scaled result to target.
    ///
    /// target += c * (P(X) - P(z)) / (X - z)
    /// See: <https://en.wikipedia.org/wiki/Synthetic_division>
    pub fn divide_out_point_into(&self, z: &FieldElement, c: &FieldElement, target: &mut Self) {
        let mut remainder = FieldElement::zero();
        for (coefficient, target) in self.0.iter().rev().zip(target.0.iter_mut().rev()) {
            *target += c * &remainder;
            remainder *= z;
            remainder += coefficient;
        }
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
