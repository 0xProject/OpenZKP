use std::ops::{Add, Neg, Mul, AddAssign, SubAssign, MulAssign};
use num::{Zero, One};
use crate::field::FieldElement;
use lazy_static::lazy_static;

// Curve parameters

// Alpha = 1
// Beta  = 0x6f21413efbe40de150e596d72f7a8c5609ad26c15c915c1f4cdfcb99cee9e89
// Order = 0x800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f

lazy_static! {
    static ref BETA: FieldElement = FieldElement::new(&[
        0x9cee9e89, 0xf4cdfcb9, 0x15c915c1, 0x609ad26c,
        0x72f7a8c5, 0x150e596d, 0xefbe40de, 0x06f21413
    ]);
}

pub struct CurvePoint {
    // TODO: Point at infinity.
    // TODO: Jacobian coordinates.
    x: FieldElement,
    y: FieldElement,
}

impl CurvePoint {
    pub fn double(self) -> CurvePoint {
        assert!(self.x.clone() != FieldElement::zero());
        let one = FieldElement::one().clone();
        let two = one.clone() + one.clone();
        let three = two.clone() + one.clone();
        let m = (three.clone() * self.x.clone() * self.x.clone() + one.clone()) / (two.clone() * self.y.clone());
        let x = m.clone() * m.clone() - two.clone() * self.x.clone();
        let y = m.clone() * (self.x.clone() - x.clone()) - self.y.clone();
        CurvePoint {x, y}
    }
}

impl Add for CurvePoint {
    type Output = Self;
    fn add(self, rhs: CurvePoint) -> Self {
        assert!(self.x.clone() - rhs.x.clone() != FieldElement::zero());
        let m = (self.y.clone() - rhs.y.clone()) / (self.x.clone() - rhs.x.clone());
        let x = m.clone() * m.clone() - self.x.clone() - rhs.x.clone();
        let y = m.clone() * (self.x.clone() - x.clone()) - self.y.clone();
        CurvePoint {x, y}
    }
}
