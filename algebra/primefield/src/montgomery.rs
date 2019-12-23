// Names `from` and `to` are not very meaningful on their own
#![allow(clippy::module_name_repetitions)]
use crate::field::FieldElement;
use zkp_macros_decl::u256h;
use zkp_u256::{
    utils::{adc, mac, sbb},
    U256,
};

// TODO: Maybe move this file to U256?
// TODO: Make these `const fn` once https://github.com/rust-lang/rust/issues/49146 lands.

// M64 = -MODULUS^(-1) mod 2^64
const M64: u64 = 0xffff_ffff_ffff_ffff; // = -1

// R2 = 2^512 mod MODULUS
pub(crate) const R1: U256 =
    u256h!("07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1");
const R2: U256 = u256h!("07ffd4ab5e008810ffffffffff6f800000000001330ffffffffffd737e000401");
const R3: U256 = u256h!("038e5f79873c0a6df47d84f8363000187545706677ffcc06cc7177d1406df18e");

// We rebind variables for readability
#[allow(clippy::shadow_unrelated)]
pub(crate) fn to_montgomery_const(x: &U256) -> U256 {
    let k = x.limb(0).wrapping_mul(R2.limb(0)).wrapping_mul(M64);
    let (a0, carry) = mac(0, x.limb(0), R2.limb(0), 0);
    let (a1, carry) = mac(0, x.limb(0), R2.limb(1), carry);
    let (a2, carry) = mac(0, x.limb(0), R2.limb(2), carry);
    let (a3, carry) = mac(0, x.limb(0), R2.limb(3), carry);
    let a4 = carry;
    let (_, carry) = mac(a0, k, FieldElement::MODULUS.limb(0), 0);
    let (a0, carry) = mac(a1, k, FieldElement::MODULUS.limb(1), carry);
    let (a1, carry) = mac(a2, k, FieldElement::MODULUS.limb(2), carry);
    let (a2, carry) = mac(a3, k, FieldElement::MODULUS.limb(3), carry);
    let a3 = a4 + carry;
    let k = x
        .limb(1)
        .wrapping_mul(R2.limb(0))
        .wrapping_add(a0)
        .wrapping_mul(M64);
    let (a0, carry) = mac(a0, x.limb(1), R2.limb(0), 0);
    let (a1, carry) = mac(a1, x.limb(1), R2.limb(1), carry);
    let (a2, carry) = mac(a2, x.limb(1), R2.limb(2), carry);
    let (a3, carry) = mac(a3, x.limb(1), R2.limb(3), carry);
    let a4 = carry;
    let (_, carry) = mac(a0, k, FieldElement::MODULUS.limb(0), 0);
    let (a0, carry) = mac(a1, k, FieldElement::MODULUS.limb(1), carry);
    let (a1, carry) = mac(a2, k, FieldElement::MODULUS.limb(2), carry);
    let (a2, carry) = mac(a3, k, FieldElement::MODULUS.limb(3), carry);
    let a3 = a4 + carry;
    let k = x
        .limb(2)
        .wrapping_mul(R2.limb(0))
        .wrapping_add(a0)
        .wrapping_mul(M64);
    let (a0, carry) = mac(a0, x.limb(2), R2.limb(0), 0);
    let (a1, carry) = mac(a1, x.limb(2), R2.limb(1), carry);
    let (a2, carry) = mac(a2, x.limb(2), R2.limb(2), carry);
    let (a3, carry) = mac(a3, x.limb(2), R2.limb(3), carry);
    let a4 = carry;
    let (_, carry) = mac(a0, k, FieldElement::MODULUS.limb(0), 0);
    let (a0, carry) = mac(a1, k, FieldElement::MODULUS.limb(1), carry);
    let (a1, carry) = mac(a2, k, FieldElement::MODULUS.limb(2), carry);
    let (a2, carry) = mac(a3, k, FieldElement::MODULUS.limb(3), carry);
    let a3 = a4 + carry;
    let k = x
        .limb(3)
        .wrapping_mul(R2.limb(0))
        .wrapping_add(a0)
        .wrapping_mul(M64);
    let (a0, carry) = mac(a0, x.limb(3), R2.limb(0), 0);
    let (a1, carry) = mac(a1, x.limb(3), R2.limb(1), carry);
    let (a2, carry) = mac(a2, x.limb(3), R2.limb(2), carry);
    let (a3, carry) = mac(a3, x.limb(3), R2.limb(3), carry);
    let a4 = carry;
    let (_, carry) = mac(a0, k, FieldElement::MODULUS.limb(0), 0);
    let (a0, carry) = mac(a1, k, FieldElement::MODULUS.limb(1), carry);
    let (a1, carry) = mac(a2, k, FieldElement::MODULUS.limb(2), carry);
    let (a2, carry) = mac(a3, k, FieldElement::MODULUS.limb(3), carry);
    let a3 = a4 + carry;

    // The result (a0, a1, a2, a3) may be off by at most one modulus.
    // In a `const fn` we can not conditionally subtract, so instead
    // we always subtract
    let (a0, borrow) = sbb(a0, FieldElement::MODULUS.limb(0), 0);
    let (a1, borrow) = sbb(a1, FieldElement::MODULUS.limb(1), borrow);
    let (a2, borrow) = sbb(a2, FieldElement::MODULUS.limb(2), borrow);
    let (a3, borrow) = sbb(a3, FieldElement::MODULUS.limb(3), borrow);
    // Now we may have accidentally subtracted where we shouldn't.
    // If this is the case `borrow == 1` and else `borrow = 0`. We can
    // use  this to conditionally add back a modulus.
    let (a0, carry) = adc(a0, borrow * FieldElement::MODULUS.limb(0), 0);
    let (a1, carry) = adc(a1, borrow * FieldElement::MODULUS.limb(1), carry);
    let (a2, carry) = adc(a2, borrow * FieldElement::MODULUS.limb(2), carry);
    let (a3, _) = adc(a3, borrow * FieldElement::MODULUS.limb(3), carry);
    // Return the now reduced result
    U256::from_limbs(a0, a1, a2, a3)
}

