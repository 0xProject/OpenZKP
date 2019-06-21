use crate::{field::*, u256::U256, u256h};
use hex_literal::*;
use rayon::prelude::*;
use tiny_keccak::Keccak;

pub trait ChannelReadable<T> {
    fn read(&mut self) -> T;
}

impl ChannelReadable<[u8; 32]> for Channel {
    fn read(&mut self) -> [u8; 32] {
        self.bytes()
    }
}

impl ChannelReadable<FieldElement> for Channel {
    fn read(&mut self) -> FieldElement {
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
                % u256h!("1000000000000000000000000000000000000000000000000000000000000000"); //2^256
            if seed < MODULUS {
                return FieldElement::from(seed)
                    / FieldElement::from(u256h!(
                        "07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1"
                    ));
            }
        }
    }
}

pub trait ChannelWritable<T> {
    fn write(&mut self, data: T);
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
<<<<<<< HEAD

    pub fn write(&mut self, data: &[u8]) {
=======
    pub fn write_bytes(&mut self, data: &[u8]) {
>>>>>>> Added traits to read and write from a channel
        self.proof.extend_from_slice(data);
        let mut res: [u8; 32] = [0; 32];
        let mut sha3 = Keccak::new_keccak256();
        sha3.update(&self.digest);
        sha3.update(data);
        sha3.finalize(&mut res);
        self.digest = res;
        self.counter = 0;
    }

<<<<<<< HEAD
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

=======
>>>>>>> Added traits to read and write from a channel
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

    pub fn pow_find_nonce(&self, pow_bits: u64) -> u64 {
        let mut seed = hex!("0123456789abcded").to_vec();
        seed.extend_from_slice(&self.digest);
        for byte in pow_bits.to_be_bytes().iter() {
            if *byte > 0 {
                seed.push(*byte);
                break;
            }
        }
        let mut seed_res = [0_u8; 32];
        let mut sha3 = Keccak::new_keccak256();
        sha3.update(&seed);
        sha3.finalize(&mut seed_res);

        let test_value = U256::from(2_u64).pow(256 - pow_bits).unwrap();
        for n in 0..(u64::max_value() as usize) {
            let mut sha3 = Keccak::new_keccak256();
            let mut res = [0; 32];
            sha3.update(&seed_res);
            sha3.update(&(n.to_be_bytes()));
            sha3.finalize(&mut res);
            let final_int = U256::from_bytes_be(&res);
            if final_int.leading_zeros() == pow_bits as usize && final_int < test_value {
                // Only do the large int compare if the quick logs match
                return n as u64;
            }
        }
        0
    }

    // TODO - Make tests compatible with the proof of work values from this function
    pub fn pow_find_nonce_threaded(&self, pow_bits: u64) -> u64 {
        let mut seed = hex!("0123456789abcded").to_vec();
        seed.extend_from_slice(&self.digest);
        for byte in pow_bits.to_be_bytes().iter() {
            if *byte > 0 {
                seed.push(*byte);
                break;
            }
        }
        let mut seed_res = [0_u8; 32];
        let mut sha3 = Keccak::new_keccak256();
        sha3.update(&seed);
        sha3.finalize(&mut seed_res);

        let test_value = U256::from(2_u64).pow(256 - pow_bits).unwrap();
        let ret = (0..(u64::max_value() as usize))
            .into_par_iter()
            .find_any(|n| -> bool {
                let mut sha3 = Keccak::new_keccak256();
                let mut res = [0; 32];
                sha3.update(&seed_res);
                sha3.update(&(n.to_be_bytes()));
                sha3.finalize(&mut res);
                let final_int = U256::from_bytes_be(&res);
                if final_int.leading_zeros() == pow_bits as usize {
                    final_int < test_value
                } else {
                    false
                }
            });
        ret.unwrap() as u64
    }
}

pub fn pow_verify(n: u64, pow_bits: u64, proof: &Channel) -> bool {
    let mut seed = hex!("0123456789abcded").to_vec();
    seed.extend_from_slice(&proof.digest);
    for byte in pow_bits.to_be_bytes().iter() {
        if *byte > 0 {
            seed.push(*byte);
            break;
        }
    }
    let mut seed_res = [0_u8; 32];
    let mut sha3 = Keccak::new_keccak256();
    sha3.update(&seed);
    sha3.finalize(&mut seed_res);

    let test_value = U256::from(2_u64).pow(256 - pow_bits).unwrap();
    let mut sha3 = Keccak::new_keccak256();
    let mut res = [0; 32];
    sha3.update(&seed_res);
    sha3.update(&(n.to_be_bytes()));
    sha3.finalize(&mut res);
    let final_int = U256::from_bytes_be(&res);
    final_int < test_value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fibonacci::*;
    use crate::field::*;
    use crate::u256::U256;
    use crate::u256h;
    use hex_literal::*;

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
