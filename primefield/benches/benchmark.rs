// TODO: Use u256h everywhere
#![allow(clippy::unreadable_literal)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hex_literal::*;
use primefield::{u256h, FieldElement, U256};

fn u256_add(crit: &mut Criterion) {
    let a = U256::new(
        0x531bb3056443bc99,
        0x4ef3d9c2cb44b6a3,
        0x471cec503f118188,
        0x01c9e043b135fa21,
    );
    let b = U256::new(
        0x25f92cbba85e5776,
        0x39bd034f968ab8a2,
        0x015941bf06591cd1,
        0x04742d726d4800e1,
    );
    crit.bench_function("U256 add", move |bench| {
        bench.iter(|| black_box(&a).clone() + black_box(&b))
    });
}

fn u256_mul(crit: &mut Criterion) {
    let a = U256::new(
        0x531bb3056443bc99,
        0x4ef3d9c2cb44b6a3,
        0x471cec503f118188,
        0x01c9e043b135fa21,
    );
    let b = U256::new(
        0x25f92cbba85e5776,
        0x39bd034f968ab8a2,
        0x015941bf06591cd1,
        0x04742d726d4800e1,
    );
    crit.bench_function("U256 mul", move |bench| {
        bench.iter(|| black_box(&a).clone() * black_box(&b))
    });
}

fn u256_invmod256(crit: &mut Criterion) {
    let n = U256::new(
        0x1717f47973471ed5,
        0xe106229070982941,
        0xd82120c54277c73e,
        0x07717a21e77894e8,
    );
    crit.bench_function("U256 invmod256", move |bench| {
        bench.iter(|| black_box(&n).invmod256())
    });
}

fn u256_invmod(crit: &mut Criterion) {
    let m = U256::new(
        0x0000000000000001,
        0x0000000000000000,
        0x0000000000000000,
        0x0800000000000011,
    );
    let n = U256::new(
        0x1717f47973471ed5,
        0xe106229070982941,
        0xd82120c54277c73e,
        0x07717a21e77894e8,
    );
    crit.bench_function("U256 invmod", move |bench| {
        bench.iter(|| black_box(&n).invmod(black_box(&m)))
    });
}

fn u256_divrem(crit: &mut Criterion) {
    let a = U256::new(
        0x531bb3056443bc99,
        0x4ef3d9c2cb44b6a3,
        0x471cec503f118188,
        0x01c9e043b135fa21,
    );
    let b = u256h!("0800000000000011000000000000000000000000000000000000000000000001");
    crit.bench_function("U256 divrem", move |bench| {
        bench.iter(|| black_box(black_box(&a).divrem(black_box(&b))))
    });
}

fn u256_mulmod(crit: &mut Criterion) {
    let a = U256::new(
        0x531bb3056443bc99,
        0x4ef3d9c2cb44b6a3,
        0x471cec503f118188,
        0x01c9e043b135fa21,
    );
    let b = U256::new(
        0x1717f47973471ed5,
        0xe106229070982941,
        0xd82120c54277c73e,
        0x07717a21e77894e8,
    );
    let m = u256h!("0800000000000011000000000000000000000000000000000000000000000001");
    crit.bench_function("U256 mulmod", move |bench| {
        bench.iter(|| black_box(black_box(&a).mulmod(black_box(&b), black_box(&m))))
    });
}

fn field_add(crit: &mut Criterion) {
    let a = FieldElement::new(&[
        0x0f3855f5, 0x37862eb2, 0x275b919f, 0x325329cb, 0xe968e6a2, 0xa2ceee5c, 0xd5f1d547,
        0x07211989,
    ]);
    let b = FieldElement::new(&[
        0x32c781dd, 0x6f6a3b68, 0x3bac723c, 0xd5893114, 0xd0178b37, 0x5476714f, 0x1c567d5a,
        0x0219cad4,
    ]);
    crit.bench_function("Field add", move |bench| {
        bench.iter(|| black_box(&a).clone() + black_box(&b).clone())
    });
}

fn field_mul(crit: &mut Criterion) {
    let a = FieldElement::new(&[
        0x0f3855f5, 0x37862eb2, 0x275b919f, 0x325329cb, 0xe968e6a2, 0xa2ceee5c, 0xd5f1d547,
        0x07211989,
    ]);
    let b = FieldElement::new(&[
        0x32c781dd, 0x6f6a3b68, 0x3bac723c, 0xd5893114, 0xd0178b37, 0x5476714f, 0x1c567d5a,
        0x0219cad4,
    ]);
    crit.bench_function("Field mul", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a) * black_box(&b));
        })
    });
}

fn field_sqrt(crit: &mut Criterion) {
    let a = FieldElement::new(&[
        0x0f3855f5, 0x37862eb2, 0x275b919f, 0x325329cb, 0xe968e6a2, 0xa2ceee5c, 0xd5f1d547,
        0x07211989,
    ]);
    crit.bench_function("Field square root", move |bench| {
        bench.iter(|| {
            black_box(black_box(&a).square_root());
        })
    });
}

fn field_inv(crit: &mut Criterion) {
    let a = FieldElement::new(&[
        0x0f3855f5, 0x37862eb2, 0x275b919f, 0x325329cb, 0xe968e6a2, 0xa2ceee5c, 0xd5f1d547,
        0x07211989,
    ]);
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
