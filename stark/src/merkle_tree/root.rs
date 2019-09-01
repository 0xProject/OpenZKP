use super::{Hash, Proof};

#[derive(Clone, Debug)]
pub struct Root {
    depth: usize,
    root:  Hash,
}

impl Root {
    pub fn verify(&self, proof: &Proof) -> bool {}
}
