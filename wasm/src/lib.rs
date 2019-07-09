#![warn(clippy::all)]
#![deny(warnings)]
mod utils;

use cfg_if::cfg_if;
use primefield::U256;
use serde::{Deserialize, Serialize};
use starkdex::wrappers;
use std::u64;
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

fn u64_from_string(s: &str) -> u64 {
    u64::from_str_radix(s, 10).expect("Expected decimal number (as string) less than 2^64.")
}

fn from_string(s: &str) -> [u8; 32] {
    U256::from_decimal_str(s)
        .expect("Expected decimal number (as string) less than 2^256.")
        .to_bytes_be()
}

fn to_string(b: &[u8; 32]) -> String {
    U256::from_bytes_be(b).to_decimal_str()
}

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
    // Debugging support is limited: https://github.com/rustwasm/wasm-bindgen/issues/1289
}

#[wasm_bindgen]
pub fn nop(a: &str, b: &str) -> String {
    let elements = [from_string(a), from_string(b)];
    let h = elements[1];
    to_string(&h)
}

#[wasm_bindgen]
pub fn pedersen_hash(a: &str, b: &str) -> JsValue {
    let msg_hash = to_string(&wrappers::hash(&from_string(a), &from_string(b)));

    #[derive(Serialize, Deserialize)]
    pub struct Result {
        msg_hash: String,
    }
    JsValue::from_serde(&Result { msg_hash }).unwrap()
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
    console_error_panic_hook::set_once();
    let (x, y) = wrappers::public_key(&from_string(private_key));
    JsValue::from_serde(&CurvePoint {
        x: to_string(&x),
        y: to_string(&y),
    })
    .unwrap()
}

#[wasm_bindgen]
pub fn sign(message_hash: &str, private_key: &str) -> JsValue {
    let (r, w) = wrappers::sign(&from_string(message_hash), &from_string(private_key));
    JsValue::from_serde(&Signature {
        r: to_string(&r),
        w: to_string(&w),
    })
    .unwrap()
}

#[wasm_bindgen]
pub fn verify(message_hash: &str, signature: &JsValue, public_key: &JsValue) -> JsValue {
    let s: Signature = signature.into_serde().unwrap();
    let p: CurvePoint = public_key.into_serde().unwrap();
    let r = from_string(&s.r);
    let w = from_string(&s.w);
    let x = from_string(&p.x);
    let y = from_string(&p.y);
    let is_valid = wrappers::verify(&from_string(message_hash), (&r, &w), (&x, &y));

    #[derive(Serialize, Deserialize)]
    struct Result {
        is_valid: bool,
    }
    JsValue::from_serde(&Result { is_valid }).unwrap()
}

fn parse_message(message: &JsValue) -> wrappers::MakerMessage {
    #[derive(Debug, Serialize, Deserialize)]
    struct MakerMessage {
        vault_a:  u32,
        vault_b:  u32,
        amount_a: String,
        amount_b: String,
        token_a:  String,
        token_b:  String,
        trade_id: u32,
    }
    let message: MakerMessage = message.into_serde().unwrap();
    wrappers::MakerMessage {
        vault_a:  message.vault_a,
        vault_b:  message.vault_b,
        amount_a: u64_from_string(&message.amount_a),
        amount_b: u64_from_string(&message.amount_b),
        token_a:  from_string(&message.token_a),
        token_b:  from_string(&message.token_b),
        trade_id: message.trade_id,
    }
}

#[wasm_bindgen]
pub fn maker_hash(message: &JsValue) -> String {
    let message = parse_message(message);
    to_string(&wrappers::maker_hash(&message))
}

#[wasm_bindgen]
pub fn taker_hash(maker_message_hash: &str, vault_a: u32, vault_b: u32) -> String {
    to_string(&wrappers::taker_hash(
        &from_string(maker_message_hash),
        vault_a,
        vault_b,
    ))
}

#[wasm_bindgen]
pub fn maker_sign(message: &JsValue, private_key: &str) -> JsValue {
    let message = parse_message(message);
    let maker_msg = wrappers::maker_hash(&message);
    let (r, w) = wrappers::sign(&maker_msg, &from_string(private_key));

    #[derive(Serialize, Deserialize)]
    struct Result {
        maker_msg: String,
        r:         String,
        w:         String,
    }
    let maker_msg = to_string(&maker_msg);
    let r = to_string(&r);
    let w = to_string(&w);
    JsValue::from_serde(&Result { maker_msg, r, w }).unwrap()
}

#[wasm_bindgen]
pub fn taker_sign(message: &JsValue, vault_a: u32, vault_b: u32, private_key: &str) -> JsValue {
    let message = parse_message(message);
    let maker_msg = wrappers::maker_hash(&message);
    let taker_msg = wrappers::taker_hash(&maker_msg, vault_a, vault_b);
    let (r, w) = wrappers::sign(&taker_msg, &from_string(private_key));

    #[derive(Serialize, Deserialize)]
    struct Result {
        maker_msg: String,
        taker_msg: String,
        r:         String,
        w:         String,
    }
    let maker_msg = to_string(&maker_msg);
    let taker_msg = to_string(&taker_msg);
    let r = to_string(&r);
    let w = to_string(&w);
    JsValue::from_serde(&Result {
        maker_msg,
        taker_msg,
        r,
        w,
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
    wrappers::maker_verify(&message, (&r, &w), (&x, &y))
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
    wrappers::taker_verify(&message, vault_a, vault_b, (&r, &w), (&x, &y))
}
