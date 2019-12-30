use std::{
    ops::{Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, Neg, ShrAssign, Sub, SubAssign},
    prelude::v1::*,
};
use zkp_u256::{
    AddInline, SubInline, MulInline, NegInline
};
use crate::{FieldUInt, FieldParameters, Field};

macro_rules! assign_ops_from_trait {
    (
        $rhs:ident,
        $op_trait:ident,
        $op_fn:ident,
        $trait:ident,
        $trait_assign_fn:ident
    ) => {
        impl<UInt, Parameters> $op_trait<$rhs> for Field<UInt, Parameters>
        where
            UInt: FieldUInt,
            Parameters: FieldParameters<UInt>,
        {
            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(&mut self, rhs: $rhs) {
                <Field<UInt, Parameters> as $trait<&$rhs>>::$trait_assign_fn(self, &rhs)
            }
        }

        impl<UInt, Parameters> $op_trait<&$rhs> for Field<UInt, Parameters>
        where
            UInt: FieldUInt,
            Parameters: FieldParameters<UInt>,
        {
            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(&mut self, rhs: &$rhs) {
                <Field<UInt, Parameters> as $trait<&$rhs>>::$trait_assign_fn(self, rhs)
            }
        }
    };
}

macro_rules! self_ops_from_trait {
    (
        $op_trait:ident,
        $op_fn:ident,
        $trait:ident,
        $trait_fn:ident,
        $trait_assign_fn:ident
    ) => {
        impl<UInt, Parameters> $op_trait<&Field<UInt, Parameters>> for &Field<UInt, Parameters>
        where
            UInt: FieldUInt,
            Parameters: FieldParameters<UInt>,
        {
            type Output = Field<UInt, Parameters>;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(self, rhs: &Self::Output) -> Self::Output {
                <Self::Output as $trait<&Self::Output>>::$trait_fn(self, rhs)
            }
        }

        impl<UInt, Parameters> $op_trait<&Field<UInt, Parameters>> for Field<UInt, Parameters>
        where
            UInt: FieldUInt,
            Parameters: FieldParameters<UInt>,
        {
            type Output = Field<UInt, Parameters>;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(mut self, rhs: &Self::Output) -> Self::Output {
                <Self::Output as $trait<&Self::Output>>::$trait_assign_fn(&mut self, rhs);
                self
            }
        }

        impl<UInt, Parameters> $op_trait<Field<UInt, Parameters>> for &Field<UInt, Parameters>
        where
            UInt: FieldUInt,
            Parameters: FieldParameters<UInt>,
        {
            type Output = Field<UInt, Parameters>;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(self, mut rhs: Self::Output) -> Self::Output {
                <Self::Output as $trait<&Self::Output>>::$trait_assign_fn(&mut rhs, self);
                rhs
            }
        }

        impl<UInt, Parameters> $op_trait<Field<UInt, Parameters>> for Field<UInt, Parameters>
        where
            UInt: FieldUInt,
            Parameters: FieldParameters<UInt>,
        {
            type Output = Field<UInt, Parameters>;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(mut self, rhs: Self::Output) -> Self::Output {
                <Self::Output as $trait<&Self::Output>>::$trait_assign_fn(&mut self, &rhs);
                self
            }
        }
    };
}

macro_rules! noncommutative_self_ops_from_trait {
    (
        $op_trait:ident,
        $op_fn:ident,
        $trait:ident,
        $trait_fn:ident,
        $trait_assign_fn:ident
    ) => {
        impl<UInt, Parameters> $op_trait<&Field<UInt, Parameters>> for &Field<UInt, Parameters>
        where
            UInt: FieldUInt,
            Parameters: FieldParameters<UInt>,
        {
            type Output = Field<UInt, Parameters>;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(self, rhs: &Self::Output) -> Self::Output {
                <Self::Output as $trait<&Self::Output>>::$trait_fn(self, rhs)
            }
        }

        impl<UInt, Parameters> $op_trait<&Field<UInt, Parameters>> for Field<UInt, Parameters>
        where
            UInt: FieldUInt,
            Parameters: FieldParameters<UInt>,
        {
            type Output = Field<UInt, Parameters>;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(mut self, rhs: &Self::Output) -> Self::Output {
                <Self::Output as $trait<&Self::Output>>::$trait_assign_fn(&mut self, rhs);
                self
            }
        }

        impl<UInt, Parameters> $op_trait<Field<UInt, Parameters>> for &Field<UInt, Parameters>
        where
            UInt: FieldUInt,
            Parameters: FieldParameters<UInt>,
        {
            type Output = Field<UInt, Parameters>;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(self, rhs: Self::Output) -> Self::Output {
                <Self::Output as $trait<&Self::Output>>::$trait_fn(self, &rhs);
                rhs
            }
        }

        impl<UInt, Parameters> $op_trait<Field<UInt, Parameters>> for Field<UInt, Parameters>
        where
            UInt: FieldUInt,
            Parameters: FieldParameters<UInt>,
        {
            type Output = Field<UInt, Parameters>;

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

impl<UInt, Parameters> Neg for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    type Output = Field<UInt, Parameters>;

    #[inline(always)]
    fn neg(mut self) -> Self::Output {
        <Self::Output as NegInline>::neg_assign(&mut self);
        self
    }
}

impl<UInt, Parameters> Neg for &Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    type Output = Field<UInt, Parameters>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        <Self::Output as NegInline>::neg(self)
    }
}


impl<UInt, Parameters> DivAssign<&Self> for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    #[inline(always)]
    fn div_assign(&mut self, rhs: &Self) {
        *self *= rhs.inv().expect("Division by zero")
    }
}

impl<UInt, Parameters> DivAssign<Self> for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv().expect("Division by zero")
    }
}

impl<UInt, Parameters> Div<Self> for &Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    type Output = Field<UInt, Parameters>;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv().expect("Division by zero")
    }
}

impl<UInt, Parameters> Div<Field<UInt, Parameters>> for &Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    type Output = Field<UInt, Parameters>;

    #[inline(always)]
    fn div(self, rhs: Self::Output) -> Self::Output {
        self * rhs.inv().expect("Division by zero")
    }
}

impl<UInt, Parameters> Div<&Field<UInt, Parameters>> for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    type Output = Field<UInt, Parameters>;

    #[inline(always)]
    fn div(self, rhs: &Self::Output) -> Self::Output {
        self * rhs.inv().expect("Division by zero")
    }
}

impl<UInt, Parameters> Div<Field<UInt, Parameters>> for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    type Output = Field<UInt, Parameters>;

    #[inline(always)]
    fn div(self, rhs: Self::Output) -> Self::Output {
        self * rhs.inv().expect("Division by zero")
    }
}
