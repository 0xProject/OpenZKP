mod curve25519_donna;
mod dalek;
mod matter;
mod secp256k1_bindings;
mod secp256k1_native;
mod starkcrypto;
use crate::curve25519_donna::*;
use crate::dalek::*;
use crate::matter::*;
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
            Fun::new("ed25519 dalek", dalek_ed25519_verify),
            Fun::new("curve25519 donna", curve25519_donna_verify),
        ],
        (),
    );
    c.bench_functions(
        "Field mul",
        vec![
            Fun::new("starkcrypto", starkcrypto_field_mul),
            Fun::new("matter", matter_field_mul),
            Fun::new("dalek", dalek_scalar_mul),
        ],
        (),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
