// False positive: attribute has a use
#[allow(clippy::useless_attribute)]
// False positive: Importing preludes is allowed
#[allow(clippy::wildcard_imports)]
use std::prelude::v1::*;

use crate::U256;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::u64;

impl U256 {
    pub fn from_bytes_be(n: &[u8; 32]) -> Self {
        Self::from_limbs([
            u64::from_be_bytes([n[24], n[25], n[26], n[27], n[28], n[29], n[30], n[31]]),
            u64::from_be_bytes([n[16], n[17], n[18], n[19], n[20], n[21], n[22], n[23]]),
            u64::from_be_bytes([n[8], n[9], n[10], n[11], n[12], n[13], n[14], n[15]]),
            u64::from_be_bytes([n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7]]),
        ])
    }

    pub fn to_bytes_be(&self) -> [u8; 32] {
        let mut r = [0; 32];
        let mut n = self.clone();
        // We want truncation here
        #[allow(clippy::cast_possible_truncation)]
        for i in (0..32).rev() {
            r[i] = n.limb(0) as u8;
            n >>= 8;
        }
        r
    }
}

impl Serialize for U256 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            self.to_hex_string().serialize(serializer)
        } else {
            self.to_bytes_be().serialize(serializer)
        }
    }
}

impl<'a> Deserialize<'a> for U256 {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            <&str>::deserialize(deserializer).map(U256::from_hex_str)
        } else {
            <[u8; 32]>::deserialize(deserializer).map(|b| U256::from_bytes_be(&b))
        }
    }
}

macro_rules! impl_from_uint {
    ($type:ty) => {
        impl From<$type> for U256 {
            // $type could be u64, which triggers the lint.
            #[allow(trivial_numeric_casts)]
            fn from(n: $type) -> Self {
                Self::from_limbs([n as u64, 0, 0, 0])
            }
        }
    };
}

impl_from_uint!(u8);
impl_from_uint!(u16);
impl_from_uint!(u32);
impl_from_uint!(u64);
impl_from_uint!(usize);

impl From<u128> for U256 {
    fn from(n: u128) -> Self {
        // We want truncation here
        #[allow(clippy::cast_possible_truncation)]
        Self::from_limbs([n as u64, (n >> 64) as u64, 0, 0])
    }
}

macro_rules! impl_from_int {
    ($t:ty) => {
        impl From<$t> for U256 {
            // We want twos-complement casting
            #[allow(clippy::cast_sign_loss)]
            // We want truncation here
            #[allow(clippy::cast_possible_truncation)]
            fn from(n: $t) -> Self {
                if n >= 0 {
                    Self::from_limbs([n as u64, 0, 0, 0])
                } else {
                    Self::from_limbs([
                        n as u64,
                        u64::max_value(),
                        u64::max_value(),
                        u64::max_value(),
                    ])
                }
            }
        }
    };
}

impl_from_int!(i8);
impl_from_int!(i16);
impl_from_int!(i32);
impl_from_int!(i64);
impl_from_int!(isize);

impl From<i128> for U256 {
    // We want twos-complement casting
    #[allow(clippy::cast_sign_loss)]
    // We want truncation here
    #[allow(clippy::cast_possible_truncation)]
    fn from(n: i128) -> Self {
        if n >= 0 {
            Self::from_limbs([n as u64, (n >> 64) as u64, 0, 0])
        } else {
            Self::from_limbs([
                n as u64,
                (n >> 64) as u64,
                u64::max_value(),
                u64::max_value(),
            ])
        }
    }
}

macro_rules! as_int {
    ($name:ident, $type:ty) => {
        // $type could be u64, which triggers the lint.
        #[allow(trivial_numeric_casts)]
        pub fn $name(&self) -> $type {
            self.limb(0) as $type
        }
    };
}

// We don't want newlines between the macro invocations.
#[rustfmt::skip]
impl U256 {
    as_int!(as_u8, u8);
    as_int!(as_u16, u16);
    as_int!(as_u32, u32);
    as_int!(as_u64, u64);
    as_int!(as_usize, usize);
    as_int!(as_i8, i8);
    as_int!(as_i16, i16);
    as_int!(as_i32, i32);
    as_int!(as_i64, i64);
    as_int!(as_isize, isize);

    // Clippy is afraid that casting u64 to u128 is lossy
    #[allow(clippy::cast_lossless)]
    pub fn as_u128(&self) -> u128 {
        (self.limb(0) as u128) | ((self.limb(1) as u128) << 64)
    }

    // Clippy is afraid that casting u64 to u128 is lossy
    #[allow(clippy::cast_lossless)]
    pub fn as_i128(&self) -> i128 {
        (self.limb(0) as i128) | ((self.limb(1) as i128) << 64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::identities::One;
    use proptest::prelude::*;

    #[cfg(feature = "parity_codec")]
    use parity_scale_codec::{Decode, Encode};

    #[test]
    fn test_one() {
        let one = U256::one();
        let serialized = serde_json::to_string(&one).unwrap();
        assert_eq!(
            serialized,
            "\"0x0000000000000000000000000000000000000000000000000000000000000001\""
        );
    }

    #[test]
    fn test_serde_json() {
        proptest!(|(x: U256)| {
            let serialized = serde_json::to_string(&x)?;
            let deserialized: U256 = serde_json::from_str(&serialized)?;
            prop_assert_eq!(deserialized, x);
        });
    }

    #[test]
    fn test_serde_bincode() {
        proptest!(|(x: U256)| {
            let serialized = bincode::serialize(&x)?;
            let deserialized: U256 = bincode::deserialize(&serialized)?;
            prop_assert_eq!(deserialized, x);
        });
    }

    #[cfg(feature = "parity_codec")]
    #[test]
    fn test_parity_codec_one() {
        let one = U256::one();
        let serialized = one.encode();
        assert_eq!(
            hex::encode(serialized),
            "0100000000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[cfg(feature = "parity_codec")]
    #[test]
    fn test_parity_codec() {
        proptest!(|(x: U256)| {
            let serialized = x.encode();
            // Deserialize consumes a mutable slice reference.
            let mut slice = serialized.as_slice();
            let deserialized: U256 = U256::decode(&mut slice)?;
            prop_assert_eq!(slice.len(), 0); // Consumes all
            prop_assert_eq!(deserialized, x);
        });
    }

    #[cfg(feature = "parity_codec")]
    #[test]
    fn test_parity_little_endian() {
        proptest!(|(x: U256)| {
            let serialized = x.encode();
            // Encoding is lsb first (little-endian order)
            // We prefer big-endian in IO, but the actual memory layout is
            // little-endian. Having the encoding be identical to the memory
            // layout may give a performance advantage down the line, which
            // seems to be the goal of the Parity Scale codec.
            let little_endian: Vec<u8> = x.to_bytes_be().iter().rev().cloned().collect();
            prop_assert_eq!(serialized, little_endian);
        });
    }
}
