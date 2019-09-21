use crate::constraints::Constraints;
#[cfg(feature = "prover")]
use crate::trace_table::TraceTable;

pub trait Verifiable {
    fn constraints(&self) -> Constraints;
    fn trace_length(&self) -> usize;
    fn trace_columns(&self) -> usize;

    // fn verify(&self, proof) -> Result<(), VerifierError> {
    //
    // }
}

#[cfg(feature = "prover")]
pub trait Provable<T: Verifiable> {
    fn trace(&self, witness: &T) -> TraceTable;

    // fn prove(&self, claim: &Claim) -> Result<Proof, ProverError> {
    //
    // }
}
