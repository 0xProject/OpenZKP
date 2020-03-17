use zkp_macros_decl::u256h;
use zkp_primefield::*;
use zkp_u256::{MontgomeryParameters, U256};

pub type Element = PrimeField<Order>;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Order();

// TODO: derive one of these from the other.
impl MontgomeryParameters for Order {
    type UInt = U256;

    const M64: u64 = 0xbb6b_3c4c_e8bd_e631;
    const MODULUS: U256 =
        u256h!("0800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f");
    const R1: U256 = u256h!("07fffffffffffdf10000000000000008c75ec4b46df16bee51925a0bf4fca74f");
    const R2: U256 = u256h!("07d9e57c2333766ebaf0ab4cf78bbabb509cf64d14ce60b96021b3f1ea1c688d");
    const R3: U256 = u256h!("01b2ba88ca1fe18a1f0d9dedfedfda501da2136eb8b3f20e81147668fddd0429");
}

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

#[allow(clippy::redundant_clone)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_correct() {
        let half_order: U256 = Element::order() / U256::from(2);
        assert_eq!(
            Element::generator().pow(&half_order) + Element::one(),
            Element::zero()
        );
    }
}
