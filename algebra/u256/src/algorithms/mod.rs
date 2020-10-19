mod binary_operator_macros;
pub(crate) mod limb_operations;
pub(crate) mod montgomery;

pub(crate) use montgomery::{mul_redc_inline, redc_inline, square_redc_inline};

// False positives, we re-export in `lib.rs`
#[allow(unreachable_pub)]
pub use montgomery::to_montgomery_const;
// False positives, we re-export in `lib.rs`
#[allow(unreachable_pub)]
pub use limb_operations::{adc, div_2_1, mac, msb, sbb};
