use crate::{
    constraint::{Constant, Constraint, PeriodicColumn, Trace, X},
    pedersen_merkle::{
        inputs::PublicInput,
        periodic_columns::{
            LEFT_X_COEFFICIENTS, LEFT_Y_COEFFICIENTS, RIGHT_X_COEFFICIENTS, RIGHT_Y_COEFFICIENTS,
        },
    },
    polynomial::{DensePolynomial, SparsePolynomial},
};
use elliptic_curve::Affine;
use primefield::FieldElement;
use starkdex::SHIFT_POINT;
use std::{prelude::v1::*, vec};
use u256::U256;

// TODO: Naming
#[allow(clippy::module_name_repetitions)]
pub fn get_pedersen_merkle_constraints(public_input: &PublicInput) -> Vec<Constraint> {
    let path_length = public_input.path_length;
    let trace_length = path_length * 256;
    let root = public_input.root.clone();
    let leaf = public_input.leaf.clone();
    let field_element_bits = 252;

    let trace_generator = FieldElement::root(trace_length).unwrap();
    let no_rows = Constant(1.into());
    let first_row = X - 1;
    let last_row = X - trace_generator.pow(trace_length - 1);
    let every_row = X.pow(trace_length) - 1;
    let hash_end_rows = X.pow(path_length) - trace_generator.pow(path_length * (trace_length - 1));
    let field_element_end_rows =
        X.pow(path_length) - trace_generator.pow(path_length * field_element_bits);
    let hash_start_rows = X.pow(path_length) - 1;

    let (shift_point_x, shift_point_y) = match SHIFT_POINT {
        Affine::Zero => panic!(),
        Affine::Point { x, y } => (x, y),
    };

    let q_x_left = PeriodicColumn(SparsePolynomial::periodic(
        &LEFT_X_COEFFICIENTS,
        path_length,
    ));
    let q_y_left = PeriodicColumn(SparsePolynomial::periodic(
        &LEFT_Y_COEFFICIENTS,
        path_length,
    ));
    let q_x_right = PeriodicColumn(SparsePolynomial::periodic(
        &RIGHT_X_COEFFICIENTS,
        path_length,
    ));
    let q_y_right = PeriodicColumn(SparsePolynomial::periodic(
        &RIGHT_Y_COEFFICIENTS,
        path_length,
    ));

    let left_bit = Trace(0, 0) - Trace(0, 1) * 2;
    let right_bit = Trace(4, 0) - Trace(4, 1) * 2;

    vec![
        Constraint {
            base:        Trace(0, 0),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
        },
        Constraint {
            base:        Trace(1, 0),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
        },
        Constraint {
            base:        Trace(2, 0),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
        },
        Constraint {
            base:        Trace(3, 0),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
        },
        Constraint {
            base:        Trace(4, 0),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
        },
        Constraint {
            base:        Trace(5, 0),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
        },
        Constraint {
            base:        Trace(6, 0),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
        },
        Constraint {
            base:        Trace(7, 0),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
        },
        Constraint {
            base:        (Trace(0, 0) - leaf.clone()) * (Trace(4, 0) - leaf.clone()),
            numerator:   no_rows.clone(),
            denominator: first_row.clone(),
        },
        Constraint {
            base:        root.clone() - Trace(6, 0),
            numerator:   no_rows.clone(),
            denominator: last_row.clone(),
        },
        Constraint {
            base:        (Trace(6, 0) - Trace(0, 1)) * (Trace(6, 0) - Trace(4, 1)),
            numerator:   last_row.clone(),
            denominator: hash_end_rows.clone(),
        },
        Constraint {
            base:        Trace(6, 0) - shift_point_x.clone(),
            numerator:   no_rows.clone(),
            denominator: hash_start_rows.clone(),
        },
        Constraint {
            base:        Trace(7, 0) - shift_point_y.clone(),
            numerator:   no_rows.clone(),
            denominator: hash_start_rows.clone(),
        },
        Constraint {
            base:        left_bit.clone() * (left_bit.clone() - 1),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        left_bit.clone() * (Trace(7, 0) - q_y_left)
                - Trace(1, 1) * (Trace(6, 0) - q_x_left.clone()),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        Trace(1, 1) * Trace(1, 1)
                - left_bit.clone() * (Trace(6, 0) + q_x_left + Trace(2, 1)),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        left_bit.clone() * (Trace(7, 0) + Trace(3, 1))
                - Trace(1, 1) * (Trace(6, 0) - Trace(2, 1)),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        (1 - left_bit.clone()) * (Trace(6, 0) - Trace(2, 1)),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        (1 - left_bit.clone()) * (Trace(7, 0) - Trace(3, 1)),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        (Trace(0, 0)),
            numerator:   no_rows.clone(),
            denominator: field_element_end_rows.clone(),
        },
        Constraint {
            base:        (Trace(0, 0)),
            numerator:   no_rows.clone(),
            denominator: hash_end_rows.clone(),
        },
        Constraint {
            base:        right_bit.clone() * (right_bit.clone() - 1),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        right_bit.clone() * (Trace(3, 1) - q_y_right.clone())
                - Trace(5, 1) * (Trace(2, 1) - q_x_right.clone()),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        Trace(5, 1) * Trace(5, 1)
                - right_bit.clone() * (Trace(2, 1) + q_x_right.clone() + Trace(6, 1)),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        right_bit.clone() * (Trace(3, 1) + Trace(7, 1))
                - Trace(5, 1) * (Trace(2, 1) - Trace(6, 1)),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        (1 - right_bit.clone()) * (Trace(2, 1) - Trace(6, 1)),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        (1 - right_bit.clone()) * (Trace(3, 1) - Trace(7, 1)),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        (Trace(4, 0)),
            numerator:   no_rows.clone(),
            denominator: field_element_end_rows.clone(),
        },
        Constraint {
            base:        (Trace(4, 0)),
            numerator:   no_rows.clone(),
            denominator: hash_end_rows.clone(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        pedersen_merkle::{
            inputs::{short_private_input, SHORT_PUBLIC_INPUT},
            trace_table::get_trace_table,
        },
        proof_params::ProofParams,
        proofs::stark_proof,
    };

    // TODO: Implement verifier and re-enable
    #[test]
    fn short_pedersen_merkle() {
        let public_input = SHORT_PUBLIC_INPUT;
        let private_input = short_private_input();
        let trace_table = get_trace_table(&public_input, &private_input);

        let constraints = &get_pedersen_merkle_constraints(&public_input);

        let proof = stark_proof(&trace_table, &constraints, &public_input, &ProofParams {
            blowup:                   16,
            pow_bits:                 0,
            queries:                  13,
            fri_layout:               vec![3, 2],
            constraints_degree_bound: 2,
        });

        assert_eq!(
            hex::encode(proof.proof[0..32].to_vec()),
            "e2c4e35c37e33aa3b439592f2f3c5c82f464f026000000000000000000000000"
        );
        assert_eq!(
            hex::encode(proof.proof[32..64].to_vec()),
            "c5df989253ac4c3eff4fdb4130f832db1d2a9826000000000000000000000000"
        );

        // FRI commitments
        assert_eq!(
            hex::encode(proof.proof[640..672].to_vec()),
            "744f04f8bcd9c5aafb8907586428fbe9dd81b976000000000000000000000000"
        );
        assert_eq!(
            hex::encode(proof.proof[672..704].to_vec()),
            "ce329839a5eccb8009ffebf029312989e68f1cde000000000000000000000000"
        );
    }
}
