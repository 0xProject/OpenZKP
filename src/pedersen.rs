use crate::curve::Affine;
use crate::field::FieldElement;
use crate::jacobian::Jacobian;
use crate::pedersen_points::PEDERSEN_POINTS;
use crate::u256::U256;
use crate::u256h;
use hex_literal::*;

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

pub const N_ELEMENT_BITS: usize = 252;

pub fn hash(elements: &[U256]) -> U256 {
    let mut result = Jacobian::from(SHIFT_POINT);
    for (i, element) in elements.iter().enumerate() {
        assert!(element.bits() <= N_ELEMENT_BITS);
        let start = 2 + i * N_ELEMENT_BITS;
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
    fn test_hash_1() {
        let elements = [
            u256h!("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb"),
            u256h!("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a")
        ];
        let result = u256h!("02d895bd76790645fb867eaf57037e4aa8af1bbb139a84d01e311a7c53f3111b");
        assert_eq!(hash(&elements), result);
    }

    #[test]
    fn test_hash_2() {
        let elements = [
            u256h!("058f580910a6ca59b28927c08fe6c43e2e303ca384badc365795fc645d479d45"),
            u256h!("078734f65a067be9bdb39de18434d71e79f7b6466a4b66bbd979ab9e7515fe0b"),
        ];
        let result = u256h!("014023b44fbb1e6f2a79c929c6da775be3c4b9e043d439385b5050fdc69177e3");
        assert_eq!(hash(&elements), result);
    }
}
