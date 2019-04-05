mod curve25519_donna;
mod ed25519_dalek;
mod secp256k1_bindings;
mod secp256k1_native;
mod starkcrypto;
use crate::curve25519_donna::*;
use crate::ed25519_dalek::*;
use crate::secp256k1_bindings::*;
use crate::secp256k1_native::*;
use crate::starkcrypto::*;

use criterion::{criterion_group, criterion_main, Criterion, Fun};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_functions(
        "ECDA Verify",
        vec![
            Fun::new("starkcrypto", starkcrypto_verify),
            Fun::new("secp256k1 native", secp256k1_native_verify),
            Fun::new("secp256k1 bindings", secp256k1_bindings_verify),
            Fun::new("ed25519 dalek", ed25519_dalek_verify),
            Fun::new("curve25519 donna", curve25519_donna_verify),
        ],
        (),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
