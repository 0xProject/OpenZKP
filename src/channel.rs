use crate::{field::*, u256::U256, u256h};
use hex_literal::*;
use rayon::prelude::*;
use tiny_keccak::Keccak;

pub trait Readable<T> {
    fn read(&mut self) -> T;
}

pub trait Writable<T> {
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
            .find(|&nonce| Channel::pow_verify_with_seed(nonce, pow_bits, &seed))
            .expect("No valid nonce found")
    }

    // TODO - Make tests compatible with the proof of work values from this function
    pub fn pow_find_nonce_threaded(&self, pow_bits: u8) -> u64 {
        let seed = self.pow_seed(pow_bits);
        // NOTE: Rayon does not support open ended ranges, so we need to use a closed
        // one.
        (0..u64::max_value())
            .into_par_iter()
            .find_any(|&nonce| Channel::pow_verify_with_seed(nonce, pow_bits, &seed))
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

    pub fn pow_verify(&self, nonce: u64, pow_bits: u8) -> bool {
        let seed = self.pow_seed(pow_bits);
        Channel::pow_verify_with_seed(nonce, pow_bits, &seed)
    }

    fn pow_verify_with_seed(nonce: u64, pow_bits: u8, seed: &[u8; 32]) -> bool {
        // OPT: Inline Keccak256 and work directly on buffer using 'keccakf'
        let mut keccak = Keccak::new_keccak256();
        let mut digest = [0_u8; 32];
        keccak.update(seed);
        keccak.update(&(nonce.to_be_bytes()));
        keccak.finalize(&mut digest);
        // OPT: Check performance impact of conversion
        let work = U256::from_bytes_be(&digest).leading_zeros();
        work >= pow_bits as usize
    }
}

impl Readable<FieldElement> for Channel {
    fn read(&mut self) -> FieldElement {
        const MASK: U256 =
            u256h!("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
        loop {
            let number: U256 = self.read();
            let seed = number & MASK;
            if seed < MODULUS {
                // TODO: Avoid accessing FieldElement members directly
                break FieldElement(seed);
            }
        }
    }
}

impl Readable<U256> for Channel {
    fn read(&mut self) -> U256 {
        U256::from_bytes_be(&self.read())
    }
}

impl Readable<[u8; 32]> for Channel {
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

impl Writable<&[u8]> for Channel {
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
impl Writable<&[u8; 32]> for Channel {
    fn write(&mut self, data: &[u8; 32]) {
        self.write(&data[..]);
    }
}

impl Writable<u64> for Channel {
    fn write(&mut self, data: u64) {
        self.write(&data.to_be_bytes()[..]);
    }
}

// OPT - Remove allocation of vectors
impl Writable<&[FieldElement]> for Channel {
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

impl Writable<&FieldElement> for Channel {
    fn write(&mut self, data: &FieldElement) {
        // TODO: Avoid accessing FieldElement members directly
        self.write(&data.0.to_bytes_be()[..]);
    }
}

// Note -- This method of writing is distinct from the field element, and is
// used in the decommitment when groups are decommited from the rows
impl Writable<Vec<U256>> for Channel {
    fn write(&mut self, data: Vec<U256>) {
        for element in data {
            self.write(element)
        }
    }
}

impl Writable<U256> for Channel {
    fn write(&mut self, data: U256) {
        self.write(&data.to_bytes_be()[..]);
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

    // Note - This test depends on the specific ordering of the subtests because of
    // the nature of the channel
    #[test]
    fn test_channel_read() {
        let mut source = Channel::new(hex!("0123456789abcded").to_vec().as_slice());
        let rand_bytes: [u8; 32] = source.read();
        assert_eq!(
            rand_bytes,
            hex!("7d84f75ca3e9328b92123c1790834ee0084e02c09b379c6f95c5d2ae8739b9c8")
        );
        let rand_int: U256 = source.read();
        assert_eq!(
            rand_int,
            u256h!("4ed5f0fd8cffa8dec69beebab09ee881e7369d6d084b90208a079eedc67d2d45")
        );
        let rand_element: FieldElement = source.read();
        assert_eq!(
            rand_element,
            FieldElement(u256h!(
                "0389a47fe0e1e5f9c05d8dcb27b069b67b1c7ec61a5c0a3f54d81aea83d2c8f0"
            ))
        );
    }

    // Note - This test depends on the specific ordering of the subtests because of
    // the nature of the channel
    #[test]
    fn test_channel_write() {
        let mut source = Channel::new(hex!("0123456789abcded").to_vec().as_slice());
        let rand_bytes: [u8; 32] = source.read();
        source.write(&rand_bytes);
        assert_eq!(
            source.digest,
            hex!("3174a00d031bc8deff799e24a78ee347b303295a6cb61986a49873d9b6f13a0d")
        );
        source.write(11_028_357_238_u64);
        assert_eq!(
            source.digest,
            hex!("21571e2a323daa1e6f2adda87ce912608e1325492d868e8fe41626633d6acb93")
        );
        source.write(&FieldElement(u256h!(
            "0389a47fe0e1e5f9c05d8dcb27b069b67b1c7ec61a5c0a3f54d81aea83d2c8f0"
        )));
        assert_eq!(
            source.digest,
            hex!("34a12938f047c34da72b5949434950fa2b24220270fd26e6f64b6eb5e86c6626")
        );
        source.write(
            vec![
                FieldElement(u256h!(
                    "0389a47fe0e1e5f9c05d8dcb27b069b67b1c7ec61a5c0a3f54d81aea83d2c8f0"
                )),
                FieldElement(u256h!(
                    "129ab47fe0e1a5f9c05d8dcb27b069b67b1c7ec61a5c0a3f54d81aea83d2c8f0"
                )),
            ]
            .as_slice(),
        );
        assert_eq!(
            source.digest,
            hex!("a748ff89e2c4322afb061ef3321e207b3fe32c35f181de0809300995dd9b92fd")
        );
    }
}
