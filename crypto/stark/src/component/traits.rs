use crate::{
    constraint_check::check_constraints, Constraints, Provable, RationalExpression, TraceTable,
    Verifiable,
};
use std::collections::HashMap;

/// Proof component
///
/// Currently has
/// * Fixed size trace table.
/// * Constraints
/// * Return trace table.
///
/// To do:
/// * Generate constraint system without public input.
/// * Generate
pub trait Component {
    type Claim;
    type Witness;

    fn trace(
        &self,
        claim: &Self::Claim,
        witness: &Self::Witness,
    ) -> (
        Vec<RationalExpression>,
        HashMap<String, (usize, RationalExpression)>,
        TraceTable,
    );

    fn check(&self, claim: &Self::Claim, witness: &Self::Witness) -> Result<(), (usize, usize)> {
        let (dimensions, expressions, _) = self.constraints(claim);
        let channel_seed = Vec::default();

        let constraints = Constraints::from_expressions(dimensions, channel_seed, expressions)
            .expect("Could not produce Constraint object for Component");
        let trace = self.trace(claim, witness);
        check_constraints(&constraints, &trace)
    }
}

impl<T> Verifiable for T
where
    T: Component<Claim = ()>,
{
    fn constraints(&self) -> Constraints {
        let claim = ();
        let (dimensions, expressions, _) = self.constraints(&claim);
        let channel_seed = Vec::default();

        Constraints::from_expressions(dimensions, channel_seed, expressions)
            .expect("Could not produce Constraint object for Component")
    }
}

impl<T, W> Provable<&W> for T
where
    T: Component<Claim = (), Witness = W>,
{
    fn trace(&self, witness: &W) -> TraceTable {
        let claim = ();
        <Self as Component>::trace(self, &claim, witness)
    }
}
