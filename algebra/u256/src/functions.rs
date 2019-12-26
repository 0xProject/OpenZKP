use crate::{
    algorithms::{gcd, gcd_extended},
    U256,
};

impl U256 {
    #[inline(always)]
    pub fn gcd(a: &Self, b: &Self) -> Self {
        gcd(a.clone(), b.clone())
    }

    #[inline(always)]
    pub fn gcd_extended(a: &Self, b: &Self) -> (Self, Self, Self, bool) {
        gcd_extended(a.clone(), b.clone())
    }

    // TODO: Factorial, Totient, Carmichael, Jacobi, Legendre, Binomial, etc.
    // See https://gmplib.org/manual/Number-Theoretic-Functions.html
}
