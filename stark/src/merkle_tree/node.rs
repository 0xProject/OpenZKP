#[derive(Clone, Debug)]
struct Node<'a>(&'a Hash, &'a Hash);

impl Hashable for Node<'_> {
    fn hash(&self) -> Hash {
        let mut hasher = MaskedKeccak::new();
        hasher.update(self.0.as_bytes());
        hasher.update(self.1.as_bytes());
        hasher.hash()
    }
}
