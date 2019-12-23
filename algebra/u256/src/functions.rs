use crate::{
    algorithms::{gcd, gcd_extended},
    U256,
};

impl U256 {
    pub fn gcd(a: &U256, b: &U256) -> U256 {
        gcd(a.clone(), b.clone())
    }

    pub fn gcd_extended(a: &U256, b: &U256) -> (U256, U256, U256, bool) {
        gcd_extended(a.clone(), b.clone())
    }

    // TODO: Factorial, Totient, Carmichael, Jacobi, Legendre, Binomial, etc.
    // See https://gmplib.org/manual/Number-Theoretic-Functions.html
}
