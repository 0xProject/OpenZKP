mod constraint;
mod polynomial_expression;
mod trace_expression;

pub(crate) use constraint::{combine_constraints, Constraint};
pub(crate) use polynomial_expression::PolynomialExpression::{Constant, PeriodicColumn, X};
pub(crate) use trace_expression::TraceExpression::Trace;
