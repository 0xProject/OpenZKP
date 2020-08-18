use crate::{Signature, GENERATOR_TABLE};
use tiny_keccak::{Hasher, Sha3};
use zkp_elliptic_curve::{base_mul, Affine, ScalarFieldElement};
use zkp_primefield::{Inv, Zero};
use zkp_u256::U256;

#[cfg(any(test, feature = "proptest"))]
use proptest_derive::Arbitrary;

#[derive(PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "std", derive(Debug))]
#[cfg_attr(test, derive(Arbitrary))]
#[cfg_attr(test, proptest(no_params))]
pub struct PrivateKey(ScalarFieldElement);

impl PrivateKey {
    pub fn as_scalar_field_element(&self) -> &ScalarFieldElement {
        &self.0
    }

    pub fn sign(&self, digest: &ScalarFieldElement) -> Signature {
        for nonce in 0..1000 {
            let k = self.hash(digest, nonce);
            if k.is_zero() {
                continue;
            }
            match Affine::from(&base_mul(&*GENERATOR_TABLE, &k)) {
                Affine::Zero => continue,
                Affine::Point { x, .. } => {
                    let r = ScalarFieldElement::from(x.to_uint());
                    if r.is_zero() {
                        continue;
                    }
                    let s = &r * &self.0 + digest;
                    match s.inv() {
                        None => continue,
                        Some(inverse) => return Signature::new(r, k * inverse),
                    }
                }
            }
        }
        panic!("Could not find k for ECDSA after 1000 tries.")
    }

    fn hash(&self, digest: &ScalarFieldElement, nonce: u64) -> ScalarFieldElement {
        let mut output = [0; 32];
        let mut sha3 = Sha3::v256();
        sha3.update(
            &[
                self.0.to_uint().to_bytes_be(),
                digest.to_uint().to_bytes_be(),
                U256::from(nonce).to_bytes_be(),
            ]
            .concat(),
        );
        sha3.finalize(&mut output);
        U256::from_bytes_be(&output).into()
    }
}

impl<T: Into<ScalarFieldElement>> From<T> for PrivateKey {
    fn from(i: T) -> Self {
        Self(i.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Signature;
    use zkp_macros_decl::u256h;

    #[test]
    fn test_sign() {
        let digest = ScalarFieldElement::from(u256h!(
            "01921ce52df68f0185ade7572776513304bdd4a07faf6cf28cefc65a86fc496c"
        ));
        let private_key = PrivateKey::from(u256h!(
            "03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc"
        ));
        let expected = Signature::new(
            ScalarFieldElement::from(u256h!(
                "006d1f96368ae3a73893790a957d86850d443e77c157682cc65f4943b8385bcb"
            )),
            ScalarFieldElement::from(u256h!(
                "05a48d5ab6ccea487a6d0c2e9bc5ea5e5c7857252f72937250ef3ad8b290b29f"
            )),
        );
        let result = private_key.sign(&digest);
        assert_eq!(result, expected);
    }
}
