mod constraint;
mod trace_expression;
// mod fraction;
mod polynomial_expression;
// mod trace_multinomial;

pub use constraint::{combine_constraints, Constraint};
pub use polynomial_expression::PolynomialExpression::{Constant, PeriodicColumn, X};
pub use trace_expression::TraceExpression::Trace;
