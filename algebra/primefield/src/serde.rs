use crate::{uint::UInt, Parameters, PrimeField};
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

impl<U, P> Serialize for PrimeField<P>
where
    U: UInt + Serialize,
    P: Parameters<UInt = U>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_uint().serialize(serializer)
    }
}

impl<'de, U, P> Deserialize<'de> for PrimeField<P>
where
    U: UInt + Deserialize<'de>,
    P: Parameters<UInt = U>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let u = U::deserialize(deserializer)?;
        Ok(Self::from_uint(&u))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proth_field::Proth;
    use num_traits::identities::One;
    use proptest::prelude::*;

    #[test]
    fn test_one_string() {
        let one = PrimeField::<Proth>::one();
        let serialized = serde_json::to_string(&one).unwrap();
        assert_eq!(
            serialized,
            "\"0x0000000000000000000000000000000000000000000000000000000000000001\""
        );
    }

    #[test]
    fn test_serde() {
        proptest!(|(x: PrimeField<Proth>)| {
            let serialized = serde_json::to_string_pretty(&x)?;
            let deserialized: PrimeField<Proth> = serde_json::from_str(&serialized)?;
            prop_assert_eq!(deserialized, x);
        });
    }
}
