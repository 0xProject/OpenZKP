// TODO: Naming?
#![allow(clippy::module_name_repetitions)]
use crate::proof_of_work;
use std::{convert::TryInto, prelude::v1::*};
use tiny_keccak::{Hasher, Keccak};
use zkp_hash::Hash;
use zkp_macros_decl::u256h;
use zkp_primefield::FieldElement;
use zkp_u256::U256;

pub(crate) trait RandomGenerator<T> {
    fn get_random(&mut self) -> T;
}

pub(crate) trait Writable<T> {
    fn write(&mut self, data: T);
}

pub(crate) trait Replayable<T> {
    fn replay(&mut self) -> T;

    fn replay_many(&mut self, count: usize) -> Vec<T> {
        (0..count).map(|_| self.replay()).collect()
    }
}

// TODO: Limit to crate
#[derive(PartialEq, Eq, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug))]
pub(crate) struct PublicCoin {
    pub(crate) digest: [u8; 32],
    counter:           u64,
}

#[derive(PartialEq, Eq, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug))]
pub(crate) struct ProverChannel {
    pub(crate) coin:  PublicCoin,
    pub(crate) proof: Vec<u8>,
}

#[derive(PartialEq, Eq, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug))]
pub(crate) struct VerifierChannel {
    pub(crate) coin:  PublicCoin,
    pub(crate) proof: Vec<u8>,
    proof_index:      usize,
}

impl PublicCoin {
    pub(crate) fn seed(&mut self, seed: &[u8]) {
        let mut keccak = Keccak::v256();
        keccak.update(seed);
        keccak.finalize(&mut self.digest);
        self.counter = 0;
    }
}

impl From<Vec<u8>> for ProverChannel {
    fn from(proof_data: Vec<u8>) -> Self {
        Self {
            coin:  PublicCoin::default(),
            proof: proof_data,
        }
    }
}

#[cfg(feature = "prover")]
impl ProverChannel {
    pub(crate) fn initialize(&mut self, seed: &[u8]) {
        self.coin.seed(seed);
    }
}

impl VerifierChannel {
    pub(crate) fn new(proof: Vec<u8>) -> Self {
        Self {
            coin: PublicCoin::default(),
            proof,
            proof_index: 0,
        }
    }

    pub(crate) fn initialize(&mut self, seed: &[u8]) {
        self.coin.seed(seed);
    }

    pub(crate) fn at_end(self) -> bool {
        self.proof_index == self.proof.len()
    }

    pub(crate) fn get_coefficients(&mut self, n: usize) -> Vec<FieldElement> {
        (0..n).map(|_| self.get_random()).collect()
    }

    // This differs from Replayable::<FieldElement>::replay_many in that it only
    // updates the public coin once, with the contents of the entire layer, instead
    // of onces for each FieldElement in the layer.
    pub(crate) fn replay_fri_layer(&mut self, size: usize) -> Vec<FieldElement> {
        let start_index = self.proof_index;
        self.proof_index += 32 * size;
        let layer_contents = &self.proof[start_index..self.proof_index];

        self.coin.write(layer_contents);

        layer_contents
            .chunks_exact(32)
            .map(|bytes| {
                FieldElement::from_montgomery(U256::from_bytes_be(bytes.try_into().unwrap()))
            })
            .collect()
    }

    fn read_32_bytes(&mut self) -> [u8; 32] {
        let mut holder = [0_u8; 32];
        let from = self.proof_index;
        let to = from + 32;
        self.proof_index = to;
        // OPT: Use arrayref crate or similar to avoid copy
        holder.copy_from_slice(&self.proof[from..to]);
        self.coin.write(&holder[..]);
        holder
    }
}

impl RandomGenerator<proof_of_work::ChallengeSeed> for PublicCoin {
    fn get_random(&mut self) -> proof_of_work::ChallengeSeed {
        self.counter += 1;
        // FIX: Use get_random::<[u8;32]>();
        proof_of_work::ChallengeSeed::from_bytes(self.digest)
    }
}

