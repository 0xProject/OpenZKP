use crate::{Constraints, ProverChannel, TraceTable};

pub trait ConstraintSystem
where
    ProverChannel: Writable<&Public>,
    VerifierChannel: Replayable<Public>,
{
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
