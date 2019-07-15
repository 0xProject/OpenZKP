use crate::{montgomery::*, square_root::square_root};
use hex_literal::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use u256::{commutative_binop, noncommutative_binop, u256h, U256};

// TODO: Implement Serde

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FieldElement(pub U256);

impl FieldElement {
    pub const GENERATOR: FieldElement = FieldElement(u256h!(
        "07fffffffffff9b0ffffffffffffffffffffffffffffffffffffffffffffffa1"
    ));
    /// Prime modulus of the field.
    ///
    /// Equal to (1 << 59) | (1 << 4) | 1.
    pub const MODULUS: U256 =
        u256h!("0800000000000011000000000000000000000000000000000000000000000001");
    // 3, in montgomery form.
    pub const NEGATIVE_ONE: FieldElement = FieldElement(u256h!(
        "0000000000000220000000000000000000000000000000000000000000000020"
    ));
    pub const ONE: FieldElement = FieldElement(R1);
    pub const ZERO: FieldElement = FieldElement(U256::ZERO);

    pub const fn from_montgomery(n: U256) -> Self {
        FieldElement(n)
    }

    pub fn from_hex_str(s: &str) -> Self {
        FieldElement::from(U256::from_hex_str(s))
    }

    #[allow(clippy::cast_lossless)]
    pub fn new(limbs: &[u32; 8]) -> Self {
        let mut bu = U256::new(
            ((limbs[1] as u64) << 32) | (limbs[0] as u64),
            ((limbs[3] as u64) << 32) | (limbs[2] as u64),
            ((limbs[5] as u64) << 32) | (limbs[4] as u64),
            ((limbs[7] as u64) << 32) | (limbs[6] as u64),
        );
        bu = to_montgomery(&bu);
        FieldElement(bu)
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        from_montgomery(&self.0).to_bytes_be()
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.0 == U256::ZERO
    }

    #[inline(always)]
    pub fn is_one(&self) -> bool {
        self.0 == R1
    }

    #[inline(always)]
    pub fn inv(&self) -> Option<FieldElement> {
        inv_redc(&self.0).map(FieldElement)
    }

    #[inline(always)]
    pub fn double(&self) -> FieldElement {
        // TODO: Optimize
        self.clone() + self
    }

    #[inline(always)]
    pub fn triple(&self) -> FieldElement {
        // TODO: Optimize
        self.clone() + self + self
    }

    #[inline(always)]
    pub fn square(&self) -> FieldElement {
        FieldElement(sqr_redc(&self.0))
    }

    pub fn square_root(&self) -> Option<FieldElement> {
        square_root(self)
    }

    #[inline(always)]
    pub fn neg_assign(&mut self) {
        *self = self.neg()
    }

    pub fn pow(&self, exponent: U256) -> FieldElement {
        let mut result = FieldElement::ONE;
        let mut square = self.clone();
        let mut remaining_exponent = exponent;
        while !remaining_exponent.is_zero() {
            if remaining_exponent.is_odd() {
                result *= &square;
            }
            remaining_exponent >>= 1;
            square = square.square();
        }
        result
    }

    // OPT: replace this with a constant array of roots of unity.
    pub fn root(n: U256) -> Option<FieldElement> {
        if n.is_zero() {
            return Some(FieldElement::ONE);
        }
        let (q, rem) = (FieldElement::MODULUS - U256::ONE).divrem(&n).unwrap();
        if rem != U256::ZERO {
            return None;
        }
        Some(FieldElement::GENERATOR.pow(q))
    }
}

pub fn invert_batch(to_be_inverted: &[FieldElement]) -> Vec<FieldElement> {
    let n = to_be_inverted.len();
    let mut inverses = cumulative_product(to_be_inverted);

    // TODO: Enforce check to prevent uninvertable elements.
    let mut inverse = inverses[n - 1].inv().unwrap();
    for i in (1..n).rev() {
        inverses[i] = &inverses[i - 1] * &inverse;
        inverse *= &to_be_inverted[i];
    }
    inverses[0] = inverse;
    inverses
}

fn cumulative_product(elements: &[FieldElement]) -> Vec<FieldElement> {
    elements
        .iter()
        .scan(FieldElement::ONE, |running_product, x| {
            *running_product *= x;
            Some(running_product.clone())
        })
        .collect()
}

impl From<U256> for FieldElement {
    fn from(n: U256) -> Self {
        FieldElement(to_montgomery(&n))
    }
}

