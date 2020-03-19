#![warn(clippy::all)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;
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
    crit.bench_function("and", move |bench| {
        let a: &U256 = &random();
        let b: &U256 = &random();
        bench.iter(|| black_box(a) & black_box(b))
    });
}

fn shl(crit: &mut Criterion) {
    crit.bench_function("shl", move |bench| {
        let a: &U256 = &random();
        let b = random::<usize>() % 256;
        bench.iter(|| black_box(a).clone() << b)
    });
}

fn add(crit: &mut Criterion) {
    crit.bench_function("add", move |bench| {
        let a: &U256 = &random();
        let b: &U256 = &random();
        bench.iter(|| black_box(a) + black_box(b))
    });
}

fn sub(crit: &mut Criterion) {
    crit.bench_function("sub", move |bench| {
        let a: &U256 = &random();
        let b: &U256 = &random();
        bench.iter(|| black_box(a) - black_box(b))
    });
}

fn sqr(crit: &mut Criterion) {
    crit.bench_function("sqr", move |bench| {
        let a: &U256 = &random();
        bench.iter(|| black_box(a).square_inline())
    });
}

fn sqr_full(crit: &mut Criterion) {
    crit.bench_function("sqr full", move |bench| {
        let a = &random::<U256>();
        bench.iter(|| black_box(a).square_full_inline())
    });
}

fn mul(crit: &mut Criterion) {
    crit.bench_function("mul", move |bench| {
        let a: &U256 = &random();
        let b: &U256 = &random();
        bench.iter(|| black_box(a) * black_box(b))
    });
}

fn mul_full(crit: &mut Criterion) {
    crit.bench_function("mul full", move |bench| {
        let a: &U256 = &random();
        let b: &U256 = &random();
        bench.iter(|| black_box(a).mul_full_inline(black_box(b)))
    });
}

fn invmod256(crit: &mut Criterion) {
    crit.bench_function("invmod256", move |bench| {
        // Value must be odd
        let a: &U256 = &(random::<U256>() | U256::ONE);
        bench.iter(|| black_box(a).inv())
    });
}

fn invmod(crit: &mut Criterion) {
    // Fixed Proth-prime modulus
    let m = &u256h!("0800000000000011000000000000000000000000000000000000000000000001");
    crit.bench_function("invmod", move |bench| {
        // Should not be zero, but chance is neglible
        let a: &U256 = &(random::<U256>() % m);
        bench.iter(|| black_box(a).inv_mod(black_box(m)))
    });
}

fn divrem(crit: &mut Criterion) {
    crit.bench_function("divrem", move |bench| {
        let a: &U256 = &random();
        let b: &U256 = &random();
        bench.iter(|| black_box(black_box(a).div_rem(black_box(b))))
    });
}

fn mulmod(crit: &mut Criterion) {
    let m = &u256h!("0800000000000011000000000000000000000000000000000000000000000001");
    crit.bench_function("mulmod", move |bench| {
        let a = &(random::<U256>() % m);
        let b = &(random::<U256>() % m);
        bench.iter(|| black_box(black_box(&a).mulmod(black_box(&b), black_box(&m))))
    });
}

fn montgomery_redc(crit: &mut Criterion) {
    crit.bench_function("redc", move |bench| {
        let a = &(random::<U256>() % Generic::MODULUS);
        let b = &(random::<U256>() % Generic::MODULUS);
        bench.iter(|| U256::redc_inline::<Generic>(black_box(a), black_box(b)))
    });
}

fn montgomery_mul_redc(crit: &mut Criterion) {
    crit.bench_function("mul redc", move |bench| {
        let a = &(random::<U256>() % Generic::MODULUS);
        let b = &(random::<U256>() % Generic::MODULUS);
        bench.iter(|| black_box(a).mul_redc_inline::<Generic>(black_box(b)))
    });
}

fn montgomery_mulmod(crit: &mut Criterion) {
    crit.bench_function("mont mulmod", move |bench| {
        let a = &(random::<U256>() % Generic::MODULUS);
        let b = &(random::<U256>() % Generic::MODULUS);
        bench.iter(|| black_box(a).mul_mod::<Generic>(black_box(b)))
    });
}

fn montgomery_proth_redc(crit: &mut Criterion) {
    crit.bench_function("proth redc", move |bench| {
        let a = &(random::<U256>() % Proth::MODULUS);
        let b = &(random::<U256>() % Proth::MODULUS);
        bench.iter(|| U256::redc_inline::<Proth>(black_box(a), black_box(b)))
    });
}

fn montgomery_proth_mul_redc(crit: &mut Criterion) {
    crit.bench_function("proth mul redc", move |bench| {
        let a = &(random::<U256>() % Proth::MODULUS);
        let b = &(random::<U256>() % Proth::MODULUS);
        bench.iter(|| black_box(a).mul_redc_inline::<Proth>(black_box(b)))
    });
}

fn montgomery_proth_mulmod(crit: &mut Criterion) {
    crit.bench_function("proth mont mulmod", move |bench| {
        let a = &(random::<U256>() % Proth::MODULUS);
        let b = &(random::<U256>() % Proth::MODULUS);
        bench.iter(|| black_box(a).mul_mod::<Proth>(black_box(b)))
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
