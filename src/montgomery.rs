use crate::field::MODULUS;
use crate::u256::U256;
use crate::u256h;
use crate::utils::{adc, mac};
use hex_literal::*;

// M64 = -MODULUS^(-1) mod 2^64
pub const M64: u64 = 0xffff_ffff_ffff_ffff; // = -1

// R2 = 2^512 mod MODULUS
pub const R1: U256 = u256h!("07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1");
pub const R2: U256 = u256h!("07ffd4ab5e008810ffffffffff6f800000000001330ffffffffffd737e000401");
pub const R3: U256 = u256h!("038e5f79873c0a6df47d84f8363000187545706677ffcc06cc7177d1406df18e");

// TODO: Make const fn
#[inline(always)]
pub fn redc(lo: &U256, hi: &U256) -> U256 {
    // Algorithm 14.32 from Handbook of Applied Cryptography.
    // TODO: Optimize for the specific values of M64 and MODULUS.
    let ui = lo.c0.wrapping_mul(M64);
    let (_a0, carry) = mac(lo.c0, ui, MODULUS.c0, 0);
    let (a1, carry) = mac(lo.c1, ui, MODULUS.c1, carry);
    let (a2, carry) = mac(lo.c2, ui, MODULUS.c2, carry);
    let (a3, carry) = mac(lo.c3, ui, MODULUS.c3, carry);
    let (a4, carry) = adc(hi.c0, 0, carry);
    let (a5, carry) = adc(hi.c1, 0, carry);
    let (a6, carry) = adc(hi.c2, 0, carry);
    let (a7, _carry) = adc(hi.c3, 0, carry);
    let ui = a1.wrapping_mul(M64);
    let (_a1, carry) = mac(a1, ui, MODULUS.c0, 0);
    let (a2, carry) = mac(a2, ui, MODULUS.c1, carry);
    let (a3, carry) = mac(a3, ui, MODULUS.c2, carry);
    let (a4, carry) = mac(a4, ui, MODULUS.c3, carry);
    let (a5, carry) = adc(a5, 0, carry);
    let (a6, carry) = adc(a6, 0, carry);
    let (a7, _carry) = adc(a7, 0, carry);
    let ui = a2.wrapping_mul(M64);
    let (_a2, carry) = mac(a2, ui, MODULUS.c0, 0);
    let (a3, carry) = mac(a3, ui, MODULUS.c1, carry);
    let (a4, carry) = mac(a4, ui, MODULUS.c2, carry);
    let (a5, carry) = mac(a5, ui, MODULUS.c3, carry);
    let (a6, carry) = adc(a6, 0, carry);
    let (a7, _carry) = adc(a7, 0, carry);
    let ui = a3.wrapping_mul(M64);
    let (_a3, carry) = mac(a3, ui, MODULUS.c0, carry);
    let (a4, carry) = mac(a4, ui, MODULUS.c1, carry);
    let (a5, carry) = mac(a5, ui, MODULUS.c2, carry);
    let (a6, carry) = mac(a6, ui, MODULUS.c3, carry);
    let (a7, _carry) = adc(a7, 0, carry);

    // Final reduction
    let mut r = U256::new(a4, a5, a6, a7);
    if r >= MODULUS {
        r -= &MODULUS;
    }
    r
}

pub fn mul_redc(a: &U256, b: &U256) -> U256 {
    // TODO: Algorithm 14.36 from Handbook of Applied Cryptography
    let (lo, hi) = a.mul_full(b);
    redc(&lo, &hi)
}

pub fn sqr_redc(a: &U256) -> U256 {
    // TODO: special case
    mul_redc(&a, &a)
}

pub fn inv_redc(n: &U256) -> U256 {
    mul_redc(&n.invmod(&MODULUS).unwrap(), &R3)
}

pub fn to_montgomery(n: &U256) -> U256 {
    mul_redc(n, &R2)
}

pub fn from_montgomery(n: &U256) -> U256 {
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
}
