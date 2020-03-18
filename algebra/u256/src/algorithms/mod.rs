pub(crate) mod assembly;
mod binary_operator_macros;
pub(crate) mod intrinsics;
mod knuth_division;
mod lehmer_gcd;
mod limb_operations;
mod montgomery;

pub(crate) use lehmer_gcd::{gcd, gcd_extended, inv_mod};
pub(crate) use montgomery::{mul_redc_inline, redc_inline, square_redc_inline};

// False positives, we re-export in `lib.rs`
#[allow(unreachable_pub)]
pub use knuth_division::{divrem_nby1, divrem_nbym};
// False positives, we re-export in `lib.rs`
#[allow(unreachable_pub)]
pub use montgomery::to_montgomery_const;
// False positives, we re-export in `lib.rs`
#[allow(unreachable_pub)]
pub use limb_operations::{adc, div_2_1, mac, msb, sbb};
