use crate::U256;

mod generic;
pub mod proth;

// TODO: Special algorithms for Solinas and Crandall primes
// <https://en.wikipedia.org/wiki/Solinas_prime>

// TODO: Provide methods to compute parameters from Modulus
// tricks from <https://medium.com/wicketh/mathemagic-512-bit-division-in-solidity-afa55870a65>
// can help here. Extra credit: make it a `const fn`.
pub trait Parameters {
    /// The modulus to implement in Montgomery form
    const MODULUS: U256;

    /// M64 = -MODULUS^(-1) mod 2^64
    const M64: u64;

    // R1 = 2^256 mod MODULUS
    const R1: U256;

    // R2 = 2^512 mod MODULUS
    const R2: U256;

    // R3 = 2^768 mod MODULUS
    const R3: U256;
}

/// Slow but compile time constant version of `to_montgomery`.
pub use generic::to_montgomery_const;

#[allow(clippy::module_name_repetitions)]
pub fn to_montgomery<M: Parameters>(n: &U256) -> U256 {
    mul_redc::<M>(n, &M::R2)
}

#[allow(clippy::module_name_repetitions)]
pub fn from_montgomery<M: Parameters>(n: &U256) -> U256 {
    redc::<M>(n, &U256::ZERO)
}

/// Multiply two numbers in non-Montgomery form.
///
/// Combined `to_montgomery`, `mul_redc`, and `from_montgomery`.
///
/// Normally this would require four `mul_redc` operations, but two
/// of them cancel out, making this an efficient way to do a single
/// modular multiplication.
///
/// # Requirements
/// Inputs are required to be reduced modulo `M::MODULUS`.
pub fn mulmod<M: Parameters>(a: &U256, b: &U256) -> U256 {
    // TODO: Is this faster than barret reduction?
    let am = mul_redc_inline::<M>(a, &M::R2);
    mul_redc_inline::<M>(&am, &b)
}

pub fn redc<M: Parameters>(lo: &U256, hi: &U256) -> U256 {
    redc_inline::<M>(lo, hi)
}

#[inline(always)]
pub fn redc_inline<M: Parameters>(lo: &U256, hi: &U256) -> U256 {
    // Select the best algorithm, the branch should be resolved compile time.
    // TODO: Make compile time constant.
    if proth::is_proth::<M>() {
        proth::redc_inline(M::MODULUS.limb(3), lo, hi)
    } else {
        generic::redc_inline::<M>(lo, hi)
    }
}

pub fn mul_redc<M: Parameters>(x: &U256, y: &U256) -> U256 {
    mul_redc_inline::<M>(x, y)
}

#[inline(always)]
pub fn mul_redc_inline<M: Parameters>(x: &U256, y: &U256) -> U256 {
    if proth::is_proth::<M>() {
        proth::mul_redc_inline(M::MODULUS.limb(3), x, y)
    } else {
        generic::mul_redc_inline::<M>(x, y)
    }
}

pub fn sqr_redc<M: Parameters>(a: &U256) -> U256 {
    sqr_redc_inline::<M>(a)
}

#[inline(always)]
pub fn sqr_redc_inline<M: Parameters>(a: &U256) -> U256 {
    let (lo, hi) = a.sqr_full_inline();
    redc_inline::<M>(&lo, &hi)
}

/// There is no inline version since the call overhead is small (inversion is
/// slow).
pub fn inv_redc<M: Parameters>(n: &U256) -> Option<U256> {
    // OPT: Fold mul into GCD computation by starting with (0, R3) instead
    // of (0, 1).
    n.invmod(&M::MODULUS)
        .map(|ni| mul_redc_inline::<M>(&ni, &M::R3))
}

// Quickcheck requires pass-by-value
#[allow(clippy::needless_pass_by_value)]
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    use zkp_macros_decl::u256h;

    struct PrimeField();

    // TODO: Non Proth example
    impl Parameters for PrimeField {
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
        assert_eq!(redc::<PrimeField>(&a, &b), c);
    }

    #[test]
    fn test_mul_redc() {
        let a = u256h!("0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c");
        let b = u256h!("024385f6bebc1c496e09955db534ef4b1eaff9a78e27d4093cfa8f7c8f886f6b");
        let c = u256h!("012b854fc6321976d374ad069cfdec8bb7b2bd184259dae8f530cbb28f0805b4");
        assert_eq!(mul_redc::<PrimeField>(&a, &b), c);
    }

    #[quickcheck]
    fn test_to_from(mut n: U256) -> bool {
        n %= PrimeField::MODULUS;
        from_montgomery::<PrimeField>(&to_montgomery::<PrimeField>(&n)) == n
    }

    #[quickcheck]
    fn test_mulmod(a: U256, b: U256) -> bool {
        let r = mulmod::<PrimeField>(&a, &b);
        let e = a.mulmod(&b, &PrimeField::MODULUS);
        r == e
    }
}
