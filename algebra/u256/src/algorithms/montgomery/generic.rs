use crate::{
    algorithms::limb_operations::{adc, mac, sbb},
    MontgomeryParameters, U256,
};

/// Slow but compile time constant version of `to_montgomery`.
// We rebind variables for readability
#[allow(clippy::shadow_unrelated)]
pub const fn to_montgomery_const(x: &U256, modulus: &U256, m64: u64, r2: &U256) -> U256 {
    let x = x.as_limbs();
    let r2 = r2.as_limbs();
    let modulus = modulus.as_limbs();
    let k = x[0].wrapping_mul(r2[0]).wrapping_mul(m64);
    let (a0, carry) = mac(0, x[0], r2[0], 0);
    let (a1, carry) = mac(0, x[0], r2[1], carry);
    let (a2, carry) = mac(0, x[0], r2[2], carry);
    let (a3, carry) = mac(0, x[0], r2[3], carry);
    let a4 = carry;
    let (_, carry) = mac(a0, k, modulus[0], 0);
    let (a0, carry) = mac(a1, k, modulus[1], carry);
    let (a1, carry) = mac(a2, k, modulus[2], carry);
    let (a2, carry) = mac(a3, k, modulus[3], carry);
    let a3 = a4 + carry;
    let k = x[1].wrapping_mul(r2[0]).wrapping_add(a0).wrapping_mul(m64);
    let (a0, carry) = mac(a0, x[1], r2[0], 0);
    let (a1, carry) = mac(a1, x[1], r2[1], carry);
    let (a2, carry) = mac(a2, x[1], r2[2], carry);
    let (a3, carry) = mac(a3, x[1], r2[3], carry);
    let a4 = carry;
    let (_, carry) = mac(a0, k, modulus[0], 0);
    let (a0, carry) = mac(a1, k, modulus[1], carry);
    let (a1, carry) = mac(a2, k, modulus[2], carry);
    let (a2, carry) = mac(a3, k, modulus[3], carry);
    let a3 = a4 + carry;
    let k = x[2].wrapping_mul(r2[0]).wrapping_add(a0).wrapping_mul(m64);
    let (a0, carry) = mac(a0, x[2], r2[0], 0);
    let (a1, carry) = mac(a1, x[2], r2[1], carry);
    let (a2, carry) = mac(a2, x[2], r2[2], carry);
    let (a3, carry) = mac(a3, x[2], r2[3], carry);
    let a4 = carry;
    let (_, carry) = mac(a0, k, modulus[0], 0);
    let (a0, carry) = mac(a1, k, modulus[1], carry);
    let (a1, carry) = mac(a2, k, modulus[2], carry);
    let (a2, carry) = mac(a3, k, modulus[3], carry);
    let a3 = a4 + carry;
    let k = x[3].wrapping_mul(r2[0]).wrapping_add(a0).wrapping_mul(m64);
    let (a0, carry) = mac(a0, x[3], r2[0], 0);
    let (a1, carry) = mac(a1, x[3], r2[1], carry);
    let (a2, carry) = mac(a2, x[3], r2[2], carry);
    let (a3, carry) = mac(a3, x[3], r2[3], carry);
    let a4 = carry;
    let (_, carry) = mac(a0, k, modulus[0], 0);
    let (a0, carry) = mac(a1, k, modulus[1], carry);
    let (a1, carry) = mac(a2, k, modulus[2], carry);
    let (a2, carry) = mac(a3, k, modulus[3], carry);
    let a3 = a4 + carry;

    // The result (a0, a1, a2, a3) may be off by at most one modulus.
    // In a `const fn` we can not conditionally subtract, so instead
    // we always subtract
    let (a0, borrow) = sbb(a0, modulus[0], 0);
    let (a1, borrow) = sbb(a1, modulus[1], borrow);
    let (a2, borrow) = sbb(a2, modulus[2], borrow);
    let (a3, borrow) = sbb(a3, modulus[3], borrow);
    // Now we may have accidentally subtracted where we shouldn't.
    // If this is the case `borrow == 1` and else `borrow = 0`. We can
    // use  this to conditionally add back a modulus.
    let (a0, carry) = adc(a0, borrow * modulus[0], 0);
    let (a1, carry) = adc(a1, borrow * modulus[1], carry);
    let (a2, carry) = adc(a2, borrow * modulus[2], carry);
    let (a3, _) = adc(a3, borrow * modulus[3], carry);
    // Return the now reduced result
    U256::from_limbs([a0, a1, a2, a3])
}

