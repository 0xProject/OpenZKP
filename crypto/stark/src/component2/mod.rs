mod empty;
mod fold;
mod horizontal;
mod mapped;
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
pub use mapped::Mapped;
pub use test::Test;
pub use transformed::{Transform, Transformed};
pub use vertical::Vertical;

/// A set of Polynomials represented by their values at roots of unity.
pub trait PolyWriter {
    /// Returns (number of polynomials, number of size)
    fn dimensions(&self) -> (usize, usize);

    /// Write to a given location
    fn write(&mut self, polynomial: usize, location: usize, value: FieldElement);
}

impl PolyWriter for TraceTable {
    // Returns the number of polynomials and the size of the polynomials. All
    // polynomials have the same size.
    fn dimensions(&self) -> (usize, usize) {
        (self.num_columns(), self.num_rows())
    }

    fn write(&mut self, polynomial: usize, location: usize, value: FieldElement) {
        self[(location, polynomial)] = value
    }
}

pub trait Component {
    type Claim;
    type Witness;

    fn claim(&self, witness: &Self::Witness) -> Self::Claim;

    fn dimensions2(&self) -> (usize, usize);

    fn constraints(&self, claim: &Self::Claim) -> Vec<RationalExpression>;

    fn trace2<P: PolyWriter>(&self, trace: &mut P, witness: &Self::Witness);

    /// Construct a trace table
    fn trace_table(&self, witness: &Self::Witness) -> TraceTable {
        let (polynomials, size) = self.dimensions2();
        let mut trace_table = TraceTable::new(size, polynomials);
        self.trace2(&mut trace_table, witness);
        trace_table
    }

    fn prove(&self, witness: &Self::Witness) -> Result<Proof, ProverError> {
        let (polynomials, size) = self.dimensions2();
        let claim = self.claim(witness);
        let channel_seed = Vec::new();
        let expressions = self.constraints(&claim);
        let trace = self.trace_table(witness);
        let constraints =
            Constraints::from_expressions((size, polynomials), channel_seed, expressions).unwrap();
        prove(&constraints, &trace)
    }

    fn verify(&self, claim: &Self::Claim, proof: &Proof) -> Result<(), VerifierError> {
        let (polynomials, size) = self.dimensions2();
        let channel_seed = Vec::new();
        let expressions = self.constraints(claim);
        let constraints =
            Constraints::from_expressions((size, polynomials), channel_seed, expressions).unwrap();
        verify(&constraints, proof)
    }

    fn check(&self, witness: &Self::Witness) -> Result<(), (usize, usize)> {
        let (polynomials, size) = self.dimensions2();
        let claim = self.claim(witness);
        let channel_seed = Vec::new();
        let expressions = self.constraints(&claim);
        // TODO: Error handling
        let constraints =
            Constraints::from_expressions((size, polynomials), channel_seed, expressions).unwrap();
        let trace = self.trace_table(witness);
        check_constraints(&constraints, &trace)
    }
}
