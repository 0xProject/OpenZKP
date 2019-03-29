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

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
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

#[derive(Serialize, Deserialize)]
pub struct MakerMessage {
    vault_a: u32,
    vault_b: u32,
    amount_a: u64,
    amount_b: u64,
    token_a: String,
    token_b: String,
    trade_id: u32,
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

fn parse_message(message: &JsValue) -> starkcrypto::MakerMessage {
    let message: MakerMessage = message.into_serde().unwrap();
    starkcrypto::MakerMessage {
        vault_a: message.vault_a,
        vault_b: message.vault_b,
        amount_a: message.amount_a,
        amount_b: message.amount_b,
        token_a: from_string(&message.token_a),
        token_b: from_string(&message.token_b),
        trade_id: message.trade_id,
    }
}

#[wasm_bindgen]
pub fn maker_hash(message: &JsValue) -> String {
    let message = parse_message(message);
    to_string(&starkcrypto::maker_hash(&message))
}

#[wasm_bindgen]
pub fn taker_hash(maker_message_hash: &str, vault_a: u32, vault_b: u32) -> String {
    to_string(&starkcrypto::taker_hash(
        &from_string(maker_message_hash),
        vault_a,
        vault_b,
    ))
}

#[wasm_bindgen]
pub fn maker_sign(message: &JsValue, private_key: &str) -> JsValue {
    let message = parse_message(message);
    let (r, w) = starkcrypto::maker_sign(&message, &from_string(private_key));
    JsValue::from_serde(&Signature {
        r: to_string(&r),
        w: to_string(&w),
    })
    .unwrap()
}

#[wasm_bindgen]
pub fn taker_sign(message: &JsValue, vault_a: u32, vault_b: u32, private_key: &str) -> JsValue {
    let message = parse_message(message);
    let (r, w) = starkcrypto::taker_sign(&message, vault_a, vault_b, &from_string(private_key));
    JsValue::from_serde(&Signature {
        r: to_string(&r),
        w: to_string(&w),
    })
    .unwrap()
}

#[wasm_bindgen]
pub fn maker_verify(message: &JsValue, signature: &JsValue, public_key: &JsValue) -> bool {
    let message = parse_message(message);
    let s: Signature = signature.into_serde().unwrap();
    let p: CurvePoint = public_key.into_serde().unwrap();
    let r = from_string(&s.r);
    let w = from_string(&s.w);
    let x = from_string(&p.x);
    let y = from_string(&p.y);
    starkcrypto::maker_verify(&message, (&r, &w), (&x, &y))
}

#[wasm_bindgen]
pub fn taker_verify(
    message: &JsValue,
    vault_a: u32,
    vault_b: u32,
    signature: &JsValue,
    public_key: &JsValue,
) -> bool {
    let message = parse_message(message);
    let s: Signature = signature.into_serde().unwrap();
    let p: CurvePoint = public_key.into_serde().unwrap();
    let r = from_string(&s.r);
    let w = from_string(&s.w);
    let x = from_string(&p.x);
    let y = from_string(&p.y);
    starkcrypto::taker_verify(&message, vault_a, vault_b, (&r, &w), (&x, &y))
}
