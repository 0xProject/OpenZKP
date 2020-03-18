#![warn(clippy::all)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zkp_macros_decl::u256h;
use zkp_u256::{
    DivRem, Inv, InvMod, Montgomery, MontgomeryParameters, MulFullInline, SquareFullInline,
    SquareInline, U256,
};

struct Generic();

impl MontgomeryParameters for Generic {
    type UInt = U256;

    const M64: u64 = 0xbb6b_3c4c_e8bd_e631;
    const MODULUS: U256 =
        u256h!("0800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f");
    const R1: U256 = u256h!("07fffffffffffdf10000000000000008c75ec4b46df16bee51925a0bf4fca74f");
    const R2: U256 = u256h!("07d9e57c2333766ebaf0ab4cf78bbabb509cf64d14ce60b96021b3f1ea1c688d");
    const R3: U256 = u256h!("01b2ba88ca1fe18a1f0d9dedfedfda501da2136eb8b3f20e81147668fddd0429");
}

struct Proth();

impl MontgomeryParameters for Proth {
    type UInt = U256;

    const M64: u64 = 0xffff_ffff_ffff_ffff;
    const MODULUS: U256 =
        u256h!("0800000000000011000000000000000000000000000000000000000000000001");
    const R1: U256 = u256h!("07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1");
    const R2: U256 = u256h!("07ffd4ab5e008810ffffffffff6f800000000001330ffffffffffd737e000401");
    const R3: U256 = u256h!("038e5f79873c0a6df47d84f8363000187545706677ffcc06cc7177d1406df18e");
}

fn and(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    let b = u256h!("04742d726d4800e1015941bf06591cd139bd034f968ab8a225f92cbba85e5776");
    crit.bench_function("and", move |bench| {
        bench.iter(|| black_box(&a).clone() & black_box(&b))
    });
}

fn shl(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    crit.bench_function("shl", move |bench| {
        bench.iter(|| black_box(&a).clone() << 3)
    });
}

fn add(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    let b = u256h!("04742d726d4800e1015941bf06591cd139bd034f968ab8a225f92cbba85e5776");
    crit.bench_function("add", move |bench| {
        bench.iter(|| black_box(&a).clone() + black_box(&b))
    });
}

fn sub(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    let b = u256h!("04742d726d4800e1015941bf06591cd139bd034f968ab8a225f92cbba85e5776");
    crit.bench_function("sub", move |bench| {
        bench.iter(|| black_box(&a).clone() - black_box(&b))
    });
}

fn sqr(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    crit.bench_function("sqr", move |bench| {
        bench.iter(|| black_box(&a).square_inline())
    });
}

fn sqr_full(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    crit.bench_function("sqr full", move |bench| {
        bench.iter(|| black_box(&a).square_full_inline())
    });
}

fn mul(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    let b = u256h!("04742d726d4800e1015941bf06591cd139bd034f968ab8a225f92cbba85e5776");
    crit.bench_function("mul", move |bench| {
        bench.iter(|| black_box(&a).clone() * black_box(&b))
    });
}

fn mul_full(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    let b = u256h!("04742d726d4800e1015941bf06591cd139bd034f968ab8a225f92cbba85e5776");
    crit.bench_function("mul full", move |bench| {
        bench.iter(|| black_box(&a).mul_full_inline(black_box(&b)))
    });
}

fn invmod256(crit: &mut Criterion) {
    let n = u256h!("07717a21e77894e8d82120c54277c73ee1062290709829411717f47973471ed5");
    crit.bench_function("invmod256", move |bench| bench.iter(|| black_box(&n).inv()));
}

fn invmod(crit: &mut Criterion) {
    let m = u256h!("0800000000000011000000000000000000000000000000000000000000000001");
    let n = u256h!("07717a21e77894e8d82120c54277c73ee1062290709829411717f47973471ed5");
    crit.bench_function("invmod", move |bench| {
        bench.iter(|| black_box(&n).inv_mod(black_box(&m)))
    });
}

fn divrem(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    let b = u256h!("0800000000000011000000000000000000000000000000000000000000000001");
    crit.bench_function("divrem", move |bench| {
        bench.iter(|| black_box(black_box(&a).div_rem(black_box(&b))))
    });
}

fn mulmod(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    let b = u256h!("07717a21e77894e8d82120c54277c73ee1062290709829411717f47973471ed5");
    let m = u256h!("0800000000000011000000000000000000000000000000000000000000000001");
    crit.bench_function("mulmod", move |bench| {
        bench.iter(|| black_box(black_box(&a).mulmod(black_box(&b), black_box(&m))))
    });
}

fn montgomery_redc(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    let b = u256h!("04742d726d4800e1015941bf06591cd139bd034f968ab8a225f92cbba85e5776");
    crit.bench_function("redc", move |bench| {
        bench.iter(|| U256::redc_inline::<Generic>(black_box(&a), black_box(&b)))
    });
}

fn montgomery_mul_redc(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    let b = u256h!("04742d726d4800e1015941bf06591cd139bd034f968ab8a225f92cbba85e5776");
    crit.bench_function("mul redc", move |bench| {
        bench.iter(|| black_box(&a).mul_redc_inline::<Generic>(black_box(&b)))
    });
}

fn montgomery_mulmod(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    let b = u256h!("04742d726d4800e1015941bf06591cd139bd034f968ab8a225f92cbba85e5776");
    crit.bench_function("mont mulmod", move |bench| {
        bench.iter(|| black_box(&a).mul_mod::<Generic>(black_box(&b)))
    });
}

fn montgomery_proth_redc(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    let b = u256h!("04742d726d4800e1015941bf06591cd139bd034f968ab8a225f92cbba85e5776");
    crit.bench_function("proth redc", move |bench| {
        bench.iter(|| U256::redc_inline::<Proth>(black_box(&a), black_box(&b)))
    });
}

fn montgomery_proth_mul_redc(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    let b = u256h!("04742d726d4800e1015941bf06591cd139bd034f968ab8a225f92cbba85e5776");
    crit.bench_function("proth mul redc", move |bench| {
        bench.iter(|| black_box(&a).mul_redc_inline::<Proth>(black_box(&b)))
    });
}

fn montgomery_proth_mulmod(crit: &mut Criterion) {
    let a = u256h!("01c9e043b135fa21471cec503f1181884ef3d9c2cb44b6a3531bb3056443bc99");
    let b = u256h!("04742d726d4800e1015941bf06591cd139bd034f968ab8a225f92cbba85e5776");
    crit.bench_function("proth mont mulmod", move |bench| {
        bench.iter(|| black_box(&a).mul_mod::<Proth>(black_box(&b)))
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    and(c);
    shl(c);
    add(c);
    sub(c);
    sqr(c);
    sqr_full(c);
    mul(c);
    mul_full(c);
    invmod256(c);
    invmod(c);
    divrem(c);
    mulmod(c);
    montgomery_redc(c);
    montgomery_mul_redc(c);
    montgomery_mulmod(c);
    montgomery_proth_redc(c);
    montgomery_proth_mul_redc(c);
    montgomery_proth_mulmod(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
