use crate::BETA;
use std::{
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    prelude::v1::*,
};
use zkp_primefield::{FieldElement, NegInline, One, Zero};
use zkp_u256::{commutative_binop, noncommutative_binop, U256};

#[derive(PartialEq, Eq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Affine {
    Zero, // Neutral element, point at infinity, additive identity, etc.
    Point { x: FieldElement, y: FieldElement },
}

impl Affine {
    pub const ZERO: Self = Self::Zero;

    #[must_use]
    pub fn new(x: FieldElement, y: FieldElement) -> Self {
        Self::Point { x, y }
    }

    #[must_use]
    pub fn on_curve(&self) -> bool {
        match self {
            Self::Zero => true,
            Self::Point { x, y } => y * y == x * x * x + x + BETA,
        }
    }

    pub fn double_assign(&mut self) {
        *self = self.double();
    }

    #[must_use]
    pub fn double(&self) -> Self {
        match self {
            Self::Zero => Self::Zero,
            Self::Point { x, y } => {
                if *y == FieldElement::zero() {
                    Self::Zero
                } else {
                    let m = ((x + x + x) * x + FieldElement::one()) / (y + y);
                    let nx = &m * &m - x - x;
                    let ny = m * (x - &nx) - y;
                    Self::Point { x: nx, y: ny }
                }
            }
        }
    }

    pub fn neg_assign(&mut self) {
        match self {
            Self::Zero => {}
            Self::Point { y, .. } => y.neg_assign(),
        }
    }
}

impl Default for Affine {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Neg for &Affine {
    type Output = Affine;

    fn neg(self) -> Self::Output {
        match self {
            Affine::Zero => Affine::Zero,
            Affine::Point { x, y } => {
                Affine::Point {
                    x: x.clone(),
                    y: -y,
                }
            }
        }
    }
}

impl AddAssign<&Affine> for Affine {
    fn add_assign(&mut self, rhs: &Self) {
        match self {
            Self::Zero => *self = rhs.clone(),
            Self::Point { x: ax, y: ay } => {
                match rhs {
                    Self::Zero => {}
                    Self::Point { x: bx, y: by } => {
                        if ax == bx {
                            if ay == by {
                                self.double_assign()
                            } else {
                                *self = Self::Zero
                            }
                        } else {
                            let m = (&*ay - by) / (&*ax - bx);
                            let x = &m * &m - &*ax - &*bx;
                            *ay = m * (&*ax - &x) - &*ay;
                            *ax = x;
                        }
                    }
                }
            }
        }
    }
}

// TODO: This can be more elegantly done using traits
#[macro_export]
macro_rules! curve_operations {
    ($type:ident) => {
        impl SubAssign<&$type> for $type {
            fn sub_assign(&mut self, rhs: &Self) {
                *self += &rhs.neg()
            }
        }

        impl Mul<&U256> for &$type {
            type Output = $type;

            fn mul(self, scalar: &U256) -> $type {
                use zkp_u256::Binary;
                // OPT: Use WNAF
                let mut r = self.clone();
                for i in (0..scalar.most_significant_bit().unwrap_or_default()).rev() {
                    r.double_assign();
                    if scalar.bit(i) {
                        r += self;
                    }
                }
                r
            }
        }

        impl MulAssign<&U256> for $type {
            fn mul_assign(&mut self, scalar: &U256) {
                *self = &*self * scalar;
            }
        }

        impl MulAssign<U256> for $type {
            fn mul_assign(&mut self, scalar: U256) {
                *self *= &scalar;
            }
        }

        impl Mul<U256> for $type {
            type Output = Self;

            fn mul(self, scalar: U256) -> Self {
                &self * &scalar
            }
        }

        impl Mul<&U256> for $type {
            type Output = Self;

            fn mul(self, scalar: &U256) -> Self {
                &self * scalar
            }
        }

        impl Mul<U256> for &$type {
            type Output = $type;

            fn mul(self, scalar: U256) -> $type {
                self * &scalar
            }
        }

        // TODO: Left multiplication by scalar
    };
}

curve_operations!(Affine);
commutative_binop!(Affine, Add, add, AddAssign, add_assign);
noncommutative_binop!(Affine, Sub, sub, SubAssign, sub_assign);

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[cfg(test)]
impl Arbitrary for Affine {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        if u8::arbitrary(g) < 50 {
            Self::Zero
        } else {
            // TODO: Make sure it is on the curve
            Self::Point {
                x: FieldElement::arbitrary(g),
                y: FieldElement::arbitrary(g),
            }
        }
    }
}

