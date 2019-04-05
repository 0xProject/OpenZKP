/// Implement infix operator using assignment version.
#[macro_export]
macro_rules! commutative_binop {
    ($type:ident, $trait:ident, $trait_fn:ident, $inplace:ident, $inplace_fn:ident) => {
        // &mut <op>= value
        // Note: a value is wasted
        impl $inplace<$type> for $type {
            #[inline(always)]
            fn $inplace_fn(&mut self, rhs: $type) {
                self.$inplace_fn(&rhs)
            }
        }

        // Value <op> value
        // Note: a value is wasted
        impl $trait<$type> for $type {
            type Output = $type;
            #[inline(always)]
            fn $trait_fn(self, rhs: $type) -> $type {
                self.$trait_fn(&rhs)
            }
        }

        // Value <op> reference
        impl $trait<&$type> for $type {
            type Output = $type;
            #[inline(always)]
            fn $trait_fn(mut self, rhs: &$type) -> $type {
                self.$inplace_fn(rhs);
                self
            }
        }

        // Reference <op> value
        impl $trait<$type> for &$type {
            type Output = $type;
            #[inline(always)]
            fn $trait_fn(self, rhs: $type) -> $type {
                rhs.$trait_fn(self)
            }
        }

        // Reference <op> reference
        // Note: a clone is necessary
        impl $trait<&$type> for &$type {
            type Output = $type;
            #[inline(always)]
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
            #[inline(always)]
            fn $inplace_fn(&mut self, rhs: $type) {
                self.$inplace_fn(&rhs)
            }
        }

        // Value <op> value
        // Note: a value is wasted
        impl $trait<$type> for $type {
            type Output = $type;
            #[inline(always)]
            fn $trait_fn(self, rhs: $type) -> $type {
                self.$trait_fn(&rhs)
            }
        }

        // Value <op> reference
        impl $trait<&$type> for $type {
            type Output = $type;
            #[inline(always)]
            fn $trait_fn(mut self, rhs: &$type) -> $type {
                self.$inplace_fn(rhs);
                self
            }
        }

        // Reference <op> value
        // Note: a clone is necessary
        impl $trait<$type> for &$type {
            type Output = $type;
            #[inline(always)]
            fn $trait_fn(self, rhs: $type) -> $type {
                // TODO: Use places-reversed version of in-place operator instead.
                self.clone().$trait_fn(rhs)
            }
        }

        // Reference <op> reference
        // Note: a clone is necessary
        impl $trait<&$type> for &$type {
            type Output = $type;
            #[inline(always)]
            fn $trait_fn(self, rhs: &$type) -> $type {
                self.clone().$trait_fn(rhs)
            }
        }
    };
}
