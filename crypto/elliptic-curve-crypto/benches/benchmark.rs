#![warn(clippy::all)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zkp_elliptic_curve_crypto::{private_to_public, sign, verify};
use zkp_macros_decl::u256h;
use zkp_u256::U256;

fn ecdsa_sign(crit: &mut Criterion) {
    let message_hash = u256h!("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb");
    let private_key = u256h!("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a");
    crit.bench_function("Ecdsa sign", move |bench| {
        bench.iter(|| black_box(sign(&message_hash, &private_key)))
    });
}

fn ecdsa_verify(crit: &mut Criterion) {
    let message_hash = u256h!("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb");
    let private_key = u256h!("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a");
    let public = private_to_public(&private_key);
    let (r, w) = sign(&message_hash, &private_key);
    crit.bench_function("Ecdsa verify", move |bench| {
        bench.iter(|| black_box(verify(&message_hash, &r, &w, &public)))
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    ecdsa_sign(c);
    ecdsa_verify(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
