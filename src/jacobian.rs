use crate::curve::{Affine, BETA, ORDER};
use crate::field::FieldElement;
use crate::u256::U256;
use crate::u256h;
use crate::{commutative_binop, curve_operations, noncommutative_binop};
use hex_literal::*;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// See http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html

#[derive(Clone, Debug)]
struct Jacobian {
    x: FieldElement,
    y: FieldElement,
    z: FieldElement,
}

impl Jacobian {
    pub const ZERO: Jacobian = Jacobian {
        x: FieldElement::ZERO,
        y: FieldElement::ZERO,
        z: FieldElement::ZERO,
    };

    pub fn on_curve(&self) -> bool {
        // TODO: Compute without inverting Z
        Affine::from(self).on_curve()
    }

    pub fn double_assign(&mut self) {
        // See http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html#doubling-dbl-2007-bl
        let xx = self.x.square();
        let yy = self.y.square();
        let yyyy = yy.square();
        let zz = self.z.square();
        let s = ((&self.x + &yy).square() - &xx - &yyyy).double();
        let m = xx.triple() + /* ALPHA * */ zz.square();
        self.z = (&self.y + &self.z).square() - yy - zz;
        self.x = m.square() - s.double();
        self.y = m * (s - &self.x) - yyyy.double().double().double(); // TODO: .octuple()
    }

    pub fn neg_assign(&mut self) {
        self.y.neg_assign();
    }

    pub fn double(&self) -> Jacobian {
        let mut r = self.clone();
        r.double_assign();
        r
    }
}

impl PartialEq for Jacobian {
    fn eq(&self, rhs: &Jacobian) -> bool {
        // TODO: without inverting Z
        Affine::from(self) == Affine::from(rhs)
    }
}

impl From<&Affine> for Jacobian {
    fn from(other: &Affine) -> Jacobian {
        match other {
            Affine::Zero => Jacobian::ZERO,
            Affine::Point { x, y } => Jacobian {
                x: x.clone(),
                y: y.clone(),
                z: FieldElement::ONE,
            },
        }
    }
}

impl From<Affine> for Jacobian {
    fn from(other: Affine) -> Jacobian {
        match other {
            Affine::Zero => Jacobian::ZERO,
            Affine::Point { x, y } => Jacobian {
                x,
                y,
                z: FieldElement::ONE,
            },
        }
    }
}

impl From<&Jacobian> for Affine {
    fn from(other: &Jacobian) -> Affine {
        match other.z.inv() {
            None => Affine::ZERO,
            Some(zi) => {
                let zi2 = zi.square();
                let zi3 = zi * &zi2;
                Affine::Point {
                    x: &other.x * zi2,
                    y: &other.y * zi3,
                }
            }
        }
    }
}

impl Neg for &Jacobian {
    type Output = Jacobian;
    fn neg(self) -> Jacobian {
        let mut r = self.clone();
        r.neg_assign();
        r
    }
}

impl AddAssign<&Jacobian> for Jacobian {
    fn add_assign(&mut self, rhs: &Jacobian) {
        // See http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html#addition-add-2007-bl
        let z1z1 = self.z.square();
        let z2z2 = rhs.z.square();
        let u1 = &self.x * &z2z2;
        let u2 = &rhs.x * &z1z1;
        let s1 = &self.y * &rhs.z * &z2z2;
        let s2 = &rhs.y * &self.z * &z1z1;
        let h = u2 - &u1;
        let i = h.double().square();
        let j = &h * &i;
        let r = (s2 - &s1).double();
        let v = u1 * i;
        self.x = r.square() - &j - v.double();
        self.y = r * (v - &self.x) - (s1 * j).double();
        self.z = ((&self.z + &rhs.z).square() - z1z1 - z2z2) * h;
    }
}

impl AddAssign<&Affine> for Jacobian {
    fn add_assign(&mut self, rhs: &Affine) {
        match rhs {
            Affine::Zero => self.z = FieldElement::ZERO,
            Affine::Point { x, y } => {
                // See http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html#addition-madd-2007-bl
                let z1z1 = self.z.square();
                let u2 = x * &z1z1;
                let s2 = y * &self.z * &z1z1;
                let h = u2 - &self.x;
                let hh = h.square();
                let i = hh.double().double(); // TODO .quadruple()
                let j = &h * &i;
                let r = (s2 - &self.y).double();
                let v = &self.x * i;
                self.x = r.square() - &j - v.double();
                self.y = r * (v - &self.x) - (&self.y * j).double();
                self.z = (&self.z + h).square() - z1z1 - hh;
            }
        }
    }
}

// TODO: Various Add implementations mixing Affine and Jacobian values and refs.

