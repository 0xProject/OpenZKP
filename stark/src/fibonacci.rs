use crate::{
    channel::*,
    polynomial::{DensePolynomial, SparsePolynomial},
    proofs::{geometric_series, Constraint},
    utils::Reversible,
    TraceTable,
};
use macros_decl::u256h;
use primefield::FieldElement;
use rayon::prelude::*;
use std::convert::TryInto;
use u256::U256;

#[allow(dead_code)] // TODO
#[derive(Debug, PartialEq, Clone)]
pub struct PublicInput {
    pub index: usize,
    pub value: FieldElement,
}

#[derive(Debug)]
pub struct PrivateInput {
    pub secret: FieldElement,
}

// TODO: We are abusing Writable here to do initialization. We should
// probably have a dedicated trait for initializing a channel.
impl Writable<&PublicInput> for ProverChannel {
    fn write(&mut self, public: &PublicInput) {
        let mut bytes = [public.index.to_be_bytes()].concat();
        bytes.extend_from_slice(&public.value.as_montgomery().to_bytes_be());
        self.initialize(bytes.as_slice());
    }
}

impl Replayable<PublicInput> for VerifierChannel {
    fn replay(&mut self) -> PublicInput {
        // Need to make a temporary copy here to satisfy the borrow checker.
        // We can not guarantee proof won't change in `initialize`.
        // TODO: Move to verifier.
        self.initialize(self.proof[0..40].to_vec().as_slice());
        PublicInput {
            index: u64::from_be_bytes((&self.proof[0..8]).try_into().unwrap())
                .try_into()
                .expect("Index too large."),
            value: FieldElement::from_montgomery(U256::from_bytes_be(
                (&self.proof[8..40]).try_into().unwrap(),
            )),
        }
    }
}

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

    let trace_generator = FieldElement::root(U256::from(trace_length as u64)).unwrap();
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
            base:        Box::new(|tp| &tp[0] - &tp[1]),
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
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::proofs::{get_constraint_polynomial, interpolate_trace_table};
//     #[test]
//     fn mason() {
//         let x = FieldElement::ZERO;
//         let claim = FieldElement::from(u256h!(
//
// "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
//         ));
//
//         let witness = FieldElement::from(u256h!(
//
// "00000000000000000000000000000000000000000000000000000000cafebabe"
//         ));
//         let trace_table = get_trace_table(1024, witness);
//         let trace_polynomials = interpolate_trace_table(&trace_table);
//
//         let mut constraint_coefficients = vec![FieldElement::ZERO; 20];
//         constraint_coefficients[0] = FieldElement::ONE;
//         constraint_coefficients[1] = FieldElement::ONE;
//         constraint_coefficients[2] = FieldElement::ONE;
//         constraint_coefficients[3] = FieldElement::ONE;
//         constraint_coefficients[4] = FieldElement::ONE;
//         constraint_coefficients[5] = FieldElement::ONE;
//         constraint_coefficients[6] = FieldElement::ONE;
//
//         let old = eval_c_direct(
//             &x,
//             &trace_polynomials,
//             1000usize,
//             claim.clone(),
//             &constraint_coefficients,
//         );
//
//         let p = SparsePolynomial::new(&[FieldElement::ONE,
// -&FieldElement::ONE]);         assert_eq!(p.evaluate(&FieldElement::ONE),
// FieldElement::ZERO);
//
//         let constraint_polynomial = get_constraint_polynomial(
//             &trace_polynomials,
//             &get_fibonacci_constraints(1024, claim, 1000usize),
//             &constraint_coefficients,
//         );
//         let new = constraint_polynomial.evaluate(&x);
//
//         assert_eq!(old, new);
//     }
// }
