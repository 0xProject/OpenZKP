//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use starkcrypto;
use starkcrypto_wasm::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn test_hash_1() {
    let a = "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb";
    let b = "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a";
    let result = pedersen_hash(a, b);
    let expected = "02d895bd76790645fb867eaf57037e4aa8af1bbb139a84d01e311a7c53f3111b";
    assert_eq!(expected, result);
}
