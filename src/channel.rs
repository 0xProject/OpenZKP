use crate::{field::*, u256::U256, u256h};
use hex_literal::*;
use rayon::prelude::*;
use tiny_keccak::Keccak;

pub trait ChannelReadable<T> {
    fn read(&mut self) -> T;
}

pub trait ChannelWritable<T> {
    fn write(&mut self, data: T);
}

#[derive(PartialEq, Eq, Clone, Default)]
pub struct Channel {
    pub digest: [u8; 32],
    counter:    u64,
    pub proof:  Vec<u8>,
}

impl Channel {
    pub fn new(seed: &[u8]) -> Self {
        let mut digest: [u8; 32] = [0; 32];
        let mut keccak = Keccak::new_keccak256();
        keccak.update(seed);
        keccak.finalize(&mut digest);
        Self {
            digest,
            counter: 0,
            proof: seed.to_vec(),
        }
    }

    pub fn pow_find_nonce(&self, pow_bits: u8) -> u64 {
        let seed = self.pow_seed(pow_bits);

        (0u64..)
            .find(|&nonce| pow_verify_with_seed(nonce, pow_bits, &seed))
            .expect("No valid nonce found")
    }

    // TODO - Make tests compatible with the proof of work values from this function
    pub fn pow_find_nonce_threaded(&self, pow_bits: u8) -> u64 {
        let seed = self.pow_seed(pow_bits);
        // NOTE: Rayon does not support open ended ranges, so we need to use a closed
        // one.
        (0..u64::max_value())
            .into_par_iter()
            .find_any(|&nonce| pow_verify_with_seed(nonce, pow_bits, &seed))
            .expect("No valid nonce found")
    }

    pub fn pow_seed(&self, pow_bits: u8) -> [u8; 32] {
        let mut seed = [0_u8; 32];
        let mut keccak = Keccak::new_keccak256();
        keccak.update(&hex!("0123456789abcded"));
        keccak.update(&self.digest);
        keccak.update(&[pow_bits]);
        keccak.finalize(&mut seed);
        seed
    }

    pub fn pow_verify(&self, n: u64, pow_bits: u8) -> bool {
        let seed = self.pow_seed(pow_bits);
        pow_verify_with_seed(n, pow_bits, &seed)
    }
}

fn pow_verify_with_seed(n: u64, pow_bits: u8, seed: &[u8; 32]) -> bool {
    // OPT: Inline Keccak256 and work directly on buffer using 'keccakf'
    let mut keccak = Keccak::new_keccak256();
    let mut digest = [0; 32];
    keccak.update(seed);
    keccak.update(&(n.to_be_bytes()));
    keccak.finalize(&mut digest);
    // OPT: Check performance impact of conversion
    let work = U256::from_bytes_be(&digest).leading_zeros();
    work >= pow_bits as usize
}

impl ChannelReadable<FieldElement> for Channel {
    fn read(&mut self) -> FieldElement {
        const MASK: U256 =
            u256h!("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
        loop {
            let number: U256 = self.read();
            let seed = number & MASK;
            if seed < MODULUS {
                // TODO: Avoid accessing FieldElement members directly
                return FieldElement { 0: seed };
            }
        }
    }
}

impl ChannelReadable<U256> for Channel {
    fn read(&mut self) -> U256 {
        U256::from_bytes_be(&self.read())
    }
}

impl ChannelReadable<[u8; 32]> for Channel {
    fn read(&mut self) -> [u8; 32] {
        let mut result = [0; 32];
        let mut keccak = Keccak::new_keccak256();
        keccak.update(&self.digest);
        keccak.update(&[0_u8; 24]);
        keccak.update(&self.counter.to_be_bytes());
        keccak.finalize(&mut result);
        self.counter += 1;
        result
    }
}

impl ChannelWritable<&[u8]> for Channel {
    fn write(&mut self, data: &[u8]) {
        self.proof.extend_from_slice(data);
        let mut result: [u8; 32] = [0; 32];
        let mut keccak = Keccak::new_keccak256();
        keccak.update(&self.digest);
        keccak.update(data);
        keccak.finalize(&mut result);
        self.digest = result;
        self.counter = 0;
    }
}

// TODO - Make into a hash type label
impl ChannelWritable<&[u8; 32]> for Channel {
    fn write(&mut self, data: &[u8; 32]) {
        self.write(&data[..]);
    }
}

impl ChannelWritable<u64> for Channel {
    fn write(&mut self, data: u64) {
        self.write(&data.to_be_bytes()[..]);
    }
}

impl ChannelWritable<&[FieldElement]> for Channel {
    fn write(&mut self, data: &[FieldElement]) {
        let mut container = Vec::with_capacity(32 * data.len());
        for element in data {
            for byte in U256::to_bytes_be(&element.0).iter() {
                container.push(byte.clone());
            }
        }
        self.write(container.as_slice());
    }
}

impl ChannelWritable<&FieldElement> for Channel {
    fn write(&mut self, data: &FieldElement) {
        // TODO: Avoid accessing FieldElement members directly
        self.write(&data.0.to_bytes_be()[..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proof_of_work_test() {
        let rand_source = Channel::new(hex!("0123456789abcded").to_vec().as_slice());
        let work = rand_source.pow_find_nonce(15);
        assert!(&rand_source.pow_verify(work, 15));
    }

    #[test]
    fn threaded_proof_of_work_test() {
        let rand_source = Channel::new(hex!("0123456789abcded").to_vec().as_slice());
        let work = rand_source.pow_find_nonce_threaded(15);
        assert!(&rand_source.pow_verify(work, 15));
    }
}
