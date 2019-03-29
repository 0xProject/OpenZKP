use crate::pedersen::hash;
use num::{traits::cast::FromPrimitive, traits::Zero, BigUint};

pub struct MakerMessage<T> {
    pub vault_a: u32,
    pub vault_b: u32,
    pub amount_a: u64,
    pub amount_b: u64,
    pub token_a: T,
    pub token_b: T,
    pub trade_id: u32,
}

pub fn hash_maker(message: &MakerMessage<BigUint>) -> BigUint {
    let mut packed = BigUint::zero();
    packed += BigUint::from(message.vault_a);
    packed <<= 32;
    packed += BigUint::from(message.vault_b);
    packed <<= 63;
    packed += BigUint::from(message.amount_a);
    packed <<= 63;
    packed += BigUint::from(message.amount_b);
    packed <<= 32;
    packed += BigUint::from(message.trade_id);
    hash(&[
        hash(&[message.token_a.clone(), message.token_b.clone()]),
        packed,
    ])
}

pub fn hash_taker(maker_hash: &BigUint, vault_a: u32, vault_b: u32) -> BigUint {
    hash(&[
        hash(&[maker_hash.clone(), BigUint::from(vault_a)]),
        BigUint::from(vault_b),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_maker() {
        let result = hash_maker(&MakerMessage {
            vault_a: 21,
            vault_b: 27,
            amount_a: 2154686749748910716,
            amount_b: 1470242115489520459,
            token_a: BigUint::from_slice(&[
                0x7e661020, 0x19de8ab9, 0xed18ca96, 0x7989c35c, 0x9e1a4f0f, 0xa9d827a7, 0x3597691e,
                0x005fa338,
            ]),
            token_b: BigUint::from_slice(&[
                0xef18d56a, 0x0c08dca2, 0xdbde659e, 0xc7734bf8, 0xf01471c9, 0xfb3d2965, 0xc824a3b0,
                0x00774961,
            ]),
            trade_id: 0,
        });
        let expected = BigUint::from_slice(&[
            0x9aac35d0, 0x64bf61c3, 0x66ebef73, 0xc0b5d6f2, 0x29b4d30a, 0xd7b4e9d3, 0xda71b3f5,
            0x01e542e2,
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hash_taker() {
        let result = hash_taker(
            &BigUint::from_slice(&[
                0x9aac35d0, 0x64bf61c3, 0x66ebef73, 0xc0b5d6f2, 0x29b4d30a, 0xd7b4e9d3, 0xda71b3f5,
                0x01e542e2,
            ]),
            2,
            31,
        );
        let expected = BigUint::from_slice(&[
            0xb8cba1b1, 0x9b231805, 0x5499fa31, 0x1d602af0, 0x2d00cad6, 0xf4f1d615, 0x6e819b57,
            0x039df78a,
        ]);
        assert_eq!(result, expected);
    }
}
