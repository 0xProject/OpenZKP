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
    let a = FieldElement::new(&[
        0x0f3855f5, 0x37862eb2, 0x275b919f, 0x325329cb, 0xe968e6a2, 0xa2ceee5c, 0xd5f1d547,
        0x07211989,
    ]);
    let b = FieldElement::new(&[
        0x32c781dd, 0x6f6a3b68, 0x3bac723c, 0xd5893114, 0xd0178b37, 0x5476714f, 0x1c567d5a,
        0x0219cad4,
    ]);
    bench.iter(|| {
        black_box(black_box(&a) * black_box(&b));
    })
}
