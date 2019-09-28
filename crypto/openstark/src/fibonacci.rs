use crate::{
    constraint_system::Verifiable, constraints::Constraints,
    rational_expression::RationalExpression,
};
use primefield::FieldElement;
use std::{convert::TryInto, prelude::v1::*};
use u256::U256;

#[cfg(feature = "prover")]
use crate::constraint_system::Provable;
#[cfg(feature = "prover")]
use crate::TraceTable;

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Claim {
    pub index: usize,
    pub value: FieldElement,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Witness {
    pub secret: FieldElement,
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

        Constraints::from_expressions(
            (trace_length, 2),
        vec![
            (Trace(0, 1) - Trace(1, 0)) * reevery_row(),
            (Trace(1, 1) - Trace(0, 0) - Trace(1, 0)) * reevery_row(),
            (Trace(0, 0) - 1.into()) * on_row(0),
            (Trace(0, 0) - (&self.value).into()) * on_row(self.index),
        ]).unwrap()
    }

    fn trace_length(&self) -> usize {
        self.index.next_power_of_two()
    }

    fn trace_columns(&self) -> usize {
        2
    }
}

#[cfg(feature = "prover")]
impl Provable<Claim> for Witness {
    fn trace(&self, claim: &Claim) -> TraceTable {
        let mut trace = TraceTable::new(claim.trace_length(), 2);
        trace[(0, 0)] = 1.into();
        trace[(0, 1)] = self.secret.clone();
        for i in 0..(claim.trace_length() - 1) {
            trace[(i + 1, 0)] = trace[(i, 1)].clone();
            trace[(i + 1, 1)] = &trace[(i, 0)] + &trace[(i, 1)];
        }
        trace
    }
}

impl From<&Claim> for Vec<u8> {
    fn from(claim: &Claim) -> Self {
        let mut bytes = [claim.index.to_be_bytes()].concat();
        bytes.extend_from_slice(&claim.value.as_montgomery().to_bytes_be());
        bytes
    }
}

// Used in substrate-runtime
impl From<&[u8]> for Claim {
    fn from(claim: &[u8]) -> Self {
        assert!(claim.len() >= 40);
        let index64 = u64::from_be_bytes((&claim[0..8]).try_into().unwrap());
        // TODO: Use TryFrom
        #[allow(clippy::cast_possible_truncation)]
        let index = index64 as usize;
        let value =
            FieldElement::from_montgomery(U256::from_bytes_be((&claim[8..40]).try_into().unwrap()));
        Self { index, value }
    }
}

pub fn get_value(index: usize, secret: &FieldElement) -> FieldElement {
    let mut x = FieldElement::ONE;
    let mut y = secret.clone();
    for _ in 0..index {
        let (a, b) = (y.clone(), x + y);
        x = a;
        y = b;
    }
    x
}
