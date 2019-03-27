use crate::field::FieldElement;
use crate::curve::CurvePoint;
use crate::pedersen_points::PEDERSEN_POINTS;
use num::{BigUint, integer::Integer};
use lazy_static::lazy_static;

lazy_static! {
    static ref SHIFT_POINT: CurvePoint = CurvePoint{
        x: FieldElement::new(&[
            0x50ca6804, 0x551fde40, 0x22947733, 0x716b0b10,
            0xeb599f16, 0x00ee1b87, 0xa8c16007, 0x049ee3eb]),
        y: FieldElement::new(&[
            0x6e10268a, 0xd0405d26, 0xc0e056c1, 0x4e621062,
            0x06ea0ed3, 0xf346d49d, 0x4b3bc6dd, 0x03ca0cfe]),
    };
}

pub const N_ELEMENT_BITS: usize = 251;

pub fn hash(elements: &[BigUint]) -> BigUint {
    let mut result = SHIFT_POINT.clone();
    for (i, element) in elements.iter().enumerate() {
        let mut bits = element.clone();
        assert!(element.bits() <= N_ELEMENT_BITS);
        // point_list = CONSTANT_POINTS[1 + i * N_ELEMENT_BITS:1 + (i + 1) * N_ELEMENT_BITS]
        let start = 1 + i * N_ELEMENT_BITS;
        let end = start + N_ELEMENT_BITS; 
        for point in PEDERSEN_POINTS[start..end].iter() {
            if bits.is_odd() {
                result += point.clone();
            }
            bits >>= 1;
        }
    }
    result.x.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_hash_1() {
        let elements = [
            BigUint::from_slice(&[0x405371cb, 0x28feb561, 0xa1393627, 0x9c53068d, 0x1a575610, 0x5caf6453, 0x35c87824, 0x03d937c0]),
            BigUint::from_slice(&[0x9cd8b31a, 0xbbc6aeff, 0x5695e02f, 0x791bf627, 0x880906c2, 0xe1e4bbe2, 0x0250e382, 0x0208a0a1]),
        ];
        let result = BigUint::from_slice(&[0x53f3111b, 0x1e311a7c, 0x139a84d0, 0xa8af1bbb, 0x57037e4a, 0xfb867eaf, 0x76790645, 0x02d895bd]);
        assert_eq!(hash(&elements), result);
    }

    #[test]
    fn test_hash_2() {
        let elements = [
            BigUint::from_slice(&[0x5d479d45, 0x5795fc64, 0x84badc36, 0x2e303ca3, 0x8fe6c43e, 0xb28927c0, 0x10a6ca59, 0x058f5809]),
            BigUint::from_slice(&[0x7515fe0b, 0xd979ab9e, 0x6a4b66bb, 0x79f7b646, 0x8434d71e, 0xbdb39de1, 0x5a067be9, 0x078734f6]),
        ];
        let result = BigUint::from_slice(&[0xc69177e3, 0x5b5050fd, 0x43d43938, 0xe3c4b9e0, 0xc6da775b, 0x2a79c929, 0x4fbb1e6f, 0x014023b4]);
        assert_eq!(hash(&elements), result);
    }
}