impl Writable<proof_of_work::Response> for ProverChannel {
    fn write(&mut self, data: proof_of_work::Response) {
        self.write(&data.nonce().to_be_bytes()[..]);
    }
}

impl Replayable<proof_of_work::Response> for VerifierChannel {
    fn replay(&mut self) -> proof_of_work::Response {
        let mut holder = [0_u8; 8];
        let from = self.proof_index;
        let to = from + 8;
        self.proof_index = to;
        holder.copy_from_slice(&self.proof[from..to]);
        self.coin.write(&holder[..]);
        let nonce = u64::from_be_bytes(holder);
        proof_of_work::Response::from_nonce(nonce)
    }
}

impl RandomGenerator<FieldElement> for PublicCoin {
    fn get_random(&mut self) -> FieldElement {
        const MASK: U256 =
            u256h!("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
        loop {
            let number: U256 = self.get_random();
            let seed = number & MASK;
            if seed < FieldElement::MODULUS {
                break FieldElement::from_montgomery(seed);
            }
        }
    }
}

impl RandomGenerator<U256> for PublicCoin {
    fn get_random(&mut self) -> U256 {
        U256::from_bytes_be(&self.get_random())
    }
}

impl RandomGenerator<[u8; 32]> for PublicCoin {
    fn get_random(&mut self) -> [u8; 32] {
        let mut result = [0; 32];
        let mut keccak = Keccak::v256();
        keccak.update(&self.digest);
        keccak.update(&[0_u8; 24]);
        keccak.update(&self.counter.to_be_bytes());
        keccak.finalize(&mut result);
        self.counter += 1;
        result
    }
}

impl<T> RandomGenerator<T> for ProverChannel
where
    PublicCoin: RandomGenerator<T>,
{
    fn get_random(&mut self) -> T {
        self.coin.get_random()
    }
}

impl<T> RandomGenerator<T> for VerifierChannel
where
    PublicCoin: RandomGenerator<T>,
{
    fn get_random(&mut self) -> T {
        self.coin.get_random()
    }
}

impl Writable<&[u8]> for PublicCoin {
    fn write(&mut self, data: &[u8]) {
        let mut result: [u8; 32] = [0; 32];
        let mut keccak = Keccak::v256();
        keccak.update(&self.digest);
        keccak.update(data);
        keccak.finalize(&mut result);
        // FIX: Hash counter into digest.
        self.digest = result;
        self.counter = 0;
    }
}

// Note - that this default implementation allows writing a sequence of &[u8] to
// the proof with the same encoding for the writing and the non writing. However
// by writing directly to the coin, other writes for the channel could separate
// encoding from random perturbation.
impl Writable<&[u8]> for ProverChannel {
    fn write(&mut self, data: &[u8]) {
        self.proof.extend_from_slice(data);
        self.coin.write(data);
    }
}

impl Writable<&Hash> for ProverChannel {
    fn write(&mut self, data: &Hash) {
        self.write(data.as_bytes());
    }
}

impl Writable<&zkp_merkle_tree::Commitment> for ProverChannel {
    fn write(&mut self, data: &zkp_merkle_tree::Commitment) {
        self.write(data.hash())
    }
}

impl Writable<&zkp_merkle_tree::Proof> for ProverChannel {
    fn write(&mut self, data: &zkp_merkle_tree::Proof) {
        for hash in data.hashes() {
            self.write(hash)
        }
    }
}

// OPT - Remove allocation of vectors
impl Writable<&[FieldElement]> for ProverChannel {
    fn write(&mut self, data: &[FieldElement]) {
        let mut container = Vec::with_capacity(32 * data.len());
        for element in data {
            for byte in &element.as_montgomery().to_bytes_be() {
                container.push(*byte);
            }
        }
        self.write(container.as_slice());
    }
}

impl Writable<&FieldElement> for ProverChannel {
    fn write(&mut self, data: &FieldElement) {
        self.write(&data.as_montgomery().to_bytes_be()[..]);
    }
}

// Note -- This method of writing is distinct from the field element, and is
// used in the decommitment when groups are decommited from the rows
impl Writable<Vec<U256>> for ProverChannel {
    fn write(&mut self, data: Vec<U256>) {
        for element in data {
            self.write(element)
        }
    }
}

impl Writable<U256> for ProverChannel {
    fn write(&mut self, data: U256) {
        self.write(&data.to_bytes_be()[..]);
    }
}

impl Replayable<Hash> for VerifierChannel {
    fn replay(&mut self) -> Hash {
        let hash: [u8; 32] = self.read_32_bytes();
        Hash::new(hash)
    }
}

impl Replayable<U256> for VerifierChannel {
    fn replay(&mut self) -> U256 {
        let big_endian_bytes: [u8; 32] = self.read_32_bytes();
        U256::from_bytes_be(&big_endian_bytes)
    }
}

impl Replayable<FieldElement> for VerifierChannel {
    fn replay(&mut self) -> FieldElement {
        let montgomery_modulus: U256 = self.replay();
        FieldElement::from_montgomery(montgomery_modulus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zkp_macros_decl::{hex, u256h};

    // Note - This test depends on the specific ordering of the subtests because of
    // the nature of the channel
    #[test]
    fn test_channel_get_random() {
        let mut source = ProverChannel::default();
        source.initialize(hex!("0123456789abcded").to_vec().as_slice());
        let rand_bytes: [u8; 32] = source.get_random();
        assert_eq!(
            rand_bytes,
            hex!("7d84f75ca3e9328b92123c1790834ee0084e02c09b379c6f95c5d2ae8739b9c8")
        );
        let rand_int: U256 = source.get_random();
        assert_eq!(
            rand_int,
            u256h!("4ed5f0fd8cffa8dec69beebab09ee881e7369d6d084b90208a079eedc67d2d45")
        );
        let rand_element: FieldElement = source.get_random();
        assert_eq!(
            rand_element,
            FieldElement::from_montgomery(u256h!(
                "0389a47fe0e1e5f9c05d8dcb27b069b67b1c7ec61a5c0a3f54d81aea83d2c8f0"
            ))
        );
    }

    // Note - This test depends on the specific ordering of the subtests because of
    // the nature of the channel
    #[test]
    fn test_channel_write() {
        let mut source = ProverChannel::default();
        source.initialize(&hex!("0123456789abcded"));
        let rand_bytes: [u8; 32] = source.get_random();
        source.write(&rand_bytes[..]);
        assert_eq!(
            source.coin.digest,
            hex!("3174a00d031bc8deff799e24a78ee347b303295a6cb61986a49873d9b6f13a0d")
        );
        source.write(proof_of_work::Response::from_nonce(11_028_357_238_u64));
        assert_eq!(
            source.coin.digest,
            hex!("21571e2a323daa1e6f2adda87ce912608e1325492d868e8fe41626633d6acb93")
        );
        source.write(&FieldElement::from_montgomery(u256h!(
            "0389a47fe0e1e5f9c05d8dcb27b069b67b1c7ec61a5c0a3f54d81aea83d2c8f0"
        )));
        assert_eq!(
            source.coin.digest,
            hex!("34a12938f047c34da72b5949434950fa2b24220270fd26e6f64b6eb5e86c6626")
        );
        source.write(
            vec![
                FieldElement::from_montgomery(u256h!(
                    "0389a47fe0e1e5f9c05d8dcb27b069b67b1c7ec61a5c0a3f54d81aea83d2c8f0"
                )),
                FieldElement::from_montgomery(u256h!(
                    "029ab47fe0e1a5f9c05d8dcb27b069b67b1c7ec61a5c0a3f54d81aea83d2c8f0"
                )),
            ]
            .as_slice(),
        );
        assert_eq!(
            source.coin.digest,
            hex!("586b2c12cd444cfe29932fcb167fc0be2e575a8d68e4a41d35de8602b0aea929")
        );
    }

    #[test]
    fn verifier_channel_test() {
        let mut source = ProverChannel::default();
        source.initialize(&hex!("0123456789abcded"));
        let rand_bytes: [u8; 32] = source.get_random();
        source.write(&rand_bytes[..]);
        source.write(proof_of_work::Response::from_nonce(11_028_357_238_u64));
        let written_field_element = FieldElement::from_montgomery(u256h!(
            "0389a47fe0e1e5f9c05d8dcb27b069b67b1c7ec61a5c0a3f54d81aea83d2c8f0"
        ));
        source.write(&written_field_element);
        let written_field_element_vec = vec![
            FieldElement::from_montgomery(u256h!(
                "0389a47fe0e1e5f9c05d8dcb27b069b67b1c7ec61a5c0a3f54d81aea83d2c8f0"
            )),
            FieldElement::from_montgomery(u256h!(
                "029ab47fe0e1a5f9c05d8dcb27b069b67b1c7ec61a5c0a3f54d81aea83d2c8f0"
            )),
        ];
        source.write(written_field_element_vec.as_slice());

        let written_big_int_vec = vec![
            u256h!("0389a47fe0e1e5f9c05d8dcb27b069b67b1c7ec61a5c0a3f54d81aea83d2c8f0"),
            u256h!("129ab47fe0e1a5f9c05d8dcb27b069b67b1c7ec61a5c0a3f54d81aea83d2c8f0"),
        ];
        source.write(written_big_int_vec.clone());

        let mut verifier = VerifierChannel::new(source.proof.clone());
        verifier.initialize(&hex!("0123456789abcded"));
        let bytes_test: [u8; 32] = verifier.read_32_bytes();
        assert_eq!(bytes_test, rand_bytes);
        assert_eq!(
            verifier.coin.digest,
            hex!("3174a00d031bc8deff799e24a78ee347b303295a6cb61986a49873d9b6f13a0d")
        );
        let pow_response_test: proof_of_work::Response = verifier.replay();
        assert_eq!(pow_response_test.nonce(), 11_028_357_238_u64);
        assert_eq!(
            verifier.coin.digest,
            hex!("21571e2a323daa1e6f2adda87ce912608e1325492d868e8fe41626633d6acb93")
        );
        let field_element_test: FieldElement = verifier.replay();
        assert_eq!(field_element_test, written_field_element);
        assert_eq!(
            verifier.coin.digest,
            hex!("34a12938f047c34da72b5949434950fa2b24220270fd26e6f64b6eb5e86c6626")
        );
        let field_element_vec_test: Vec<FieldElement> = verifier.replay_fri_layer(2);
        assert_eq!(field_element_vec_test, written_field_element_vec);
        assert_eq!(
            verifier.coin.digest,
            hex!("586b2c12cd444cfe29932fcb167fc0be2e575a8d68e4a41d35de8602b0aea929")
        );
        let bit_int_vec_test: Vec<U256> = verifier.replay_many(2);
        assert_eq!(bit_int_vec_test, written_big_int_vec);
        assert_eq!(verifier.coin.digest, source.coin.digest);
    }

    #[test]
    fn test_challenge_seed_from_channel() {
        let mut rand_source = ProverChannel::default();
        rand_source.initialize(&hex!("0123456789abcded"));
        // Verify that reading challenges does not depend on public coin counter.
        // FIX: Make it depend on public coin counter.
        let seed1: proof_of_work::ChallengeSeed = rand_source.get_random();
        let seed2: proof_of_work::ChallengeSeed = rand_source.get_random();
        assert_eq!(seed1, seed2);
    }
}