// We rebind variables for readability
#[allow(clippy::shadow_unrelated)]
#[inline(always)]
pub(crate) fn redc_inline<M: MontgomeryParameters<UInt = U256>>(lo: &U256, hi: &U256) -> U256 {
    let modulus = M::MODULUS.as_limbs();
    // Algorithm 14.32 from Handbook of Applied Cryptography.
    // TODO: Optimize for the specific values of M64 and MODULUS.
    let ui = lo.limb(0).wrapping_mul(M::M64);
    let (_a0, carry) = mac(lo.limb(0), ui, modulus[0], 0);
    let (a1, carry) = mac(lo.limb(1), ui, modulus[1], carry);
    let (a2, carry) = mac(lo.limb(2), ui, modulus[2], carry);
    let (a3, carry) = mac(lo.limb(3), ui, modulus[3], carry);
    let (a4, carry2) = adc(hi.limb(0), 0, carry);
    let ui = a1.wrapping_mul(M::M64);
    let (_a1, carry) = mac(a1, ui, modulus[0], 0);
    let (a2, carry) = mac(a2, ui, modulus[1], carry);
    let (a3, carry) = mac(a3, ui, modulus[2], carry);
    let (a4, carry) = mac(a4, ui, modulus[3], carry);
    let (a5, carry2) = adc(hi.limb(1), carry2, carry);
    let ui = a2.wrapping_mul(M::M64);
    let (_a2, carry) = mac(a2, ui, modulus[0], 0);
    let (a3, carry) = mac(a3, ui, modulus[1], carry);
    let (a4, carry) = mac(a4, ui, modulus[2], carry);
    let (a5, carry) = mac(a5, ui, modulus[3], carry);
    let (a6, carry2) = adc(hi.limb(2), carry2, carry);
    let ui = a3.wrapping_mul(M::M64);
    let (_a3, carry) = mac(a3, ui, modulus[0], 0);
    let (a4, carry) = mac(a4, ui, modulus[1], carry);
    let (a5, carry) = mac(a5, ui, modulus[2], carry);
    let (a6, carry) = mac(a6, ui, modulus[3], carry);
    let (a7, _) = adc(hi.limb(3), carry2, carry);

    // Final reduction
    let mut r = U256::from_limbs([a4, a5, a6, a7]);
    if r >= M::MODULUS {
        r -= &M::MODULUS;
    }
    r
}

