use crate::{MontgomeryParameters, SquareFullInline, U256};

mod generic;
mod proth;

// TODO: Special algorithms for Solinas and Crandall primes
// <https://en.wikipedia.org/wiki/Solinas_prime>

// False positive, we re-export the function.
#[allow(unreachable_pub)]
pub use generic::to_montgomery_const;

#[inline(always)]
pub(crate) fn redc_inline<M: MontgomeryParameters<UInt = U256>>(lo: &U256, hi: &U256) -> U256 {
    // Select the best algorithm, the branch should be resolved compile time.
    // TODO: Make compile time constant.
    if proth::is_proth::<M>() {
        proth::redc_inline(M::MODULUS.limb(3), lo, hi)
    } else {
        generic::redc_inline::<M>(lo, hi)
    }
}

#[inline(always)]
pub(crate) fn square_redc_inline<M: MontgomeryParameters<UInt = U256>>(x: &U256) -> U256 {
    // OPT: Dedicated implementations, optimized for special primes
    let (lo, hi) = x.square_full_inline();
    redc_inline::<M>(&lo, &hi)
}

#[inline(always)]
pub(crate) fn mul_redc_inline<M: MontgomeryParameters<UInt = U256>>(x: &U256, y: &U256) -> U256 {
    if proth::is_proth::<M>() {
        proth::mul_redc_inline(M::MODULUS.limb(3), x, y)
    } else {
        generic::mul_redc_inline::<M>(x, y)
    }
}

// Quickcheck requires pass-by-value
#[allow(clippy::needless_pass_by_value)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::MontgomeryParameters;
    use zkp_macros_decl::u256h;

    struct PrimeField();

    // TODO: Non Proth example
    impl MontgomeryParameters for PrimeField {
        type UInt = U256;

        const M64: u64 = 0xffff_ffff_ffff_ffff;
        const MODULUS: U256 =
            u256h!("0800000000000011000000000000000000000000000000000000000000000001");
        const R1: U256 = u256h!("07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1");
        const R2: U256 = u256h!("07ffd4ab5e008810ffffffffff6f800000000001330ffffffffffd737e000401");
        const R3: U256 = u256h!("038e5f79873c0a6df47d84f8363000187545706677ffcc06cc7177d1406df18e");
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

    // TODO
    // #[quickcheck]
    // fn test_to_from(mut n: U256) -> bool {
    // n %= PrimeField::MODULUS;
    // from_montgomery::<PrimeField>(&to_montgomery::<PrimeField>(&n)) == n
    // }
    //
    // #[quickcheck]
    // fn test_mulmod(a: U256, b: U256) -> bool {
    // let r = mulmod::<PrimeField>(&a, &b);
    // let e = a.mulmod(&b, &PrimeField::MODULUS);
    // r == e
    // }
}
