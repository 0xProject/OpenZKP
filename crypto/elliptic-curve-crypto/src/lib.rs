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

use lazy_static::*;
use std::prelude::v1::*;
use tiny_keccak::sha3_256;
use zkp_elliptic_curve::{
    base_mul, double_base_mul, window_table_affine, Affine, GENERATOR, ORDER,
};
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

// TODO (SECURITY): Use side-channel-resistant math
pub fn private_to_public(private_key: &U256) -> Affine {
    Affine::from(&base_mul(&*GENERATOR_TABLE, private_key % ORDER))
}

fn divmod(a: &U256, b: &U256) -> Option<U256> {
    b.invmod(&ORDER).map(|bi| a.mulmod(&bi, &ORDER))
}

// TODO (SECURITY): The signatures are malleable in s -> MODULUS - s.
pub fn sign(msg_hash: &U256, private_key: &U256) -> (U256, U256) {
    assert!(msg_hash.bits() <= 251);
    for i in 0..1000 {
        let k = U256::from_bytes_be(&sha3_256(
            &[
                private_key.to_bytes_be(),
                msg_hash.to_bytes_be(),
                U256::from(i).to_bytes_be(),
            ]
            .concat(),
        )) >> 4;
        if k == U256::ZERO || k.bits() > 251 {
            continue;
        }
        match Affine::from(&base_mul(&*GENERATOR_TABLE, k.clone())) {
            Affine::Zero => continue,
            Affine::Point { x, .. } => {
                let r = U256::from(x);
                if r == U256::ZERO || r.bits() > 251 {
                    continue;
                }
                match divmod(&k, &(msg_hash + r.mulmod(private_key, &ORDER))) {
                    None => continue,
                    Some(w) => return (r, w),
                }
            }
        }
    }
    panic!("Could not find k for ECDSA after 1000 tries.")
}

// TODO (SECURITY): The signatures are malleable in s -> MODULUS - s.
pub fn verify(msg_hash: &U256, r: &U256, w: &U256, public_key: &Affine) -> bool {
    assert!(r != &U256::ZERO);
    assert!(r.bits() <= 251);
    assert!(w != &U256::ZERO);
    assert!(w.bits() <= 251);
    assert!(public_key.on_curve());

    match Affine::from(&double_base_mul(
        &*GENERATOR_TABLE,
        msg_hash.mulmod(&w, &ORDER),
        &public_key,
        r.mulmod(&w, &ORDER),
    )) {
        Affine::Zero => false,
        Affine::Point { x, .. } => U256::from(x) == *r,
    }
}

// Quickcheck needs pass by value
#[allow(clippy::needless_pass_by_value)]
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    use zkp_macros_decl::u256h;
    use zkp_primefield::FieldElement;

    #[test]
    fn test_pubkey() {
        let private_key =
            u256h!("03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let expected = Affine::Point {
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
            u256h!("01921ce52df68f0185ade7572776513304bdd4a07faf6cf28cefc65a86fc496c");
        let private_key =
            u256h!("03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let expected = (
            u256h!("049ae96821351a2bbc91d3d1e84bc825bea2cb645a7184446dd92f4f1bc4f5b8"),
            u256h!("03cdabfdd233bf8146621fd2e938ef5b326c485eac8fbe59aa9ae39adfaf4cbc"),
        );
        let result = sign(&message_hash, &private_key);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_verify() {
        let message_hash =
            u256h!("01e542e2da71b3f5d7b4e9d329b4d30ac0b5d6f266ebef7364bf61c39aac35d0");
        let public_key = Affine::Point {
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
    fn test_ecdsa(mut message_hash: U256, private_key: U256) -> bool {
        message_hash >>= 5; // Need message_hash <= 2**251
        let public_key = private_to_public(&private_key);
        let (r, w) = sign(&message_hash, &private_key);
        verify(&message_hash, &r, &w, &public_key)
    }
}
