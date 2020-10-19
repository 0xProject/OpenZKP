// Allow `asm!` and `llvm_asm!` in this file.
#![allow(unsafe_code)]

// Re-use x86_64 routines
pub(crate) use super::x86_64::*;

use crate::{MontgomeryParameters, U256};
use std::mem::MaybeUninit;

// TODO: Intrinsics based approach usind AD_X is currently blocked on LLVM.
// See <https://github.com/rust-lang/stdarch/issues/666>
// See <https://bugs.llvm.org/show_bug.cgi?id=41546>

// For instruction timings and through puts
// See <https://gmplib.org/~tege/x86-timing.pdf>
// See <https://agner.org/optimize/>
// See <https://www.felixcloutier.com/x86/>

// For examples using mulx/adcx
// See <https://www.intel.com/content/dam/www/public/us/en/documents/white-papers/large-integer-squaring-ia-paper.pdf>
// See <https://www.intel.com/content/dam/www/public/us/en/documents/white-papers/ia-large-integer-arithmetic-paper.pdf>
// See <https://gmplib.org/repo/gmp/file/tip/mpn/x86_64/mulx/adx/addmul_1.asm>
// See <https://github.com/microsoft/SymCrypt/blob/master/lib/amd64/fdef_mulx.asm>

// <https://web.archive.org/web/20181104011912/https://locklessinc.com/articles/gcc_asm/>
// <https://releases.llvm.org/9.0.0/docs/LangRef.html#inline-assembler-expressions>
// NOTE: LLVM currently always takes `m` when offered `rm`, but this seems fine
// for our use case.

#[inline(always)]
pub(crate) fn proth_redc_asm(m3: u64, lo: &U256, hi: &U256) -> U256 {
    // TODO: Fix carry bug
    const ZERO: u64 = 0;
    let lo = lo.as_limbs();
    let hi = hi.as_limbs();
    let mut result = MaybeUninit::<[u64; 4]>::uninit();
    unsafe {
        llvm_asm!(r"
        // RDX contains M3 and we keep it there the whole time.
        // OPT: Use operand constraints to put it there.
        mov $4, %rdx

        // [r8, r9, r10, CF] = -[lo[0] lo[1] lo[2]]
        mov 0($1), %r8
        xor %r9, %r9
        xor %r10, %r10
        neg %r8
        sbb 8($1), %r9
        sbb 16($1), %r10
        // Remaining CF is for lo[3]

        // Clear OF (by adding zero+OF to zero)
        mov  $$0, %rax             // Note: we can't use xor here
        adox %rax, %rax

        // Add m3 * [k0 k1 k2] to [lo[3]+CF hi[0] hi[1] hi[2] hi[3]]
        // and store in [r8 r11 r9 r10, r12]
        mulx %r8, %r8, %r11
        adcx 24($1), %r8
        mov %r12, 24($0)
        adox 0($2), %r11
        mulx %r9, %rax, %r9
        adcx %rax, %r11
        adox 8($2), %r9
        mulx %r10, %rax, %r10
        adcx %rax, %r9
        adox 16($2), %r10
        adcx $3, %r10
        mov $3, %r12
        adox 24($2), %r12
        adcx $3, %r12

        // Compute k3, CF is for r11
        neg  %r8
        adcx $3, %r11
        adcx $3, %r9

        // Add m3 * k3 to [r10 r12]
        mulx %r8, %rax, %rbx
        adcx %rax, %r10
        adcx %rbx, %r12                    // No carry, CF = 0

        // Result can be up to 2 * modulus
        // We need to conditionally subtract one modulus.
        // This step takes 1.1ns or about 22% of total time.
        // We could leave it out, but that complicates the function signature.

        // Reduce result
        mov %r11, %rax
        mov %r9, %rbx
        mov %r10, %r13
        mov %r12, %r14

        sub $$1, %rax
        sbb $$0, %rbx
        sbb $$0, %r13
        sbb %rdx, %r14

        // Conditionally store reduced result if CF=1
        cmovnc %rax, %r11
        cmovnc %rbx, %r9
        cmovnc %r13, %r10
        cmovnc %r14, %r12

        // Store result
        mov %r11, 0($0)
        mov %r9, 8($0)
        mov %r10, 16($0)
        mov %r12, 24($0)
        "
            :
            : "r"(result.as_mut_ptr()), "r"(lo), "r"(hi), "m"(ZERO), "m"(m3)
            : "rax", "rbx", "rdx", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "cc", "memory"
        )
    }
    let result = unsafe { result.assume_init() };
    U256::from_limbs(result)
}

