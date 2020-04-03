use super::{Component, PolyWriter};
use crate::{RationalExpression, TraceTable};

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Empty(usize, usize);

impl Empty {
    pub fn new(polynomials: usize, locations: usize) -> Empty {
        Empty(polynomials, locations)
    }
}

impl Component for Empty {
    type Claim = ();
    type Witness = ();

    fn dimensions2(&self) -> (usize, usize) {
        (self.0, self.1)
    }

    fn constraints(&self, _claim: &Self::Claim) -> Vec<RationalExpression> {
        Vec::new()
    }

    fn trace2<P: PolyWriter>(
        &self,
        _trace: &mut P,
        _claim: &Self::Claim,
        _witness: &Self::Witness,
    ) {
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
        proptest!(|(log_locations in 0_usize..10, polynomials in 0_usize..10)| {
            let locations = 1 << log_locations;
            let component = Empty::new(polynomials, locations);
            let claim = ();
            let witness = ();
            prop_assert_eq!(component.check(&claim, &witness), Ok(()));
        });
    }
}
