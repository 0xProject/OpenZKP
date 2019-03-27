use std::ops::{Add, Neg, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use num::traits::{Zero, One, Inv};
use num::traits::cast::FromPrimitive;
use num_bigint::BigUint;
use lazy_static::lazy_static;

// Note: BigUInt does not support compile time initialization
lazy_static! {
    pub static ref ZERO: BigUint = BigUint::from_slice(&[
        0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);
    pub static ref ONE: BigUint = BigUint::from_slice(&[
        0x00000001, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);
    pub static ref MODULUS: BigUint = BigUint::from_slice(&[
        0x00000001, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000011, 0x08000000
    ]);
    pub static ref INVEXP: BigUint = BigUint::from_slice(&[
        0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
        0xffffffff, 0xffffffff, 0x00000010, 0x08000000
    ]);
}

#[derive(PartialEq,Eq,Clone,Debug)]
pub struct FieldElement(BigUint);

impl FieldElement {
    // TODO: const ZERO ONE
    pub fn new(limbs: &[u32; 8]) -> Self {
        FieldElement(BigUint::from_slice(limbs))
    }
}

impl Zero for FieldElement {
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
    fn zero() -> Self {
        FieldElement(ZERO.clone())
    }
}

impl One for FieldElement {
    fn one() -> Self {
        FieldElement(ONE.clone())
    }
}

impl Neg for FieldElement {
    type Output = FieldElement;
    fn neg(self) -> Self::Output {
        let mut n = (&*MODULUS).clone();
        n -= self.0;
        FieldElement(n)
    }
}

impl Inv for FieldElement {
    type Output = Self;
    fn inv(self) -> Self::Output {
        // Fermats little theorem
        // TODO: Better.
        FieldElement(self.0.modpow(&*INVEXP, &*MODULUS))
    }
}

impl AddAssign<&FieldElement> for FieldElement {
    fn add_assign(&mut self, rhs: &FieldElement) {
        self.0 += &rhs.0;
        self.0 %= &*MODULUS;
    }
}

impl SubAssign<&FieldElement> for FieldElement {
    fn sub_assign(&mut self, rhs: &FieldElement) {
        self.0 += &*MODULUS;
        self.0 -= &rhs.0;
        self.0 %= &*MODULUS;
    }
}

impl MulAssign<&FieldElement> for FieldElement {
    fn mul_assign(&mut self, rhs: &FieldElement) {
        self.0 *= &rhs.0;
        self.0 %= &*MODULUS;
    }
}

impl DivAssign<&FieldElement> for FieldElement {
    fn div_assign(&mut self, rhs: &FieldElement) {
        let i: FieldElement = rhs.clone().inv();
        self.mul_assign(&i);
    }
}

impl Add for FieldElement {
    type Output = Self;
    fn add(self, rhs: FieldElement) -> Self::Output {
        let mut result = self.clone();
        result += &rhs;
        result
    }
}

impl Sub for FieldElement {
    type Output = Self;
    fn sub(self, rhs: FieldElement) -> Self::Output {
        let mut result = self.clone();
        result -= &rhs;
        result
    }
}

impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, rhs: FieldElement) -> Self::Output {
        let mut result = self.clone();
        result *= &rhs;
        result
    }
}

impl Div for FieldElement {
    type Output = Self;
    fn div(self, rhs: FieldElement) -> Self::Output {
        self * rhs.inv()
    }
}

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[cfg(test)]
use rand::Rng;

#[cfg(test)]
impl Arbitrary for FieldElement {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let mut n = BigUint::from_slice(&[
            g.gen(), g.gen(), g.gen(), g.gen(),
            g.gen(), g.gen(), g.gen(), g.gen()
        ]);
        n %= &*MODULUS;
        FieldElement(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    #[test]
    fn add_identity(a: FieldElement) -> bool {
        a.clone() + FieldElement::zero() == a
    }

    #[quickcheck]
    #[test]
    fn mul_identity(a: FieldElement) -> bool {
        a.clone() * FieldElement::one() == a
    }

    #[quickcheck]
    #[test]
    fn commutative_add(a: FieldElement, b: FieldElement) -> bool {
        a.clone() + b.clone() == b + a
    }

    #[quickcheck]
    #[test]
    fn commutative_mul(a: FieldElement, b: FieldElement) -> bool {
        a.clone() * b.clone() == b * a
    }

    #[quickcheck]
    #[test]
    fn associative_add(a: FieldElement, b: FieldElement, c: FieldElement) -> bool {
        a.clone() + (b.clone() + c.clone()) == (a + b) + c
    }

    #[quickcheck]
    #[test]
    fn associative_mul(a: FieldElement, b: FieldElement, c: FieldElement) -> bool {
        a.clone() * (b.clone() * c.clone()) == (a * b) * c
    }

    #[quickcheck]
    #[test]
    fn inverse_add(a: FieldElement) -> bool {
        a.clone() + a.neg() == FieldElement::zero()
    }

    #[quickcheck]
    #[test]
    fn inverse_mul(a: FieldElement) -> bool {
        a.clone() * a.inv() == FieldElement::one()
    }

    #[quickcheck]
    #[test]
    fn distributivity(a: FieldElement, b: FieldElement, c: FieldElement) -> bool {
        a.clone() * (b.clone() + c.clone()) == (a.clone() * b) + (a * c)
    }
}
