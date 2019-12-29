use crate::{
    algorithms::{gcd, gcd_extended},
    GCD, U256,
};

impl GCD for U256 {
    #[inline(always)]
    fn gcd(a: &Self, b: &Self) -> Self {
        gcd(a.clone(), b.clone())
    }

    #[inline(always)]
    fn gcd_extended(a: &Self, b: &Self) -> (Self, Self, Self, bool) {
        gcd_extended(a.clone(), b.clone())
    }
}
