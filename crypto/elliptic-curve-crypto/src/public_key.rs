use crate::{PrivateKey, Signature, GENERATOR_TABLE};
#[cfg(feature = "parity_codec")]
use parity_scale_codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use zkp_elliptic_curve::{base_mul, double_base_mul, Affine, ScalarFieldElement};
use zkp_primefield::Zero;

#[derive(PartialEq, Eq, Clone, Default, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "parity_codec", derive(Encode, Decode))]
pub struct PublicKey(Affine);

impl PublicKey {
    pub fn as_affine(&self) -> &Affine {
        &self.0
    }

    pub fn verify(&self, digest: &ScalarFieldElement, signature: &Signature) -> bool {
        assert!(!signature.r().is_zero());
        assert!(!signature.w().is_zero());
        assert!(self.0.is_on_curve());

        let generator_factor = digest * signature.w();
        let pubkey_factor = signature.r() * signature.w();
        Affine::from(&double_base_mul(
            &*GENERATOR_TABLE,
            &generator_factor,
            &self.0,
            &pubkey_factor,
        ))
        .x()
        .map_or(false, |x| {
            &ScalarFieldElement::from(x.to_uint()) == signature.r()
        })
    }
}

impl From<&PrivateKey> for PublicKey {
    fn from(private_key: &PrivateKey) -> Self {
        Self(Affine::from(&base_mul(
            &*GENERATOR_TABLE,
            private_key.as_scalar_field_element(),
        )))
    }
}

impl From<Affine> for PublicKey {
    fn from(a: Affine) -> Self {
        Self(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zkp_macros_decl::{field_element, u256h};
    use zkp_primefield::FieldElement;
    use zkp_u256::U256;

    #[test]
    fn test_pubkey() {
        let private_key = PrivateKey::from(u256h!(
            "03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc"
        ));
        let expected = PublicKey::from(Affine::new(
            field_element!("077a3b314db07c45076d11f62b6f9e748a39790441823307743cf00d6597ea43"),
            field_element!("054d7beec5ec728223671c627557efc5c9a6508425dc6c900b7741bf60afec06"),
        ));
        let result = PublicKey::from(&private_key);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_verify() {
        let digest = ScalarFieldElement::from(u256h!(
            "01e542e2da71b3f5d7b4e9d329b4d30ac0b5d6f266ebef7364bf61c39aac35d0"
        ));
        let public_key = PublicKey::from(Affine::new(
            field_element!("077a3b314db07c45076d11f62b6f9e748a39790441823307743cf00d6597ea43"),
            field_element!("054d7beec5ec728223671c627557efc5c9a6508425dc6c900b7741bf60afec06"),
        ));
        let signature = Signature::new(
            ScalarFieldElement::from(u256h!(
                "01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca"
            )),
            ScalarFieldElement::from(u256h!(
                "07656a287e3be47c6e9a29482aecc10cd8b1ae4797b4b956a3573b425d1e66c9"
            )),
        );
        assert!(public_key.verify(&digest, &signature));
    }
}
