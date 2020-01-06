use crate::U256;
use num_traits::{
    Bounded, FromPrimitive, MulAdd, MulAddAssign, Num, One, ToPrimitive, Unsigned, Zero,
};

impl Bounded for U256 {
    #[inline(always)]
    fn min_value() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    fn max_value() -> Self {
        Self::MAX
    }
}

impl Zero for U256 {
    #[inline(always)]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self == &Self::ZERO
    }
}

impl One for U256 {
    #[inline(always)]
    fn one() -> Self {
        Self::ONE
    }

    #[inline(always)]
    fn is_one(&self) -> bool {
        self == &Self::ONE
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

    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl Unsigned for U256 {}

impl FromPrimitive for U256 {
    #[inline(always)]
    fn from_i64(n: i64) -> Option<Self> {
        Some(Self::from(n))
    }

    #[inline(always)]
    fn from_u64(n: u64) -> Option<Self> {
        Some(Self::from(n))
    }

    // TODO: fn from_u128
    // TODO: fn from_i128
}

impl ToPrimitive for U256 {
    fn to_u128(&self) -> Option<u128> {
        if *self < Self::from_limbs([0, 0, 1, 0]) {
            // Casting u64 to u128 is safe
            #[allow(clippy::cast_lossless)]
            Some((self.limb(0) as u128) | ((self.limb(1) as u128) << 64))
        } else {
            None
        }
    }

    fn to_i128(&self) -> Option<i128> {
        self.to_u128().as_ref().and_then(ToPrimitive::to_i128)
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_u128().as_ref().and_then(ToPrimitive::to_u64)
    }

    fn to_i64(&self) -> Option<i64> {
        self.to_u128().as_ref().and_then(ToPrimitive::to_i64)
    }
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
