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
    pub digest:  [u8; 32],
    pub counter: u64,
    pub proof:   Vec<u8>,
}

impl Channel {
    pub fn new(data: &[u8]) -> Self {
        let mut digest: [u8; 32] = [0; 32];
        let mut keccak = Keccak::new_keccak256();
        keccak.update(data);
        keccak.finalize(&mut digest);
        let counter = 0;
        let proof = data.to_vec();
        Self {
            digest,
            counter,
            proof,
        }
    }

    pub fn write_bytes(&mut self, data: &[u8]) {
        self.proof.extend_from_slice(data);
        let mut res: [u8; 32] = [0; 32];
        let mut keccak = Keccak::new_keccak256();
        keccak.update(&self.digest);
        keccak.update(data);
        keccak.finalize(&mut res);
        self.digest = res;
        self.counter = 0;
    }

    pub fn bytes(&mut self) -> [u8; 32] {
        let mut res = [0; 32];
        let zero = [0_u8; 24];
        let mut keccak = Keccak::new_keccak256();

        keccak.update(&self.digest);
        keccak.update(&zero);
        keccak.update(&self.counter.to_be_bytes());
        keccak.finalize(&mut res);
        self.counter += 1;
        res
    }

    pub fn pow_find_nonce(&self, pow_bits: u8) -> u64 {
        let mut seed_res = [0_u8; 32];
        let mut keccak = Keccak::new_keccak256();
        keccak.update(&hex!("0123456789abcded"));
        keccak.update(&self.digest);
        keccak.update(&[pow_bits as u8]);
        keccak.finalize(&mut seed_res);

        let test_value = U256::from(2_u64).pow((255 - pow_bits + 1).into()).unwrap();
        for n in 0_u64.. {
            if test_int(n, pow_bits, &test_value, &seed_res) {
                return n as u64;
            }
        }
        0
    }

    // TODO - Make tests compatible with the proof of work values from this function
    pub fn pow_find_nonce_threaded(&self, pow_bits: u8) -> u64 {
        let mut seed_res = [0_u8; 32];
        let mut keccak = Keccak::new_keccak256();
        keccak.update(&hex!("0123456789abcded"));
        keccak.update(&self.digest);
        keccak.update(&[pow_bits as u8]);
        keccak.finalize(&mut seed_res);

        let test_value = U256::from(2_u64).pow((255 - pow_bits + 1).into()).unwrap();
        let ret = (0..u64::max_value())
            .into_par_iter()
            .find_any(|n| -> bool { test_int(*n, pow_bits, &test_value, &seed_res) });
        ret.unwrap() as u64
    }
}

pub fn pow_verify(n: u64, pow_bits: u8, proof: &Channel) -> bool {
    let mut seed_res = [0_u8; 32];
    let mut keccak = Keccak::new_keccak256();
    keccak.update(&hex!("0123456789abcded"));
    keccak.update(&proof.digest);
    keccak.update(&[pow_bits as u8]);
    keccak.finalize(&mut seed_res);

    let test_value = U256::from(2_u64).pow((255 - pow_bits + 1).into()).unwrap();
    test_int(n, pow_bits, &test_value, &seed_res)
}

fn test_int(n: u64, pow_bits: u8, test_value: &U256, seed_res: &[u8; 32]) -> bool {
    // OPT: Inline Keccak256 and work directly on buffer using 'keccakf'
    let mut keccak = Keccak::new_keccak256();
    let mut res = [0; 32];
    keccak.update(seed_res);
    keccak.update(&(n.to_be_bytes()));
    keccak.finalize(&mut res);
    // OPT: Check performance impact of conversion
    let final_int = U256::from_bytes_be(&res);
    if final_int.leading_zeros() == pow_bits as usize {
        final_int < *test_value
    } else {
        false
    }
}

impl ChannelReadable<FieldElement> for Channel {
    fn read(&mut self) -> FieldElement {
        loop {
            let mut res: [u8; 32] = [0; 32];
            let zero = [0_u8; 24];
            let mut keccak = Keccak::new_keccak256();
            keccak.update(&self.digest);
            keccak.update(&zero);
            keccak.update(&self.counter.to_be_bytes());
            keccak.finalize(&mut res);
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
}

impl ChannelReadable<U256> for Channel {
    fn read(&mut self) -> U256 {
        let mut res: [u8; 32] = [0; 32];
        let zero = [0_u8; 24];
        let mut keccak = Keccak::new_keccak256();
        keccak.update(&self.digest);
        keccak.update(&zero);
        keccak.update(&self.counter.to_be_bytes());
        keccak.finalize(&mut res);
        self.counter += 1;
        U256::from_bytes_be(&res)
    }
}

impl ChannelReadable<[u8; 32]> for Channel {
    fn read(&mut self) -> [u8; 32] {
        let mut res = [0; 32];
        let zero = [0_u8; 24];
        let mut keccak = Keccak::new_keccak256();
        keccak.update(&self.digest);
        keccak.update(&zero);
        keccak.update(&self.counter.to_be_bytes());
        keccak.finalize(&mut res);
        self.counter += 1;
        res
    }
}

impl ChannelWritable<&[u8]> for Channel {
    fn write(&mut self, data: &[u8]) {
        self.write_bytes(data);
    }
}

// TODO - Make into a hash type label
impl ChannelWritable<&[u8; 32]> for Channel {
    fn write(&mut self, data: &[u8; 32]) {
        self.write_bytes(data);
    }
}

impl ChannelWritable<u64> for Channel {
    fn write(&mut self, data: u64) {
        self.write_bytes(&data.to_be_bytes());
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
        self.write_bytes(&container.as_slice());
    }
}

impl ChannelWritable<&FieldElement> for Channel {
    fn write(&mut self, data: &FieldElement) {
        self.write_bytes(&data.0.to_bytes_be());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proof_of_work_test() {
        let rand_source = Channel::new(hex!("0123456789abcded").to_vec().as_slice());
        let work = rand_source.pow_find_nonce(15);
        assert!(pow_verify(work, 15, &rand_source));
    }

    #[test]
    fn threaded_proof_of_work_test() {
        let rand_source = Channel::new(hex!("0123456789abcded").to_vec().as_slice());
        let work = rand_source.pow_find_nonce_threaded(15);
        assert!(pow_verify(work, 15, &rand_source));
    }
}
