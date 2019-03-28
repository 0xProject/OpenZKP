//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use starkcrypto::pedersen::hash;
use starkcrypto::num::{BigUint};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn test_hash_1() {
    let elements = [
        BigUint::from_slice(&[0x405371cb, 0x28feb561, 0xa1393627, 0x9c53068d, 0x1a575610, 0x5caf6453, 0x35c87824, 0x03d937c0]),
        BigUint::from_slice(&[0x9cd8b31a, 0xbbc6aeff, 0x5695e02f, 0x791bf627, 0x880906c2, 0xe1e4bbe2, 0x0250e382, 0x0208a0a1]),
    ];
    let result = BigUint::from_slice(&[0x53f3111b, 0x1e311a7c, 0x139a84d0, 0xa8af1bbb, 0x57037e4a, 0xfb867eaf, 0x76790645, 0x02d895bd]);
    assert_eq!(hash(&elements), result);
}
