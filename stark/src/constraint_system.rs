use crate::{Constraints, TraceTable};

pub trait ConstraintSystem {
    type Public;
    type Private;

    // TODO: This should return a `Result` with the `Error` type being associated
    // type.
    fn constraints(public: &Self::Public) -> Constraints;

    // TODO: This should return a `Result` with the `Error` type being associated
    // type. TODO: The prover should check the trace table against the
    // `Constraints` at least in debug mode.
    fn trace(public: &Self::Public, private: &Self::Private) -> TraceTable;
}
