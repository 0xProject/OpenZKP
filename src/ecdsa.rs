use crate::curve::{CurvePoint, ORDER};
use crate::field::FieldElement;
use crate::u256::U256;
use crate::u256h;
use hex_literal::*;
use num::traits::{One, Zero};

pub const GENERATOR: CurvePoint = CurvePoint {
    x: FieldElement(u256h!(
        "01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca"
    )),
    y: FieldElement(u256h!(
        "005668060aa49730b7be4801df46ec62de53ecd11abe43a32873000c36e8dc1f"
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
    {
        // Todo Loop
        let k = U256::ONE;

        let r: U256 = (GENERATOR.clone() * k.clone()).x.0;
        assert!(r > U256::ZERO);
        assert!(r.bits() <= 251); // TODO: Retry

        let r = r % &ORDER;

        assert!(msg_hash + r.clone() * &private_key % &ORDER != U256::ZERO);
        let w: U256 = divmod(&k, &(msg_hash + &r * private_key.clone()));

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
    let x = (a + b).x.0;
    &x == r
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::FieldElement;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    #[test]
    fn test_ecdsa(message_hash: FieldElement, private_key: FieldElement) -> bool {
        let public_key = private_to_public(&private_key.0);
        let (r, w) = sign(&message_hash.0, &private_key.0);
        verify(&message_hash.0, &r, &w, &public_key)
    }
}
