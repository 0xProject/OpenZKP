use crate::rational_expression::RationalExpression;
use std::prelude::v1::*;

pub struct Constraint {
    pub expr: RationalExpression,
}

// TODO: Show expression
#[cfg(feature = "std")]
impl std::fmt::Debug for Constraint {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Constraint(...)")
    }
}
