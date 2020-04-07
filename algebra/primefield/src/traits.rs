// False positives, see <https://github.com/rust-lang/rust/issues/55058>
#![allow(single_use_lifetimes)]

use crate::{AddInline, Inv, MulInline, NegInline, One, Pow, SquareInline, SubInline, Zero};
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

/// Trait containing operations provided by [`crate::Field`].
pub trait FieldLike:
    Sized
    + Clone
    + PartialEq
    + Eq
    + Zero
    + One
    + for<'a> AddInline<&'a Self>
    + for<'a> SubInline<&'a Self>
    + NegInline
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
        + NegInline
        + SquareInline
        + for<'a> MulInline<&'a Self>
        + FieldOps
        + for<'a> FieldOps<&'a Self>
        + FieldAssignOps
        + for<'a> FieldAssignOps<&'a Self>
        + Root<usize>
{
}

// TODO: Custom by-reference traits for Pow and Inv
pub trait RefFieldLike<Base>:
    Inv<Output = Option<Base>>
    + Pow<usize, Output = Base>
    + FieldOps<Base, Base>
    + for<'r> FieldOps<&'r Base, Base>
{
}
impl<Base> RefFieldLike<Base> for &Base where
    Self: Inv<Output = Option<Base>>
        + Pow<usize, Output = Base>
        + FieldOps<Base, Base>
        + for<'b> FieldOps<&'b Base, Base>
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

/// Fast-Fourier Transforms
///
/// This trait is intended to apply on containers like `[T]`
pub trait Fft<T> {
    /// In-place permuted FFT.
    fn fft(&mut self);

    /// In-place permuted inverse FFT.
    ///
    /// Note, it requires inpute to be in non-permuted order and output
    /// will be in permuted order.
    fn ifft(&mut self);

    /// Copy values from source multiplied by powers of cofactor.
    fn clone_shifted(&mut self, source: &[T], cofactor: &T);

    /// In-place permuted FFT with a cofactor.
    fn fft_cofactor(&mut self, cofactor: &T);

    /// In-place permuted inverse FFT with a cofactor.
    fn ifft_cofactor(&mut self, cofactor: &T);

    /// In-place permuted FFT.
    fn fft_root(&mut self, root: &T);
}