// We rebind variables for readability
#[allow(clippy::shadow_unrelated)]
pub(crate) fn redc(lo: &U256, hi: &U256) -> U256 {
    // Algorithm 14.32 from Handbook of Applied Cryptography.
    // TODO: Optimize for the specific values of M64 and MODULUS.
    let ui = lo.limb(0).wrapping_mul(M64);
    let (_a0, carry) = mac(lo.limb(0), ui, FieldElement::MODULUS.limb(0), 0);
    let (a1, carry) = mac(lo.limb(1), ui, FieldElement::MODULUS.limb(1), carry);
    let (a2, carry) = mac(lo.limb(2), ui, FieldElement::MODULUS.limb(2), carry);
    let (a3, carry) = mac(lo.limb(3), ui, FieldElement::MODULUS.limb(3), carry);
    let (a4, carry2) = adc(hi.limb(0), 0, carry);
    let ui = a1.wrapping_mul(M64);
    let (_a1, carry) = mac(a1, ui, FieldElement::MODULUS.limb(0), 0);
    let (a2, carry) = mac(a2, ui, FieldElement::MODULUS.limb(1), carry);
    let (a3, carry) = mac(a3, ui, FieldElement::MODULUS.limb(2), carry);
    let (a4, carry) = mac(a4, ui, FieldElement::MODULUS.limb(3), carry);
    let (a5, carry2) = adc(hi.limb(1), carry2, carry);
    let ui = a2.wrapping_mul(M64);
    let (_a2, carry) = mac(a2, ui, FieldElement::MODULUS.limb(0), 0);
    let (a3, carry) = mac(a3, ui, FieldElement::MODULUS.limb(1), carry);
    let (a4, carry) = mac(a4, ui, FieldElement::MODULUS.limb(2), carry);
    let (a5, carry) = mac(a5, ui, FieldElement::MODULUS.limb(3), carry);
    let (a6, carry2) = adc(hi.limb(2), carry2, carry);
    let ui = a3.wrapping_mul(M64);
    let (_a3, carry) = mac(a3, ui, FieldElement::MODULUS.limb(0), 0);
    let (a4, carry) = mac(a4, ui, FieldElement::MODULUS.limb(1), carry);
    let (a5, carry) = mac(a5, ui, FieldElement::MODULUS.limb(2), carry);
    let (a6, carry) = mac(a6, ui, FieldElement::MODULUS.limb(3), carry);
    let (a7, _) = adc(hi.limb(3), carry2, carry);

    // Final reduction
    let mut r = U256::from_limbs(a4, a5, a6, a7);
    if r >= FieldElement::MODULUS {
        r -= &FieldElement::MODULUS;
    }
    r
}

