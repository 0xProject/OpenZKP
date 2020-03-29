mod empty;
mod fold;
mod horizontal;
mod test;
mod vertical;

use crate::{
    constraint_check::check_constraints,
    proof::Proof,
    prover::prove,
    verifier::{verify, Error as VerifierError},
    Constraints, ProverError, RationalExpression, TraceTable,
};

pub use empty::Empty;
pub use fold::Fold;
pub use horizontal::Horizontal;
pub use test::Test;
pub use vertical::Vertical;

pub trait Component {
    type Claim;
    type Witness;

    fn dimensions(&self) -> (usize, usize);

    fn constraints(&self, claim: &Self::Claim) -> Vec<RationalExpression>;

    fn trace(&self, claim: &Self::Claim, witness: &Self::Witness) -> TraceTable;

    fn prove(&self, claim: &Self::Claim, witness: &Self::Witness) -> Result<Proof, ProverError> {
        let trace_nrows = self.dimensions();
        let channel_seed = Vec::new();
        let expressions = self.constraints(claim);
        let trace = self.trace(claim, witness);
        let constraints =
            Constraints::from_expressions(trace_nrows, channel_seed, expressions).unwrap();
        prove(&constraints, &trace)
    }

    fn verify(&self, claim: &Self::Claim, proof: &Proof) -> Result<(), VerifierError> {
        let trace_nrows = self.dimensions();
        let channel_seed = Vec::new();
        let expressions = self.constraints(claim);
        let constraints =
            Constraints::from_expressions(trace_nrows, channel_seed, expressions).unwrap();
        verify(&constraints, proof)
    }

    fn check(&self, claim: &Self::Claim, witness: &Self::Witness) -> Result<(), (usize, usize)> {
        let trace_nrows = self.dimensions();
        let channel_seed = Vec::new();
        let expressions = self.constraints(claim);
        // TODO: Error handling
        let constraints =
            Constraints::from_expressions(trace_nrows, channel_seed, expressions).unwrap();
        let table = self.trace(claim, witness);
        check_constraints(&constraints, &table)
    }
}
