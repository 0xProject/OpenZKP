#![warn(clippy::all)]
#![deny(warnings)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ecc::{mul, private_to_public, sign, verify, Affine, Jacobian};
use hex_literal::*;
use primefield::FieldElement;
use u256::{u256h, U256};

fn curve_add(crit: &mut Criterion) {
    let a = Affine::Point {
        x: FieldElement(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement(u256h!(
            "0176a4c00d1ce6b642176e460624b1699da148593f701cac4df2280c2edb163f"
        )),
    };
    let b = Affine::Point {
        x: FieldElement(u256h!(
            "03722d346a64345ec69b4a36c97247fa924bedfbd371d5bdedeb7db3fcf32a78"
        )),
        y: FieldElement(u256h!(
            "07444fb1e7e4751935707758c5b9bb6bc270056bc12a00d1f5b82ba217a20876"
        )),
    };
    crit.bench_function("Curve add", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a) + black_box(&b));
        })
    });
}

fn curve_dbl(crit: &mut Criterion) {
    let a = Affine::Point {
        x: FieldElement(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement(u256h!(
            "0176a4c00d1ce6b642176e460624b1699da148593f701cac4df2280c2edb163f"
        )),
    };
    crit.bench_function("Curve dbl", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a).double());
        })
    });
}

fn curve_mul(crit: &mut Criterion) {
    let a = Affine::Point {
        x: FieldElement(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement(u256h!(
            "0176a4c00d1ce6b642176e460624b1699da148593f701cac4df2280c2edb163f"
        )),
    };
    let b = u256h!("014023b44fbb1e6f2a79c929c6da775be3c4b9e043d439385b5050fdc69177e3");
    crit.bench_function("Curve mul", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a) * black_box(&b));
        })
    });
}

fn jacobian_to_affine(crit: &mut Criterion) {
    let a = Jacobian::from(Affine::Point {
        x: FieldElement(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement(u256h!(
            "0176a4c00d1ce6b642176e460624b1699da148593f701cac4df2280c2edb163f"
        )),
    });
    crit.bench_function("Jacobian to Affine", move |bench| {
        bench.iter(|| {
            black_box(Affine::from(black_box(&a)));
        })
    });
}

fn jacobian_add(crit: &mut Criterion) {
    let a = Jacobian::from(Affine::Point {
        x: FieldElement(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement(u256h!(
            "0176a4c00d1ce6b642176e460624b1699da148593f701cac4df2280c2edb163f"
        )),
    });
    let b = Jacobian::from(Affine::Point {
        x: FieldElement(u256h!(
            "03722d346a64345ec69b4a36c97247fa924bedfbd371d5bdedeb7db3fcf32a78"
        )),
        y: FieldElement(u256h!(
            "07444fb1e7e4751935707758c5b9bb6bc270056bc12a00d1f5b82ba217a20876"
        )),
    });
    crit.bench_function("Jacobian add", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a) + black_box(&b));
        })
    });
}

fn jacobian_add_affine(crit: &mut Criterion) {
    let a = Jacobian::from(Affine::Point {
        x: FieldElement(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement(u256h!(
            "0176a4c00d1ce6b642176e460624b1699da148593f701cac4df2280c2edb163f"
        )),
    });
    let b = Affine::Point {
        x: FieldElement(u256h!(
            "03722d346a64345ec69b4a36c97247fa924bedfbd371d5bdedeb7db3fcf32a78"
        )),
        y: FieldElement(u256h!(
            "07444fb1e7e4751935707758c5b9bb6bc270056bc12a00d1f5b82ba217a20876"
        )),
    };
    crit.bench_function("Jacobian add affine", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a) + black_box(&b));
        })
    });
}

fn jacobian_dbl(crit: &mut Criterion) {
    let a = Jacobian::from(Affine::Point {
        x: FieldElement(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement(u256h!(
            "0176a4c00d1ce6b642176e460624b1699da148593f701cac4df2280c2edb163f"
        )),
    });
    crit.bench_function("Jacobian dbl", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a).double());
        })
    });
}

fn jacobian_mul(crit: &mut Criterion) {
    let a = Jacobian::from(Affine::Point {
        x: FieldElement(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement(u256h!(
            "0176a4c00d1ce6b642176e460624b1699da148593f701cac4df2280c2edb163f"
        )),
    });
    let b = u256h!("014023b44fbb1e6f2a79c929c6da775be3c4b9e043d439385b5050fdc69177e3");
    crit.bench_function("Jacobian mul", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a) * black_box(&b));
        })
    });
}

fn jacobian_mul_affine(crit: &mut Criterion) {
    let a = Affine::Point {
        x: FieldElement(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement(u256h!(
            "0176a4c00d1ce6b642176e460624b1699da148593f701cac4df2280c2edb163f"
        )),
    };
    let b = u256h!("014023b44fbb1e6f2a79c929c6da775be3c4b9e043d439385b5050fdc69177e3");
    crit.bench_function("Jacobian mul affine", move |bench| {
        bench.iter(|| {
            black_box(Jacobian::mul(black_box(&a), black_box(&b)));
        })
    });
}

fn wnaf_mul_affine(crit: &mut Criterion) {
    let a = Affine::Point {
        x: FieldElement(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement(u256h!(
            "0176a4c00d1ce6b642176e460624b1699da148593f701cac4df2280c2edb163f"
        )),
    };
    let b = u256h!("014023b44fbb1e6f2a79c929c6da775be3c4b9e043d439385b5050fdc69177e3");
    crit.bench_function("Wnaf mul", move |bench| {
        bench.iter(|| {
            black_box(mul(black_box(&a), black_box(&b)));
        })
    });
}

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
    curve_add(c);
    curve_dbl(c);
    curve_mul(c);
    jacobian_add(c);
    jacobian_add_affine(c);
    jacobian_dbl(c);
    jacobian_mul(c);
    jacobian_mul_affine(c);
    jacobian_to_affine(c);
    wnaf_mul_affine(c);
    ecdsa_sign(c);
    ecdsa_verify(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
