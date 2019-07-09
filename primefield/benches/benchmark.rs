#![warn(clippy::all)]
#![deny(warnings)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hex_literal::*;
use primefield::{u256h, FieldElement, U256};

fn u256_add(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    let b = u256h!("04742d726d4800e1015941bf06591cd139bd034f968ab8a225f92cbba85e5776");
    crit.bench_function("U256 add", move |bench| {
        bench.iter(|| black_box(&a).clone() + black_box(&b))
    });
}

fn u256_mul(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    let b = u256h!("04742d726d4800e1015941bf06591cd139bd034f968ab8a225f92cbba85e5776");
    crit.bench_function("U256 mul", move |bench| {
        bench.iter(|| black_box(&a).clone() * black_box(&b))
    });
}

fn u256_invmod256(crit: &mut Criterion) {
    let n = u256h!("07717a21e77894e8d82120c54277c73ee1062290709829411717f47973471ed5");
    crit.bench_function("U256 invmod256", move |bench| {
        bench.iter(|| black_box(&n).invmod256())
    });
}

fn u256_invmod(crit: &mut Criterion) {
    let m = u256h!("0800000000000011000000000000000000000000000000000000000000000001");
    let n = u256h!("07717a21e77894e8d82120c54277c73ee1062290709829411717f47973471ed5");
    crit.bench_function("U256 invmod", move |bench| {
        bench.iter(|| black_box(&n).invmod(black_box(&m)))
    });
}

fn u256_divrem(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    let b = u256h!("0800000000000011000000000000000000000000000000000000000000000001");
    crit.bench_function("U256 divrem", move |bench| {
        bench.iter(|| black_box(black_box(&a).divrem(black_box(&b))))
    });
}

fn u256_mulmod(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    let b = u256h!("07717a21e77894e8d82120c54277c73ee1062290709829411717f47973471ed5");
    let m = u256h!("0800000000000011000000000000000000000000000000000000000000000001");
    crit.bench_function("U256 mulmod", move |bench| {
        bench.iter(|| black_box(black_box(&a).mulmod(black_box(&b), black_box(&m))))
    });
}

fn field_add(crit: &mut Criterion) {
    let a = FieldElement(u256h!(
        "03f9b5d66dd1e8ef70ead1370f862cc9c29e319a176e9f5b7f10c24c4de29f0f"
    ));
    let b = FieldElement(u256h!(
        "0560d4ae8cd8a5974b122d8cf65967e5c83911ed0c74f02899727b3f2e916e23"
    ));
    crit.bench_function("Field add", move |bench| {
        bench.iter(|| black_box(&a).clone() + black_box(&b).clone())
    });
}

fn field_mul(crit: &mut Criterion) {
    let a = FieldElement(u256h!(
        "03f9b5d66dd1e8ef70ead1370f862cc9c29e319a176e9f5b7f10c24c4de29f0f"
    ));
    let b = FieldElement(u256h!(
        "0560d4ae8cd8a5974b122d8cf65967e5c83911ed0c74f02899727b3f2e916e23"
    ));
    crit.bench_function("Field mul", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a) * black_box(&b));
        })
    });
}

fn field_sqrt(crit: &mut Criterion) {
    let a = FieldElement(u256h!(
        "03f9b5d66dd1e8ef70ead1370f862cc9c29e319a176e9f5b7f10c24c4de29f0f"
    ));
    crit.bench_function("Field square root", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a).square_root());
        })
    });
}

fn field_inv(crit: &mut Criterion) {
    let a = FieldElement(u256h!(
        "03f9b5d66dd1e8ef70ead1370f862cc9c29e319a176e9f5b7f10c24c4de29f0f"
    ));
    crit.bench_function("Field inv", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a).clone().inv());
        })
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    u256_add(c);
    u256_mul(c);
    u256_invmod256(c);
    u256_invmod(c);
    u256_divrem(c);
    u256_mulmod(c);
    field_add(c);
    field_mul(c);
    field_inv(c);
    field_sqrt(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
