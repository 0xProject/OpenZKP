use num_traits::{Bounded, Zero, One, Inv, MulAdd, MulAddAssign, Pow, Num, Unsigned};
use crate::U256;

impl Bounded for U256 {
    #[inline(always)]
    fn min_value() -> Self {
        U256::ZERO
    }

    #[inline(always)]
    fn max_value() -> Self {
        U256::MAX
    }
}

impl Zero for U256 {
    #[inline(always)]
    fn zero() -> Self {
        U256::ZERO
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self == &U256::ZERO
    }
}

impl One for U256 {
    #[inline(always)]
    fn one() -> Self {
        U256::ONE
    }

    #[inline(always)]
    fn is_one(&self) -> bool {
        self == &U256::ONE
    }
}

impl MulAdd for &U256 {
    type Output = U256;

    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self::Output {
        // OPT: Fused algorithm
        (self * a) + b
    }
}

impl MulAddAssign<&U256, &U256> for U256 {
    #[inline(always)]
    fn mul_add_assign(&mut self, a: &Self, b: &Self) {
        // OPT: Fused algorithm
        *self *= a;
        *self += b;
    }
}

/// Ring inversion.
// TODO: Make custom trait that adds `fn is_unit(&self) -> bool`.
// TODO: Implement Inv for u8..u128
impl Inv for &U256 {
    type Output = Option<U256>;

    fn inv(self) -> Self::Output {
        self.invmod256()
    }
}

// TODO: Other flavours of exponent
impl Pow<u64> for &U256 {
    type Output = U256;

    fn pow(self, exponent: u64) -> Self::Output {
        // TODO
        self.pow(exponent).unwrap()
    }
}

impl Num for U256 {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl Unsigned for U256 {}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::{NumAssign, NumAssignRef, NumRef, RefNum};

    // Assert that U256 implements a number of traits
    trait TraitBounds: NumAssign + NumAssignRef + NumRef + RefNum<Self> {}
    impl TraitBounds for U256 {}
}