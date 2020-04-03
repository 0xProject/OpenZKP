// This sequence needs to be repeated in each project as a workaround.
//       See https://github.com/rust-lang/cargo/issues/5034
// For clippy lints see: https://rust-lang.github.io/rust-clippy/master
// For rustc lints see: https://doc.rust-lang.org/rustc/lints/index.html
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(
    // Enable sets of warnings
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    rust_2018_idioms,
    future_incompatible,
    unused,

    // Additional unused warnings (not included in `unused`)
    unused_lifetimes,
    unused_qualifications,
    unused_results,

    // Additional misc. warnings
    anonymous_parameters,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    // missing_docs,
    missing_doc_code_examples,
    private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    variant_size_differences
)]
#![cfg_attr(feature = "std", warn(missing_debug_implementations,))]
// rand_xoshiro v0.4.0 is required for a zkp-stark example and v0.3.1 for criterion
#![allow(clippy::multiple_crate_versions)]
// TODO: Toggle based on stable/nightly
// #![allow(clippy::missing_errors_doc)]
// TODO: Add `must_use` attributes
#![allow(clippy::must_use_candidate)]

use lazy_static::*;
use std::prelude::v1::*;
use tiny_keccak::{Hasher, Sha3};
use zkp_elliptic_curve::{
    base_mul, double_base_mul, window_table_affine, Affine, ScalarFieldElement, GENERATOR,
};
use zkp_primefield::*;
use zkp_u256::U256;

#[cfg(not(feature = "std"))]
extern crate no_std_compat as std;

lazy_static! {
    static ref GENERATOR_TABLE: [Affine; 32] = {
        let mut naf = <[Affine; 32]>::default();
        window_table_affine(&GENERATOR, &mut naf);
        naf
    };
}

// TODO (SECURITY): The signatures are malleable in w -> -w.
#[derive(PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Signature {
    r: ScalarFieldElement,
    w: ScalarFieldElement,
}

// TODO (SECURITY): Use side-channel-resistant math
pub fn private_to_public(private_key: &ScalarFieldElement) -> Affine {
    Affine::from(&base_mul(&*GENERATOR_TABLE, private_key))
}

pub fn sign(digest: &ScalarFieldElement, private_key: &ScalarFieldElement) -> Signature {
    for nonce in 0..1000 {
        let k = get_k(private_key, digest, nonce);
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
                let s = &r * private_key + digest;
                match s.inv() {
                    None => continue,
                    Some(inverse) => return Signature { r, w: k * inverse },
                }
            }
        }
    }
    panic!("Could not find k for ECDSA after 1000 tries.")
}

fn get_k(
    private_key: &ScalarFieldElement,
    digest: &ScalarFieldElement,
    nonce: u64,
) -> ScalarFieldElement {
    let mut output = [0; 32];
    let mut sha3 = Sha3::v256();
    sha3.update(
        &[
            private_key.to_uint().to_bytes_be(),
            digest.to_uint().to_bytes_be(),
            U256::from(nonce).to_bytes_be(),
        ]
        .concat(),
    );
    sha3.finalize(&mut output);
    U256::from_bytes_be(&output).into()
}

pub fn verify(digest: &ScalarFieldElement, signature: &Signature, public_key: &Affine) -> bool {
    assert!(!signature.r.is_zero());
    assert!(!signature.w.is_zero());
    assert!(public_key.is_on_curve());

    let generator_factor = digest * &signature.w;
    let pubkey_factor = &signature.r * &signature.w;
    match Affine::from(&double_base_mul(
        &*GENERATOR_TABLE,
        &generator_factor,
        &public_key,
        &pubkey_factor,
    )) {
        Affine::Zero => false,
        Affine::Point { x, .. } => ScalarFieldElement::from(x.to_uint()) == signature.r,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use zkp_macros_decl::{field_element, u256h};
    use zkp_primefield::FieldElement;

    #[test]
    fn test_pubkey() {
        let private_key = ScalarFieldElement::from(u256h!(
            "03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc"
        ));
        let expected = Affine::new(
            field_element!("077a3b314db07c45076d11f62b6f9e748a39790441823307743cf00d6597ea43"),
            field_element!("054d7beec5ec728223671c627557efc5c9a6508425dc6c900b7741bf60afec06"),
        );
        let result = private_to_public(&private_key);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sign() {
        let digest = ScalarFieldElement::from(u256h!(
            "01921ce52df68f0185ade7572776513304bdd4a07faf6cf28cefc65a86fc496c"
        ));
        let private_key = ScalarFieldElement::from(u256h!(
            "03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc"
        ));
        let expected = Signature {
            r: ScalarFieldElement::from(u256h!(
                "006d1f96368ae3a73893790a957d86850d443e77c157682cc65f4943b8385bcb"
            )),
            w: ScalarFieldElement::from(u256h!(
                "05a48d5ab6ccea487a6d0c2e9bc5ea5e5c7857252f72937250ef3ad8b290b29f"
            )),
        };
        let result = sign(&digest, &private_key);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_verify() {
        let digest = ScalarFieldElement::from(u256h!(
            "01e542e2da71b3f5d7b4e9d329b4d30ac0b5d6f266ebef7364bf61c39aac35d0"
        ));
        let public_key = Affine::Point {
            x: field_element!("077a3b314db07c45076d11f62b6f9e748a39790441823307743cf00d6597ea43"),
            y: field_element!("054d7beec5ec728223671c627557efc5c9a6508425dc6c900b7741bf60afec06"),
        };
        let r = ScalarFieldElement::from(u256h!(
            "01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca"
        ));
        let w = ScalarFieldElement::from(u256h!(
            "07656a287e3be47c6e9a29482aecc10cd8b1ae4797b4b956a3573b425d1e66c9"
        ));
        let signature = Signature { r, w };
        assert!(verify(&digest, &signature, &public_key));
    }

    proptest!(
        #[test]
        fn test_ecdsa(digest: ScalarFieldElement, private_key: ScalarFieldElement) {
            let public_key = private_to_public(&private_key);
            let signature = sign(&digest, &private_key);
            prop_assert!(verify(&digest, &signature, &public_key));
        }
    );
}
