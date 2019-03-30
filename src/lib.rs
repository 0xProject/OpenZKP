#![warn(clippy::all)]
#![feature(const_fn)]
pub mod curve;
pub mod ecdsa;
pub mod field;
pub mod orders;
pub mod pedersen;
mod pedersen_points;
pub mod u256;
mod utils;
use curve::CurvePoint;
use field::FieldElement;
pub use num;
use num::BigUint;

fn from_bytes(bytes: &[u8; 32]) -> BigUint {
    BigUint::from_bytes_be(bytes)
}

fn to_bytes(num: &BigUint) -> [u8; 32] {
    assert!(num.bits() <= 256);
    let vec = num.to_bytes_be();
    let mut array = [0; 32];
    let padding: usize = 32 - vec.len();
    array[padding..].copy_from_slice(vec.as_slice());
    array
}

pub fn hash(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let hash = pedersen::hash(&[from_bytes(a), from_bytes(b)]);
    to_bytes(&hash)
}

pub fn public_key(private_key: &[u8; 32]) -> ([u8; 32], [u8; 32]) {
    let p = ecdsa::private_to_public(&from_bytes(private_key));
    (p.x.to_bytes(), p.y.to_bytes())
}

pub fn sign(message_hash: &[u8; 32], private_key: &[u8; 32]) -> ([u8; 32], [u8; 32]) {
    let (r, w) = ecdsa::sign(&from_bytes(message_hash), &from_bytes(private_key));
    (to_bytes(&r), to_bytes(&w))
}

pub fn verify(
    message_hash: &[u8; 32],
    signature: (&[u8; 32], &[u8; 32]),
    public_key: (&[u8; 32], &[u8; 32]),
) -> bool {
    ecdsa::verify(
        &from_bytes(message_hash),
        &from_bytes(signature.0),
        &from_bytes(signature.1),
        &CurvePoint {
            x: FieldElement::from(public_key.0),
            y: FieldElement::from(public_key.1),
        },
    )
}

pub type MakerMessage = orders::MakerMessage<[u8; 32]>;

pub fn maker_hash(message: &MakerMessage) -> [u8; 32] {
    let m = orders::MakerMessage {
        vault_a: message.vault_a,
        vault_b: message.vault_b,
        amount_a: message.amount_a,
        amount_b: message.amount_b,
        token_a: from_bytes(&message.token_a),
        token_b: from_bytes(&message.token_b),
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
