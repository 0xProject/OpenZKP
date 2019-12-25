use crate::{
    algorithms::limb_operations::{adc, mac, macc, sbb},
    U256,
};
use std::mem::MaybeUninit;

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

pub fn proth_redc<M: Parameters>(lo: &U256, hi: &U256) -> U256 {
    proth_redc_inline::<M>(lo, hi)
}

// See https://hackmd.io/7PFyv-itRBa0a0nYCAklmA?both
// See https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/modmul_no_precomp.pdf
// See https://pdfs.semanticscholar.org/c751/a321dd430ebbcfb4dcce1f86f88256e0af5a.pdf
// This is algorithm 14.32 optimized for the facts that
//   m_0 = 1. m_1 =0, m_2 = 0, m' = -1
#[inline(always)]
pub fn proth_redc_inline<M: Parameters>(lo: &U256, hi: &U256) -> U256 {
    let modulus = M::MODULUS.as_limbs();
    let m3 = modulus[3];
    assert_eq!(modulus[0], 1);
    assert_eq!(modulus[1], 0);
    assert_eq!(modulus[2], 0);
    assert_eq!(M::M64, u64::max_value());

    let lo = lo.as_limbs();
    let hi = hi.as_limbs();

    let (a0, carry) = sbb(0, lo[0], 0);
    let (a1, carry) = sbb(0, lo[1], carry);
    let (a2, carry) = sbb(0, lo[2], carry);
    let (a3, hcarry) = mac(lo[3], a0, m3, carry);
    let (a3, carry) = sbb(0, a3, 0);
    let (a4, hcarry) = macc(hi[0], a1, m3, hcarry, carry);
    let (a5, hcarry) = mac(hi[1], a2, m3, hcarry);
    let (a6, hcarry) = mac(hi[2], a3, m3, hcarry);
    let (a7, _carry) = adc(hi[3], 0, hcarry);

    // Final reduction
    let mut r = U256::from_limbs([a4, a5, a6, a7]);
    if r >= M::MODULUS {
        r -= &M::MODULUS;
    }
    r
}

#[inline(always)]
pub fn proth_redc_asm() {
    
}


#[inline(always)]
pub fn full_mul_asm(x: &U256, y: &U256) -> (U256, U256) {
    let x = x.as_limbs();
    let y = y.as_limbs();
    const ZERO: u64 = 0;

    let mut lo = MaybeUninit::<[u64; 4]>::uninit();
    let mut hi = MaybeUninit::<[u64; 4]>::uninit();

    unsafe { asm!(r"
        xor %rax, %rax               // CF, OF cleared

        // Set x[0] * y
        // [lo[0] r8 r9 r10 r11]
        mov  0($2), %rdx             // x[0]
        mulx 0($3), %rax, %r8        // * y[0]
        mov  %rax, 0($0)             // Store lowest limb
        mulx 8($3), %rax, %r9        // * y[1]
        adcx %rax, %r8
        mulx 16($3), %rax, %r10      // * y[2]
        adcx %rax, %r9
        mulx 24($3), %rax, %r11      // * y[3]
        adcx %rax, %r10
        adcx $4, %r11                // No carry, CF cleared

        // Add x[1] * y
        // [lo[1] r9 r10 r11 r8]
        mov  8($2), %rdx             // x[1]
        mulx 0($3), %rax, %rbx       // * y[0]
        adcx %rax, %r8
        adox %rbx, %r9
        mov  %r8, 8($0)              // Store and free r8
        mulx 8($3), %rax, %rbx       // * y[1]
        adcx %rax, %r9
        adox %rbx, %r10
        mulx 16($3), %rax, %rbx      // * y[2]
        adcx %rax, %r10
        adox %rbx, %r11
        mulx 24($3), %rax, %r8       // * y[3]
        adcx %rax, %r11
        adox $4, %r8                 // No carry, OF cleared
        adcx $4, %r8                 // No carry, CF cleared

        // Add x[2] * y
        // [lo[2] r10 r11 r8 r9]
        mov  16($2), %rdx            // x[2]
        mulx 0($3), %rax, %rbx       // * y[0]
        adcx %rax, %r9
        adox %rbx, %r10
        mov  %r9, 16($0)             // Store and free r9
        mulx 8($3), %rax, %rbx       // * y[1]
        adcx %rax, %r10
        adox %rbx, %r11
        mulx 16($3), %rax, %rbx      // * y[2]
        adcx %rax, %r11
        adox %rbx, %r8
        mulx 24($3), %rax, %r9       // * y[3]
        adcx %rax, %r8
        adox $4, %r9                 // No carry, OF cleared
        adcx $4, %r9                 // No carry, CF cleared

        // Add x[3] * y
        // [lo[3] r11 r8 r9 r10]
        mov  24($2), %rdx            // x[3]
        mulx 0($3), %rax, %rbx       // * y[0]
        adcx %rax, %r10
        adox %rbx, %r11
        mov  %r10, 24($0)            // Store and free r9
        mulx 8($3), %rax, %rbx       // * y[1]
        adcx %rax, %r11
        adox %rbx, %r8
        mulx 16($3), %rax, %rbx      // * y[2]
        adcx %rax, %r8
        adox %rbx, %r9
        mulx 24($3), %rax, %r10      // * y[3]
        adcx %rax, %r9
        adox $4, %r10                // No carry, OF cleared
        adcx $4, %r10                // No carry, CF cleared

        // Store high limbs
        mov %r11, 0($1)
        mov %r8, 8($1)
        mov %r9, 16($1)
        mov %r10, 24($1)
        "
        :
        : "r"(lo.as_mut_ptr()), "r"(hi.as_mut_ptr()), "r"(x), "r"(y), "m"(ZERO)
        : "rax", "rbx", "rdx", "r8", "r9", "r10", "r11", "cc", "memory"
    )}
    let lo = unsafe { lo.assume_init() };
    let hi = unsafe { hi.assume_init() };

    (U256::from_limbs(lo), U256::from_limbs(hi))
}

