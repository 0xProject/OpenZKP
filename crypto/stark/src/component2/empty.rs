use super::Component;
use crate::{RationalExpression, TraceTable};

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Empty(usize, usize);

impl Empty {
    pub fn new(rows: usize, columns: usize) -> Empty {
        Empty(rows, columns)
    }
}

impl Component for Empty {
    type Claim = ();
    type Witness = ();

    fn dimensions(&self) -> (usize, usize) {
        (self.0, self.1)
    }

    fn constraints(&self, _claim: &Self::Claim) -> Vec<RationalExpression> {
        Vec::new()
    }

    fn trace(&self, _claim: &Self::Claim, _witness: &Self::Witness) -> TraceTable {
        TraceTable::new(self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // More readable being explicit
    #[allow(clippy::let_unit_value)]
    #[test]
    fn test_empty_check() {
        proptest!(|(log_rows in 0_usize..10, cols in 0_usize..10)| {
            let rows = 1 << log_rows;
            let component = Empty::new(rows, cols);
            let claim = ();
            let witness = ();
            prop_assert_eq!(component.check(&claim, &witness), Ok(()));
        });
    }
}
