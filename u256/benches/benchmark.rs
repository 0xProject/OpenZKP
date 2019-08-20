#![warn(clippy::all)]
#![deny(warnings)]

#[cfg(not(feature = "bench"))]
compile_error!("Building bench requires feature bench.");

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use macros_decl::u256h;
use u256::U256;

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

fn criterion_benchmark(c: &mut Criterion) {
    u256_add(c);
    u256_mul(c);
    u256_invmod256(c);
    u256_invmod(c);
    u256_divrem(c);
    u256_mulmod(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
