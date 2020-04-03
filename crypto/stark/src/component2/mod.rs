mod empty;
mod fold;
mod horizontal;
mod test;
mod transformed;
mod vertical;

use crate::{
    constraint_check::check_constraints,
    proof::Proof,
    prover::prove,
    verifier::{verify, Error as VerifierError},
    Constraints, ProverError, RationalExpression, TraceTable,
};
use std::ops::{Index, IndexMut};
use zkp_primefield::FieldElement;

pub use empty::Empty;
pub use fold::Fold;
pub use horizontal::Horizontal;
pub use test::Test;
pub use vertical::Vertical;

/// A set of Polynomials represented by their values at roots of unity.
///
/// $P_0, P_1, \dots P_{n-1}$ of individual degrees $\deg P_i$.
///
/// ```
/// ```
pub trait Polynomials: IndexMut<(usize, usize), Output = FieldElement> {
    /// The number of Polynomials in the set.
    fn count(&self) -> usize;

    /// The size of the `n`-th Polynomial
    ///
    /// A polynomial's `size` is $\deg(P) + 1$ and represents the number of
    /// distinct values it can represent.
    ///
    /// Panics if `n > count()`.
    fn size(&self, n: usize) -> usize;
}

impl PolynomialBuilder for TraceTable {
    fn count(&self) -> usize {
        self.num_columns()
    }

    fn size(&self, n: usize) -> usize {
        assert!(n < self.num_columns());
        self.num_rows()
    }
}

pub trait Component {
    type Claim;
    type Witness;

    fn dimensions(&self) -> (usize, usize);

    fn constraints(&self, claim: &Self::Claim) -> Vec<RationalExpression>;

    fn trace2<P: PolynomialBuilder>(
        &self,
        trace: &mut P,
        claim: &Self::Claim,
        witness: &Self::Witness,
    ) {
        unimplemented!()
    }

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
