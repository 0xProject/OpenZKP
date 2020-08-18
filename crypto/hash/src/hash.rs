#[cfg(feature = "std")]
use std::fmt;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Hash([u8; 32]);

impl Hash {
    #[must_use]
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "std")]
impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hash(0x{:})", hex::encode(self.0))
    }
}
