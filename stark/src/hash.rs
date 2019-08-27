#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}
