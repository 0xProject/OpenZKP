#![warn(clippy::all)]
use criterion::{black_box, Criterion};
use rand::prelude::*;
use zkp_primefield::{FieldElement, Inv, SquareRoot};

fn field_add(crit: &mut Criterion) {
    crit.bench_function("Field add", move |bench| {
        let a: &FieldElement = &random();
        let b: &FieldElement = &random();
        bench.iter(|| black_box(a) + black_box(b))
    });
}

fn field_mul(crit: &mut Criterion) {
    crit.bench_function("Field mul", move |bench| {
        let a: &FieldElement = &random();
        let b: &FieldElement = &random();
        bench.iter(|| {
            black_box(black_box(a) * black_box(b));
        })
    });
}

fn field_inv(crit: &mut Criterion) {
    crit.bench_function("Field inv", move |bench| {
        let a: &FieldElement = &random();
        bench.iter(|| {
            black_box(black_box(a).inv());
        })
    });
}

fn field_sqrt(crit: &mut Criterion) {
    crit.bench_function("Field square root", move |bench| {
        let a: &FieldElement = &random();
        let a = &(a * a);
        bench.iter(|| {
            black_box(black_box(a).square_root());
        })
    });
}

pub fn group(crit: &mut Criterion) {
    field_add(crit);
    field_mul(crit);
    field_inv(crit);
    field_sqrt(crit);
}
