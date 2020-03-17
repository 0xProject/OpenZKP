use crate::{
    algorithms::limb_operations::{adc, mac, sbb},
    MontgomeryParameters, U256,
};

// See https://hackmd.io/7PFyv-itRBa0a0nYCAklmA?both
// See https://eprint.iacr.org/2012/309.pdf
// See https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/modmul_no_precomp.pdf
// See https://pdfs.semanticscholar.org/c751/a321dd430ebbcfb4dcce1f86f88256e0af5a.pdf

// TODO: Make const fn
pub(crate) fn is_proth<M: MontgomeryParameters<UInt = U256>>() -> bool {
    let modulus = M::MODULUS.as_limbs();
    modulus[0] == 1 && modulus[1] == 0 && modulus[2] == 0
}

// This is algorithm 14.32 optimized for the facts that
//   m_0 = 1. m_1 =0, m_2 = 0, m' = -1
// We rebind variables for readability
#[allow(clippy::shadow_unrelated)]
#[inline(always)]
pub(crate) fn redc_inline(m3: u64, lo: &U256, hi: &U256) -> U256 {
    let lo = lo.as_limbs();
    let hi = hi.as_limbs();

    let (a0, carry0) = sbb(0, lo[0], 0);
    let (a1, carry0) = sbb(0, lo[1], carry0);
    let (a2, carry0) = sbb(0, lo[2], carry0);
    let (a3, carry1) = mac(lo[3], a0, m3, 0);
    let (a3, carry0) = sbb(0, a3, carry0);

    let (a4, carry0) = adc(0, hi[0], carry0);
    let (a5, carry0) = adc(0, hi[1], carry0);
    let (a6, carry0) = adc(0, hi[2], carry0);
    let (a7, _carry0) = adc(0, hi[3], carry0);

    let (a4, carry1) = mac(a4, a1, m3, carry1);
    let (a5, carry1) = mac(a5, a2, m3, carry1);
    let (a6, carry1) = mac(a6, a3, m3, carry1);
    let (a7, _carry1) = adc(a7, 0, carry1);

    // Final reduction
    let (r0, carry) = sbb(a4, 1, 0);
    let (r1, carry) = sbb(a5, 0, carry);
    let (r2, carry) = sbb(a6, 0, carry);
    let (r3, carry) = sbb(a7, m3, carry);
    if carry != 0 {
        U256::from_limbs([a4, a5, a6, a7])
    } else {
        U256::from_limbs([r0, r1, r2, r3])
    }
}

// We rebind variables for readability
#[allow(clippy::shadow_unrelated)]
#[inline(always)]
pub(crate) fn mul_redc_inline(m3: u64, x: &U256, y: &U256) -> U256 {
    let x = x.as_limbs();
    let y = y.as_limbs();

    let (a0, carry) = mac(0, x[0], y[0], 0);
    let (a1, carry) = mac(0, x[0], y[1], carry);
    let (a2, carry) = mac(0, x[0], y[2], carry);
    let (a3, carry) = mac(0, x[0], y[3], carry);
    let a4 = carry;
    let (k, carry) = sbb(0, a0, 0);
    let (a3, carry1) = mac(a3, k, m3, 0);
    let (a1, carry) = mac(a1, x[1], y[0], carry);
    let (a2, carry) = mac(a2, x[1], y[1], carry);
    let (a3, carry) = mac(a3, x[1], y[2], carry);
    let (a4, carry) = mac(a4, x[1], y[3], carry);
    let a5 = carry;
    let (k, carry) = sbb(0, a1, 0);
    let (a4, carry1) = mac(a4, k, m3, carry1);
    let (a2, carry) = mac(a2, x[2], y[0], carry);
    let (a3, carry) = mac(a3, x[2], y[1], carry);
    let (a4, carry) = mac(a4, x[2], y[2], carry);
    let (a5, carry) = mac(a5, x[2], y[3], carry);
    let a6 = carry;
    let (k, carry) = sbb(0, a2, 0);
    let (a5, carry1) = mac(a5, k, m3, carry1);
    let (a3, carry) = mac(a3, x[3], y[0], carry);
    let (a4, carry) = mac(a4, x[3], y[1], carry);
    let (a5, carry) = mac(a5, x[3], y[2], carry);
    let (a6, carry) = mac(a6, x[3], y[3], carry);
    let a7 = carry;
    let (k, carry) = sbb(0, a3, 0);
    let (a6, carry1) = adc(a6, 0, carry1);
    let (a4, carry) = adc(a4, 0, carry);
    let (a5, carry) = adc(a5, 0, carry);
    let (a6, carry) = mac(a6, k, m3, carry);
    let a7 = a7 + carry + carry1;

    // Final reduction
    let mut r = U256::from_limbs([a4, a5, a6, a7]);
    if r >= U256::from_limbs([1, 0, 0, m3]) {
        r -= U256::from_limbs([1, 0, 0, m3]);
    }
    r
}

// Quickcheck requires pass-by-value
#[allow(clippy::needless_pass_by_value)]
#[cfg(test)]
mod tests {
    use super::{super::generic, *};
    use crate::MontgomeryParameters;
    use quickcheck_macros::quickcheck;
    use zkp_macros_decl::u256h;

    struct PrimeField();

    const M3: u64 = 0x0800_0000_0000_0011;

    impl MontgomeryParameters for PrimeField {
        type UInt = U256;

        const M64: u64 = 0xffff_ffff_ffff_ffff;
        const MODULUS: U256 = U256::from_limbs([1, 0, 0, M3]);
        const R1: U256 = u256h!("07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1");
        const R2: U256 = u256h!("07ffd4ab5e008810ffffffffff6f800000000001330ffffffffffd737e000401");
        const R3: U256 = u256h!("038e5f79873c0a6df47d84f8363000187545706677ffcc06cc7177d1406df18e");
    }

    #[test]
    fn test_redc() {
        let a = u256h!("0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c");
        let b = u256h!("024385f6bebc1c496e09955db534ef4b1eaff9a78e27d4093cfa8f7c8f886f6b");
        let c = u256h!("012e440f0965e7029c218b64f1010006b5c4ba8b1497c4174a32fec025c197bc");
        assert_eq!(redc_inline(M3, &a, &b), c);
    }

    #[test]
    fn test_mul_redc() {
        let a = u256h!("0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c");
        let b = u256h!("024385f6bebc1c496e09955db534ef4b1eaff9a78e27d4093cfa8f7c8f886f6b");
        let c = u256h!("012b854fc6321976d374ad069cfdec8bb7b2bd184259dae8f530cbb28f0805b4");
        assert_eq!(mul_redc_inline(M3, &a, &b), c);
    }

    #[quickcheck]
    fn test_redc_generic_consistent(lo: U256, hi: U256) -> bool {
        if hi >= PrimeField::MODULUS {
            return true;
        }
        let result = redc_inline(M3, &lo, &hi);
        let expected = generic::redc_inline::<PrimeField>(&lo, &hi);
        result == expected
    }

    #[quickcheck]
    fn test_mul_redc_generic_consistent(x: U256, y: U256) -> bool {
        if x >= PrimeField::MODULUS {
            return true;
        }
        if y >= PrimeField::MODULUS {
            return true;
        }
        let result = mul_redc_inline(M3, &x, &y);
        let expected = generic::mul_redc_inline::<PrimeField>(&x, &y);
        result == expected
    }
}
