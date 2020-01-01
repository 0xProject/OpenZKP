// False positives, see <https://github.com/rust-lang/rust/issues/55058>
#![allow(single_use_lifetimes)]

use crate::{AddInline, Inv, MulInline, One, Pow, SquareInline, SubInline, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use zkp_u256::Montgomery;

/// Requirements for the base unsigned integer type
// TODO: Fix naming
#[allow(clippy::module_name_repetitions)]
// Lint has a false positive here
#[allow(single_use_lifetimes)]
pub trait UInt:
    Clone
    + PartialEq
    + PartialOrd
    + Zero
    + One
    + for<'a> AddInline<&'a Self>
    + for<'a> SubInline<&'a Self>
    + Montgomery
{
}
impl<T> UInt for T where
    T: Clone
        + PartialEq
        + PartialOrd
        + Zero
        + One
        + for<'a> AddInline<&'a T>
        + for<'a> SubInline<&'a T>
        + Montgomery
{
}
