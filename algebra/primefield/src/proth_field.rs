use crate::{Parameters, PrimeField};
#[cfg(feature = "std")]
use hex::{decode, encode};
use num_traits::identities::Zero;
#[cfg(feature = "std")]
use serde::{
    de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, Serializer},
};
#[cfg(feature = "std")]
use std::fmt;
use std::marker::PhantomData;
use zkp_macros_decl::u256h;
use zkp_u256::{to_montgomery_const, U256};

// TODO: Fix naming
#[allow(clippy::module_name_repetitions)]
pub type FieldElement = PrimeField<Proth>;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Proth();

impl Parameters for Proth {
    type UInt = U256;

    /// 3, in montgomery form.
    const GENERATOR: U256 =
        u256h!("07fffffffffff9b0ffffffffffffffffffffffffffffffffffffffffffffffa1");
    const M64: u64 = 0xffff_ffff_ffff_ffff;
    const MODULUS: U256 =
        u256h!("0800000000000011000000000000000000000000000000000000000000000001");
    ///
    const ORDER: U256 = u256h!("0800000000000011000000000000000000000000000000000000000000000000");
    const R1: U256 = u256h!("07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1");
    const R2: U256 = u256h!("07ffd4ab5e008810ffffffffff6f800000000001330ffffffffffd737e000401");
    const R3: U256 = u256h!("038e5f79873c0a6df47d84f8363000187545706677ffcc06cc7177d1406df18e");
}

impl FieldElement {
    /// Creates a constant value from a `U256` constant in Montgomery form.
    // TODO: Make member of `Field` after <https://github.com/rust-lang/rust/issues/57563>
    pub const fn from_montgomery_const(uint: U256) -> Self {
        Self {
            uint,
            _parameters: PhantomData,
        }
    }

    /// Creates a constant value from a `U256` constant.
    ///
    /// It does compile-time conversion to Montgomery form.
    // TODO: Make member of `Field` after <https://github.com/rust-lang/rust/issues/57563>
    pub const fn from_uint_const(n: &U256) -> Self {
        let uint = to_montgomery_const(n, &Proth::MODULUS, Proth::M64, &Proth::R2);
        Self {
            uint,
            _parameters: PhantomData,
        }
    }
}

// TODO: Find a way to create generic implementations of these
impl From<FieldElement> for U256 {
    #[inline(always)]
    fn from(other: FieldElement) -> Self {
        other.to_uint()
    }
}

impl From<&FieldElement> for U256 {
    #[inline(always)]
    fn from(other: &FieldElement) -> Self {
        other.to_uint()
    }
}

impl Default for FieldElement {
    fn default() -> Self {
        Self::zero()
    }
}

#[cfg(feature = "std")]
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

#[cfg(feature = "std")]
struct FieldElementVisitor;

#[cfg(feature = "std")]
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

#[cfg(feature = "std")]
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
