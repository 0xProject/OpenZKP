use crate::{algorithms::limb_operations::mac, U256};
use core::arch::x86_64::{_addcarryx_u64, _mulx_u64};

// TODO: Intrinsics based approach usind ADX is  currently blocked on LLVM.
// See <https://github.com/rust-lang/stdarch/issues/666>
// See <https://bugs.llvm.org/show_bug.cgi?id=41546>

#[inline(always)]
pub fn mul(a: u64, b: u64) -> (u64, u64) {
    let mut hi = 0_u64; // TODO: MaybeUninit?
    let lo = unsafe { _mulx_u64(a, b, &mut hi) };
    (lo, hi)
}

#[inline(always)]
pub fn add(a: u64, b: u64, carry: u8) -> (u64, u8) {
    let mut r = 0_u64; // TODO: MaybeUninit?
    let carry = unsafe { _addcarryx_u64(carry, a, b, &mut r) };
    (r, carry)
}

#[inline(always)]
pub fn mul_full(x: &U256, y: &U256) -> (U256, U256) {
    const ZERO: u64 = 0;
    let x = x.as_limbs();
    let y = y.as_limbs();

    // x[0] *
    let (r0, r1) = mul(x[0], y[0]);
    let (p1, r2) = mul(x[0], y[1]);
    let (r1, c2) = add(r1, p1, 0);
    let (p2, r3) = mul(x[0], y[2]);
    let (r2, c3) = add(r2, p2, c2);
    let (p3, r4) = mul(x[0], y[3]);
    let (r3, c4) = add(r3, p3, c3);
    let (r4, _c5) = add(r4, 0, c4);

    // x[1] *
    let (a, b) = mul(x[1], y[0]);
    let (r1, ca) = add(r1, a, 0);
    let (r2, cb) = add(r2, b, 0);
    let (a, b) = mul(x[1], y[1]);
    let (r2, ca) = add(r2, a, ca);
    let (r3, cb) = add(r3, b, cb);
    let (a, b) = mul(x[1], y[2]);
    let (r3, ca) = add(r3, a, ca);
    let (r4, cb) = add(r4, b, cb);
    let (a, r5) = mul(x[1], y[3]);
    let (r4, ca) = add(r4, a, ca);
    let (r5, _cb) = add(r5, 0, cb);
    let (r5, _ca) = add(r5, 0, ca);

    // ...
    let (r2, carry) = mac(r2, x[2], y[0], 0);
    let (r3, carry) = mac(r3, x[2], y[1], carry);
    let (r4, carry) = mac(r4, x[2], y[2], carry);
    let (r5, r6) = mac(r5, x[2], y[3], carry);
    let (r3, carry) = mac(r3, x[3], y[0], 0);
    let (r4, carry) = mac(r4, x[3], y[1], carry);
    let (r5, carry) = mac(r5, x[3], y[2], carry);
    let (r6, r7) = mac(r6, x[3], y[3], carry);
    (
        U256::from_limbs([r0, r1, r2, r3]),
        U256::from_limbs([r4, r5, r6, r7]),
    )
}
