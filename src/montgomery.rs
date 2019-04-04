use crate::field::MODULUS;
use crate::u256::U256;
use crate::u256h;
use crate::utils::{adc, mac};
use hex_literal::*;

// Min = -MODULUS^(-1) mod 2^256
pub const Min: U256 = u256h!("0800000000000010ffffffffffffffffffffffffffffffffffffffffffffffff");

// M64 = -MODULUS^(-1) mod 2^64
// TODO: Optimize for it being -1
pub const M64: u64 = 0xffff_ffff_ffff_ffff; // = -1

// R2 = 2^512 mod MODULUS
pub const R1: U256 = u256h!("07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1");
pub const R2: U256 = u256h!("07ffd4ab5e008810ffffffffff6f800000000001330ffffffffffd737e000401");
pub const R3: U256 = u256h!("038e5f79873c0a6df47d84f8363000187545706677ffcc06cc7177d1406df18e");

// TODO: make const
pub fn redc(lo: &U256, hi: &U256) -> U256 {
    // Algorithm 14.32 from Handbook of Applied Cryptography.
    let mut a = [lo.c0, lo.c1, lo.c2, lo.c3, hi.c0, hi.c1, hi.c2, hi.c3];
    let m = [MODULUS.c0, MODULUS.c1, MODULUS.c2, MODULUS.c3];
    for i in 0..4 {
        let ui = a[i].wrapping_mul(M64);
        let mut carry = 0;
        for j in 0..4 {
            let (ai, c) = mac(a[i + j], ui, m[j], carry);
            a[i + j] = ai;
            carry = c;
        }
        for j in (i + 4)..8 {
            let (ai, c) = adc(a[j], 0, carry);
            a[j] = ai;
            carry = c;
        }
        //debug_assert!(carry == 0);
        //debug_assert!(a[i] == 0);
    }
    let mut r = U256::new(a[4], a[5], a[6], a[7]);
    if r >= MODULUS {
        r -= &MODULUS;
    }
    r
}

pub fn mul_redc(a: &U256, b: &U256) -> U256 {
    // TODO: Algorithm 14.36
    let (lo, hi) = a.mul_full(b);
    redc(&lo, &hi)
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
