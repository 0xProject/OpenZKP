// Criterion requires the second argument to be by reference
#![allow(clippy::trivially_copy_pass_by_ref)]

use criterion::{black_box, Bencher};
use zkp_elliptic_curve_crypto::{private_to_public, sign, verify};
use zkp_macros_decl::{field_element, u256h};
use zkp_primefield::FieldElement;
use zkp_u256::U256;

pub fn starkcrypto_verify(bench: &mut Bencher, _i: &()) {
    let message_hash = u256h!("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb");
    let private_key = u256h!("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a");
    let public = private_to_public(&private_key);
    let (r, w) = sign(&message_hash, &private_key);
    bench.iter(|| black_box(verify(&message_hash, &r, &w, &public)))
}

pub fn starkcrypto_field_mul(bench: &mut Bencher, _i: &()) {
    let a = field_element!("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb");
    let b = field_element!("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a");
    bench.iter(|| {
        black_box(black_box(&a) * black_box(&b));
    })
}

pub fn starkcrypto_field_sqr(bench: &mut Bencher, _i: &()) {
    let a = field_element!("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb");
    bench.iter(|| {
        black_box(black_box(&a).square());
    })
}

pub fn starkcrypto_field_inv(bench: &mut Bencher, _i: &()) {
    let a = field_element!("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb");
    bench.iter(|| {
        black_box(black_box(&a).inv());
    })
}
