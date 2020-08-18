// Using `Self` makes things less readable here.
#![allow(clippy::use_self)]

use crate::{One, Parameters, PrimeField, Zero};
use std::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
use zkp_u256::{AddInline, Inv, MulInline, NegInline, SubInline};

macro_rules! assign_ops_from_trait {
    ($rhs:ident, $op_trait:ident, $op_fn:ident, $trait:ident, $trait_assign_fn:ident) => {
        impl<P: Parameters> $op_trait<$rhs> for PrimeField<P> {
            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(&mut self, rhs: $rhs) {
                <PrimeField<P> as $trait<&$rhs>>::$trait_assign_fn(self, &rhs)
            }
        }

        impl<P: Parameters> $op_trait<&$rhs> for PrimeField<P> {
            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(&mut self, rhs: &$rhs) {
                <PrimeField<P> as $trait<&$rhs>>::$trait_assign_fn(self, rhs)
            }
        }
    };
}

macro_rules! self_ops_from_trait {
    ($op_trait:ident, $op_fn:ident, $trait:ident, $trait_fn:ident, $trait_assign_fn:ident) => {
        impl<P: Parameters> $op_trait<&PrimeField<P>> for &PrimeField<P> {
            type Output = PrimeField<P>;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(self, rhs: &Self::Output) -> Self::Output {
                <Self::Output as $trait<&Self::Output>>::$trait_fn(self, rhs)
            }
        }

        impl<P: Parameters> $op_trait<&PrimeField<P>> for PrimeField<P> {
            type Output = PrimeField<P>;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(mut self, rhs: &Self::Output) -> Self::Output {
                <Self::Output as $trait<&Self::Output>>::$trait_assign_fn(&mut self, rhs);
                self
            }
        }

        impl<P: Parameters> $op_trait<PrimeField<P>> for &PrimeField<P> {
            type Output = PrimeField<P>;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(self, mut rhs: Self::Output) -> Self::Output {
                <Self::Output as $trait<&Self::Output>>::$trait_assign_fn(&mut rhs, self);
                rhs
            }
        }

        impl<P: Parameters> $op_trait<PrimeField<P>> for PrimeField<P> {
            type Output = PrimeField<P>;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(mut self, rhs: Self::Output) -> Self::Output {
                <Self::Output as $trait<&Self::Output>>::$trait_assign_fn(&mut self, &rhs);
                self
            }
        }
    };
}

macro_rules! noncommutative_self_ops_from_trait {
    ($op_trait:ident, $op_fn:ident, $trait:ident, $trait_fn:ident, $trait_assign_fn:ident) => {
        impl<P: Parameters> $op_trait<&PrimeField<P>> for &PrimeField<P> {
            type Output = PrimeField<P>;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(self, rhs: &Self::Output) -> Self::Output {
                <Self::Output as $trait<&Self::Output>>::$trait_fn(self, rhs)
            }
        }

        impl<P: Parameters> $op_trait<&PrimeField<P>> for PrimeField<P> {
            type Output = PrimeField<P>;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(mut self, rhs: &Self::Output) -> Self::Output {
                <Self::Output as $trait<&Self::Output>>::$trait_assign_fn(&mut self, rhs);
                self
            }
        }

        impl<P: Parameters> $op_trait<PrimeField<P>> for &PrimeField<P> {
            type Output = PrimeField<P>;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(self, rhs: Self::Output) -> Self::Output {
                <Self::Output as $trait<&Self::Output>>::$trait_fn(self, &rhs)
            }
        }

        impl<P: Parameters> $op_trait<PrimeField<P>> for PrimeField<P> {
            type Output = PrimeField<P>;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(mut self, rhs: Self::Output) -> Self::Output {
                <Self::Output as $trait<&Self::Output>>::$trait_assign_fn(&mut self, &rhs);
                self
            }
        }
    };
}

assign_ops_from_trait!(Self, AddAssign, add_assign, AddInline, add_assign);
assign_ops_from_trait!(Self, SubAssign, sub_assign, SubInline, sub_assign);
assign_ops_from_trait!(Self, MulAssign, mul_assign, MulInline, mul_assign);
self_ops_from_trait!(Add, add, AddInline, add, add_assign);
noncommutative_self_ops_from_trait!(Sub, sub, SubInline, sub, sub_assign);
self_ops_from_trait!(Mul, mul, MulInline, mul, mul_assign);

impl<P: Parameters> Neg for PrimeField<P> {
    type Output = PrimeField<P>;

    #[inline(always)]
    fn neg(mut self) -> Self::Output {
        <Self::Output as NegInline>::neg_assign(&mut self);
        self
    }
}

impl<P: Parameters> Neg for &PrimeField<P> {
    type Output = PrimeField<P>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        <Self::Output as NegInline>::neg(self)
    }
}

impl<P: Parameters> DivAssign<&Self> for PrimeField<P> {
    // Division suspiciously requires multiplication
    #[allow(clippy::suspicious_op_assign_impl)]
    #[inline(always)]
    fn div_assign(&mut self, rhs: &Self) {
        *self *= rhs.inv().expect("Division by zero")
    }
}

impl<P: Parameters> DivAssign<Self> for PrimeField<P> {
    // Division suspiciously requires multiplication
    #[allow(clippy::suspicious_op_assign_impl)]
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv().expect("Division by zero")
    }
}

impl<P: Parameters> Div<&PrimeField<P>> for &PrimeField<P> {
    type Output = PrimeField<P>;

    // Division suspiciously requires multiplication
    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline(always)]
    fn div(self, rhs: &Self::Output) -> Self::Output {
        self * rhs.inv().expect("Division by zero")
    }
}

impl<P: Parameters> Div<PrimeField<P>> for &PrimeField<P> {
    type Output = PrimeField<P>;

    // Division suspiciously requires multiplication
    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline(always)]
    fn div(self, rhs: Self::Output) -> Self::Output {
        self * rhs.inv().expect("Division by zero")
    }
}

impl<P: Parameters> Div<&PrimeField<P>> for PrimeField<P> {
    type Output = PrimeField<P>;

    // Division suspiciously requires multiplication
    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline(always)]
    fn div(self, rhs: &Self::Output) -> Self::Output {
        self * rhs.inv().expect("Division by zero")
    }
}

impl<P: Parameters> Div<PrimeField<P>> for PrimeField<P> {
    type Output = PrimeField<P>;

    // Division suspiciously requires multiplication
    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline(always)]
    fn div(self, rhs: Self::Output) -> Self::Output {
        self * rhs.inv().expect("Division by zero")
    }
}

impl<P: Parameters> Sum<Self> for PrimeField<P> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<'a, P: Parameters> Sum<&'a Self> for PrimeField<P> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + b)
    }
}

impl<P: Parameters> Product<Self> for PrimeField<P> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), Mul::mul)
    }
}

impl<'a, P: Parameters> Product<&'a Self> for PrimeField<P> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::one(), |a, b| a * b)
    }
}
