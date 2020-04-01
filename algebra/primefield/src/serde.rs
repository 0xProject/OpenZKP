use crate::{Parameters, PrimeField};
use hex::{decode, encode};
use std::{fmt, marker::PhantomData};
use zkp_u256::U256;

use serde::{
    de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, Serializer},
};

impl<P: Parameters<UInt = U256>> Serialize for PrimeField<P> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = U256::from(self).to_bytes_be();
        if serializer.is_human_readable() {
            serializer.serialize_str(&encode(&bytes))
        } else {
            serializer.serialize_bytes(&bytes)
        }
    }
}

struct PrimeFieldVisitor<P> {
    _parameters: PhantomData<P>,
}

impl<'de, P: Parameters<UInt = U256>> Visitor<'de> for PrimeFieldVisitor<P> {
    type Value = PrimeField<P>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "an array of 32 bytes")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() <= 32 {
            let mut held_array = [0_u8; 32];
            held_array.clone_from_slice(v);
            let parsed_uint = U256::from_bytes_be(&held_array);
            // Return a nice error message  if larger than the modulus
            if parsed_uint > P::MODULUS {
                Err(E::custom(format!(
                    "Doesn't fit into the field: {:?}",
                    parsed_uint
                )))
            } else {
                Ok(PrimeField::from(parsed_uint))
            }
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
        Ok(Self::Value::from(U256::from_bytes_be(&held_array)))
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
        Ok(PrimeField::from(U256::from_bytes_be(&held_array)))
    }
}

impl<'de, P: Parameters<UInt = U256>> Deserialize<'de> for PrimeField<P> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = PrimeFieldVisitor::<P> {
            _parameters: PhantomData,
        };
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(visitor)
        } else {
            deserializer.deserialize_bytes(visitor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proth_field::Proth;
    use proptest::prelude::*;

    #[test]
    fn test_serde() {
        proptest!(|(x: PrimeField<Proth>)| {
            let serialized = serde_json::to_string(&x)?;
            let deserialized: PrimeField<Proth> = serde_json::from_str(&serialized)?;
            prop_assert_eq!(deserialized, x);
        });
    }
}
