#[cfg(feature = "prover")]
use crate::TraceTable;
use crate::{
    constraint_system::{Provable, Verifiable},
    constraints::Constraints,
    polynomial::DensePolynomial,
    rational_expression::RationalExpression::*,
    rational_expression::RationalExpression,
};
use macros_decl::field_element;
use primefield::{fft::ifft, FieldElement};
use u256::U256;
use super::elliptic_helpers::*;

struct Claim {
    hash: U256,
    who: (FieldElement, FieldElement),
}

struct Witness {
    signatures: (FieldElement, FieldElement),
}

impl Verifiable for Claim {
    fn constraints(&self) -> Constraints {
        use RationalExpression::*;

        let trace_length = self.trace_length();
        let trace_generator = FieldElement::root(trace_length).unwrap();

        // Constraint repetitions
        let g = Constant(trace_generator);
        let on_row = |index| (X - g.pow(index)).inv();
        let reevery_row = || (X - g.pow(trace_length - 1)) / (X.pow(trace_length) - 1.into());

        Constraints::new(vec![

        ])
    }

    fn trace_length(&self) -> usize {
        512
    }

    fn trace_columns(&self) -> usize {
        5
    }
}

impl Provable<Claim> for Witness {
    #[cfg(feature = "prover")]
    fn trace(&self, claim: &Claim) -> TraceTable {
        let mut trace = TraceTable::new(512, 5);

        trace
    }
}