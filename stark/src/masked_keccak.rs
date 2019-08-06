use crate::hash::Hash;
use tiny_keccak::Keccak;

pub struct MaskedKeccak(Keccak);

impl MaskedKeccak {
    pub fn new() -> Self {
        MaskedKeccak(Keccak::new_keccak256())
    }

    pub fn update(&mut self, input: &[u8]) {
        self.0.update(input)
    }

    pub fn hash(self) -> Hash {
        let mut result: [u8; 32] = [0; 32];
        self.0.finalize(&mut result);
        for byte in result[20..].iter_mut() {
            *byte = 0;
        }
        Hash::new(result)
    }
}
