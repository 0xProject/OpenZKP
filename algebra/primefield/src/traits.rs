// False positives, see <https://github.com/rust-lang/rust/issues/55058>
#![allow(single_use_lifetimes)]

use crate::{AddInline, MulInline, One, Pow, SquareInline, SubInline, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Trait for types implementing field operations
///
/// Like [`num_traits::NumOps`] but for Fields, so without `Rem`.
pub trait FieldOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
{
}
impl<T, Rhs, Output> FieldOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
{
}

pub trait FieldAssignOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs>
{
}
impl<T, Rhs> FieldAssignOps<Rhs> for T where
    T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs>
{
}

pub trait FieldLike:
    Sized
    + Clone
    + PartialEq
    + Eq
    + Zero
    + One
    + for<'a> AddInline<&'a Self>
    + for<'a> SubInline<&'a Self>
    + SquareInline
    + for<'a> MulInline<&'a Self>
    + FieldOps
    + for<'a> FieldOps<&'a Self>
    + FieldAssignOps
    + for<'a> FieldAssignOps<&'a Self>
    + Root<usize>
{
}

impl<T> FieldLike for T where
    Self: Sized
        + Clone
        + PartialEq
        + Eq
        + Zero
        + One
        + for<'a> AddInline<&'a Self>
        + for<'a> SubInline<&'a Self>
        + SquareInline
        + for<'a> MulInline<&'a Self>
        + FieldOps
        + for<'a> FieldOps<&'a Self>
        + FieldAssignOps
        + for<'a> FieldAssignOps<&'a Self>
        + Root<usize>
{
}

pub trait RefFieldLike<Base>:
    Pow<usize, Output = Base> + FieldOps<Base, Base> + for<'r> FieldOps<&'r Base, Base>
{
}

impl<Base> RefFieldLike<Base> for &Base where
    Self: Pow<usize, Output = Base> + FieldOps<Base, Base> + for<'b> FieldOps<&'b Base, Base>
{
}

/// Primitive roots of unity
// TODO: Rename primitive_root ?
pub trait Root<Order>: Sized {
    fn root(order: Order) -> Option<Self>;
}

/// Square roots
pub trait SquareRoot: Sized {
    fn is_quadratic_residue(&self) -> bool;

    fn square_root(&self) -> Option<Self>;
}
