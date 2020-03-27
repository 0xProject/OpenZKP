use crate::{
    constraint_check::check_constraints, Constraints, Provable, RationalExpression, TraceTable,
    Verifiable,
};
use std::collections::HashMap;
use zkp_primefield::{FieldElement, Pow, Root};

pub trait Component {
    type Claim;
    type Witness;

    fn dimensions(&self) -> (usize, usize);

    fn constraints(&self, claim: &Self::Claim) -> Vec<RationalExpression>;

    fn trace(&self, claim: &Self::Claim, witness: &Self::Witness) -> TraceTable;

    fn prove(&self, claim: &Self::Claim, witness: &Self::Witness) {
        let _constraints = self.constraints(claim);
        let _trace = self.trace(claim, witness);
        unimplemented!()
    }

    fn verify(&self, claim: &Self::Claim) {
        let _constraints = self.constraints(claim);
        unimplemented!()
    }

    fn check(&self, claim: &Self::Claim, witness: &Self::Witness) {
        unimplemented!()
    }
}

pub struct Empty(usize, usize);

impl Empty {
    fn new(rows: usize, columns: usize) -> Empty {
        Empty(rows, columns)
    }
}

impl Component for Empty {
    type Claim = ();
    type Witness = ();

    fn dimensions(&self) -> (usize, usize) {
        (self.0, self.1)
    }

    fn constraints(&self, claim: &Self::Claim) -> Vec<RationalExpression> {
        Vec::new()
    }

    fn trace(&self, claim: &Self::Claim, witness: &Self::Witness) -> TraceTable {
        TraceTable::new(self.0, self.1)
    }
}

#[cfg(test)]
mod tests {}
