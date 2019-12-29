use crate::U256;
use num_traits::{
    Bounded, FromPrimitive, Inv, MulAdd, MulAddAssign, Num, One, Pow, ToPrimitive, Unsigned, Zero,
};

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

impl Num for U256 {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl Unsigned for U256 {}

impl FromPrimitive for U256 {
    #[inline(always)]
    fn from_i64(n: i64) -> Option<Self> {
        Some(U256::from(n))
    }

    #[inline(always)]
    fn from_u64(n: u64) -> Option<Self> {
        Some(U256::from(n))
    }

    // TODO: fn from_u128
    // TODO: fn from_i128
}

impl ToPrimitive for U256 {
    fn to_u64(&self) -> Option<u64> {
        todo!()
        // if self <= u64::max_value() {
        // Some(self.limb(0))
        // } else {
        // None
        // }
    }

    #[inline(always)]
    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    // TODO: to_i128, to_u128
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{Binary, BinaryAssignRef};
    use num_traits::{NumAssign, NumAssignRef, NumRef, RefNum};

    // Assert that U256 implements a number of traits
    trait TraitBounds:
        NumAssign + NumAssignRef + NumRef + RefNum<Self> + Binary + BinaryAssignRef
    {
    }
    impl TraitBounds for U256 {}
}
