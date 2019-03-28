mod utils;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use starkcrypto;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

fn from_string(s: &str) -> [u8; 32] {
    // TODO: Skip '0x' prefix
    // TODO: Decoding error.
    let h = hex::decode(s).unwrap();
    let mut array = [0; 32];
    array.copy_from_slice(h.as_slice());
    array
}

fn to_string(b: &[u8; 32]) -> String {
    hex::encode(b)
}

#[wasm_bindgen]
pub fn nop(a: &str, b: &str) -> String {
    let elements = [from_string(a), from_string(b)];
    let h = elements[1].clone();
    to_string(&h)
}

#[wasm_bindgen]
pub fn pedersen_hash(a: &str, b: &str) -> String {
    to_string(&starkcrypto::hash(&from_string(a), &from_string(b)))
}

#[derive(Serialize, Deserialize)]
pub struct Signature {
    r: String,
    w: String,
}

#[derive(Serialize, Deserialize)]
pub struct CurvePoint {
    x: String,
    y: String,
}

#[wasm_bindgen]
pub fn public_key(private_key: &str) -> JsValue {
    let (x, y) = starkcrypto::public_key(&from_string(private_key));
    JsValue::from_serde(&CurvePoint {
        x: to_string(&x),
        y: to_string(&y),
    })
    .unwrap()
}

#[wasm_bindgen]
pub fn sign(message_hash: &str, private_key: &str) -> JsValue {
    let (r, w) = starkcrypto::sign(&from_string(message_hash), &from_string(private_key));
    JsValue::from_serde(&Signature {
        r: to_string(&r),
        w: to_string(&w),
    })
    .unwrap()
}

#[wasm_bindgen]
pub fn verify(message_hash: &str, signature: &JsValue, public_key: &JsValue) -> bool {
    let s: Signature = signature.into_serde().unwrap();
    let p: CurvePoint = public_key.into_serde().unwrap();
    let r = from_string(&s.r);
    let w = from_string(&s.w);
    let x = from_string(&p.x);
    let y = from_string(&p.y);
    starkcrypto::verify(&from_string(message_hash), (&r, &w), (&x, &y))
}
