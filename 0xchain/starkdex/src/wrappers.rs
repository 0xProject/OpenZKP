use crate::orders;
use elliptic_curve::Affine;
use elliptic_curve_crypto as ecc;
use std::prelude::v1::*;
use u256::U256;

fn from_bytes(bytes: &[u8; 32]) -> U256 {
    U256::from_bytes_be(bytes)
}

fn from_24_bytes(bytes: &[u8; 24]) -> U256 {
    let mut padded = [0_u8; 32];
    let mut padded_vec = vec![0, 0, 0, 0, 0, 0, 0, 0];
    padded_vec.extend_from_slice(bytes);
    padded.copy_from_slice(padded_vec.as_slice());
    U256::from_bytes_be(&padded)
}

fn to_bytes(num: &U256) -> [u8; 32] {
    num.to_bytes_be()
}

pub fn hash(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let hash = crate::hash(&[from_bytes(a), from_bytes(b)]);
    to_bytes(&hash)
}

pub fn public_key(private_key: &[u8; 32]) -> ([u8; 32], [u8; 32]) {
    let p = ecc::private_to_public(&from_bytes(private_key));
    match p {
        Affine::Zero => panic!(),
        Affine::Point { x, y } => (U256::from(x).to_bytes_be(), U256::from(y).to_bytes_be()),
    }
}

pub fn sign(message_hash: &[u8; 32], private_key: &[u8; 32]) -> ([u8; 32], [u8; 32]) {
    let (r, w) = ecc::sign(&from_bytes(message_hash), &from_bytes(private_key));
    (to_bytes(&r), to_bytes(&w))
}

pub fn verify(
    message_hash: &[u8; 32],
    signature: (&[u8; 32], &[u8; 32]),
    public_key: (&[u8; 32], &[u8; 32]),
) -> bool {
    ecc::verify(
        &from_bytes(message_hash),
        &from_bytes(signature.0),
        &from_bytes(signature.1),
        &Affine::Point {
            x: U256::from_bytes_be(public_key.0).into(),
            y: U256::from_bytes_be(public_key.1).into(),
        },
    )
}

pub type MakerMessage = orders::MakerMessage<[u8; 24]>;

pub fn maker_hash(message: &MakerMessage) -> [u8; 32] {
    let m = orders::MakerMessage {
        vault_a:  message.vault_a,
        vault_b:  message.vault_b,
        amount_a: message.amount_a,
        amount_b: message.amount_b,
        token_a:  from_24_bytes(&message.token_a),
        token_b:  from_24_bytes(&message.token_b),
        trade_id: message.trade_id,
    };
    let h = orders::hash_maker(&m);
    to_bytes(&h)
}

pub fn taker_hash(maker_hash: &[u8; 32], vault_a: u32, vault_b: u32) -> [u8; 32] {
    let h = orders::hash_taker(&from_bytes(maker_hash), vault_a, vault_b);
    to_bytes(&h)
}

pub fn maker_sign(message: &MakerMessage, private_key: &[u8; 32]) -> ([u8; 32], [u8; 32]) {
    sign(&maker_hash(message), private_key)
}

pub fn taker_sign(
    message: &MakerMessage,
    vault_a: u32,
    vault_b: u32,
    private_key: &[u8; 32],
) -> ([u8; 32], [u8; 32]) {
    sign(
        &taker_hash(&maker_hash(message), vault_a, vault_b),
        private_key,
    )
}

pub fn maker_verify(
    message: &MakerMessage,
    signature: (&[u8; 32], &[u8; 32]),
    public_key: (&[u8; 32], &[u8; 32]),
) -> bool {
    verify(&maker_hash(message), signature, public_key)
}

pub fn taker_verify(
    message: &MakerMessage,
    vault_a: u32,
    vault_b: u32,
    signature: (&[u8; 32], &[u8; 32]),
    public_key: (&[u8; 32], &[u8; 32]),
) -> bool {
    verify(
        &taker_hash(&maker_hash(message), vault_a, vault_b),
        signature,
        public_key,
    )
}
