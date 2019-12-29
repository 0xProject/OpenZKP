use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

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
        256 - self.leading_zeros()
    }

    fn msb(&self) -> usize {
        255 - self.leading_zeros()
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
