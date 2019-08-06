use crate::{hash::Hash, masked_keccak::MaskedKeccak};
use primefield::FieldElement;
use u256::U256;

pub trait Hashable {
    fn hash(&self) -> Hash;
}

impl Hashable for Hash {
    fn hash(&self) -> Hash {
        self.clone()
    }
}

impl Hashable for U256 {
    fn hash(&self) -> Hash {
        // U256 values are passed as-is
        // OPT: Figure out a way to get in-place access.
        Hash::new(self.to_bytes_be())
    }
}

impl Hashable for FieldElement {
    fn hash(&self) -> Hash {
        // We hash as U256 in Montgomery form (which is identity-hashed)
        self.as_montogomery_u256().hash()
    }
}

impl<T: Hashable> Hashable for [T] {
    fn hash(&self) -> Hash {
        if self.len() == 1 {
            // For a single element, return its hash.
            self[0].hash()
        } else {
            // Concatenate the element hashes and hash the result.
            let mut hasher = MaskedKeccak::new();
            for value in self {
                hasher.update(value.hash().as_bytes());
            }
            hasher.hash()
        }
    }
}

impl<T: Hashable> Hashable for Vec<T> {
    fn hash(&self) -> Hash {
        self.as_slice().hash()
    }
}
