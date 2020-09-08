#[cfg(feature = "parity_codec")]
use parity_scale_codec::{Decode, Encode};
use zkp_macros_decl::u256h;
use zkp_primefield::{Parameters, PrimeField};
use zkp_u256::U256;

pub type Element = PrimeField<Order>;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
#[cfg_attr(feature = "parity_codec", derive(Encode, Decode))]
pub struct Order();

impl Parameters for Order {
    type UInt = U256;

    const GENERATOR: U256 = u256h!("03");
    const M64: u64 = 0xbb6b_3c4c_e8bd_e631;
    const MODULUS: U256 =
        u256h!("0800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f");
    const ORDER: U256 = u256h!("0800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2e");
    const R1: U256 = u256h!("07fffffffffffdf10000000000000008c75ec4b46df16bee51925a0bf4fca74f");
    const R2: U256 = u256h!("07d9e57c2333766ebaf0ab4cf78bbabb509cf64d14ce60b96021b3f1ea1c688d");
    const R3: U256 = u256h!("01b2ba88ca1fe18a1f0d9dedfedfda501da2136eb8b3f20e81147668fddd0429");
}

#[cfg(test)]
mod tests {
    use super::*;
    use zkp_primefield::{One, Pow, Zero};

    const PRIME_FACTORS: [U256; 5] = [
        u256h!("02"),
        u256h!("03"),
        u256h!("16996f"),
        u256h!("046fcf43"),
        u256h!("03677abc4eb30ccc5c90ee8268a99c0bbc3cf562d68c5c461f11"),
    ];

    #[test]
    fn prime_factors_correct() {
        assert_eq!(
            PRIME_FACTORS.iter().cloned().product::<U256>(),
            Element::order()
        );
        for factor in &PRIME_FACTORS {
            assert!((Element::order() % factor).is_zero());
        }
    }

    #[test]
    fn generator_correct() {
        for factor in &PRIME_FACTORS {
            let exponent = Element::order() / factor;
            assert!(!Element::generator().pow(&exponent).is_one());
        }
    }
}
