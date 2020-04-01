use crate::{
    proth_field::{FieldElement, Proth},
    Parameters,
};
use hex::{decode, encode};
use std::fmt;
use zkp_u256::U256;

use serde::{
    de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, Serializer},
};

impl Serialize for FieldElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&encode(&U256::from(self).to_bytes_be()))
        } else {
            serializer.serialize_bytes(&U256::from(self).to_bytes_be())
        }
    }
}

struct FieldElementVisitor;

impl<'de> Visitor<'de> for FieldElementVisitor {
    type Value = FieldElement;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "a byte array containing 32 bytes for field element deseralization"
        )
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
            if parsed_uint > Proth::MODULUS {
                Err(E::custom(format!(
                    "Doesn't fit into the field: {:?}",
                    parsed_uint
                )))
            } else {
                Ok(FieldElement::from(parsed_uint))
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
        Ok(FieldElement::from(U256::from_bytes_be(&held_array)))
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
        Ok(FieldElement::from(U256::from_bytes_be(&held_array)))
    }
}

impl<'de> Deserialize<'de> for FieldElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(FieldElementVisitor)
        } else {
            deserializer.deserialize_bytes(FieldElementVisitor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_serde() {
        proptest!(|(x: FieldElement)| {
            let serialized = serde_json::to_string(&x)?;
            let deserialized: FieldElement = serde_json::from_str(&serialized)?;
            prop_assert_eq!(deserialized, x);
        });
    }
}
