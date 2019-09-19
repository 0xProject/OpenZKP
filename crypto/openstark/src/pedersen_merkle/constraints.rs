use crate::{
    constraint::Constraint,
    pedersen_merkle::{
        inputs::PublicInput,
        periodic_columns::{
            LEFT_X_COEFFICIENTS, LEFT_Y_COEFFICIENTS, RIGHT_X_COEFFICIENTS, RIGHT_Y_COEFFICIENTS,
        },
    },
    polynomial::DensePolynomial,
    rational_expression::RationalExpression,
};
use elliptic_curve::Affine;
use primefield::FieldElement;
use starkdex::SHIFT_POINT;
use std::{prelude::v1::*, vec};

// TODO: Naming
#[allow(clippy::module_name_repetitions)]
pub fn get_pedersen_merkle_constraints(public_input: &PublicInput) -> Vec<Constraint> {
    use RationalExpression::*;

    let path_length = public_input.path_length;
    let trace_length = path_length * 256;
    let root = public_input.root.clone();
    let leaf = public_input.leaf.clone();
    let field_element_bits = 252;

    let (shift_point_x, shift_point_y) = match SHIFT_POINT {
        Affine::Zero => panic!(),
        Affine::Point { x, y } => (x, y),
    };

    // Periodic columns
    let periodic = |coefficients| {
        Polynomial(
            DensePolynomial::new(coefficients),
            Box::new(X.pow(path_length)),
        )
    };
    let periodic_left_x = periodic(&LEFT_X_COEFFICIENTS);
    let periodic_left_y = periodic(&LEFT_Y_COEFFICIENTS);
    let periodic_right_x = periodic(&RIGHT_X_COEFFICIENTS);
    let periodic_right_y = periodic(&RIGHT_Y_COEFFICIENTS);

    // Repeating patterns
    // TODO: Clean this up
    let trace_generator = Constant(FieldElement::root(trace_length).unwrap());
    let on_first_row = |a: RationalExpression| a / (X - Constant(FieldElement::ONE));
    let on_last_row = |a: RationalExpression| a / (X - trace_generator.pow(trace_length - 1));
    let on_hash_end_rows = |a: RationalExpression| {
        a * (X - trace_generator.pow(trace_length - 1))
            / (X.pow(path_length) - trace_generator.pow(path_length * (trace_length - 1)))
    };
    let on_no_hash_rows = |a: RationalExpression| {
        a / (X.pow(path_length) - trace_generator.pow(path_length * (trace_length - 1)))
    };
    let on_hash_start_rows = |a: RationalExpression| a / (X.pow(path_length) - 1.into());
    let on_hash_loop_rows = |a: RationalExpression| {
        a * (X.pow(path_length) - trace_generator.pow(path_length * (trace_length - 1)))
            / (X.pow(trace_length) - 1.into())
    };
    let on_fe_end_rows = |a: RationalExpression| {
        a / (X.pow(path_length) - trace_generator.pow(path_length * field_element_bits))
    };

    // Common sub-expressions
    let left_bit = Trace(0, 0) - Trace(0, 1) * 2.into();
    let right_bit = Trace(4, 0) - Trace(4, 1) * 2.into();

    vec![
        Constraint { expr: Trace(0, 0) },
        Constraint { expr: Trace(1, 0) },
        Constraint { expr: Trace(2, 0) },
        Constraint { expr: Trace(3, 0) },
        Constraint { expr: Trace(4, 0) },
        Constraint { expr: Trace(5, 0) },
        Constraint { expr: Trace(6, 0) },
        Constraint { expr: Trace(7, 0) },
        Constraint {
            expr: on_first_row(
                (Constant(leaf.clone()) - Trace(0, 0)) * (Constant(leaf.clone()) - Trace(4, 0)),
            ),
        },
        Constraint {
            expr: on_last_row(Constant(root.clone()) - Trace(6, 0)),
        },
        Constraint {
            expr: on_hash_end_rows(Trace(6, 0) - Trace(0, 1)) * (Trace(6, 0) - Trace(4, 1)),
        },
        Constraint {
            expr: on_hash_start_rows(Trace(6, 0) - Constant(shift_point_x.clone())),
        },
        Constraint {
            expr: on_hash_start_rows(Trace(7, 0) - Constant(shift_point_y.clone())),
        },
        Constraint {
            expr: on_hash_loop_rows(left_bit.clone() * (left_bit.clone() - 1.into())),
        },
        Constraint {
            expr: on_hash_loop_rows(
                left_bit.clone() * (Trace(7, 0) - periodic_left_y.clone())
                    - Trace(1, 1) * (Trace(6, 0) - periodic_left_x.clone()),
            ),
        },
        Constraint {
            expr: on_hash_loop_rows(
                Trace(1, 1) * Trace(1, 1)
                    - left_bit.clone() * (Trace(6, 0) + periodic_left_x.clone() + Trace(2, 1)),
            ),
        },
        Constraint {
            expr: on_hash_loop_rows(
                left_bit.clone() * (Trace(7, 0) + Trace(3, 1))
                    - Trace(1, 1) * (Trace(6, 0) - Trace(2, 1)),
            ),
        },
        Constraint {
            expr: on_hash_loop_rows(
                (Constant(FieldElement::ONE) - left_bit.clone()) * (Trace(6, 0) - Trace(2, 1)),
            ),
        },
        Constraint {
            expr: on_hash_loop_rows(
                (Constant(FieldElement::ONE) - left_bit.clone()) * (Trace(7, 0) - Trace(3, 1)),
            ),
        },
        Constraint {
            expr: on_fe_end_rows(Trace(0, 0)),
        },
        Constraint {
            expr: on_no_hash_rows(Trace(0, 0)),
        },
        Constraint {
            expr: on_hash_loop_rows(right_bit.clone() * (right_bit.clone() - 1.into())),
        },
        Constraint {
            expr: on_hash_loop_rows(
                right_bit.clone() * (Trace(3, 1) - periodic_right_y.clone())
                    - Trace(5, 1) * (Trace(2, 1) - periodic_right_x.clone()),
            ),
        },
        Constraint {
            expr: on_hash_loop_rows(
                Trace(5, 1) * Trace(5, 1)
                    - right_bit.clone() * (Trace(2, 1) + periodic_right_x.clone() + Trace(6, 1)),
            ),
        },
        Constraint {
            expr: on_hash_loop_rows(
                right_bit.clone() * (Trace(3, 1) + Trace(7, 1))
                    - Trace(5, 1) * (Trace(2, 1) - Trace(6, 1)),
            ),
        },
        Constraint {
            expr: on_hash_loop_rows(
                (Constant(FieldElement::ONE) - right_bit.clone()) * (Trace(2, 1) - Trace(6, 1)),
            ),
        },
        Constraint {
            expr: on_hash_loop_rows(
                (Constant(FieldElement::ONE) - right_bit.clone()) * (Trace(3, 1) - Trace(7, 1)),
            ),
        },
        Constraint {
            expr: on_fe_end_rows(Trace(4, 0)),
        },
        Constraint {
            expr: on_no_hash_rows(Trace(4, 0)),
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

    #[test]
    fn short_pedersen_merkle() {
        crate::tests::init();

        let public_input = SHORT_PUBLIC_INPUT;
        let private_input = short_private_input();
        let trace_table = get_trace_table(&public_input, &private_input);

        let constraints = &get_pedersen_merkle_constraints(&public_input);

        let proof = stark_proof(&trace_table, &constraints, &public_input, &ProofParams {
            blowup:     16,
            pow_bits:   0,
            queries:    13,
            fri_layout: vec![3, 2],
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
