// Allow `asm!` and `llvm_asm!` in this file.
#![allow(unsafe_code)]

// Re-use generic routines
pub(crate) use super::generic::*;

use crate::U256;

// Rust's `asm!` syntax is unstable and specified here:
// <https://doc.rust-lang.org/unstable-book/library-features/asm.html>
// <https://github.com/Amanieu/rfcs/blob/inline-asm/text/0000-inline-asm.md>

/// Reduce at most once
#[inline(always)]
pub(crate) fn reduce_1(s: &U256, modulus: &U256) -> U256 {
    let s = s.as_limbs();
    let r0: u64;
    let r1: u64;
    let r2: u64;
    let r3: u64;
    unsafe {
        asm!(r"
        // Copy value (s) to result (r)
        // Subtract modulus (m) from value (s)
        mov {r0}, {s0}
        sub {s0}, [{m} + 0x00]
        mov {r1}, {s1}
        sbb {s1}, [{m} + 0x08]
        mov {r2}, {s2}
        sbb {s2}, [{m} + 0x10]
        mov {r3}, {s3}
        sbb {s3}, [{m} + 0x18]

        // Conditional copy reduced value (s) to result (r)
        cmovnc {r0}, {s0}
        cmovnc {r1}, {s1}
        cmovnc {r2}, {s2}
        cmovnc {r3}, {s3}
        ",
        // FIXME: Once <https://github.com/rust-lang/rust/issues/57775> is solved we
        // could inline the modulus as a constant passed through generics. Currently
        // we pass it as a reference to an array which requires an additional register.
        m = in(reg) modulus,
        s0 = inout(reg) s[0] => _,
        s1 = inout(reg) s[1] => _,
        s2 = inout(reg) s[2] => _,
        s3 = inout(reg) s[3] => _,
        r0 = out(reg) r0,
        r1 = out(reg) r1,
        r2 = out(reg) r2,
        r3 = out(reg) r3,
        options(pure, nomem, nostack)
        );
    }
    U256::from_limbs([r0, r1, r2, r3])
}
