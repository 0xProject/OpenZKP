/// Implement infix operator using assignment version.
#[macro_export]
macro_rules! commutative_binop {
    ($type:ident, $trait:ident, $trait_fn:ident, $inplace:ident, $inplace_fn:ident) => {
        // &mut <op>= value
        // Note: a value is wasted
        impl $inplace<$type> for $type {
            #[inline(always)] // Simple wrapper in hot path
            fn $inplace_fn(&mut self, rhs: Self) {
                self.$inplace_fn(&rhs)
            }
        }

        // Value <op> value
        // Note: a value is wasted
        impl $trait<$type> for $type {
            type Output = Self;

            #[inline(always)] // Simple wrapper in hot path
            fn $trait_fn(self, rhs: Self) -> Self {
                self.$trait_fn(&rhs)
            }
        }

        // Value <op> reference
        impl $trait<&$type> for $type {
            type Output = Self;

            #[inline(always)] // Simple wrapper in hot path
            fn $trait_fn(mut self, rhs: &Self) -> Self {
                self.$inplace_fn(rhs);
                self
            }
        }

        // Reference <op> value
        impl $trait<$type> for &$type {
            type Output = $type;

            #[inline(always)] // Simple wrapper in hot path
            fn $trait_fn(self, rhs: $type) -> $type {
                rhs.$trait_fn(self)
            }
        }

        // Reference <op> reference
        // Note: a clone is necessary
        impl $trait<&$type> for &$type {
            type Output = $type;

            #[inline(always)] // Simple wrapper in hot path
            fn $trait_fn(self, rhs: &$type) -> $type {
                self.clone().$trait_fn(rhs)
            }
        }
    };
}

/// Implement infix operator using assignment version.
/// It is assumed `OpAssign<&Type>` is implemented.
#[macro_export]
macro_rules! noncommutative_binop {
    ($type:ident, $trait:ident, $trait_fn:ident, $inplace:ident, $inplace_fn:ident) => {
        // &mut <op>= value
        // Note: a value is wasted
        impl $inplace<$type> for $type {
            #[inline(always)] // Simple wrapper in hot path
            fn $inplace_fn(&mut self, rhs: Self) {
                self.$inplace_fn(&rhs)
            }
        }

        // Value <op> value
        // Note: a value is wasted
        impl $trait<$type> for $type {
            type Output = Self;

            #[inline(always)] // Simple wrapper in hot path
            fn $trait_fn(self, rhs: Self) -> Self {
                self.$trait_fn(&rhs)
            }
        }

        // Value <op> reference
        impl $trait<&$type> for $type {
            type Output = Self;

            #[inline(always)] // Simple wrapper in hot path
            fn $trait_fn(mut self, rhs: &Self) -> Self {
                self.$inplace_fn(rhs);
                self
            }
        }

        // Reference <op> value
        // Note: a clone is necessary
        impl $trait<$type> for &$type {
            type Output = $type;

            #[inline(always)] // Simple wrapper in hot path
            fn $trait_fn(self, rhs: $type) -> $type {
                // TODO: Use places-reversed version of in-place operator instead.
                self.clone().$trait_fn(rhs)
            }
        }

        // Reference <op> reference
        // Note: a clone is necessary
        impl $trait<&$type> for &$type {
            type Output = $type;

            #[inline(always)] // Simple wrapper in hot path
            fn $trait_fn(self, rhs: &$type) -> $type {
                self.clone().$trait_fn(rhs)
            }
        }
    };
}

/// Implement assignment operator using OpInline trait.
#[macro_export]
macro_rules! assign_ops_from_trait {
    ($type:ident, $rhs:ident, $op_trait:ident, $op_fn:ident, $trait:ident, $trait_assign_fn:ident) => {
        impl $op_trait<$rhs> for $type {
            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(&mut self, rhs: $rhs) {
                <$type as $trait<&$rhs>>::$trait_assign_fn(self, &rhs)
            }
        }

        impl $op_trait<&$rhs> for $type {
            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(&mut self, rhs: &$rhs) {
                <$type as $trait<&$rhs>>::$trait_assign_fn(self, rhs)
            }
        }
    }
}

/// Implement infix operator using OpInline trait, preferring _assign versions
/// where possible.
#[macro_export]
macro_rules! self_ops_from_trait {
    ($type:ident, $op_trait:ident, $op_fn:ident, $trait:ident, $trait_fn:ident, $trait_assign_fn:ident) => {
        impl $op_trait<&$type> for &$type {
            type Output = $type;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(self, rhs: &$type) -> $type {
                <$type as $trait<&$type>>::$trait_fn(self, rhs)
            }
        }

        impl $op_trait<&$type> for $type {
            type Output = $type;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(mut self, rhs: &$type) -> $type {
                <$type as $trait<&$type>>::$trait_assign_fn(&mut self, rhs);
                self
            }
        }

        impl $op_trait<$type> for &$type {
            type Output = $type;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(self, mut rhs: $type) -> $type {
                <$type as $trait<&$type>>::$trait_assign_fn(&mut rhs, self);
                rhs
            }
        }

        impl $op_trait<$type> for $type {
            type Output = $type;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(mut self, rhs: $type) -> $type {
                <$type as $trait<&$type>>::$trait_assign_fn(&mut self, &rhs);
                self
            }
        }
    }
}


/// Implement infix operator using OpInline trait, preferring _assign versions
/// where possible.
#[macro_export]
macro_rules! noncommutative_self_ops_from_trait {
    ($type:ident, $op_trait:ident, $op_fn:ident, $trait:ident, $trait_fn:ident, $trait_assign_fn:ident) => {
        impl $op_trait<&$type> for &$type {
            type Output = $type;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(self, rhs: &$type) -> $type {
                <$type as $trait<&$type>>::$trait_fn(self, rhs)
            }
        }

        impl $op_trait<&$type> for $type {
            type Output = $type;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(mut self, rhs: &$type) -> $type {
                <$type as $trait<&$type>>::$trait_assign_fn(&mut self, rhs);
                self
            }
        }

        impl $op_trait<$type> for &$type {
            type Output = $type;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(self, rhs: $type) -> $type {
                <$type as $trait<&$type>>::$trait_fn(self, &rhs)
            }
        }

        impl $op_trait<$type> for $type {
            type Output = $type;

            #[inline(always)] // Simple wrapper in hot path
            fn $op_fn(mut self, rhs: $type) -> $type {
                <$type as $trait<&$type>>::$trait_assign_fn(&mut self, &rhs);
                self
            }
        }
    }
}