// Quickcheck needs pass by value
#[allow(clippy::needless_pass_by_value)]
// We allow these in tests for readability/ease of editing
#[allow(clippy::redundant_clone)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ORDER;
    use quickcheck_macros::quickcheck;
    use zkp_macros_decl::u256h;
    use zkp_u256::U256;

    #[test]
    fn test_add() {
        let a = Affine::new(
            FieldElement::from(u256h!(
                "01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca"
            )),
            FieldElement::from(u256h!(
                "005668060aa49730b7be4801df46ec62de53ecd11abe43a32873000c36e8dc1f"
            )),
        );
        let b = Affine::new(
            FieldElement::from(u256h!(
                "00f24921907180cd42c9d2d4f9490a7bc19ac987242e80ac09a8ac2bcf0445de"
            )),
            FieldElement::from(u256h!(
                "018a7a2ab4e795405f924de277b0e723d90eac55f2a470d8532113d735bdedd4"
            )),
        );
        let c = Affine::new(
            FieldElement::from(u256h!(
                "0457342950d2475d9e83a4de8beb3c0850181342ea04690d804b37aa907b735f"
            )),
            FieldElement::from(u256h!(
                "00011bd6102b929632ce605b5ae1c9c6c1b8cba2f83aa0c5a6d1247318871137"
            )),
        );
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_double() {
        let a = Affine::new(
            FieldElement::from(u256h!(
                "01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca"
            )),
            FieldElement::from(u256h!(
                "005668060aa49730b7be4801df46ec62de53ecd11abe43a32873000c36e8dc1f"
            )),
        );
        let b = Affine::new(
            FieldElement::from(u256h!(
                "0759ca09377679ecd535a81e83039658bf40959283187c654c5416f439403cf5"
            )),
            FieldElement::from(u256h!(
                "06f524a3400e7708d5c01a28598ad272e7455aa88778b19f93b562d7a9646c41"
            )),
        );
        assert_eq!(a.double(), b);
    }

    #[test]
    fn test_mul() {
        let p = Affine::new(
            FieldElement::from(u256h!(
                "01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca"
            )),
            FieldElement::from(u256h!(
                "005668060aa49730b7be4801df46ec62de53ecd11abe43a32873000c36e8dc1f"
            )),
        );
        let c = u256h!("07374b7d69dc9825fc758b28913c8d2a27be5e7c32412f612b20c9c97afbe4dd");
        let expected = Affine::new(
            FieldElement::from(u256h!(
                "00f24921907180cd42c9d2d4f9490a7bc19ac987242e80ac09a8ac2bcf0445de"
            )),
            FieldElement::from(u256h!(
                "018a7a2ab4e795405f924de277b0e723d90eac55f2a470d8532113d735bdedd4"
            )),
        );
        let result = p.clone() * c;
        assert_eq!(result, expected);
    }

    #[allow(clippy::eq_op)]
    #[quickcheck]
    fn add_commutative(a: Affine, b: Affine) -> bool {
        &a + &b == &b + &a
    }

    #[quickcheck]
    fn distributivity(p: Affine, mut a: U256, mut b: U256) -> bool {
        a %= &ORDER;
        b %= &ORDER;
        let c = &a + &b;
        // TODO: c %= &ORDER;
        (&p * a) + (&p * b) == &p * c
    }
}
