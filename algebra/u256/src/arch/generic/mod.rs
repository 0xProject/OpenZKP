mod knuth_division;
mod lehmer_gcd;

use crate::U256;

pub(crate) use knuth_division::{divrem_nby1, divrem_nbym};
pub(crate) use lehmer_gcd::{gcd, gcd_extended, inv_mod};

/// Reduce at most once
#[inline(always)]
pub(crate) fn reduce_1(s: &U256, modulus: &U256) -> U256 {
    if s >= modulus {
        s - modulus
    } else {
        s.clone()
    }
}
