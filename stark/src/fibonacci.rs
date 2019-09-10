use crate::{channel::*, constraint::Constraint, polynomial::SparsePolynomial};
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

    vec![
        Constraint {
            base:        Box::new(|tp| tp[0].next() - &tp[1]),
            numerator:   last_row.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        Box::new(|tp| tp[1].next() - &tp[1] - &tp[0]),
            numerator:   last_row.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        Box::new(|tp| &tp[0] - SparsePolynomial::new(&[(FieldElement::ONE, 0)])),
            numerator:   no_rows.clone(),
            denominator: first_row,
        },
        Constraint {
            base:        Box::new(move |tp| {
                &tp[0] - SparsePolynomial::new(&[(claim_value.clone(), 0)])
            }),
            numerator:   no_rows,
            denominator: claim_index_row,
        },
    ]
}
