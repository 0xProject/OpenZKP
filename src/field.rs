use std::ops::{Add, Mul, AddAssign, SubAssign, MulAssign, DivAssign};
use num::{Zero, One};
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

impl Add for FieldElement {
    type Output = Self;
    fn add(self, rhs: FieldElement) -> Self {
        let mut result = self.clone();
        result += &rhs;
        result
    }
}

impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, rhs: FieldElement) -> Self {
        let mut result = self.clone();
        result *= &rhs;
        result
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
    fn inverse_add() {
    }

    #[quickcheck]
    #[test]
    fn inverse_mul() {
    }

    #[quickcheck]
    fn distributivity(a: FieldElement, b: FieldElement, c: FieldElement) -> bool {
        a.clone() * (b.clone() + c.clone()) == (a.clone() * b) + (a * c)
    }
}
