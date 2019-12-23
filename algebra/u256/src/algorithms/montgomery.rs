use crate::{
    algorithms::limb_operations::{adc, mac, sbb},
    U256,
};

// TODO: const-compute from modulus
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

pub fn redc<M: Parameters>(lo: &U256, hi: &U256) -> U256 {
    redc_inline::<M>(lo, hi)
}

// We rebind variables for readability
#[allow(clippy::shadow_unrelated)]
#[inline(always)]
pub fn redc_inline<M: Parameters>(lo: &U256, hi: &U256) -> U256 {
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

pub fn mul_redc<M: Parameters>(x: &U256, y: &U256) -> U256 {
    mul_redc_inline::<M>(x, y)
}

// We rebind variables for readability
#[allow(clippy::shadow_unrelated)]
#[inline(always)]
pub fn mul_redc_inline<M: Parameters>(x: &U256, y: &U256) -> U256 {
    let x = x.as_limbs();
    let modulus = M::MODULUS.as_limbs();

    let k = x[0].wrapping_mul(y.limb(0)).wrapping_mul(M::M64);
    let (a0, carry) = mac(0, x[0], y.limb(0), 0);
    let (a1, carry) = mac(0, x[0], y.limb(1), carry);
    let (a2, carry) = mac(0, x[0], y.limb(2), carry);
    let (a3, carry) = mac(0, x[0], y.limb(3), carry);
    let a4 = carry;
    let (_a, carry) = mac(a0, k, modulus[0], 0);
    let (a0, carry) = mac(a1, k, modulus[1], carry);
    let (a1, carry) = mac(a2, k, modulus[2], carry);
    let (a2, carry) = mac(a3, k, modulus[3], carry);
    let a3 = a4 + carry;
    let k = x[1]
        .wrapping_mul(y.limb(0))
        .wrapping_add(a0)
        .wrapping_mul(M::M64);
    let (a0, carry) = mac(a0, x[1], y.limb(0), 0);
    let (a1, carry) = mac(a1, x[1], y.limb(1), carry);
    let (a2, carry) = mac(a2, x[1], y.limb(2), carry);
    let (a3, carry) = mac(a3, x[1], y.limb(3), carry);
    let a4 = carry;
    let (_a, carry) = mac(a0, k, modulus[0], 0);
    let (a0, carry) = mac(a1, k, modulus[1], carry);
    let (a1, carry) = mac(a2, k, modulus[2], carry);
    let (a2, carry) = mac(a3, k, modulus[3], carry);
    let a3 = a4 + carry;
    let k = x[2]
        .wrapping_mul(y.limb(0))
        .wrapping_add(a0)
        .wrapping_mul(M::M64);
    let (a0, carry) = mac(a0, x[2], y.limb(0), 0);
    let (a1, carry) = mac(a1, x[2], y.limb(1), carry);
    let (a2, carry) = mac(a2, x[2], y.limb(2), carry);
    let (a3, carry) = mac(a3, x[2], y.limb(3), carry);
    let a4 = carry;
    let (_a, carry) = mac(a0, k, modulus[0], 0);
    let (a0, carry) = mac(a1, k, modulus[1], carry);
    let (a1, carry) = mac(a2, k, modulus[2], carry);
    let (a2, carry) = mac(a3, k, modulus[3], carry);
    let a3 = a4 + carry;
    let k = x[3]
        .wrapping_mul(y.limb(0))
        .wrapping_add(a0)
        .wrapping_mul(M::M64);
    let (a0, carry) = mac(a0, x[3], y.limb(0), 0);
    let (a1, carry) = mac(a1, x[3], y.limb(1), carry);
    let (a2, carry) = mac(a2, x[3], y.limb(2), carry);
    let (a3, carry) = mac(a3, x[3], y.limb(3), carry);
    let a4 = carry;
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

pub fn sqr_redc<M: Parameters>(a: &U256) -> U256 {
    sqr_redc_inline::<M>(a)
}

pub fn sqr_redc_inline<M: Parameters>(a: &U256) -> U256 {
    let (lo, hi) = a.sqr_full_inline();
    redc_inline::<M>(&lo, &hi)
}

pub fn inv_redc<M: Parameters>(n: &U256) -> Option<U256> {
    // OPT: Fold mul into GCD computation by starting with (0, R3) instead
    // of (0, 1).
    n.invmod(&M::MODULUS)
        .map(|ni| mul_redc_inline::<M>(&ni, &M::R3))
}

#[allow(clippy::module_name_repetitions)]
pub fn to_montgomery<M: Parameters>(n: &U256) -> U256 {
    mul_redc::<M>(n, &M::R2)
}

#[allow(clippy::module_name_repetitions)]
pub fn from_montgomery<M: Parameters>(n: &U256) -> U256 {
    redc::<M>(n, &U256::ZERO)
}

// https://doc.rust-lang.org/1.12.0/book/inline-assembly.html
// https://llvm.org/docs/LangRef.html#inline-assembler-expressions
// https://www.intel.com/content/dam/www/public/us/en/documents/white-papers/large-integer-squaring-ia-paper.pdf
// https://github.com/AztecProtocol/barretenberg/blob/master/src/barretenberg/fields/asm_macros.hpp

pub fn mulx(a: &U256, b: &U256) -> U256 {
    const ZERO: u64 = 0;
    const MODULUS_0: u64 = 1;
    const MODULUS_1: u64 = 0;
    const MODULUS_2: u64 = 0;
    const MODULUS_3: u64 = 0x0800000000000011;
    const M64: u64 = 0xffff_ffff_ffff_ffff;
    let a = a.as_limbs();
    let b = b.as_limbs();
    let mut result: [u64; 4] = [0,0,0,0];
    // MULX dst_high, dst_low, src_b (src_a = %rdx)
    // src_b can be register or memory, not immediate
    unsafe {
        asm!(r"
            movq 0($1), %rdx
            xorq %r8, %r8
            mulxq 8($2), %r8, %r9
            mulxq 24($2), %rdi, %r12
            mulxq 0($2), %r13, %r14
            mulxq 16($2), %r15, %r10
            movq %r13, %rdx
            mulxq $8, %rdx, %r11
            adcxq %r8, %r14
            adoxq %rdi, %r10
            adcxq %r9, %r15
            adoxq $3, %r12
            adcxq $3, %r10
            mulxq $4, %r8, %r9
            mulxq $5, %rdi, %r11
            adoxq %r8, %r13
            adcxq %rdi, %r14
            adoxq %r9, %r14
            adcxq %r11, %r15
            mulxq $6, %r8, %r9
            mulxq $7, %rdi, %r11
            adoxq %r8, %r15
            adcxq %rdi, %r10
            adoxq %r9, %r10
            adcxq %r11, %r12
            adoxq $3, %r12
            movq 8($1), %rdx
            mulxq 0($2), %r8, %r9
            mulxq 8($2), %rdi, %r11
            adcxq %r8, %r14
            adoxq %r9, %r15
            adcxq %rdi, %r15
            adoxq %r11, %r10
            mulxq 16($2), %r8, %r9
            mulxq 24($2), %rdi, %r13
            adcxq %r8, %r10
            adoxq %rdi, %r12
            adcxq %r9, %r12
            adoxq $3, %r13
            adcxq $3, %r13
            movq %r14, %rdx
            mulxq $8, %rdx, %r8
            mulxq $4, %r8, %r9
            mulxq $5, %rdi, %r11
            adoxq %r8, %r14
            adcxq %rdi, %r15
            adoxq %r9, %r15
            adcxq %r11, %r10
            mulxq $6, %r8, %r9
            mulxq $7, %rdi, %r11
            adoxq %r8, %r10
            adcxq %r9, %r12
            adoxq %rdi, %r12
            adcxq %r11, %r13
            adoxq $3, %r13
            movq 16($1), %rdx
            mulxq 0($2), %r8, %r9
            mulxq 8($2), %rdi, %r11
            adcxq %r8, %r15
            adoxq %r9, %r10
            adcxq %rdi, %r10
            adoxq %r11, %r12
            mulxq 16($2), %r8, %r9
            mulxq 24($2), %rdi, %r14
            adcxq %r8, %r12
            adoxq %r9, %r13
            adcxq %rdi, %r13
            adoxq $3, %r14
            adcxq $3, %r14
            movq %r15, %rdx
            mulxq $8, %rdx, %r8
            mulxq $4, %r8, %r9
            mulxq $5, %rdi, %r11
            adoxq %r8, %r15
            adcxq %r9, %r10
            adoxq %rdi, %r10
            adcxq %r11, %r12
            mulxq $6, %r8, %r9
            mulxq $7, %rdi, %r11
            adoxq %r8, %r12
            adcxq %r9, %r13
            adoxq %rdi, %r13
            adcxq %r11, %r14
            adoxq $3, %r14
            movq 24($1), %rdx
            mulxq 0($2), %r8, %r9
            mulxq 8($2), %rdi, %r11
            adcxq %r8, %r10
            adoxq %r9, %r12
            adcxq %rdi, %r12
            adoxq %r11, %r13
            mulxq 16($2), %r8, %r9
            mulxq 24($2), %rdi, %r15
            adcxq %r8, %r13
            adoxq %r9, %r14
            adcxq %rdi, %r14
            adoxq $3, %r15
            adcxq $3, %r15
            movq %r10, %rdx
            mulxq $8, %rdx, %r8
            mulxq $4, %r8, %r9
            mulxq $5, %rdi, %r11
            adoxq %r8, %r10
            adcxq %r9, %r12
            adoxq %rdi, %r12
            adcxq %r11, %r13
            mulxq $6, %r8, %r9
            mulxq $7, %rdi, %rdx
            adoxq %r8, %r13
            adcxq %r9, %r14
            adoxq %rdi, %r14
            adcxq %rdx, %r15
            adoxq $3, %r15

            movq %r12, 0($0)
            movq %r13, 8($0)
            movq %r14, 16($0)
            movq %r15, 24($0)
            "
            : 
            : "r"(&result), "r"(a), "r"(b),
              "m"(ZERO), "m"(MODULUS_0), "m"(MODULUS_1), "m"(MODULUS_2), "m"(MODULUS_3), "m"(M64)
            : "rdx", "rdi", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15", "cc", "memory"
        );
    }
    U256::from_limbs(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    use zkp_macros_decl::u256h;

    struct PrimeField();

    impl Parameters for PrimeField {
        const M64: u64 = 0xffff_ffff_ffff_ffff;
        const MODULUS: U256 =
            u256h!("0800000000000011000000000000000000000000000000000000000000000001");
        // = -1
        const R1: U256 = u256h!("07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1");
        const R2: U256 = u256h!("07ffd4ab5e008810ffffffffff6f800000000001330ffffffffffd737e000401");
        const R3: U256 = u256h!("038e5f79873c0a6df47d84f8363000187545706677ffcc06cc7177d1406df18e");
    }
    #[test]
    fn test_mulx() {
        let a = u256h!("0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c");
        let b = u256h!("024385f6bebc1c496e09955db534ef4b1eaff9a78e27d4093cfa8f7c8f886f6b");
        let c = u256h!("012b854fc6321976d374ad069cfdec8bb7b2bd184259dae8f530cbb28f0805b4");
        assert_eq!(mulx(&a, &b), c);
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
}
