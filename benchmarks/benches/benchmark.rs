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

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    starkcrypto_verify(c);
    secp256k1_native_verify(c);
    secp256k1_bindings_verify(c);
    ed25519_dalek_verify(c);
    curve25519_donna_verify(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
