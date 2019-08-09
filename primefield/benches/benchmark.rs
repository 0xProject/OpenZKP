#![warn(clippy::all)]
#![deny(warnings)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use macros_decl::u256h;
use primefield::FieldElement;
use u256::U256;

fn field_add(crit: &mut Criterion) {
    let a = FieldElement::from_montgomery(u256h!(
        "03f9b5d66dd1e8ef70ead1370f862cc9c29e319a176e9f5b7f10c24c4de29f0f"
    ));
    let b = FieldElement::from_montgomery(u256h!(
        "0560d4ae8cd8a5974b122d8cf65967e5c83911ed0c74f02899727b3f2e916e23"
    ));
    crit.bench_function("Field add", move |bench| {
        bench.iter(|| black_box(&a).clone() + black_box(&b).clone())
    });
}

fn field_mul(crit: &mut Criterion) {
    let a = FieldElement::from_montgomery(u256h!(
        "03f9b5d66dd1e8ef70ead1370f862cc9c29e319a176e9f5b7f10c24c4de29f0f"
    ));
    let b = FieldElement::from_montgomery(u256h!(
        "0560d4ae8cd8a5974b122d8cf65967e5c83911ed0c74f02899727b3f2e916e23"
    ));
    crit.bench_function("Field mul", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a) * black_box(&b));
        })
    });
}

fn field_sqrt(crit: &mut Criterion) {
    let a = FieldElement::from_montgomery(u256h!(
        "03f9b5d66dd1e8ef70ead1370f862cc9c29e319a176e9f5b7f10c24c4de29f0f"
    ));
    crit.bench_function("Field square root", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a).square_root());
        })
    });
}

fn field_inv(crit: &mut Criterion) {
    let a = FieldElement::from_montgomery(u256h!(
        "03f9b5d66dd1e8ef70ead1370f862cc9c29e319a176e9f5b7f10c24c4de29f0f"
    ));
    crit.bench_function("Field inv", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a).clone().inv());
        })
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    field_add(c);
    field_mul(c);
    field_inv(c);
    field_sqrt(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