pub fn mul_redc<M: Parameters>(x: &U256, y: &U256) -> U256 {
    mul_redc_inline::<M>(x, y)
}

// We rebind variables for readability
#[allow(clippy::shadow_unrelated)]
#[inline(always)]
pub fn mul_redc_inline<M: Parameters>(x: &U256, y: &U256) -> U256 {
    let (lo, hi) = full_mul_asm(x, y);
    return proth_redc_inline::<M>(&lo, &hi);
    // return proth_mul_redc_asm::<M>(x, y);

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

// We rebind variables for readability
#[allow(clippy::shadow_unrelated)]
#[inline(always)]
pub fn proth_mul_redc_inline<M: Parameters>(x: &U256, y: &U256) -> U256 {
    let modulus = M::MODULUS.as_limbs();
    let m3 = modulus[3];
    assert_eq!(modulus[0], 1);
    assert_eq!(modulus[1], 0);
    assert_eq!(modulus[2], 0);
    assert_eq!(M::M64, u64::max_value());

    let x = x.as_limbs();
    let y = y.as_limbs();

    let (a0, carry) = mac(0, x[0], y[0], 0);
    let (a1, carry) = mac(0, x[0], y[1], carry);
    let (a2, carry) = mac(0, x[0], y[2], carry);
    let (a3, carry) = mac(0, x[0], y[3], carry);
    let a4 = carry;
    let (k, carry) = sbb(0, a0, 0);
    let (a3, hcarry) = mac(a3, k, m3, 0);
    let (a1, carry) = mac(a1, x[1], y[0], carry);
    let (a2, carry) = mac(a2, x[1], y[1], carry);
    let (a3, carry) = mac(a3, x[1], y[2], carry);
    let (a4, carry) = macc(a4, x[1], y[3], carry, hcarry);
    let a5 = carry;
    let (k, carry) = sbb(0, a1, 0);
    let (a4, hcarry) = mac(a4, k, m3, 0);
    let (a2, carry) = mac(a2, x[2], y[0], carry);
    let (a3, carry) = mac(a3, x[2], y[1], carry);
    let (a4, carry) = mac(a4, x[2], y[2], carry);
    let (a5, carry) = macc(a5, x[2], y[3], carry, hcarry);
    let a6 = carry;
    let (k, carry) = sbb(0, a2, 0);
    let (a5, hcarry) = mac(a5, k, m3, 0);
    let (a3, carry) = mac(a3, x[3], y[0], carry);
    let (a4, carry) = mac(a4, x[3], y[1], carry);
    let (a5, carry) = mac(a5, x[3], y[2], carry);
    let (a6, carry) = macc(a6, x[3], y[3], carry, hcarry);
    let a7 = carry;
    let (k, carry) = sbb(0, a3, 0);
    let (a4, carry) = adc(a4, 0, carry);
    let (a5, carry) = adc(a5, 0, carry);
    let (a6, carry) = mac(a6, k, m3, carry);
    let a7 = a7 + carry;

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
pub fn proth_mul_redc_asm<M: Parameters>(x: &U256, y: &U256) -> U256 {
    let modulus = M::MODULUS.as_limbs();
    let m3 = modulus[3];
    assert_eq!(modulus[0], 1);
    assert_eq!(modulus[1], 0);
    assert_eq!(modulus[2], 0);
    assert_eq!(M::M64, u64::max_value());

    let x = x.as_limbs();
    let y = y.as_limbs();

    let (a0, carry) = mac(0, x[0], y[0], 0);
    let (a1, carry) = mac(0, x[0], y[1], carry);
    let (a2, carry) = mac(0, x[0], y[2], carry);
    let (a3, carry) = mac(0, x[0], y[3], carry);
    let a4 = carry;
    let (k, carry) = sbb(0, a0, 0);
    let (a3, hcarry) = mac(a3, k, m3, 0);
    let (a1, carry) = mac(a1, x[1], y[0], carry);
    let (a2, carry) = mac(a2, x[1], y[1], carry);
    let (a3, carry) = mac(a3, x[1], y[2], carry);
    let (a4, carry) = macc(a4, x[1], y[3], carry, hcarry);
    let a5 = carry;    
    let (k, carry) = sbb(0, a1, 0);
    let (a4, hcarry) = mac(a4, k, m3, 0);
    let (a2, carry) = mac(a2, x[2], y[0], carry);
    let (a3, carry) = mac(a3, x[2], y[1], carry);
    let (a4, carry) = mac(a4, x[2], y[2], carry);
    let (a5, carry) = macc(a5, x[2], y[3], carry, hcarry);
    let a6 = carry;
    let (k, carry) = sbb(0, a2, 0);
    let (a5, hcarry) = mac(a5, k, m3, 0);
    let (a3, carry) = mac(a3, x[3], y[0], carry);
    let (a4, carry) = mac(a4, x[3], y[1], carry);
    let (a5, carry) = mac(a5, x[3], y[2], carry);
    let (a6, carry) = macc(a6, x[3], y[3], carry, hcarry);
    let a7 = carry;
    let (k, carry) = sbb(0, a3, 0);
    let (a4, carry) = adc(a4, 0, carry);
    let (a5, carry) = adc(a5, 0, carry);
    let (a6, carry) = mac(a6, k, m3, carry);
    let a7 = a7 + carry;

    // Final reduction
    let mut r = U256::from_limbs([a4, a5, a6, a7]);
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


// LEA amd INC can add without affecting flags.
// NOT INC  can be used for a carry free NEG
// NEG sets CF and clobbers OF.

#[inline(always)]
pub fn mulx(a: &U256, b: &U256) -> U256 {
    const ZERO: u64 = 0; // $3
    const MODULUS_0: u64 = 1; // $4
    const MODULUS_1: u64 = 0; // $5
    const MODULUS_2: u64 = 0; // $6
    const MODULUS_3: u64 = 0x0800000000000011; // $7
    const M64: u64 = 0xffff_ffff_ffff_ffff; // -1 $8
                                            // TODO: Optimize for special primes where the above values hold
    let a = a.as_limbs();
    let b = b.as_limbs();
    let mut result: [u64; 4] = [0, 0, 0, 0];
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
    fn test_proth_redc() {
        let a = u256h!("0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c");
        let b = u256h!("024385f6bebc1c496e09955db534ef4b1eaff9a78e27d4093cfa8f7c8f886f6b");
        let c = u256h!("012e440f0965e7029c218b64f1010006b5c4ba8b1497c4174a32fec025c197bc");
        assert_eq!(proth_redc::<PrimeField>(&a, &b), c);
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
