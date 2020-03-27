use crate::{constraint_check::check_constraints, Constraints, RationalExpression, TraceTable};

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
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest!(
        #[test]
        fn test_empty(log_rows in 0_usize..10, cols in 0_usize..10) {
            let rows = 1 << log_rows;
            let component = Empty::new(rows, cols);
            let claim = ();
            let witness = ();
            prop_assert_eq!(component.check(&claim, &witness), Ok(()));
        }
    );
}
