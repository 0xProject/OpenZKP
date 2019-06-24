use crate::{field::*, u256::U256, u256h};
use hex_literal::*;
use tiny_keccak::Keccak;

#[derive(PartialEq, Eq, Clone, Default)]
pub struct Channel {
    pub digest:  [u8; 32],
    pub counter: u64,
    pub proof:   Vec<u8>,
}

impl Channel {
    pub fn new(data: &[u8]) -> Self {
        let mut digest: [u8; 32] = [0; 32];
        let mut sha3 = Keccak::new_keccak256();
        sha3.update(data);
        sha3.finalize(&mut digest);
        let counter = 0;
        let proof = data.to_vec();
        Self {
            digest,
            counter,
            proof,
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.proof.extend_from_slice(data);
        let mut res: [u8; 32] = [0; 32];
        let mut sha3 = Keccak::new_keccak256();
        sha3.update(&self.digest);
        sha3.update(data);
        sha3.finalize(&mut res);
        self.digest = res;
        self.counter = 0;
    }

    pub fn write_element(&mut self, data: &FieldElement) {
        self.write(&data.0.to_bytes_be());
    }

    pub fn write_element_list(&mut self, data: &[FieldElement]) {
        let mut container = Vec::with_capacity(32 * data.len());
        for element in data {
            for byte in U256::to_bytes_be(&element.0).iter() {
                container.push(byte.clone());
            }
        }
        self.write(&container.as_slice());
    }

    pub fn element(&mut self) -> FieldElement {
        loop {
            let mut res: [u8; 32] = [0; 32];
            let zero = [0_u8; 24];
            let mut sha3 = Keccak::new_keccak256();
            sha3.update(&self.digest);
            sha3.update(&zero);
            sha3.update(&self.counter.to_be_bytes());
            sha3.finalize(&mut res);
            self.counter += 1;
            let seed = U256::from_bytes_be(&res)
                % u256h!("1000000000000000000000000000000000000000000000000000000000000000"); // 2^256
            if seed < MODULUS {
                return FieldElement::from(seed)
                    / FieldElement::from(u256h!(
                        "07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1"
                    ));
            }
        }
    }

    pub fn bytes(&mut self) -> [u8; 32] {
        let mut res = [0; 32];
        let zero = [0_u8; 24];
        let mut sha3 = Keccak::new_keccak256();

        sha3.update(&self.digest);
        sha3.update(&zero);
        sha3.update(&self.counter.to_be_bytes());
        sha3.finalize(&mut res);
        self.counter += 1;
        res
    }
}