// We rebind variables for readability
#[allow(clippy::shadow_unrelated)]
#[inline(always)]
pub(crate) fn mul_redc_inline<M: MontgomeryParameters<UInt = U256>>(x: &U256, y: &U256) -> U256 {
    let x = x.as_limbs();
    let modulus = M::MODULUS.as_limbs();

    let (a0, carry) = mac(0, x[0], y.limb(0), 0);
    let (a1, carry) = mac(0, x[0], y.limb(1), carry);
    let (a2, carry) = mac(0, x[0], y.limb(2), carry);
    let (a3, carry) = mac(0, x[0], y.limb(3), carry);
    let a4 = carry;
    let k = a0.wrapping_mul(M::M64);
    let (_a, carry) = mac(a0, k, modulus[0], 0);
    let (a0, carry) = mac(a1, k, modulus[1], carry);
    let (a1, carry) = mac(a2, k, modulus[2], carry);
    let (a2, carry) = mac(a3, k, modulus[3], carry);
    let a3 = a4 + carry;
    let (a0, carry) = mac(a0, x[1], y.limb(0), 0);
    let (a1, carry) = mac(a1, x[1], y.limb(1), carry);
    let (a2, carry) = mac(a2, x[1], y.limb(2), carry);
    let (a3, carry) = mac(a3, x[1], y.limb(3), carry);
    let a4 = carry;
    let k = a0.wrapping_mul(M::M64);
    let (_a, carry) = mac(a0, k, modulus[0], 0);
    let (a0, carry) = mac(a1, k, modulus[1], carry);
    let (a1, carry) = mac(a2, k, modulus[2], carry);
    let (a2, carry) = mac(a3, k, modulus[3], carry);
    let a3 = a4 + carry;
    let (a0, carry) = mac(a0, x[2], y.limb(0), 0);
    let (a1, carry) = mac(a1, x[2], y.limb(1), carry);
    let (a2, carry) = mac(a2, x[2], y.limb(2), carry);
    let (a3, carry) = mac(a3, x[2], y.limb(3), carry);
    let a4 = carry;
    let k = a0.wrapping_mul(M::M64);
    let (_a, carry) = mac(a0, k, modulus[0], 0);
    let (a0, carry) = mac(a1, k, modulus[1], carry);
    let (a1, carry) = mac(a2, k, modulus[2], carry);
    let (a2, carry) = mac(a3, k, modulus[3], carry);
    let a3 = a4 + carry;
    let (a0, carry) = mac(a0, x[3], y.limb(0), 0);
    let (a1, carry) = mac(a1, x[3], y.limb(1), carry);
    let (a2, carry) = mac(a2, x[3], y.limb(2), carry);
    let (a3, carry) = mac(a3, x[3], y.limb(3), carry);
    let a4 = carry;
    let k = a0.wrapping_mul(M::M64);
    let (_a, carry) = mac(a0, k, modulus[0], 0);
    let (a0, carry) = mac(a1, k, modulus[1], carry);
    let (a1, carry) = mac(a2, k, modulus[2], carry);
    let (a2, carry) = mac(a3, k, modulus[3], carry);
    let a3 = a4 + carry;

    // Final reduction
    let mut r = U256::from_limbs([a0, a1, a2, a3]);
    if r >= M::MODULUS {
        r -= &M::MODULUS;
    }
    r
}

// Quickcheck requires pass-by-value
#[allow(clippy::needless_pass_by_value)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::MontgomeryParameters;
    use quickcheck_macros::quickcheck;
    use zkp_macros_decl::u256h;

    struct PrimeField();

    // TODO: Non-proth prime
    // TODO: Test for small and big primes
    impl MontgomeryParameters for PrimeField {
        type UInt = U256;

        const M64: u64 = 0xffff_ffff_ffff_ffff;
        const MODULUS: U256 =
            u256h!("0800000000000011000000000000000000000000000000000000000000000001");
        // = -1
        const R1: U256 = u256h!("07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1");
        const R2: U256 = u256h!("07ffd4ab5e008810ffffffffff6f800000000001330ffffffffffd737e000401");
        const R3: U256 = u256h!("038e5f79873c0a6df47d84f8363000187545706677ffcc06cc7177d1406df18e");
    }

    #[quickcheck]
    fn test_to_montgomery_const_consistent(n: U256) {
        let result =
            to_montgomery_const(&n, &PrimeField::MODULUS, PrimeField::M64, &PrimeField::R2);
        let expected = mul_redc_inline::<PrimeField>(&n, &PrimeField::R2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_redc() {
        let a = u256h!("0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c");
        let b = u256h!("024385f6bebc1c496e09955db534ef4b1eaff9a78e27d4093cfa8f7c8f886f6b");
        let c = u256h!("012e440f0965e7029c218b64f1010006b5c4ba8b1497c4174a32fec025c197bc");
        assert_eq!(redc_inline::<PrimeField>(&a, &b), c);
    }

    #[test]
    fn test_mul_redc() {
        let a = u256h!("0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c");
        let b = u256h!("024385f6bebc1c496e09955db534ef4b1eaff9a78e27d4093cfa8f7c8f886f6b");
        let c = u256h!("012b854fc6321976d374ad069cfdec8bb7b2bd184259dae8f530cbb28f0805b4");
        assert_eq!(mul_redc_inline::<PrimeField>(&a, &b), c);
    }
}
