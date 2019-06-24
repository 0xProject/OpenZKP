use crate::{pedersen::hash, u256::U256};

#[derive(Debug)]
pub struct MakerMessage<T> {
    pub vault_a:  u32,
    pub vault_b:  u32,
    pub amount_a: u64,
    pub amount_b: u64,
    pub token_a:  T,
    pub token_b:  T,
    pub trade_id: u32,
}

pub fn hash_maker(message: &MakerMessage<U256>) -> U256 {
    let mut packed = U256::ZERO;
    packed += &U256::from(u64::from(message.vault_a));
    packed <<= 31;
    packed += &U256::from(u64::from(message.vault_b));
    packed <<= 63;
    packed += &U256::from(message.amount_a);
    packed <<= 63;
    packed += &U256::from(message.amount_b);
    packed <<= 31;
    packed += &U256::from(u64::from(message.trade_id));
    hash(&[
        hash(&[message.token_a.clone(), message.token_b.clone()]),
        packed,
    ])
}

pub fn hash_taker(maker_hash: &U256, vault_a: u32, vault_b: u32) -> U256 {
    hash(&[
        hash(&[maker_hash.clone(), U256::from(u64::from(vault_a))]),
        U256::from(u64::from(vault_b)),
    ])
}

// TODO: Replace literals with u256h!
#[allow(clippy::unreadable_literal)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::u256h;
    use hex_literal::*;

    #[test]
    fn test_hash_maker() {
        let result = hash_maker(&MakerMessage {
            vault_a:  21,
            vault_b:  27,
            amount_a: 6873058723796400,
            amount_b: 852209057714036,
            token_a:  u256h!("005fa3383597691ea9d827a79e1a4f0f7989c35ced18ca9619de8ab97e661020"),
            token_b:  u256h!("00774961c824a3b0fb3d2965f01471c9c7734bf8dbde659e0c08dca2ef18d56a"),
            trade_id: 0,
        });
        // python3 ./signature_cli.py sign_maker --vault_a 21 --vault_b 27
        // --amount_a 6873058723796400 --amount_b 852209057714036
        // --token_a 168976971209324910088270776698114429107164817795147365551345596466024812576\
        // --token_b 210761264384301076547763280170970365712477797880932555831340857495337358698\
        // --trade_id 0 --priv_key
        // 1699550429262868952957733065396688802326540225623380427551300052767936406476
        let expected = u256h!("01921ce52df68f0185ade7572776513304bdd4a07faf6cf28cefc65a86fc496c");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hash_taker() {
        let result = hash_taker(
            &u256h!("01c280f77aa5859027c67411b6859584143d49970528bcbd8db131d39ecf7eb1"),
            2,
            31,
        );
        let expected = u256h!("024e516a8e5f3a523f7725108516bbded20cb290c21925c95836fd66af4c0ec1");
        assert_eq!(result, expected);
    }
}