// Computes r[0..5] = a * b[0..4]
// Uses MULX
// Currently unused
#[inline(always)]
pub(crate) fn mul_1_asm(a: u64, b0: u64, b1: u64, b2: u64, b3: u64) -> (u64, u64, u64, u64, u64) {
    let r0: u64;
    let r1: u64;
    let r2: u64;
    let r3: u64;
    let r4: u64;
    let _lo: u64;
    // Binding `_lo` will not be used after assignment.
    #[allow(clippy::used_underscore_binding)]
    unsafe {
        llvm_asm!(r"
        mulx $7, $0, $1      // (r0, r1) = a * b0

        mulx $8, $5, $2      // (lo, r2) = a * b1
        add $5, $1           // r1 += lo (carry in CF)

        mulx $9, $5, $3      // (lo, r3) = a * b2
        adc $5, $2           // r2 += lo + CF (carry in CF)

        mulx $10, $5, $4     // (lo, r4) = a * b3
        adc $5, $3           // r3 += lo + CF (carry in CF)
        adc $11, $4          // r4 += 0 + CF (no carry, CF to 0)
        "
        : // Output constraints
            "=&r"(r0),   // $0 r0..4 are in registers
            "=&r"(r1),   // $1
            "=&r"(r2),   // $2
            "=&r"(r3),   // $3
            "=&r"(r4)    // $4
            "=&r"(_lo)   // $5 Temporary values can be in any register
        : // Input constraints
            "{rdx}"(a), // $6 a must be in RDX for MULX to work
            "rm"(b0),   // $7 b0..b3 can be register or memory
            "rm"(b1),   // $8
            "rm"(b2),   // $9
            "rm"(b3),   // $10
            "i"(0)      // $11 Immediate zero
        : // Clobbers
           "cc"         // Flags
        )
    }
    (r0, r1, r2, r3, r4)
}

// Computes r[0..4] += a * b[0..4], returns carry
// Uses MULX and ADCX/ADOX carry chain
// We want each argument to be able to get it's own register after inlining.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub(crate) fn mul_add_1_asm(
    r0: &mut u64,
    r1: &mut u64,
    r2: &mut u64,
    r3: &mut u64,
    a: u64,
    b0: u64,
    b1: u64,
    b2: u64,
    b3: u64,
) -> u64 {
    let _lo: u64;
    let _hi: u64;
    let r4: u64;
    // Bindings `_lo` and `_hi` will not be used after assignment.
    #[allow(clippy::used_underscore_binding)]
    unsafe {
        llvm_asm!(r"
        xor $4, $4            // r4 = CF = OF 0

        mulx $8, $5, $6       // a * b0
        adcx $5, $0           // r0 += lo + CF (carry in CF)
        adox $6, $1           // r1 += hi + OF (carry in OF)

        mulx $9, $5, $6       // a * b1
        adcx $5, $1           // r1 += lo + CF (carry in CF)
        adox $6, $2           // r2 += hi + OF (carry in OF)

        mulx $10, $5, $6      // a * b2
        adcx $5, $2           // r2 += lo + CF (carry in CF)
        adox $6, $3           // r3 += hi + OF (carry in OF)

        mulx $11, $5, $6      // a * b3
        adcx $5, $3           // r3 += lo + CF (carry in CF)
        adcx $4, $4           // r4 += CF (no carry, CF = 0)
        adox $6, $4           // r4 += hi + OF (no carry, OF = 0)
        "
        : // Output constraints
            "+r"(*r0),   // $0 r0..3 are in register and modified in place
            "+r"(*r1),   // $1
            "+r"(*r2),   // $2
            "+r"(*r3),   // $3
            "=&r"(r4)    // $4 r4 is output to a register
            "=&r"(_lo),  // $5 Temporary values can be in any register
            "=&r"(_hi)   // $6
        : // Input constraints
            "{rdx}"(a), // $7 a must be in RDX for MULX to work
            "rm"(b0),   // $8 Second operand can be register or memory
            "rm"(b1),   // $9 Second operand can be register or memory
            "rm"(b2),   // $10 Second operand can be register or memory
            "rm"(b3)    // $11 Second operand can be register or memory
        : // Clobbers
           "cc"         // Flags
        )
    }
    r4
}

#[inline(always)]
pub(crate) fn full_mul_asm2(x: &U256, y: &U256) -> (U256, U256) {
    let x = x.as_limbs();
    let y = y.as_limbs();
    let (r0, mut r1, mut r2, mut r3, mut r4) = mul_1_asm(x[0], y[0], y[1], y[2], y[3]);
    let mut r5 = mul_add_1_asm(
        &mut r1, &mut r2, &mut r3, &mut r4, x[1], y[0], y[1], y[2], y[3],
    );
    let mut r6 = mul_add_1_asm(
        &mut r2, &mut r3, &mut r4, &mut r5, x[2], y[0], y[1], y[2], y[3],
    );
    let r7 = mul_add_1_asm(
        &mut r3, &mut r4, &mut r5, &mut r6, x[3], y[0], y[1], y[2], y[3],
    );
    (
        U256::from_limbs([r0, r1, r2, r3]),
        U256::from_limbs([r4, r5, r6, r7]),
    )
}

// TODO: Square asm
// TODO: Mul-add

