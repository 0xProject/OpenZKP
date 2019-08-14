use primefield::FieldElement;
use rayon::prelude::*;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use u256::{commutative_binop, noncommutative_binop};

#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub struct DensePolynomial(Vec<FieldElement>);

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

impl SubAssign<&DensePolynomial> for DensePolynomial {
    fn sub_assign(&mut self, other: &DensePolynomial) {
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

commutative_binop!(DensePolynomial, Add, add, AddAssign, add_assign);
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
        DensePolynomial(coefficients)
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

    #[test]
    fn example_evaluate() {
        let p = dense_polynomial(&[1, 0, 0, 2]);
        assert_eq!(p.evaluate(&FieldElement::from(2)), FieldElement::from(17));
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
    fn addition_subtraction_inverse(
        a: DensePolynomial,
        b: DensePolynomial,
        x: FieldElement,
    ) -> bool {
        // We cannot directly check for equality of the two sides because adding and
        // subtracting b can change the length of a.
        (&a + &b - b).evaluate(&x) == a.evaluate(&x)
    }
}
