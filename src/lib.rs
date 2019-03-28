mod curve;
mod ecdsa;
mod field;
mod pedersen;
mod pedersen_points;
use num::BigUint;
use field::FieldElement;
use curve::CurvePoint;

fn from_bytes(bytes: &[u8;32]) -> BigUint {
    BigUint::from_bytes_be(bytes)
}

fn to_bytes(num: &BigUint) -> [u8;32] {
    // TODO: Zero padding
    let vec = num.to_bytes_be();
    let mut array = [0; 32];
    let bytes = &vec.as_slice()[..array.len()]; // panics if not enough data
    array.copy_from_slice(bytes); 
    array
}

pub fn hash(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let hash = pedersen::hash(&[from_bytes(a), from_bytes(b)]);
    to_bytes(&hash)
}

pub fn public_key(private_key: &[u8; 32]) -> ([u8;32], [u8;32]) {
    let p = ecdsa::private_to_public(&from_bytes(private_key));
    (p.x.to_bytes(), p.y.to_bytes())
}

pub fn sign(message_hash: &[u8;32], private_key: &[u8;32]) -> ([u8;32], [u8;32]) {
    let (r, w) = ecdsa::sign(&from_bytes(message_hash), &from_bytes(private_key));
    (to_bytes(&r), to_bytes(&w))
}

pub fn verify(
    message_hash: &[u8;32],
    signature: (&[u8;32], &[u8;32]),
    public_key: (&[u8;32], &[u8;32]),
) -> bool {
    ecdsa::verify(
        &from_bytes(message_hash),
        &from_bytes(signature.0),
        &from_bytes(signature.1),
        &CurvePoint{
            x: FieldElement::from(public_key.0),
            y: FieldElement::from(public_key.1),
        },
    )
}