#[inline(always)]
pub(crate) fn mul_asm(x: &U256, y: &U256) -> U256 {
    let x = x.as_limbs();
    let y = y.as_limbs();
    let mut r = MaybeUninit::<[u64; 4]>::uninit();
    unsafe {
        llvm_asm!(r"
        xor %rax, %rax               // CF, OF cleared

        // Set x[0] * y
        // [lo[0] r8 r9 r10 r11]
        mov  0($1), %rdx             // x[0]
        mulx 0($2), %rax, %r8        // * y[0]
        mov  %rax, 0($0)             // Store lowest limb
        mulx 8($2), %rax, %r9        // * y[1]
        adcx %rax, %r8
        mulx 16($2), %rax, %r10      // * y[2]
        adcx %rax, %r9
        mulx 24($2), %rax, %r11      // * y[3]
        adcx %rax, %r10
        xor %r11, %r11

        // Add x[1] * y
        // [lo[1] r9 r10 r11]
        mov  8($1), %rdx             // x[1]
        mulx 0($2), %rax, %rbx       // * y[0]
        adcx %rax, %r8
        adox %rbx, %r9
        mov  %r8, 8($0)              // Store and free r8
        mulx 8($2), %rax, %rbx       // * y[1]
        adcx %rax, %r9
        adox %rbx, %r10
        mulx 16($2), %rax, %r11      // * y[2]
        adcx %rax, %r10
        xor %r11, %r11

        // Add x[2] * y
        // [lo[2] r10 r11]
        mov  16($1), %rdx            // x[2]
        mulx 0($2), %rax, %rbx       // * y[0]
        adcx %rax, %r9
        adox %rbx, %r10
        mov  %r9, 16($0)             // Store and free r9
        mulx 8($2), %rax, %r11       // * y[1]
        adcx %rax, %r10
        xor %r11, %r11

        // Add x[3] * y
        // [lo[3] r11]
        mov  24($1), %rdx            // x[3]
        mulx 0($2), %rax, %r11       // * y[0]
        adcx %rax, %r10
        mov  %r10, 24($0)            // Store and free r9
        "
            :
            : "r"(r.as_mut_ptr()), "r"(x), "r"(y)
            : "rax", "rbx", "rdx", "r8", "r9", "r10", "r11", "cc", "memory"
        )
    }
    let r = unsafe { r.assume_init() };
    U256::from_limbs(r)
}

#[inline(always)]
pub(crate) fn full_mul_asm(x: &U256, y: &U256) -> (U256, U256) {
    const ZERO: u64 = 0;
    let x = x.as_limbs();
    let y = y.as_limbs();
    let mut lo = MaybeUninit::<[u64; 4]>::uninit();
    let mut hi = MaybeUninit::<[u64; 4]>::uninit();

    unsafe {
        llvm_asm!(r"
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
        )
    }
    let lo = unsafe { lo.assume_init() };
    let hi = unsafe { hi.assume_init() };

    (U256::from_limbs(lo), U256::from_limbs(hi))
}

// https://doc.rust-lang.org/1.12.0/book/inline-assembly.html
// https://llvm.org/docs/LangRef.html#inline-assembler-expressions
// https://www.intel.com/content/dam/www/public/us/en/documents/white-papers/large-integer-squaring-ia-paper.pdf
//

// LEA amd INC can add without affecting flags.
// NOT INC  can be used for a carry free NEG
// NEG sets CF and clobbers OF.

// This assembly block needs to be contiguous
#[allow(clippy::too_many_lines)]
#[inline(always)]
pub(crate) fn mul_redc<M: MontgomeryParameters<UInt = U256>>(a: &U256, b: &U256) -> U256 {
    const ZERO: u64 = 0; // $3
    let modulus = M::MODULUS.as_limbs();

    let a = a.as_limbs();
    let b = b.as_limbs();
    let mut result = MaybeUninit::<[u64; 4]>::uninit();
    // MULX dst_high, dst_low, src_b (src_a = %rdx)
    // src_b can be register or memory, not immediate
    unsafe {
        llvm_asm!(r"
            // Assembly from Aztec's Barretenberg implementation, see 
            // <https://github.com/AztecProtocol/barretenberg/blob/master/src/barretenberg/fields/asm_macros.hpp>
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
            : "r"(result.as_mut_ptr()),
              "r"(a), "r"(b),
              "m"(ZERO),
              "m"(modulus[0]),
              "m"(modulus[1]),
              "m"(modulus[2]),
              "m"(modulus[3]),
              "m"(M::M64)
            : "rdx", "rdi", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15", "cc", "memory"
        );
    }
    let result = unsafe { result.assume_init() };

    // TODO: Does it need a final reduction?
    let mut r = U256::from_limbs(result);
    if r >= M::MODULUS {
        r -= &M::MODULUS;
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use zkp_macros_decl::u256h;

    const M3: u64 = 0x0800_0000_0000_0011;

    #[test]
    fn test_proth_redc() {
        let a = u256h!("0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c");
        let b = u256h!("024385f6bebc1c496e09955db534ef4b1eaff9a78e27d4093cfa8f7c8f886f6b");
        let c = crate::algorithms::montgomery::proth::redc_inline(M3, &a, &b);
        assert_eq!(proth_redc_asm(M3, &a, &b), c);
    }
}
