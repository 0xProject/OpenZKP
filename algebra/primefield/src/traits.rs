use crate::{AddInline, MulInline, One, Pow, SquareInline, SubInline, Zero};
use num_traits::Num;
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

pub trait FieldOpsRef: Sized + for<'r> FieldOps<&'r Self> {}
impl<T> FieldOpsRef for T where T: Sized + for<'r> FieldOps<&'r T> {}

pub trait RefFieldOps<Base>: Sized + for<'r> FieldOps<&'r Base, Base> {}
impl<T, Base> RefFieldOps<Base> for T where T: Sized + for<'r> FieldOps<&'r Base, Base> {}

pub trait FieldAssignOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs>
{
}
impl<T, Rhs> FieldAssignOps<Rhs> for T where
    T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs>
{
}

pub trait FieldAssignOpsRef: for<'r> FieldAssignOps<&'r Self> {}
impl<T> FieldAssignOpsRef for T where T: for<'r> FieldAssignOps<&'r T> {}

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
    + FieldOpsRef
    + FieldAssignOps
    + FieldAssignOpsRef
where
    for<'a> &'a Self: Pow<usize, Output = Self>,
{
}

impl<T> FieldLike for T
where
    T: Sized
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
        + FieldOpsRef
        + FieldAssignOps
        + FieldAssignOpsRef,
    for<'a> &'a T: Pow<usize, Output = Self>,
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
