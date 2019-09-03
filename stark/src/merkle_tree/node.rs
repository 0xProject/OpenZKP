use super::{Hash, Hashable};
use crate::masked_keccak::MaskedKeccak;

#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Node<'a>(pub &'a Hash, pub &'a Hash);

impl Hashable for Node<'_> {
    fn hash(&self) -> Hash {
        let mut hasher = MaskedKeccak::new();
        hasher.update(self.0.as_bytes());
        hasher.update(self.1.as_bytes());
        hasher.hash()
    }
}
