use lazy_static::lazy_static;
use num::bigint::BigUint;
use num::integer::Integer;
use num::traits::{Inv, One, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub fn modinv(n: &BigUint, m: &BigUint) -> Option<BigUint> {
    // Handbook of Applied Cryptography Algorithm 14.61:
    // Binary Extended GCD
    // See note 14.64 on application to modular inverse.
    // The algorithm is modified to work with non-negative numbers.
    // TODO: Constant time algorithm. (Based on fermat's little theorem or
    // constant time Euclids.)
    let mut u = n.clone();
    let mut v = m.clone();
    let mut a = BigUint::one();
    let mut c = BigUint::zero();
    while !u.is_zero() {
        while u.is_even() {
            u >>= 1;
            if a.is_odd() {
                a += m;
            }
            a >>= 1;
        }
        while v.is_even() {
            v >>= 1;
            if c.is_odd() {
                c += m;
            }
            c >>= 1;
        }
        if u >= v {
            if a < c {
                a += m;
            }
            u -= &v;
            a -= &c;
        } else {
            if c < a {
                c += m;
            }
            v -= &u;
            c -= &a;
        }
    }
    if v == BigUint::one() {
        Some(c)
    } else {
        None
    }
}

// Note: BigUInt does not support compile time initialization
lazy_static! {
    pub static ref ZERO: BigUint = BigUint::from_slice(&[
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000
    ]);
    pub static ref ONE: BigUint = BigUint::from_slice(&[
        0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000
    ]);
    pub static ref MODULUS: BigUint = BigUint::from_slice(&[
        0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000011,
        0x08000000
    ]);
    pub static ref INVEXP: BigUint = BigUint::from_slice(&[
        0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0x00000010,
        0x08000000
    ]);
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FieldElement(pub BigUint);

impl FieldElement {
    // TODO: const ZERO ONE
    pub fn new(limbs: &[u32; 8]) -> Self {
        let mut bu = BigUint::from_slice(limbs);
        bu %= &*MODULUS;
        FieldElement(bu)
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        // TODO: Zero padding
        let vec = self.0.to_bytes_be();
        let mut array = [0; 32];
        let bytes = &vec.as_slice()[..array.len()]; // panics if not enough data
        array.copy_from_slice(bytes);
        array
    }
}

// TODO: Implement Serde
impl From<&[u8; 32]> for FieldElement {
    fn from(bytes: &[u8; 32]) -> Self {
        let mut bu = BigUint::from_bytes_be(bytes);
        bu %= &*MODULUS;
        FieldElement(bu)
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

// TODO: mul2() mul3() pow2()

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
    // TODO: Option
    fn inv(self) -> Self::Output {
        // TODO: Option type.
        FieldElement(modinv(&self.0, &*MODULUS).unwrap())
    }
}

impl AddAssign<&FieldElement> for FieldElement {
    fn add_assign(&mut self, rhs: &FieldElement) {
        self.0 += &rhs.0;
        if self.0 >= *MODULUS {
            self.0 -= &*MODULUS;
        }
    }
}

impl SubAssign<&FieldElement> for FieldElement {
    fn sub_assign(&mut self, rhs: &FieldElement) {
        self.0 += &*MODULUS;
        self.0 -= &rhs.0;
        if self.0 >= *MODULUS {
            self.0 -= &*MODULUS;
        }
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
        // TODO: Generate 0, 1, p/2 and -1
        let mut n = BigUint::from_slice(&[
            g.gen(),
            g.gen(),
            g.gen(),
            g.gen(),
            g.gen(),
            g.gen(),
            g.gen(),
            g.gen(),
        ]);
        n %= &*MODULUS;
        FieldElement(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::traits::cast::FromPrimitive;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_modinv() {
        let n = BigUint::from_u64(271).unwrap();
        let m = BigUint::from_u64(383).unwrap();
        let i = BigUint::from_u64(106).unwrap();
        let r = modinv(&n, &m).unwrap();
        assert_eq!(i, r);
    }

    #[rustfmt::skip]
    #[test]
    fn test_add() {
        let a = FieldElement::new(&[0x0f3855f5, 0x37862eb2, 0x275b919f, 0x325329cb, 0xe968e6a2, 0xa2ceee5c, 0xd5f1d547, 0x07211989]);
        let b = FieldElement::new(&[0x32c781dd, 0x6f6a3b68, 0x3bac723c, 0xd5893114, 0xd0178b37, 0x5476714f, 0x1c567d5a, 0x0219cad4]);
        let c = FieldElement::new(&[0x41ffd7d1, 0xa6f06a1a, 0x630803db, 0x07dc5adf, 0xb98071da, 0xf7455fac, 0xf2485290, 0x013ae45d]);
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
