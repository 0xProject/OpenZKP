mod empty;
mod fold;
mod horizontal;
mod mapped;
mod test;
mod vertical;

use crate::{
    constraint_check::check_constraints,
    proof::Proof,
    prover::prove,
    verifier::{verify, Error as VerifierError},
    Constraints, ProverError, RationalExpression, TraceTable,
};
use log::trace;
use zkp_primefield::{FieldElement, Root};

pub use empty::Empty;
pub use fold::Fold;
pub use horizontal::Horizontal;
pub use mapped::Mapped;
pub use test::Test;
pub use vertical::Vertical;

/// A set of Polynomials represented by their values at roots of unity.
pub trait PolynomialWriter {
    /// Number of polynomials to commit to.
    fn num_polynomials(&self) -> usize;

    /// The size of the polynomials, i.e. the number of distinct locations that
    /// can be set.
    fn polynomial_size(&self) -> usize;

    /// Write to a given location
    fn write(&mut self, polynomial: usize, location: usize, value: FieldElement);
}

impl PolynomialWriter for TraceTable {
    fn num_polynomials(&self) -> usize {
        self.num_columns()
    }

    fn polynomial_size(&self) -> usize {
        self.num_rows()
    }

    fn write(&mut self, polynomial: usize, location: usize, value: FieldElement) {
        self[(location, polynomial)] = value
    }
}

pub trait Component {
    type Claim;
    type Witness;

    fn claim(&self, witness: &Self::Witness) -> Self::Claim;

    /// Number of polynomials to commit to.
    fn num_polynomials(&self) -> usize;

    /// The size of the polynomials, i.e. the number of distinct locations that
    /// can be set.
    fn polynomial_size(&self) -> usize;

    fn constraints(&self, claim: &Self::Claim) -> Vec<RationalExpression>;

    // TODO: add claim_polynomials function here.

    fn trace<P: PolynomialWriter>(&self, trace: &mut P, witness: &Self::Witness);

    fn trace_generator(&self) -> RationalExpression {
        FieldElement::root(self.polynomial_size())
            .expect("num_polynomials not power of 2.")
            .into()
    }

    /// Construct a trace table
    fn trace_table(&self, witness: &Self::Witness) -> TraceTable {
        trace!("BEGIN Component Trace");
        let polynomials = self.num_polynomials();
        let size = self.polynomial_size();
        let mut trace_table = TraceTable::new(size, polynomials);
        self.trace(&mut trace_table, witness);
        trace!("END Component Trace");
        trace_table
    }

    fn prove(&self, witness: &Self::Witness) -> Result<Proof, ProverError> {
        let polynomials = self.num_polynomials();
        let size = self.polynomial_size();
        let claim = self.claim(witness);
        let channel_seed = Vec::new();
        let expressions = self.constraints(&claim);
        let trace = self.trace_table(witness);
        let constraints =
            Constraints::from_expressions((size, polynomials), channel_seed, expressions).unwrap();
        prove(&constraints, &trace)
    }

    fn verify(&self, claim: &Self::Claim, proof: &Proof) -> Result<(), VerifierError> {
        let polynomials = self.num_polynomials();
        let size = self.polynomial_size();
        let channel_seed = Vec::new();
        let expressions = self.constraints(claim);
        let constraints =
            Constraints::from_expressions((size, polynomials), channel_seed, expressions).unwrap();
        verify(&constraints, proof)
    }

    fn check(&self, witness: &Self::Witness) -> Result<(), (usize, usize)> {
        let polynomials = self.num_polynomials();
        let size = self.polynomial_size();
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
