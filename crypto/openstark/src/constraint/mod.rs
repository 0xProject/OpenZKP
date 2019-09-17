mod constraint;
mod polynomial_expression;
mod trace_expression;

pub(crate) use constraint::{combine_constraints, Constraint};
#[cfg(feature = "prover")]
pub(crate) use polynomial_expression::PolynomialExpression::PeriodicColumn;
pub(crate) use polynomial_expression::PolynomialExpression::{Constant, X};
pub(crate) use trace_expression::TraceExpression::Trace;
