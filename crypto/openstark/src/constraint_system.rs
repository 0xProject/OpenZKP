use crate::constraints::Constraints;
#[cfg(feature = "prover")]
use crate::trace_table::TraceTable;

#[allow(dead_code)]
pub(crate) trait ConstraintSystem {
    type PrivateInput;

    // TODO: these should return results.
    fn constraints(&self) -> Constraints;
    #[cfg(feature = "prover")]
    fn trace(&self, private: &Self::PrivateInput) -> TraceTable;
}
