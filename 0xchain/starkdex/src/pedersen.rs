use crate::pedersen_points::PEDERSEN_POINTS;
use elliptic_curve::{Affine, Jacobian};
use macros_decl::u256h;
use primefield::FieldElement;
use std::prelude::v1::*;
use u256::U256;

// x = 0x049ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804
// y = 0x03ca0cfe4b3bc6ddf346d49d06ea0ed34e621062c0e056c1d0405d266e10268a
pub const SHIFT_POINT: Affine = Affine::Point {
    x: FieldElement::from_montgomery(u256h!(
        "0463d1e72d2ebf3416c727d5f24b5dc16b69f758cd49de911ad69b41a9ba0b3a"
    )),
    y: FieldElement::from_montgomery(u256h!(
        "01211aac6ce572de4298f85b038ef6a8aeae324054290152c5c9927f66d85eeb"
    )),
};

const N_ELEMENTS: usize = 2;
const N_ELEMENT_BITS: usize = 252;

pub fn hash(elements: &[U256]) -> U256 {
    assert!(elements.len() <= N_ELEMENTS);
    hash_impl(elements, 2)
}

pub fn old_hash(elements: &[U256]) -> U256 {
    assert!(elements.len() <= N_ELEMENTS);
    hash_impl(elements, 1)
}

fn hash_impl(elements: &[U256], offset: usize) -> U256 {
    assert!(offset + elements.len() * N_ELEMENT_BITS <= PEDERSEN_POINTS.len());
    let mut result = Jacobian::from(SHIFT_POINT);
    for (i, element) in elements.iter().enumerate() {
        assert!(element.bits() <= N_ELEMENT_BITS);
        let start = offset + i * N_ELEMENT_BITS;
        let end = start + N_ELEMENT_BITS;
        for (j, point) in PEDERSEN_POINTS[start..end].iter().enumerate() {
            if element.bit(j) {
                result += point;
            }
        }
    }
    match Affine::from(&result) {
        Affine::Zero => panic!(),
        Affine::Point { x, .. } => x.into(),
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn hash_enough_points() {
        const MAX_OFFSET: usize = 2;
        assert!(PEDERSEN_POINTS.len() >= MAX_OFFSET + N_ELEMENTS * N_ELEMENT_BITS);
    }

    #[test]
    fn test_hash_0_0() {
        let elements = [U256::from(0), U256::from(0)];
        let result: U256 = match SHIFT_POINT {
            Affine::Zero => panic!(),
            Affine::Point { x, .. } => x.into(),
        };
        assert_eq!(hash(&elements), result);
    }

    #[test]
    fn test_hash_1() {
        let elements = [
            u256h!("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb"),
            u256h!("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a")
        ];
        let result = u256h!("004cd9415015d53d3d71f13e865a52a70457c60fa534fe0efffe34d2f6af6744");
        assert_eq!(hash(&elements), result);
    }

    #[test]
    fn test_hash_2() {
        let elements = [
            u256h!("058f580910a6ca59b28927c08fe6c43e2e303ca384badc365795fc645d479d45"),
            u256h!("078734f65a067be9bdb39de18434d71e79f7b6466a4b66bbd979ab9e7515fe0b"),
        ];
        let result = u256h!("00ffb73f24fb724b208e8efeee07b826a537cae03691c35e679c7c61d776702d");
        assert_eq!(hash(&elements), result);
    }

    #[test]
    fn test_old_hash_0_0() {
        let elements = [U256::from(0), U256::from(0)];
        let result: U256 = match SHIFT_POINT {
            Affine::Zero => panic!(),
            Affine::Point { x, .. } => x.into(),
        };
        assert_eq!(old_hash(&elements), result);
    }

    #[test]
    fn test_old_hash_1() {
        let elements = [
            u256h!("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb"),
            u256h!("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a")
        ];
        let result = u256h!("04ca8b43f335f519a7f70e3481b99e09102a2cb69a0d210d2d85c9d4405f5594");
        assert_eq!(old_hash(&elements), result);
    }

    #[test]
    fn test_old_hash_2() {
        let elements = [
            u256h!("058f580910a6ca59b28927c08fe6c43e2e303ca384badc365795fc645d479d45"),
            u256h!("078734f65a067be9bdb39de18434d71e79f7b6466a4b66bbd979ab9e7515fe0b"),
        ];
        let result = u256h!("041c95a8d5019cbcbf9e9dd3c282ee1b5a492e1ed9b3f8c7c45ff591fa499a2c");
        assert_eq!(old_hash(&elements), result);
    }
}
