use crate::{constraint::Constraint, polynomial::SparsePolynomial};
use primefield::FieldElement;
use std::{convert::TryInto, prelude::v1::*};
use u256::U256;

#[cfg(feature = "prover")]
use crate::TraceTable;

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct PublicInput {
    pub index: usize,
    pub value: FieldElement,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct PrivateInput {
    pub secret: FieldElement,
}

impl From<&PublicInput> for Vec<u8> {
    fn from(public_input: &PublicInput) -> Self {
        let mut bytes = [public_input.index.to_be_bytes()].concat();
        bytes.extend_from_slice(&public_input.value.as_montgomery().to_bytes_be());
        bytes
    }
}

// Used in substrate-runtime
impl From<&[u8]> for PublicInput {
    fn from(public_input: &[u8]) -> Self {
        assert!(public_input.len() >= 40);
        let index64 = u64::from_be_bytes((&public_input[0..8]).try_into().unwrap());
        // TODO: Use TryFrom
        #[allow(clippy::cast_possible_truncation)]
        let index = index64 as usize;
        let value = FieldElement::from_montgomery(U256::from_bytes_be(
            (&public_input[8..40]).try_into().unwrap(),
        ));
        Self { index, value }
    }
}

#[cfg(feature = "prover")]
pub fn get_trace_table(length: usize, private: &PrivateInput) -> TraceTable {
    // Compute trace table
    let mut trace = TraceTable::new(length, 2);
    trace[(0, 0)] = 1.into();
    trace[(0, 1)] = private.secret.clone();
    for i in 0..(length - 1) {
        trace[(i + 1, 0)] = trace[(i, 1)].clone();
        trace[(i + 1, 1)] = &trace[(i, 0)] + &trace[(i, 1)];
    }
    trace
}

pub fn get_fibonacci_constraints(public_input: &PublicInput) -> Vec<Constraint> {
    let trace_length = public_input.index.next_power_of_two();
    let claim_index = public_input.index;
    let claim_value = public_input.value.clone();

    let trace_generator = FieldElement::root(trace_length).unwrap();

    let no_rows = SparsePolynomial::new(&[(FieldElement::ONE, 0)]);
    let every_row =
        SparsePolynomial::new(&[(-&FieldElement::ONE, 0), (FieldElement::ONE, trace_length)]);
    let first_row = SparsePolynomial::new(&[(-&FieldElement::ONE, 0), (FieldElement::ONE, 1)]);
    let last_row = SparsePolynomial::new(&[
        (-&trace_generator.pow(trace_length - 1), 0),
        (FieldElement::ONE, 1),
    ]);
    let claim_index_row = SparsePolynomial::new(&[
        (-&trace_generator.pow(claim_index), 0),
        (FieldElement::ONE, 1),
    ]);

    // Constraint repetitions
    use crate::rational_expression::RationalExpression;
    use RationalExpression::*;
    let g = Constant(trace_generator);
    let on_row = |index| RationalExpression::from(1) / (X - g.pow(index));
    let reevery_row = || (X - g.pow(trace_length - 1)) / (X.pow(trace_length) - 1.into());

    vec![
        Constraint {
            expr:        (Trace(0, 1) - Trace(1, 0)) * reevery_row(),
            base:        Box::new(|tp| tp[0].next() - &tp[1]),
            numerator:   last_row.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            expr:        (Trace(1, 1) - Trace(0, 0) - Trace(1, 0)) * reevery_row(),
            base:        Box::new(|tp| tp[1].next() - &tp[1] - &tp[0]),
            numerator:   last_row.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            expr:        (Trace(0, 0) - 1.into()) * on_row(0),
            base:        Box::new(|tp| &tp[0] - SparsePolynomial::new(&[(FieldElement::ONE, 0)])),
            numerator:   no_rows.clone(),
            denominator: first_row,
        },
        Constraint {
            expr:        (Trace(0, 0) - (&claim_value).into()) * on_row(claim_index),
            base:        Box::new(move |tp| {
                &tp[0] - SparsePolynomial::new(&[(claim_value.clone(), 0)])
            }),
            numerator:   no_rows,
            denominator: claim_index_row,
        },
    ]
}
