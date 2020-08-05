use crate::U256;

/// Reduce at most once
#[inline(always)]
pub(crate) fn reduce_1(s: &U256, modulus: &U256) -> U256 {
    if s >= modulus {
        s - modulus
    } else {
        s.clone()
    }
}
