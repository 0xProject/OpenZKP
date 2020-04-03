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
///
/// $P_0, P_1, \dots P_{n-1}$ of individual degrees $\deg P_i$.
///
/// ```
/// ```
pub trait PolyWriter {
    /// Returns (number of polynomials, number of locations)
    fn dimensions(&self) -> (usize, usize);

    /// Write to a given location
    fn write(&mut self, polynomial: usize, location: usize, value: &FieldElement);
}

impl PolyWriter for TraceTable {
    fn dimensions(&self) -> (usize, usize) {
        TraceTable::dimensions(self)
    }

    fn write(&mut self, polynomial: usize, location: usize, value: &FieldElement) {
        self[(location, polynomial)] = value.clone()
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
                trace.write(polynomial, location, &trace_table[(location, polynomial)]);
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
