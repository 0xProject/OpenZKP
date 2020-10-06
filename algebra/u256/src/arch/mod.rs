mod generic;

#[cfg(all(not(feature = "stable"), feature = "asm", target_arch = "x86_64"))]
mod x86_64;

#[cfg(all(
    not(feature = "stable"),
    feature = "asm",
    target_arch = "x86_64",
    target_feature = "adx"
))]
mod x86_64_adx;

// TODO: ARM64 and WASM

// Re-export most specific architecture

#[cfg(not(all(not(feature = "stable"), feature = "asm", target_arch = "x86_64")))]
pub(crate) use generic::*;

#[cfg(all(all(
    not(feature = "stable"),
    feature = "asm",
    target_arch = "x86_64",
    not(target_feature = "adx")
)))]
pub(crate) use x86_64::*;

#[cfg(all(all(
    not(feature = "stable"),
    feature = "asm",
    target_arch = "x86_64",
    target_feature = "adx"
)))]
pub(crate) use x86_64_adx::*;
