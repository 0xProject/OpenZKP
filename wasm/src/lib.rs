mod utils;

use cfg_if::cfg_if;
use starkcrypto;
use starkcrypto::num::BigUint;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn nop(a: &str, b: &str) -> String {
    let elements = [
        BigUint::parse_bytes(a.as_bytes(), 16).unwrap(),
        BigUint::parse_bytes(b.as_bytes(), 16).unwrap(),
    ];
    let h = elements[1].clone();
    h.to_str_radix(16)
}

#[wasm_bindgen]
pub fn pedersen_hash(a: &str, b: &str) -> String {
    let elements = [
        BigUint::parse_bytes(a.as_bytes(), 16).unwrap(),
        BigUint::parse_bytes(b.as_bytes(), 16).unwrap(),
    ];
    let h = starkcrypto::pedersen::hash(&elements);
    h.to_str_radix(16)
}

#[wasm_bindgen]
pub fn sign(message_hash: &str, private_key: &str) -> Box<[JsValue]> {
    let m = BigUint::parse_bytes(message_hash.as_bytes(), 16).unwrap();
    let k = BigUint::parse_bytes(private_key.as_bytes(), 16).unwrap();
    let (r, w) = starkcrypto::ecdsa::sign(&m, &k);
    vec![
        JsValue::from_str(&r.to_str_radix(16)),
        JsValue::from_str(&w.to_str_radix(16)),
    ]
    .into_boxed_slice()
}

/*
#[wasm_bindgen]
pub fn verify(message_hash: &str, signature: JsValue, public_key: JsValue) -> bool {

    let m = BigUint::parse_bytes(message_hash.as_bytes(), 16).unwrap();
    let k = BigUint::parse_bytes(private_key.as_bytes(), 16).unwrap();
    let (r, w) = starkcrypto::ecdsa::sign(&m, &k);
    vec![
        JsValue::from_str(&r.to_str_radix(16)),
        JsValue::from_str(&w.to_str_radix(16)),
    ].into_boxed_slice()
}
*/
