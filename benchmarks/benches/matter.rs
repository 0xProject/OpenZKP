// Criterion requires the second argument to be by reference
#![allow(clippy::trivially_copy_pass_by_ref)]

use criterion::{black_box, Bencher};
use matter::{Field, Fp};

pub fn matter_field_mul(bench: &mut Bencher, _i: &()) {
    let a =
        Fp::from_hex("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb").unwrap();
    let b =
        Fp::from_hex("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a").unwrap();
    bench.iter(|| {
        black_box({
            let mut r = *black_box(&a);
            r.mul_assign(black_box(&b));
            r
        });
    })
}

pub fn matter_field_sqr(bench: &mut Bencher, _i: &()) {
    let a =
        Fp::from_hex("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb").unwrap();
    bench.iter(|| {
        black_box({
            let mut r = *black_box(&a);
            r.square();
            r
        });
    })
}

pub fn matter_field_inv(bench: &mut Bencher, _i: &()) {
    let a =
        Fp::from_hex("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb").unwrap();
    bench.iter(|| {
        black_box(black_box(&a).inverse());
    })
}
