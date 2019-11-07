use super::pedersen_points::{PEDERSEN_POINTS, SHIFT_POINT};
use std::prelude::v1::*;
use zkp_elliptic_curve::{Affine, Jacobian};
use zkp_primefield::FieldElement;
use zkp_u256::U256;

const N_ELEMENT_BITS: usize = 252;

pub fn hash(a: &FieldElement, b: &FieldElement) -> FieldElement {
    let mut result = Jacobian::from(SHIFT_POINT);
    for (i, element) in [U256::from(a), U256::from(b)].iter().enumerate() {
        assert!(element.bits() <= N_ELEMENT_BITS);
        let start = 1 + i * N_ELEMENT_BITS;
        let end = start + N_ELEMENT_BITS;
        for (j, point) in PEDERSEN_POINTS[start..end].iter().enumerate() {
            if element.bit(j) {
                result += point;
            }
        }
    }
    x_coordinate(Affine::from(&result))
}

fn x_coordinate(p: Affine) -> FieldElement {
    match p {
        Affine::Zero => panic!(),
        Affine::Point { x, .. } => x,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_0_0() {
        assert_eq!(
            hash(&FieldElement::ZERO, &FieldElement::ZERO),
            x_coordinate(SHIFT_POINT)
        );
    }

    #[test]
    fn test_hash_1_0() {
        assert_eq!(
            hash(&FieldElement::ONE, &FieldElement::ZERO),
            x_coordinate(SHIFT_POINT + &PEDERSEN_POINTS[1])
        );
    }

    #[test]
    fn test_hash_0_1() {
        assert_eq!(
            hash(&FieldElement::ZERO, &FieldElement::ONE),
            x_coordinate(SHIFT_POINT + &PEDERSEN_POINTS[253])
        );
    }
}
