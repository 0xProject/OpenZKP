use crate::Constraints;
#[cfg(feature = "prover")]
use crate::TraceTable;

pub trait Verifiable {
    fn constraints(&self) -> Constraints;

    // fn verify(&self, proof) -> Result<(), VerifierError> {
    //
    // }
}

#[cfg(feature = "prover")]
pub trait Provable<T>: Verifiable {
    fn trace(&self, witness: T) -> TraceTable;

    // fn prove(&self, witness: &T) -> Result<Proof, ProverError> {
    //     let seed = Vec::from(self);
    //     let constraints = self.constraints();
    //     let trace = self.trace(witness);
    //     let domain_size = constraints.trace_nrows().trailing_zeros();
    //     let params = ProofParams::suggested(domain_size);
    //     proof(&seed, &constraints, &trace, &params)
    // }
}
