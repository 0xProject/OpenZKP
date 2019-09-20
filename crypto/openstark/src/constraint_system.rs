use crate::constraints::Constraints;
#[cfg(feature = "prover")]
use crate::trace_table::TraceTable;

pub trait ConstraintSystem {
    type PrivateInput;

    // TODO: these should return results.
    fn constraints(&self) -> Constraints;
    fn trace_length(&self) -> usize;
    fn trace_columns(&self) -> usize;
    #[cfg(feature = "prover")]
    fn trace(&self, private: &Self::PrivateInput) -> TraceTable;
}
