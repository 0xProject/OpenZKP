use crate::curve::{CurvePoint, ORDER};
use crate::field::FieldElement;
use crate::u256::U256;
use crate::u256h;
use hex_literal::*;
use tiny_keccak::sha3_256;

// x = 0x01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca
// y = 0x005668060aa49730b7be4801df46ec62de53ecd11abe43a32873000c36e8dc1f
pub const GENERATOR: CurvePoint = CurvePoint {
    x: FieldElement::from_montgomery(u256h!(
        "033840300bf6cec10429bf5184041c7b51a9bf65d4403deac9019623cf0273dd"
    )),
    y: FieldElement::from_montgomery(u256h!(
        "05a0e71610f55329fbd89a97cf4b33ad0939e3442869bbe7569d0da34235308a"
    )),
};

pub fn private_to_public(private_key: &U256) -> CurvePoint {
    GENERATOR.clone() * (private_key.clone() % &ORDER)
}

fn divmod(a: &U256, b: &U256) -> U256 {
    a.mulmod(&b.invmod(&ORDER).unwrap(), &ORDER)
}

pub fn sign(msg_hash: &U256, private_key: &U256) -> (U256, U256) {
    assert!(msg_hash.bits() <= 251);
    let k = U256::from_bytes_be(sha3_256(
        &[private_key.to_bytes_be(), msg_hash.to_bytes_be()].concat(),
    ));
    {
        // Todo Loop over k
        let r: U256 = (GENERATOR.clone() * k.clone()).x.into();
        assert!(r > U256::ZERO);
        assert!(r.bits() <= 251); // TODO: Retry

        debug_assert!(msg_hash + r.mulmod(&private_key, &ORDER) != U256::ZERO);
        let w: U256 = divmod(&k, &(msg_hash + r.mulmod(private_key, &ORDER)));
        (r, w)
    }
}

pub fn verify(msg_hash: &U256, r: &U256, w: &U256, public_key: &CurvePoint) -> bool {
    assert!(r != &U256::ZERO);
    assert!(r.bits() <= 251);
    assert!(w != &U256::ZERO);
    assert!(w.bits() <= 251);
    assert!(public_key.on_curve());

    let a = GENERATOR.clone() * msg_hash.mulmod(&w, &ORDER);
    let b = public_key.clone() * r.mulmod(&w, &ORDER);
    let x: U256 = (a + b).x.into();
    &x == r
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::FieldElement;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_pubkey() {
        let private_key =
            u256h!("03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let expected = CurvePoint {
            x: FieldElement::from(u256h!(
                "077a3b314db07c45076d11f62b6f9e748a39790441823307743cf00d6597ea43"
            )),
            y: FieldElement::from(u256h!(
                "054d7beec5ec728223671c627557efc5c9a6508425dc6c900b7741bf60afec06"
            )),
        };
        let result = private_to_public(&private_key);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sign() {
        let message_hash =
            u256h!("01e542e2da71b3f5d7b4e9d329b4d30ac0b5d6f266ebef7364bf61c39aac35d0");
        let private_key =
            u256h!("03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let expected = (
            u256h!("0494e0c7d60c4bba6ed7d18abccaeb9e23ac4d49b4dd5132aa240b4909f4fed8"),
            u256h!("066778cb0f32c51a2fff78cde3ae697fb75481fd06e0e0451e9f25b5ac4a3ec8"),
        );
        let result = sign(&message_hash, &private_key);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_verify() {
        let message_hash =
            u256h!("01e542e2da71b3f5d7b4e9d329b4d30ac0b5d6f266ebef7364bf61c39aac35d0");
        let public_key = CurvePoint {
            x: FieldElement::from(u256h!(
                "077a3b314db07c45076d11f62b6f9e748a39790441823307743cf00d6597ea43"
            )),
            y: FieldElement::from(u256h!(
                "054d7beec5ec728223671c627557efc5c9a6508425dc6c900b7741bf60afec06"
            )),
        };
        let r = u256h!("01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca");
        let w = u256h!("07656a287e3be47c6e9a29482aecc10cd8b1ae4797b4b956a3573b425d1e66c9");
        assert!(verify(&message_hash, &r, &w, &public_key));
    }

    #[quickcheck]
    #[test]
    fn test_ecdsa(mut message_hash: U256, private_key: U256) -> bool {
        message_hash >>= 5; // Need message_hash <= 2**251
        let public_key = private_to_public(&private_key);
        let (r, w) = sign(&message_hash, &private_key);
        verify(&message_hash, &r, &w, &public_key)
    }
}
