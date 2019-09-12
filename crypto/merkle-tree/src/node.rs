use hash::{Hash, Hashable, MaskedKeccak};

#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub(crate) struct Node<'a>(pub(crate) &'a Hash, pub(crate) &'a Hash);

impl Hashable for Node<'_> {
    fn hash(&self) -> Hash {
        let mut hasher = MaskedKeccak::new();
        hasher.update(self.0.as_bytes());
        hasher.update(self.1.as_bytes());
        hasher.hash()
    }
}
