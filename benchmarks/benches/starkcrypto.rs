use criterion::{black_box, Bencher};
use hex_literal::*;
use starkcrypto::ecdsa::{private_to_public, sign, verify};
use starkcrypto::field::FieldElement;
use starkcrypto::u256::U256;
use starkcrypto::u256h;

pub fn starkcrypto_verify(bench: &mut Bencher, _i: &()) {
    let message_hash = u256h!("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb");
    let private_key = u256h!("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a");
    let public = private_to_public(&private_key);
    let (r, w) = sign(&message_hash, &private_key);
    bench.iter(|| black_box(verify(&message_hash, &r, &w, &public)))
}

pub fn starkcrypto_field_mul(bench: &mut Bencher, _i: &()) {
    let a = FieldElement::from(u256h!(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb"
    ));
    let b = FieldElement::from(u256h!(
        "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a"
    ));
    bench.iter(|| {
        black_box(black_box(&a) * black_box(&b));
    })
}

pub fn starkcrypto_field_sqr(bench: &mut Bencher, _i: &()) {
    let a = FieldElement::from(u256h!(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb"
    ));
    bench.iter(|| {
        black_box(black_box(&a).square());
    })
}

pub fn starkcrypto_field_inv(bench: &mut Bencher, _i: &()) {
    let a = FieldElement::from(u256h!(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb"
    ));
    bench.iter(|| {
        black_box(black_box(&a).inv());
    })
}

pub fn starkcrypto_field_inv_lemher(bench: &mut Bencher, _i: &()) {
    let a = FieldElement::from(u256h!(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb"
    ));
    bench.iter(|| {
        black_box(black_box(&a).inv_lemher());
    })
}