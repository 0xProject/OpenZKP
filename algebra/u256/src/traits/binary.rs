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

    fn count_ones(self) -> u32;
    fn count_zeros(self) -> u32;
    fn leading_zeros(self) -> u32;
    fn trailing_zeros(self) -> u32;
    fn rotate_left(self, n: u32) -> Self;
    fn rotate_right(self, n: u32) -> Self;
    // TODO: Keep these?
    // fn signed_shl(self, n: u32) -> Self;
    // fn signed_shr(self, n: u32) -> Self;
    // fn unsigned_shl(self, n: u32) -> Self;
    // fn unsigned_shr(self, n: u32) -> Self;
    fn swap_bytes(self) -> Self;

    // TODO: What about these
    fn from_be(x: Self) -> Self;
    fn from_le(x: Self) -> Self;
    fn to_be(self) -> Self;
    fn to_le(self) -> Self;
}

pub trait BinaryAssign:
    Sized + BitAndAssign + BitOrAssign + BitXorAssign + ShlAssign<usize> + ShrAssign<usize>
{
}
