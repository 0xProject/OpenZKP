use crate::U256;
#[cfg(feature = "std")]
use hex::{decode, encode};
#[cfg(feature = "std")]
use serde::{
    de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, Serializer},
};
#[cfg(feature = "std")]
use std::fmt;
use std::{prelude::v1::*, u64};

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

#[cfg(feature = "std")]
impl Serialize for U256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.to_bytes_be();
        if serializer.is_human_readable() {
            encode(&bytes).serialize(serializer)
        } else {
            bytes.serialize(serializer)
        }
    }
}

#[cfg(feature = "std")]
struct U256Visitor;

#[cfg(feature = "std")]
impl<'de> Visitor<'de> for U256Visitor {
    type Value = U256;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "a byte array containing 32 bytes")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() <= 32 {
            let mut held_array = [0_u8; 32];
            held_array.clone_from_slice(v);
            Ok(U256::from_bytes_be(&held_array))
        } else {
            Err(E::custom(format!("Too many bytes: {}", v.len())))
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut holder = Vec::with_capacity(32);

        while let Some(elem) = seq.next_element().unwrap() {
            holder.push(elem);
        }

        let mut held_array = [0_u8; 32];
        held_array.clone_from_slice(holder.as_slice());
        Ok(U256::from_bytes_be(&held_array))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
        let mut holder: Vec<u8> = match decode(v) {
            Ok(x) => x,
            Err(r) => {
                panic!("hex decoder error: {:?}", r);
            }
        };

        let pading_len = 32 - holder.len();
        if pading_len > 0 {
            let mut new_vec: Vec<u8> = Vec::with_capacity(32);
            for _ in 0..pading_len {
                new_vec.push(0);
            }
            new_vec.append(&mut holder);
            holder = new_vec;
        }

        let mut held_array = [0_u8; 32];
        held_array.clone_from_slice(holder.as_slice());
        Ok(U256::from_bytes_be(&held_array))
    }
}

#[cfg(feature = "std")]
impl<'de> Deserialize<'de> for U256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(U256Visitor)
        } else {
            deserializer.deserialize_bytes(U256Visitor)
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

    #[test]
    fn test_one() {
        let one = U256::one();
        let serialized = serde_json::to_string(&one).unwrap();
        assert_eq!(
            serialized,
            "\"0000000000000000000000000000000000000000000000000000000000000001\""
        );
    }

    #[test]
    fn test_serde() {
        proptest!(|(x: U256)| {
            let serialized = serde_json::to_string(&x)?;
            let deserialized: U256 = serde_json::from_str(&serialized)?;
            prop_assert_eq!(deserialized, x);
        });
    }
}
