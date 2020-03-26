#![warn(clippy::all)]
use criterion::{black_box, Criterion};
use zkp_elliptic_curve::{mul, Affine, Jacobian, ScalarFieldElement};
use zkp_macros_decl::u256h;
use zkp_primefield::FieldElement;
use zkp_u256::U256;

fn curve_add(crit: &mut Criterion) {
    let a = Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0176a4c00d1ce6b642176e460624b1699da148593f701cac4df2280c2edb163f"
        )),
    };
    let b = Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03722d346a64345ec69b4a36c97247fa924bedfbd371d5bdedeb7db3fcf32a78"
        )),
        y: FieldElement::from_montgomery(u256h!(
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
        x: FieldElement::from_montgomery(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement::from_montgomery(u256h!(
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
        x: FieldElement::from_montgomery(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0176a4c00d1ce6b642176e460624b1699da148593f701cac4df2280c2edb163f"
        )),
    };
    let b = ScalarFieldElement::from(u256h!(
        "014023b44fbb1e6f2a79c929c6da775be3c4b9e043d439385b5050fdc69177e3"
    ));
    crit.bench_function("Curve mul", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a) * black_box(&b));
        })
    });
}

fn jacobian_to_affine(crit: &mut Criterion) {
    let a = Jacobian::from(Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement::from_montgomery(u256h!(
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
        x: FieldElement::from_montgomery(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0176a4c00d1ce6b642176e460624b1699da148593f701cac4df2280c2edb163f"
        )),
    });
    let b = Jacobian::from(Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03722d346a64345ec69b4a36c97247fa924bedfbd371d5bdedeb7db3fcf32a78"
        )),
        y: FieldElement::from_montgomery(u256h!(
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
        x: FieldElement::from_montgomery(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0176a4c00d1ce6b642176e460624b1699da148593f701cac4df2280c2edb163f"
        )),
    });
    let b = Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03722d346a64345ec69b4a36c97247fa924bedfbd371d5bdedeb7db3fcf32a78"
        )),
        y: FieldElement::from_montgomery(u256h!(
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
        x: FieldElement::from_montgomery(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement::from_montgomery(u256h!(
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
        x: FieldElement::from_montgomery(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0176a4c00d1ce6b642176e460624b1699da148593f701cac4df2280c2edb163f"
        )),
    });
    let b = ScalarFieldElement::from(u256h!(
        "014023b44fbb1e6f2a79c929c6da775be3c4b9e043d439385b5050fdc69177e3"
    ));
    crit.bench_function("Jacobian mul", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a) * black_box(&b));
        })
    });
}

fn jacobian_mul_affine(crit: &mut Criterion) {
    let a = Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0176a4c00d1ce6b642176e460624b1699da148593f701cac4df2280c2edb163f"
        )),
    };
    let b = ScalarFieldElement::from(u256h!(
        "014023b44fbb1e6f2a79c929c6da775be3c4b9e043d439385b5050fdc69177e3"
    ));
    crit.bench_function("Jacobian mul affine", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a) * black_box(&b));
        })
    });
}

fn wnaf_mul_affine(crit: &mut Criterion) {
    let a = Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04f50f81bf91b7ada9de33eeec4ae787bc39f520fbb5c8fa4620fecfca4d7cf5"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0176a4c00d1ce6b642176e460624b1699da148593f701cac4df2280c2edb163f"
        )),
    };
    let b = ScalarFieldElement::from(u256h!(
        "014023b44fbb1e6f2a79c929c6da775be3c4b9e043d439385b5050fdc69177e3"
    ));
    crit.bench_function("Wnaf mul", move |bench| {
        bench.iter(|| {
            black_box(mul(black_box(&a), black_box(&b)));
        })
    });
}

fn main() {
    let crit = &mut Criterion::default().configure_from_args();
    curve_add(crit);
    curve_dbl(crit);
    curve_mul(crit);
    jacobian_add(crit);
    jacobian_add_affine(crit);
    jacobian_dbl(crit);
    jacobian_mul(crit);
    jacobian_mul_affine(crit);
    jacobian_to_affine(crit);
    wnaf_mul_affine(crit);
    crit.final_summary();
}
