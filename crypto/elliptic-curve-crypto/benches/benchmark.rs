#![warn(clippy::all)]
use criterion::{black_box, Criterion};
use zkp_elliptic_curve::ScalarFieldElement;
use zkp_elliptic_curve_crypto::{private_to_public, sign, verify};
use zkp_macros_decl::u256h;
use zkp_u256::U256;

fn ecdsa_sign(crit: &mut Criterion) {
    let digest = ScalarFieldElement::from(u256h!(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb"
    ));
    let private_key = ScalarFieldElement::from(u256h!(
        "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a"
    ));
    crit.bench_function("Ecdsa sign", move |bench| {
        bench.iter(|| black_box(sign(&digest, &private_key)))
    });
}

fn ecdsa_verify(crit: &mut Criterion) {
    let digest = ScalarFieldElement::from(u256h!(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb"
    ));
    let private_key = ScalarFieldElement::from(u256h!(
        "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a"
    ));
    let public = private_to_public(&private_key);
    let signature = sign(&digest, &private_key);
    crit.bench_function("Ecdsa verify", move |bench| {
        bench.iter(|| black_box(verify(&digest, &signature, &public)))
    });
}

fn main() {
    let crit = &mut Criterion::default().configure_from_args();
    ecdsa_sign(crit);
    ecdsa_verify(crit);
    crit.final_summary();
}
