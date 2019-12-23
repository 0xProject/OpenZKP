use super::{
    inputs::Claim,
    pedersen_points::SHIFT_POINT,
    periodic_columns::{
        LEFT_X_COEFFICIENTS, LEFT_Y_COEFFICIENTS, RIGHT_X_COEFFICIENTS, RIGHT_Y_COEFFICIENTS,
    },
};
use elliptic_curve::Affine;
use openstark::{Constraints, DensePolynomial, RationalExpression, solidity_encode::autogen_constraint_poly};
use primefield::FieldElement;
use std::{prelude::v1::*, vec};

// TODO: Naming
#[allow(clippy::module_name_repetitions)]
pub fn get_pedersen_merkle_constraints(claim: &Claim) -> Constraints {
    use RationalExpression::*;

    let path_length = claim.path_length;
    let trace_length = path_length * 256;
    let root = claim.root.clone();
    let leaf = claim.leaf.clone();
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

    let expressions = vec![
        Trace(0, 0),
        Trace(1, 0),
        Trace(2, 0),
        Trace(3, 0),
        Trace(4, 0),
        Trace(5, 0),
        Trace(6, 0),
        Trace(7, 0),
        on_first_row(
            (Constant(leaf.clone()) - Trace(0, 0)) * (Constant(leaf.clone()) - Trace(4, 0)),
        ),
        on_last_row(Constant(root.clone()) - Trace(6, 0)),
        on_hash_end_rows(Trace(6, 0) - Trace(0, 1)) * (Trace(6, 0) - Trace(4, 1)),
        on_hash_start_rows(Trace(6, 0) - Constant(shift_point_x.clone())),
        on_hash_start_rows(Trace(7, 0) - Constant(shift_point_y.clone())),
        on_hash_loop_rows(left_bit.clone() * (left_bit.clone() - 1.into())),
        on_hash_loop_rows(
            left_bit.clone() * (Trace(7, 0) - periodic_left_y.clone())
                - Trace(1, 1) * (Trace(6, 0) - periodic_left_x.clone()),
        ),
        on_hash_loop_rows(
            Trace(1, 1) * Trace(1, 1)
                - left_bit.clone() * (Trace(6, 0) + periodic_left_x.clone() + Trace(2, 1)),
        ),
        on_hash_loop_rows(
            left_bit.clone() * (Trace(7, 0) + Trace(3, 1))
                - Trace(1, 1) * (Trace(6, 0) - Trace(2, 1)),
        ),
        on_hash_loop_rows(
            (Constant(FieldElement::ONE) - left_bit.clone()) * (Trace(6, 0) - Trace(2, 1)),
        ),
        on_hash_loop_rows(
            (Constant(FieldElement::ONE) - left_bit.clone()) * (Trace(7, 0) - Trace(3, 1)),
        ),
        on_fe_end_rows(Trace(0, 0)),
        on_no_hash_rows(Trace(0, 0)),
        on_hash_loop_rows(right_bit.clone() * (right_bit.clone() - 1.into())),
        on_hash_loop_rows(
            right_bit.clone() * (Trace(3, 1) - periodic_right_y.clone())
                - Trace(5, 1) * (Trace(2, 1) - periodic_right_x.clone()),
        ),
        on_hash_loop_rows(
            Trace(5, 1) * Trace(5, 1)
                - right_bit.clone() * (Trace(2, 1) + periodic_right_x.clone() + Trace(6, 1)),
        ),
        on_hash_loop_rows(
            right_bit.clone() * (Trace(3, 1) + Trace(7, 1))
                - Trace(5, 1) * (Trace(2, 1) - Trace(6, 1)),
        ),
        on_hash_loop_rows(
            (Constant(FieldElement::ONE) - right_bit.clone()) * (Trace(2, 1) - Trace(6, 1)),
        ),
        on_hash_loop_rows(
            (Constant(FieldElement::ONE) - right_bit.clone()) * (Trace(3, 1) - Trace(7, 1)),
        ),
        on_fe_end_rows(Trace(4, 0)),
        on_no_hash_rows(Trace(4, 0)),
    ];

    let path_len_const = Constant(FieldElement::from(claim.path_length));
    let root_const = Constant(claim.root.clone());
    let leaf_const = Constant(claim.leaf.clone());

    let public = vec![
        &path_len_const,
        &root_const,
        &leaf_const,
    ];

    match autogen_constraint_poly(trace_length, public.as_slice(), expressions.as_slice(), 2, 8) {
        Ok(()) => {},
        Err(error) => {
            panic!("File io problem: {:?}", error)
        },
    };

    Constraints::from_expressions((trace_length, 8), claim.into(), expressions)
    .unwrap()    
}

#[cfg(test)]
mod tests {
    use super::{
        super::inputs::{short_witness, SHORT_CLAIM},
        *,
    };
    use openstark::{prove, Provable, Verifiable};

    #[test]
    fn short_pedersen_merkle() {
        let claim = SHORT_CLAIM;
        let witness = short_witness();

        let mut constraints = claim.constraints();
        constraints.blowup = 16;
        constraints.pow_bits = 0;
        constraints.num_queries = 13;
        constraints.fri_layout = vec![3, 2];

        let trace = claim.trace(&witness);
        let proof = prove(&constraints, &trace).unwrap();

        assert_eq!(
            hex::encode(proof.as_bytes()[0..32].to_vec()),
            "e2c4e35c37e33aa3b439592f2f3c5c82f464f026000000000000000000000000"
        );
        assert_eq!(
            hex::encode(proof.as_bytes()[32..64].to_vec()),
            "c5df989253ac4c3eff4fdb4130f832db1d2a9826000000000000000000000000"
        );

        // FRI commitments
        assert_eq!(
            hex::encode(proof.as_bytes()[640..672].to_vec()),
            "744f04f8bcd9c5aafb8907586428fbe9dd81b976000000000000000000000000"
        );
        assert_eq!(
            hex::encode(proof.as_bytes()[672..704].to_vec()),
            "ce329839a5eccb8009ffebf029312989e68f1cde000000000000000000000000"
        );
    }
}
