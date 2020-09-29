// Clippy false positive
#[allow(clippy::useless_attribute)]
// We want to import an alternative prelude.
#[allow(clippy::wildcard_imports)]
use std::prelude::v1::*;

use crate::{uint::UInt, Parameters, PrimeField};
use parity_scale_codec::{Decode, Encode, Error, Input, Output};

impl<U, P> Encode for PrimeField<P>
where
    U: UInt + Encode,
    P: Parameters<UInt = U>,
{
    fn size_hint(&self) -> usize {
        self.to_uint().size_hint()
    }

    fn encode_to<T: Output>(&self, dest: &mut T) {
        self.to_uint().encode_to(dest);
    }

    fn encode(&self) -> Vec<u8> {
        self.to_uint().encode()
    }

    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
        self.to_uint().using_encoded(f)
    }
}

impl<U, P> Decode for PrimeField<P>
where
    U: UInt + Decode,
    P: Parameters<UInt = U>,
{
    fn decode<I: Input>(value: &mut I) -> Result<Self, Error> {
        Ok(Self::from_uint(&U::decode(value)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proth_field::Proth;
    use proptest::prelude::*;

    #[test]
    fn test_roundtrip() {
        proptest!(|(x: PrimeField<Proth>)| {
            let serialized = x.encode();
            // Deserialize consumes a mutable slice reference.
            let mut slice = serialized.as_slice();
            let deserialized: PrimeField<Proth> = Decode::decode(&mut slice)?;
            prop_assert_eq!(slice.len(), 0); // Consumes all
            prop_assert_eq!(deserialized, x);
        });
    }
}
