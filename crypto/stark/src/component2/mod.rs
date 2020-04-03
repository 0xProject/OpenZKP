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
    /// Returns (number of polynomials, number of locations)
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

    // TODO: Remove
    fn dimensions(&self) -> (usize, usize) {
        let (polynomials, locations) = self.dimensions2();
        (locations, polynomials)
    }

    fn dimensions2(&self) -> (usize, usize) {
        let (locations, polynomials) = self.dimensions();
        (polynomials, locations)
    }

    fn constraints(&self, claim: &Self::Claim) -> Vec<RationalExpression>;

    // TODO: Drop `claim` from `trace`
    fn trace2<P: PolyWriter>(&self, trace: &mut P, claim: &Self::Claim, witness: &Self::Witness) {
        let trace_table = self.trace(claim, witness);
        for location in 0..trace_table.num_rows() {
            for polynomial in 0..trace_table.num_columns() {
                trace.write(
                    polynomial,
                    location,
                    trace_table[(location, polynomial)].clone(),
                );
            }
        }
    }

    fn trace(&self, claim: &Self::Claim, witness: &Self::Witness) -> TraceTable {
        let (locations, polynomials) = self.dimensions();
        let mut trace = TraceTable::new(locations, polynomials);
        self.trace2(&mut trace, claim, witness);
        trace
    }

    fn prove(&self, claim: &Self::Claim, witness: &Self::Witness) -> Result<Proof, ProverError> {
        let (polynomials, locations) = self.dimensions2();
        let channel_seed = Vec::new();
        let expressions = self.constraints(claim);
        let mut trace = TraceTable::new(locations, polynomials);
        self.trace2(&mut trace, claim, witness);
        let constraints =
            Constraints::from_expressions((locations, polynomials), channel_seed, expressions)
                .unwrap();
        prove(&constraints, &trace)
    }

    fn verify(&self, claim: &Self::Claim, proof: &Proof) -> Result<(), VerifierError> {
        let (polynomials, locations) = self.dimensions2();
        let channel_seed = Vec::new();
        let expressions = self.constraints(claim);
        let constraints =
            Constraints::from_expressions((locations, polynomials), channel_seed, expressions)
                .unwrap();
        verify(&constraints, proof)
    }

    fn check(&self, claim: &Self::Claim, witness: &Self::Witness) -> Result<(), (usize, usize)> {
        let (polynomials, locations) = self.dimensions2();
        let channel_seed = Vec::new();
        let expressions = self.constraints(claim);
        // TODO: Error handling
        let constraints =
            Constraints::from_expressions((locations, polynomials), channel_seed, expressions)
                .unwrap();
        let mut trace = TraceTable::new(locations, polynomials);
        self.trace2(&mut trace, claim, witness);
        check_constraints(&constraints, &trace)
    }
}
