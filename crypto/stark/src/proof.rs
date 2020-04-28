use std::prelude::v1::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Proof(Vec<u8>);

impl Proof {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}
