// TODO
#![allow(clippy::module_name_repetitions)]
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};
use num_traits::PrimInt;

/// This is a subset of `num_traits::PrimInt`
// TODO: Submit upstream PR
pub trait Binary:
    Sized
    + Not<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
{
    fn num_bits() -> usize;

    // TODO: What about adding some bit get/set functions?
    // TODO: What about adding digit/limb get/set functions?

    fn bit(&self, i: usize) -> bool;

    fn count_ones(&self) -> usize;
    fn count_zeros(&self) -> usize;
    fn leading_zeros(&self) -> usize;
    fn trailing_zeros(&self) -> usize;

    fn rotate_left(&self, n: usize) -> Self;
    fn rotate_right(&self, n: usize) -> Self;

    // TODO: Deprecate these
    fn bits(&self) -> usize {
        Self::num_bits() - self.leading_zeros()
    }

    /// Returns the position of the most significant set bit, if any.
    fn most_significant_bit(&self) -> Option<usize> {
        (Self::num_bits() - 1).checked_sub(self.leading_zeros())
    }
}

/// Implement Binary for all primitive integers.
impl<T: PrimInt> Binary for T {
    #[inline(always)]
    fn num_bits() -> usize {
        T::zero().count_zeros() as usize
    }

    fn bit(&self, i: usize) -> bool {
        (*self >> i) & T::one() == T::one()
    }

    fn count_ones(&self) -> usize {
        <T as PrimInt>::count_ones(*self) as usize
    }

    fn count_zeros(&self) -> usize {
        <T as PrimInt>::count_zeros(*self) as usize
    }

    fn leading_zeros(&self) -> usize {
        <T as PrimInt>::leading_zeros(*self) as usize

    }

    fn trailing_zeros(&self) -> usize {
        <T as PrimInt>::trailing_zeros(*self) as usize
    }

    fn rotate_left(&self, n: usize) -> Self {
        <T as PrimInt>::rotate_left(*self, n as u32)
    }

    fn rotate_right(&self, n: usize) -> Self {
        <T as PrimInt>::rotate_right(*self, n as u32)
    }
}

pub trait BinaryOps<Rhs = Self, Output = Self>:
    Not<Output = Output>
    + BitAnd<Rhs, Output = Output>
    + BitOr<Rhs, Output = Output>
    + BitXor<Rhs, Output = Output>
{
}

pub trait BinaryAssignOps<Rhs = Self>:
    BitAndAssign<Rhs> + BitOrAssign<Rhs> + BitXorAssign<Rhs> + ShlAssign<usize> + ShrAssign<usize>
{
}

impl<T, Rhs> BinaryAssignOps<Rhs> for T where
    T: BitAndAssign<Rhs>
        + BitOrAssign<Rhs>
        + BitXorAssign<Rhs>
        + ShlAssign<usize>
        + ShrAssign<usize>
{
}

// Does not compile without lifetime annotations
#[allow(single_use_lifetimes)]
pub trait BinaryAssignRef: for<'r> BinaryAssignOps<&'r Self> {}

// Does not compile without lifetime annotations
#[allow(single_use_lifetimes)]
impl<T> BinaryAssignRef for T where T: for<'r> BinaryAssignOps<&'r T> {}