// We rebind variables for readability
#[allow(clippy::shadow_unrelated)]
pub(crate) fn mul_redc(x: &U256, y: &U256) -> U256 {
    // TODO: This might not be faster than:
    // let (lo, hi) = x.mul_full(y);
    // return redc(&lo, &hi);
    let k = x.limb(0).wrapping_mul(y.limb(0)).wrapping_mul(M64);
    let (a0, carry) = mac(0, x.limb(0), y.limb(0), 0);
    let (a1, carry) = mac(0, x.limb(0), y.limb(1), carry);
    let (a2, carry) = mac(0, x.limb(0), y.limb(2), carry);
    let (a3, carry) = mac(0, x.limb(0), y.limb(3), carry);
    let a4 = carry;
    let (_a, carry) = mac(a0, k, FieldElement::MODULUS.limb(0), 0);
    let (a0, carry) = mac(a1, k, FieldElement::MODULUS.limb(1), carry);
    let (a1, carry) = mac(a2, k, FieldElement::MODULUS.limb(2), carry);
    let (a2, carry) = mac(a3, k, FieldElement::MODULUS.limb(3), carry);
    let a3 = a4 + carry;
    let k = x
        .limb(1)
        .wrapping_mul(y.limb(0))
        .wrapping_add(a0)
        .wrapping_mul(M64);
    let (a0, carry) = mac(a0, x.limb(1), y.limb(0), 0);
    let (a1, carry) = mac(a1, x.limb(1), y.limb(1), carry);
    let (a2, carry) = mac(a2, x.limb(1), y.limb(2), carry);
    let (a3, carry) = mac(a3, x.limb(1), y.limb(3), carry);
    let a4 = carry;
    let (_a, carry) = mac(a0, k, FieldElement::MODULUS.limb(0), 0);
    let (a0, carry) = mac(a1, k, FieldElement::MODULUS.limb(1), carry);
    let (a1, carry) = mac(a2, k, FieldElement::MODULUS.limb(2), carry);
    let (a2, carry) = mac(a3, k, FieldElement::MODULUS.limb(3), carry);
    let a3 = a4 + carry;
    let k = x
        .limb(2)
        .wrapping_mul(y.limb(0))
        .wrapping_add(a0)
        .wrapping_mul(M64);
    let (a0, carry) = mac(a0, x.limb(2), y.limb(0), 0);
    let (a1, carry) = mac(a1, x.limb(2), y.limb(1), carry);
    let (a2, carry) = mac(a2, x.limb(2), y.limb(2), carry);
    let (a3, carry) = mac(a3, x.limb(2), y.limb(3), carry);
    let a4 = carry;
    let (_a, carry) = mac(a0, k, FieldElement::MODULUS.limb(0), 0);
    let (a0, carry) = mac(a1, k, FieldElement::MODULUS.limb(1), carry);
    let (a1, carry) = mac(a2, k, FieldElement::MODULUS.limb(2), carry);
    let (a2, carry) = mac(a3, k, FieldElement::MODULUS.limb(3), carry);
    let a3 = a4 + carry;
    let k = x
        .limb(3)
        .wrapping_mul(y.limb(0))
        .wrapping_add(a0)
        .wrapping_mul(M64);
    let (a0, carry) = mac(a0, x.limb(3), y.limb(0), 0);
    let (a1, carry) = mac(a1, x.limb(3), y.limb(1), carry);
    let (a2, carry) = mac(a2, x.limb(3), y.limb(2), carry);
    let (a3, carry) = mac(a3, x.limb(3), y.limb(3), carry);
    let a4 = carry;
    let (_a, carry) = mac(a0, k, FieldElement::MODULUS.limb(0), 0);
    let (a0, carry) = mac(a1, k, FieldElement::MODULUS.limb(1), carry);
    let (a1, carry) = mac(a2, k, FieldElement::MODULUS.limb(2), carry);
    let (a2, carry) = mac(a3, k, FieldElement::MODULUS.limb(3), carry);
    let a3 = a4 + carry;

    // Final reduction
    let mut r = U256::from_limbs(a0, a1, a2, a3);
    if r >= FieldElement::MODULUS {
        r -= &FieldElement::MODULUS;
    }
    r
}

pub(crate) fn sqr_redc(a: &U256) -> U256 {
    let (lo, hi) = a.sqr_full();
    redc(&lo, &hi)
}

pub(crate) fn inv_redc(n: &U256) -> Option<U256> {
    n.invmod(&FieldElement::MODULUS)
        .map(|ni| mul_redc(&ni, &R3))
}

pub(crate) fn to_montgomery(n: &U256) -> U256 {
    mul_redc(n, &R2)
}

pub(crate) fn from_montgomery(n: &U256) -> U256 {
    redc(n, &U256::ZERO)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_redc() {
        let a = u256h!("0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c");
        let b = u256h!("024385f6bebc1c496e09955db534ef4b1eaff9a78e27d4093cfa8f7c8f886f6b");
        let c = u256h!("012e440f0965e7029c218b64f1010006b5c4ba8b1497c4174a32fec025c197bc");
        assert_eq!(redc(&a, &b), c);
    }

    #[test]
    fn test_mul_redc() {
        let a = u256h!("0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c");
        let b = u256h!("024385f6bebc1c496e09955db534ef4b1eaff9a78e27d4093cfa8f7c8f886f6b");
        let c = u256h!("012b854fc6321976d374ad069cfdec8bb7b2bd184259dae8f530cbb28f0805b4");
        assert_eq!(mul_redc(&a, &b), c);
    }

    #[quickcheck]
    fn test_to_from(mut n: U256) -> bool {
        n %= FieldElement::MODULUS;
        from_montgomery(&to_montgomery(&n)) == n
    }
}
