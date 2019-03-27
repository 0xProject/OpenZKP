use crate::field::FieldElement;
use crate::curve::{CurvePoint, ORDER};
use crate::pedersen_points::PEDERSEN_POINTS;
use num::{BigUint, integer::Integer, traits::identities::{Zero, One}};
use lazy_static::lazy_static;

lazy_static! {
    static ref GENERATOR: CurvePoint = CurvePoint::new(&[
            0xc943cfca, 0x3d723d8b, 0x0d1819e0, 0xdeacfd9b,
            0x5a40f0c7, 0x7beced41, 0x8599971b, 0x01ef15c1],&[
            0x36e8dc1f, 0x2873000c, 0x1abe43a3, 0xde53ecd1,
            0xdf46ec62, 0xb7be4801, 0x0aa49730, 0x00566806]);
}

pub fn private_to_public(private_key: BigUint) -> CurvePoint {
    GENERATOR.clone() * private_key
}

fn divmod(a: &BigUint, b: &BigUint) -> BigUint {
   (a * b.modpow(&(ORDER.clone() - BigUint::one() - BigUint::one()), &*ORDER)) % &*ORDER
}

pub fn sign(msg_hash: &BigUint, private_key: &BigUint) -> (BigUint, BigUint) {
    assert!(msg_hash.bits() <= 251);

    { // Todo Loop
        let k = BigUint::one();

        let r: BigUint = (GENERATOR.clone() * k.clone()).x.0;
        assert!(r > BigUint::zero());
        assert!(r.bits() <= 251); // TODO: Retry

        assert!(msg_hash + &r * private_key % &*ORDER != BigUint::zero());
        let w: BigUint = divmod(&k, &(msg_hash + &r * private_key));

        (r, w)
    }
}

pub fn verify(msg_hash: &BigUint, r: &BigUint, w: &BigUint, public_key: &CurvePoint) -> bool {
    assert!(r != &BigUint::zero());
    assert!(r.bits() <= 251);
    assert!(w != &BigUint::zero());
    assert!(w.bits() <= 251);
    assert!(public_key.on_curve());

    let A = GENERATOR.clone() * (msg_hash * w);
    let B = public_key.clone() * (r * w);
    let x = (A + B).x.0;
    &x == r
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    #[test]
    fn test_ecdsa(message_hash: FieldElement, private_key: FieldElement) -> bool {
        let public_key = private_to_public(private_key.0.clone());
        let (r, w) = sign(&message_hash.0, &private_key.0);
        verify(&message_hash.0, &r, &w, &public_key)
    }
}