impl From<&U256> for FieldElement {
    fn from(n: &U256) -> Self {
        FieldElement(to_montgomery(n))
    }
}

impl From<FieldElement> for U256 {
    fn from(n: FieldElement) -> Self {
        from_montgomery(&n.0)
    }
}

impl From<&FieldElement> for U256 {
    fn from(n: &FieldElement) -> Self {
        from_montgomery(&n.0)
    }
}

impl From<&[u8; 32]> for FieldElement {
    fn from(bytes: &[u8; 32]) -> Self {
        FieldElement(to_montgomery(&U256::from_bytes_be(bytes)))
    }
}

impl Neg for &FieldElement {
    type Output = FieldElement;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        FieldElement(FieldElement::MODULUS - &self.0)
    }
}

impl AddAssign<&FieldElement> for FieldElement {
    #[inline(always)]
    fn add_assign(&mut self, rhs: &FieldElement) {
        self.0 += &rhs.0;
        if self.0 >= FieldElement::MODULUS {
            self.0 -= &FieldElement::MODULUS;
        }
    }
}

impl SubAssign<&FieldElement> for FieldElement {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: &FieldElement) {
        if self.0 >= rhs.0 {
            self.0 -= &rhs.0;
        } else {
            self.0 -= &rhs.0;
            self.0 += &FieldElement::MODULUS;
        }
    }
}

impl MulAssign<&FieldElement> for FieldElement {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: &FieldElement) {
        self.0 = mul_redc(&self.0, &rhs.0);
    }
}

impl DivAssign<&FieldElement> for FieldElement {
    fn div_assign(&mut self, rhs: &FieldElement) {
        *self *= rhs.inv().unwrap();
    }
}

impl std::iter::Product for FieldElement {
    fn product<I: Iterator<Item = FieldElement>>(iter: I) -> FieldElement {
        iter.fold(FieldElement::ONE, Mul::mul)
    }
}

// TODO: Implement Sum, Successors, ... for FieldElement.

commutative_binop!(FieldElement, Add, add, AddAssign, add_assign);
commutative_binop!(FieldElement, Mul, mul, MulAssign, mul_assign);
noncommutative_binop!(FieldElement, Sub, sub, SubAssign, sub_assign);
noncommutative_binop!(FieldElement, Div, div, DivAssign, div_assign);

#[cfg(feature = "quickcheck")]
use quickcheck::{Arbitrary, Gen};

#[cfg(feature = "quickcheck")]
impl Arbitrary for FieldElement {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        // TODO: Generate 0, 1, p/2 and -1
        FieldElement(U256::arbitrary(g) % FieldElement::MODULUS)
    }
}

// TODO: Use u256h literals here.
#[allow(clippy::unreadable_literal)]
#[cfg(test)]
mod tests {
    use super::*;
    use itertools::repeat_n;
    use quickcheck_macros::quickcheck;

    #[test]
    fn negative_one_is_additive_inverse_of_one() {
        assert_eq!(
            FieldElement::ONE + FieldElement::NEGATIVE_ONE,
            FieldElement::ZERO
        );
    }

