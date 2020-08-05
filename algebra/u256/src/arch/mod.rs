mod generic;

#[cfg(all(feature = "asm", target_arch = "x86_64"))]
mod x86_64;

#[cfg(all(feature = "asm", target_arch = "x86_64", target_feature = "adx"))]
mod x86_64_adx;

// TODO: ARM64 and WASM

// Re-export most specific architecture

#[cfg(not(all(feature = "asm", target_arch = "x86_64")))]
pub(crate) use generic::*;

#[cfg(all(all(feature = "asm", target_arch = "x86_64", not(target_feature = "adx"))))]
pub(crate) use x86_64::*;

#[cfg(all(all(feature = "asm", target_arch = "x86_64", target_feature = "adx")))]
pub(crate) use x86_64_adx::*;
