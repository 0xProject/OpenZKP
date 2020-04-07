#![warn(clippy::all)]
use criterion::{black_box, Criterion};
use zkp_elliptic_curve::ScalarFieldElement;
use zkp_elliptic_curve_crypto::{PrivateKey, PublicKey};
use zkp_macros_decl::u256h;
use zkp_u256::U256;

fn ecdsa_sign(crit: &mut Criterion) {
    let digest = ScalarFieldElement::from(u256h!(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb"
    ));
    let private_key = PrivateKey::from(u256h!(
        "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a"
    ));
    crit.bench_function("Ecdsa sign", move |bench| {
        bench.iter(|| black_box(private_key.sign(&digest)))
    });
}

fn ecdsa_verify(crit: &mut Criterion) {
    let digest = ScalarFieldElement::from(u256h!(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb"
    ));
    let private_key = PrivateKey::from(u256h!(
        "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a"
    ));
    let public = PublicKey::from(&private_key);
    let signature = private_key.sign(&digest);
    crit.bench_function("Ecdsa verify", move |bench| {
        bench.iter(|| black_box(public.verify(&digest, &signature)))
    });
}

fn main() {
    let crit = &mut Criterion::default().configure_from_args();
    ecdsa_sign(crit);
    ecdsa_verify(crit);
    crit.final_summary();
}
