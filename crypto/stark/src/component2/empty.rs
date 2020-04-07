use super::{Component, PolynomialWriter};
use crate::RationalExpression;

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Empty(usize, usize);

impl Empty {
    pub fn new(polynomials: usize, size: usize) -> Empty {
        Empty(polynomials, size)
    }
}

impl Component for Empty {
    type Claim = ();
    type Witness = ();

    fn num_polynomials(&self) -> usize {
        self.0
    }

    fn polynomial_size(&self) -> usize {
        self.1
    }

    fn claim(&self, _witness: &Self::Witness) -> Self::Claim {}

    fn constraints(&self, _claim: &Self::Claim) -> Vec<RationalExpression> {
        Vec::new()
    }

    fn trace<P: PolynomialWriter>(&self, _trace: &mut P, _witness: &Self::Witness) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // More readable being explicit
    #[allow(clippy::let_unit_value)]
    #[test]
    fn test_empty_check() {
        proptest!(|(log_size in 0_usize..10, polynomials in 0_usize..10)| {
            let size = 1 << log_size;
            let component = Empty::new(polynomials, size);
            let witness = ();
            prop_assert_eq!(component.check(&witness), Ok(()));
        });
    }
}
