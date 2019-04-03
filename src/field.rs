use crate::u256::U256;
use crate::u256h;
use hex_literal::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub const MODULUS: U256 =
    u256h!("0800000000000011000000000000000000000000000000000000000000000001");
pub const INVEXP: U256 = u256h!("0800000000000010ffffffffffffffffffffffffffffffffffffffffffffffff");

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FieldElement(pub U256);

impl FieldElement {
    pub const ZERO: FieldElement = FieldElement(U256::ZERO);
    pub const ONE: FieldElement = FieldElement(U256::ONE);

    pub fn new(limbs: &[u32; 8]) -> Self {
        let mut bu = U256::new(
            ((limbs[1] as u64) << 32) | (limbs[0] as u64),
            ((limbs[3] as u64) << 32) | (limbs[2] as u64),
            ((limbs[5] as u64) << 32) | (limbs[4] as u64),
            ((limbs[7] as u64) << 32) | (limbs[6] as u64),
        );
        bu %= &MODULUS;
        assert!(bu < MODULUS);
        FieldElement(bu)
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes_be()
    }

    pub fn is_zero(&self) -> bool {
        self.0 == U256::ZERO
    }

    pub fn inv(&self) -> FieldElement {
        // TODO: Option type.
        FieldElement(self.0.invmod(&MODULUS).unwrap())
    }
}

// TODO: Implement Serde
impl From<&[u8; 32]> for FieldElement {
    fn from(bytes: &[u8; 32]) -> Self {
        let mut bu = U256::from_bytes_be(bytes.clone());
        bu %= &MODULUS;
        FieldElement(bu)
    }
}

// TODO: mul2() mul3() pow2()

impl Neg for FieldElement {
    type Output = FieldElement;
    fn neg(self) -> Self::Output {
        let mut n = (&MODULUS).clone();
        n -= &self.0;
        FieldElement(n)
    }
}

impl AddAssign<&FieldElement> for FieldElement {
    fn add_assign(&mut self, rhs: &FieldElement) {
        self.0 += &rhs.0;
        if self.0 >= MODULUS {
            self.0 -= &MODULUS;
        }
    }
}

impl SubAssign<&FieldElement> for FieldElement {
    fn sub_assign(&mut self, rhs: &FieldElement) {
        self.0 += &MODULUS;
        self.0 -= &rhs.0;
        if self.0 >= MODULUS {
            self.0 -= &MODULUS;
        }
    }
}

impl MulAssign<&FieldElement> for FieldElement {
    fn mul_assign(&mut self, rhs: &FieldElement) {
        self.0 = self.0.mulmod(&rhs.0, &MODULUS);
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
        // TODO: Generate 0, 1, p/2 and -1
        FieldElement(U256::arbitrary(g) % &MODULUS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[rustfmt::skip]
    #[test]
    fn test_add() {
        let a = FieldElement(u256h!("0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c"));
        let b = FieldElement(u256h!("024385f6bebc1c496e09955db534ef4b1eaff9a78e27d4093cfa8f7c8f886f6b"));
        let c = FieldElement(u256h!("078c472ca12bc6e60589484b558ca4964cbba44205c89285bd221ecb92ce9cb7"));
        assert_eq!(a + b, c);
    }

    #[rustfmt::skip]
    #[test]
    fn test_sub() {
        let a = FieldElement::new(&[0x7d14253b, 0xef060e37, 0x98d1486f, 0x8700b80a, 0x0a83500d, 0x961ed57d, 0x68cc0469, 0x02945916]);
        let b = FieldElement::new(&[0xf3a5912a, 0x62f3d853, 0x748c8465, 0x5f9b78d9, 0x8d66de24, 0xcf8479c5, 0x08cc1bb0, 0x06566f2f]);
        let c = FieldElement::new(&[0x896e9412, 0x8c1235e3, 0x2444c40a, 0x27653f31, 0x7d1c71e9, 0xc69a5bb7, 0x5fffe8c9, 0x043de9e7]);
        assert_eq!(a - b, c);
    }

    #[rustfmt::skip]
    #[test]
    fn test_mul() {
        let a = FieldElement::new(&[0x25fb5664, 0x9884280e, 0x0dcdbb96, 0x299078c9, 0x4392fd2e, 0x5a3ba2c1, 0x76e8c4ab, 0x06456ad3]);
        let b = FieldElement::new(&[0xf4926adb, 0x7e94c9d8, 0x18646bfe, 0x75c324f5, 0x1beb13ef, 0xc4195ea4, 0xd6098107, 0x009ce793]);
        let c = FieldElement::new(&[0x8f18f110, 0x98593af8, 0x1eda2b3f, 0x92f06f39, 0x36f1d62e, 0x8c7b6e67, 0xa1175434, 0x037ad171]);
        assert_eq!(a * b, c);
    }

    #[rustfmt::skip]
    #[test]
    fn test_div() {
        let a = FieldElement::new(&[0x7d14253b, 0xef060e37, 0x98d1486f, 0x8700b80a, 0x0a83500d, 0x961ed57d, 0x68cc0469, 0x02945916]);
        let b = FieldElement::new(&[0xf3a5912a, 0x62f3d853, 0x748c8465, 0x5f9b78d9, 0x8d66de24, 0xcf8479c5, 0x08cc1bb0, 0x06566f2f]);
        let c = FieldElement::new(&[0x4fb2a90b, 0x301e1830, 0x97593d1a, 0x97e53783, 0xbf27c713, 0x1bed3220, 0x9a076875, 0x02a40705]);
        assert_eq!(a / b, c);
    }

    #[quickcheck]
    #[test]
    fn add_identity(a: FieldElement) -> bool {
        a.clone() + FieldElement::ZERO == a
    }

    #[quickcheck]
    #[test]
    fn mul_identity(a: FieldElement) -> bool {
        a.clone() * FieldElement::ONE == a
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
        a.clone() + a.neg() == FieldElement::ZERO
    }

    #[quickcheck]
    #[test]
    fn inverse_mul(a: FieldElement) -> bool {
        a.clone() * a.inv() == FieldElement::ONE
    }

    #[quickcheck]
    #[test]
    fn distributivity(a: FieldElement, b: FieldElement, c: FieldElement) -> bool {
        a.clone() * (b.clone() + c.clone()) == (a.clone() * b) + (a * c)
    }
}