    #[rustfmt::skip]
    #[test]
    fn test_add() {
        let a = FieldElement(u256h!(
            "0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c"
        ));
        let b = FieldElement(u256h!(
            "024385f6bebc1c496e09955db534ef4b1eaff9a78e27d4093cfa8f7c8f886f6b"
        ));
        let c = FieldElement(u256h!(
            "078c472ca12bc6e60589484b558ca4964cbba44205c89285bd221ecb92ce9cb7"
        ));
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
    #[test]
    fn test_batch_inv() {
        let a = FieldElement::new(&[
            0x7d14253b, 0xef060e37, 0x98d1486f, 0x8700b80a, 0x0a83500d, 0x961ed57d, 0x68cc0469,
            0x02945916,
        ]);
        let b = FieldElement::new(&[
            0xf3a5912a, 0x62f3d853, 0x748c8465, 0x5f9b78d9, 0x8d66de24, 0xcf8479c5, 0x08cc1bb0,
            0x06566f2f,
        ]);
        let c = FieldElement::new(&[
            0x4fb2a90b, 0x301e1830, 0x97593d1a, 0x97e53783, 0xbf27c713, 0x1bed3220, 0x9a076875,
            0x02a40705,
        ]);
        let d = FieldElement::new(&[
            0x7d14353b, 0xef060e37, 0x98d1486f, 0x8700b80a, 0x0a83500d, 0x961ed57d, 0x68cc0469,
            0x02945916,
        ]);
        let e = FieldElement::new(&[
            0xf3a5912a, 0x74f3d853, 0x748c8465, 0x5f9b78d9, 0x8d66de24, 0xcf8479c5, 0x08cc1bb0,
            0x06566f2f,
        ]);
        let f = FieldElement::new(&[
            0x4fb2a9bb, 0x301e1830, 0x97593d1a, 0x97e56783, 0xbf27c713, 0x1bed3220, 0x9a076875,
            0x02a40705,
        ]);

        let to_be_inverted = vec![
            a.clone(),
            b.clone(),
            c.clone(),
            d.clone(),
            e.clone(),
            f.clone(),
        ];
        let ret = invert_batch(to_be_inverted.as_slice());

        assert_eq!(ret[0], a.inv().unwrap());
        assert_eq!(ret[1], b.inv().unwrap());
        assert_eq!(ret[2], c.inv().unwrap());
        assert_eq!(ret[3], d.inv().unwrap());
        assert_eq!(ret[4], e.inv().unwrap());
        assert_eq!(ret[5], f.inv().unwrap());
    }

    #[quickcheck]
    fn add_identity(a: FieldElement) -> bool {
        &a + FieldElement::ZERO == a
    }

    #[quickcheck]
    fn mul_identity(a: FieldElement) -> bool {
        &a * FieldElement::ONE == a
    }

    #[quickcheck]
    fn commutative_add(a: FieldElement, b: FieldElement) -> bool {
        &a + &b == b + a
    }

    #[quickcheck]
    fn commutative_mul(a: FieldElement, b: FieldElement) -> bool {
        &a * &b == b * a
    }

    #[quickcheck]
    fn associative_add(a: FieldElement, b: FieldElement, c: FieldElement) -> bool {
        &a + (&b + &c) == (a + b) + c
    }

    #[quickcheck]
    fn associative_mul(a: FieldElement, b: FieldElement, c: FieldElement) -> bool {
        &a * (&b * &c) == (a * b) * c
    }

    #[quickcheck]
    fn inverse_add(a: FieldElement) -> bool {
        &a + a.neg() == FieldElement::ZERO
    }

    #[quickcheck]
    fn inverse_mul(a: FieldElement) -> bool {
        match a.inv() {
            None => a == FieldElement::ZERO,
            Some(ai) => a * ai == FieldElement::ONE,
        }
    }

    #[quickcheck]
    fn distributivity(a: FieldElement, b: FieldElement, c: FieldElement) -> bool {
        &a * (&b + &c) == (&a * b) + (a * c)
    }

    #[quickcheck]
    fn pow_0(a: FieldElement) -> bool {
        a.pow(U256::from(0u128)) == FieldElement::ONE
    }

    #[quickcheck]
    fn pow_1(a: FieldElement) -> bool {
        a.pow(U256::from(1u128)) == a
    }

    #[quickcheck]
    fn pow_2(a: FieldElement) -> bool {
        a.pow(U256::from(2u128)) == a.square()
    }

    #[quickcheck]
    fn pow_n(a: FieldElement, n: usize) -> bool {
        a.pow(U256::from(n as u128)) == repeat_n(a, n).product()
    }

    #[quickcheck]
    fn fermats_little_theorem(a: FieldElement) -> bool {
        a.pow(FieldElement::MODULUS) == a
    }

    #[test]
    fn zeroth_root_of_unity() {
        assert_eq!(
            FieldElement::root(U256::from(0u64)).unwrap(),
            FieldElement::ONE
        );
    }

    #[test]
    fn roots_of_unity_squared() {
        let powers_of_two = (0..193).map(|n| U256::ONE << n);
        let roots_of_unity: Vec<_> = powers_of_two
            .clone()
            .map(|n| FieldElement::root(n).unwrap())
            .collect();

        for (smaller_root, larger_root) in roots_of_unity[1..].iter().zip(roots_of_unity.as_slice())
        {
            assert_eq!(smaller_root.square(), *larger_root);
            assert!(!smaller_root.is_one());
        }
    }

    #[test]
    fn root_of_unity_definition() {
        let powers_of_two = (0..193).map(|n| U256::ONE << n);
        for n in powers_of_two {
            let root_of_unity = FieldElement::root(n.clone()).unwrap();
            assert_eq!(root_of_unity.pow(n), FieldElement::ONE);
        }
    }
}
