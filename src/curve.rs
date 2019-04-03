use crate::field::FieldElement;
use crate::u256::U256;
use crate::u256h;
use hex_literal::*;
use num::{One, Zero};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Shr, SubAssign};

// Curve parameters

// Alpha = 1
// Beta  = 0x06f21413efbe40de150e596d72f7a8c5609ad26c15c915c1f4cdfcb99cee9e89
// Order = 0x0800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f

pub const BETA: FieldElement = FieldElement(u256h!(
    "06f21413efbe40de150e596d72f7a8c5609ad26c15c915c1f4cdfcb99cee9e89"
));
pub const ORDER: U256 = u256h!("0800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f");

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CurvePoint {
    // TODO: Point at infinity.
    // TODO: Jacobian coordinates.
    pub x: FieldElement,
    pub y: FieldElement,
}

impl CurvePoint {
    pub fn new(x: &[u32; 8], y: &[u32; 8]) -> CurvePoint {
        CurvePoint {
            x: FieldElement::new(x),
            y: FieldElement::new(y),
        }
    }

    pub fn on_curve(&self) -> bool {
        self.y.clone() * self.y.clone()
            == self.x.clone() * self.x.clone() * self.x.clone() + self.x.clone() + BETA.clone()
    }

    pub fn double(self) -> CurvePoint {
        assert!(self.x.clone() != FieldElement::zero());
        let one = FieldElement::one().clone();
        let two = one.clone() + one.clone();
        let three = two.clone() + one.clone();
        let m = (three.clone() * self.x.clone() * self.x.clone() + one.clone())
            / (two.clone() * self.y.clone());
        let x = m.clone() * m.clone() - two.clone() * self.x.clone();
        let y = m.clone() * (self.x.clone() - x.clone()) - self.y.clone();
        CurvePoint { x, y }
    }
}

impl Add for CurvePoint {
    type Output = Self;
    fn add(self, rhs: CurvePoint) -> Self {
        assert!(self.x.clone() - rhs.x.clone() != FieldElement::zero());
        let m = (self.y.clone() - rhs.y.clone()) / (self.x.clone() - rhs.x.clone());
        let x = m.clone() * m.clone() - self.x.clone() - rhs.x.clone();
        let y = m.clone() * (self.x.clone() - x.clone()) - self.y.clone();
        CurvePoint { x, y }
    }
}

impl AddAssign for CurvePoint {
    fn add_assign(&mut self, rhs: CurvePoint) {
        let result = self.clone() + rhs;
        self.x = result.x;
        self.y = result.y;
    }
}

// This is over a multiplicative field of order 'Order'
impl Mul<U256> for CurvePoint {
    type Output = Self;
    fn mul(self, scalar: U256) -> Self::Output {
        assert!(scalar != U256::ZERO);
        if scalar == U256::ONE {
            self
        } else {
            if scalar.is_even() {
                self.double() * scalar.shr(1)
            } else {
                self.clone() + (self * (scalar - &U256::ONE))
            }
        }
    }
}

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[cfg(test)]
impl Arbitrary for CurvePoint {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        // TODO: Generate the zero point
        CurvePoint {
            x: FieldElement::arbitrary(g),
            y: FieldElement::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_add() {
        let a = CurvePoint{
            x: FieldElement::new(&[0xca9b3b7a, 0xadf5b0d8, 0x4728f1b4, 0x7a5cbd79, 0x316a86d0, 0xb9aaaf56, 0x557c9ca9, 0x0259dee2]),
            y: FieldElement::new(&[0x68173fdd, 0x25daa0d2, 0xcd94b717, 0x4f84a316, 0xd637a579, 0x236d898d, 0x787b7c9e, 0x011cf020])
        };
        let b = CurvePoint{
            x: FieldElement::new(&[0x55893510, 0x5985d659, 0xc0cda9ae, 0xfb1db2ec, 0xc78fe4ec, 0xe60f0d63, 0xfb0e0cf5, 0x0449895d]),
            y: FieldElement::new(&[0x1b78e1cc, 0x86e1e27b, 0x80a13dd1, 0x157492ef, 0x8191f8ae, 0x7fb47371, 0x8d4ef0e6, 0x07cfb4b0])
        };
        let c = CurvePoint{
            x: FieldElement::new(&[0xcaaa938d, 0x1e36e642, 0x875a7e8a, 0xb1ccde68, 0x1e961e1a, 0xbbb669e2, 0xd487aea7, 0x07ec1cca]),
            y: FieldElement::new(&[0x1879893b, 0x953ad520, 0x89ca316f, 0x999e7f28, 0x1a29f3b5, 0xb48241d7, 0x7d682604, 0x05e52087])
        };
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_double() {
        let a = CurvePoint{
            x: FieldElement::new(&[0xa19caf1f, 0x9764694b, 0xd49d26e1, 0xc2d21cea, 0x9d37cc5b, 0xce13e7e3, 0x787be6e0, 0x00ea1dff]),
            y: FieldElement::new(&[0xce7296f0, 0xd1f6f7df, 0xc9c5b41c, 0x6b889413, 0xc9449f06, 0xf44da1a6, 0x302e9f91, 0x011b6c17])
        };
        let b = CurvePoint{
            x: FieldElement::new(&[0x1f01ad3f, 0x6fe79335, 0x2cdfe101, 0x032a86e6, 0x1481bc24, 0x8fccd336, 0xf387342d, 0x017056be]),
            y: FieldElement::new(&[0x6342205c, 0x06a09929, 0x1924cee3, 0x38e46f15, 0xe0393658, 0xcc1b8a43, 0x0743351a, 0x062673bb])
        };
        assert_eq!(a.double(), b);
    }

    #[test]
    fn test_mul() {
        let a = CurvePoint{
            x: FieldElement::new(&[0x5bf31eb0, 0xfe50a889, 0x2d1a8a21, 0x3242e28e, 0x0d13fe66, 0xcf63e064, 0x9426e2c3, 0x0040ffd5]),
            y: FieldElement::new(&[0xe29859d2, 0xd21b931a, 0xea34d27d, 0x296f19b9, 0x6487ae5b, 0x524260f9, 0x069092ca, 0x060c2257])
        };
        let b = U256::from_slice(&[0x711a14cf, 0xebe54f04, 0x4729d630, 0xd14a329a, 0xf5480b47, 0x35fdc862, 0xde09131d, 0x029f7a37]);
        let c = CurvePoint{
            x: FieldElement::new(&[0x143de731, 0x4c657d7e, 0x44b99cbf, 0x49dfc2e5, 0x40ea4226, 0xaf6c4895, 0x9a141832, 0x04851acc]),
            y: FieldElement::new(&[0x138592fd, 0x1377613f, 0xd53c61dd, 0xaa8b32c1, 0xd5bf18bc, 0x3b22a665, 0xf54ed6ae, 0x07f4bb53])
        };
        assert_eq!(a * b, c);
    }

    #[quickcheck]
    #[test]
    fn add_commutative(a: CurvePoint, b: CurvePoint) -> bool {
        a.clone() + b.clone() == b.clone() + a.clone()
    }

    #[quickcheck]
    #[test]
    fn distributivity(p: CurvePoint, fa: FieldElement, fb: FieldElement) -> bool {
        let a = fa.0.clone() % &ORDER;
        let b = fb.0.clone() % &ORDER;
        let c = a.clone() + &b;
        (p.clone() * a) + (p.clone() * b) == p.clone() * c
    }
}