curve_operations!(Jacobian);
commutative_binop!(Jacobian, Add, add, AddAssign, add_assign);
noncommutative_binop!(Jacobian, Sub, sub, SubAssign, sub_assign);

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[cfg(test)]
impl Arbitrary for Jacobian {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        // To force Z to be non trivial we add two points.
        let mut r = Jacobian::from(Affine::arbitrary(g));
        r += &Affine::arbitrary(g);
        r
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_add() {
        let a = Jacobian::from(Affine::Point {
            x: FieldElement::new(&[0xca9b3b7a, 0xadf5b0d8, 0x4728f1b4, 0x7a5cbd79, 0x316a86d0, 0xb9aaaf56, 0x557c9ca9, 0x0259dee2]),
            y: FieldElement::new(&[0x68173fdd, 0x25daa0d2, 0xcd94b717, 0x4f84a316, 0xd637a579, 0x236d898d, 0x787b7c9e, 0x011cf020])
        });
        let b = Jacobian::from(Affine::Point{
            x: FieldElement::new(&[0x55893510, 0x5985d659, 0xc0cda9ae, 0xfb1db2ec, 0xc78fe4ec, 0xe60f0d63, 0xfb0e0cf5, 0x0449895d]),
            y: FieldElement::new(&[0x1b78e1cc, 0x86e1e27b, 0x80a13dd1, 0x157492ef, 0x8191f8ae, 0x7fb47371, 0x8d4ef0e6, 0x07cfb4b0])
        });
        let c = Jacobian::from(Affine::Point{
            x: FieldElement::new(&[0xcaaa938d, 0x1e36e642, 0x875a7e8a, 0xb1ccde68, 0x1e961e1a, 0xbbb669e2, 0xd487aea7, 0x07ec1cca]),
            y: FieldElement::new(&[0x1879893b, 0x953ad520, 0x89ca316f, 0x999e7f28, 0x1a29f3b5, 0xb48241d7, 0x7d682604, 0x05e52087])
        });
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_double() {
        let a = Jacobian::from(Affine::Point{
            x: FieldElement::new(&[0xa19caf1f, 0x9764694b, 0xd49d26e1, 0xc2d21cea, 0x9d37cc5b, 0xce13e7e3, 0x787be6e0, 0x00ea1dff]),
            y: FieldElement::new(&[0xce7296f0, 0xd1f6f7df, 0xc9c5b41c, 0x6b889413, 0xc9449f06, 0xf44da1a6, 0x302e9f91, 0x011b6c17])
        });
        let b = Jacobian::from(Affine::Point{
            x: FieldElement::new(&[0x1f01ad3f, 0x6fe79335, 0x2cdfe101, 0x032a86e6, 0x1481bc24, 0x8fccd336, 0xf387342d, 0x017056be]),
            y: FieldElement::new(&[0x6342205c, 0x06a09929, 0x1924cee3, 0x38e46f15, 0xe0393658, 0xcc1b8a43, 0x0743351a, 0x062673bb])
        });
        assert_eq!(a.double(), b);
    }

    #[test]
    fn test_mul() {
        let a = Jacobian::from(Affine::Point{
            x: FieldElement::new(&[0x5bf31eb0, 0xfe50a889, 0x2d1a8a21, 0x3242e28e, 0x0d13fe66, 0xcf63e064, 0x9426e2c3, 0x0040ffd5]),
            y: FieldElement::new(&[0xe29859d2, 0xd21b931a, 0xea34d27d, 0x296f19b9, 0x6487ae5b, 0x524260f9, 0x069092ca, 0x060c2257])
        });
        let b = U256::from_slice(&[0x711a14cf, 0xebe54f04, 0x4729d630, 0xd14a329a, 0xf5480b47, 0x35fdc862, 0xde09131d, 0x029f7a37]);
        let c = Jacobian::from(Affine::Point{
            x: FieldElement::new(&[0x143de731, 0x4c657d7e, 0x44b99cbf, 0x49dfc2e5, 0x40ea4226, 0xaf6c4895, 0x9a141832, 0x04851acc]),
            y: FieldElement::new(&[0x138592fd, 0x1377613f, 0xd53c61dd, 0xaa8b32c1, 0xd5bf18bc, 0x3b22a665, 0xf54ed6ae, 0x07f4bb53])
        });
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_mul_2() {
        let p = Jacobian::from(Affine::Point {
            x: FieldElement::from(u256h!("01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca")),
            y: FieldElement::from(u256h!("005668060aa49730b7be4801df46ec62de53ecd11abe43a32873000c36e8dc1f"))
        });
        let c = u256h!("07374b7d69dc9825fc758b28913c8d2a27be5e7c32412f612b20c9c97afbe4dd");
        let expected = Jacobian::from(Affine::Point {
            x: FieldElement::from(u256h!("00f24921907180cd42c9d2d4f9490a7bc19ac987242e80ac09a8ac2bcf0445de")),
            y: FieldElement::from(u256h!("018a7a2ab4e795405f924de277b0e723d90eac55f2a470d8532113d735bdedd4"))
        });
        let result = p.clone() * c;
        assert_eq!(result, expected);
    }

    #[allow(clippy::eq_op)]
    #[quickcheck]
    #[test]
    fn add_commutative(a: Jacobian, b: Jacobian) -> bool {
        &a + &b == &b + &a
    }

    #[quickcheck]
    #[test]
    fn distributivity(p: Jacobian, mut a: U256, mut b: U256) -> bool {
        a %= &ORDER;
        b %= &ORDER;
        let c = &a + &b;
        // TODO: c %= &ORDER;
        (&p * a) + (&p * b) == &p * c
    }
}
