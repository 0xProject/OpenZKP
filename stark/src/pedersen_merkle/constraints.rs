use crate::{
    pedersen_merkle::{
        inputs::PublicInput,
        periodic_columns::{
            LEFT_X_COEFFICIENTS, LEFT_Y_COEFFICIENTS, RIGHT_X_COEFFICIENTS, RIGHT_Y_COEFFICIENTS,
        },
    },
    polynomial::{DensePolynomial, SparsePolynomial},
    proofs::Constraint,
};
use ecc::Affine;
use primefield::FieldElement;
use starkdex::SHIFT_POINT;
use std::{prelude::v1::*, vec};
use u256::U256;

pub fn get_pedersen_merkle_constraints(public_input: &PublicInput) -> Vec<Constraint> {
    let path_length = public_input.path_length;
    let trace_length = path_length * 256;
    let root = public_input.root.clone();
    let leaf = public_input.leaf.clone();
    let field_element_bits = 252;

    let g = FieldElement::root(trace_length).unwrap();
    let no_rows = SparsePolynomial::new(&[(FieldElement::ONE, 0)]);
    let first_row = SparsePolynomial::new(&[(-&FieldElement::ONE, 0), (FieldElement::ONE, 1)]);
    let last_row = SparsePolynomial::new(&[(-&g.pow(trace_length - 1), 0), (FieldElement::ONE, 1)]);
    let hash_end_rows = SparsePolynomial::new(&[
        (FieldElement::ONE, path_length),
        (-&g.pow(path_length * (trace_length - 1)), 0),
    ]);
    let field_element_end_rows = SparsePolynomial::new(&[
        (-&g.pow(field_element_bits * path_length), 0),
        (FieldElement::ONE, path_length),
    ]);
    let hash_start_rows =
        SparsePolynomial::new(&[(FieldElement::ONE, path_length), (-&FieldElement::ONE, 0)]);
    let every_row =
        SparsePolynomial::new(&[(FieldElement::ONE, trace_length), (-&FieldElement::ONE, 0)]);

    let (shift_point_x, shift_point_y) = match SHIFT_POINT {
        Affine::Zero => panic!(),
        Affine::Point { x, y } => (x, y),
    };

    let q_x_left_1 = SparsePolynomial::periodic(&LEFT_X_COEFFICIENTS, path_length);
    let q_x_left_2 = SparsePolynomial::periodic(&LEFT_X_COEFFICIENTS, path_length);
    let q_y_left = SparsePolynomial::periodic(&LEFT_Y_COEFFICIENTS, path_length);
    let q_x_right_1 = SparsePolynomial::periodic(&RIGHT_X_COEFFICIENTS, path_length);
    let q_x_right_2 = SparsePolynomial::periodic(&RIGHT_X_COEFFICIENTS, path_length);
    let q_y_right = SparsePolynomial::periodic(&RIGHT_Y_COEFFICIENTS, path_length);

    fn get_left_bit(trace_polynomials: &[DensePolynomial]) -> DensePolynomial {
        &trace_polynomials[0] - &FieldElement::from(U256::from(2u64)) * &trace_polynomials[0].next()
    }
    fn get_right_bit(trace_polynomials: &[DensePolynomial]) -> DensePolynomial {
        &trace_polynomials[4] - &FieldElement::from(U256::from(2u64)) * &trace_polynomials[4].next()
    }

    vec![
        Constraint {
            base:        Box::new(|tp| tp[0].clone()),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
        },
        Constraint {
            base:        Box::new(|tp| tp[1].clone()),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
        },
        Constraint {
            base:        Box::new(|tp| tp[2].clone()),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
        },
        Constraint {
            base:        Box::new(|tp| tp[3].clone()),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
        },
        Constraint {
            base:        Box::new(|tp| tp[4].clone()),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
        },
        Constraint {
            base:        Box::new(|tp| tp[5].clone()),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
        },
        Constraint {
            base:        Box::new(|tp| tp[6].clone()),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
        },
        Constraint {
            base:        Box::new(|tp| tp[7].clone()),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| {
                (SparsePolynomial::new(&[(leaf.clone(), 0)]) - &tp[0])
                    * (SparsePolynomial::new(&[(leaf.clone(), 0)]) - &tp[4])
            }),
            numerator:   no_rows.clone(),
            denominator: first_row.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| SparsePolynomial::new(&[(root.clone(), 0)]) - &tp[6]),
            numerator:   no_rows.clone(),
            denominator: last_row.clone(),
        },
        Constraint {
            base:        Box::new(|tp| (&tp[6] - tp[0].next()) * (&tp[6] - tp[4].next())),
            numerator:   last_row.clone(),
            denominator: hash_end_rows.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| {
                &tp[6] - SparsePolynomial::new(&[(shift_point_x.clone(), 0)])
            }),
            numerator:   no_rows.clone(),
            denominator: hash_start_rows.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| {
                &tp[7] - SparsePolynomial::new(&[(shift_point_y.clone(), 0)])
            }),
            numerator:   no_rows.clone(),
            denominator: hash_start_rows.clone(),
        },
        Constraint {
            base:        Box::new(|tp| {
                let left_bit = get_left_bit(tp);
                &left_bit * (&left_bit - SparsePolynomial::new(&[(FieldElement::ONE, 0)]))
            }),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| {
                let left_bit = get_left_bit(tp);
                left_bit * (&tp[7] - q_y_left.clone())
                    - tp[1].next() * (&tp[6] - q_x_left_1.clone())
            }),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| {
                let left_bit = get_left_bit(tp);
                tp[1].next().square() - left_bit * (&tp[6] + q_x_left_2.clone() + tp[2].next())
            }),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| {
                let left_bit = get_left_bit(tp);
                &left_bit * (tp[7].clone() + tp[3].next())
                    - tp[1].next() * (tp[6].clone() - tp[2].next())
            }),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| {
                let left_bit = get_left_bit(tp);
                (SparsePolynomial::new(&[(FieldElement::ONE, 0)]) - &left_bit)
                    * (tp[6].clone() - tp[2].next())
            }),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| {
                let left_bit = get_left_bit(tp);
                (SparsePolynomial::new(&[(FieldElement::ONE, 0)]) - &left_bit)
                    * (tp[7].clone() - tp[3].next())
            }),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| tp[0].clone()),
            numerator:   no_rows.clone(),
            denominator: field_element_end_rows.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| tp[0].clone()),
            numerator:   no_rows.clone(),
            denominator: hash_end_rows.clone(),
        },
        Constraint {
            base:        Box::new(|tp| {
                let right_bit = get_right_bit(tp);
                right_bit.clone() * (&right_bit - SparsePolynomial::new(&[(FieldElement::ONE, 0)]))
            }),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| {
                let right_bit = get_right_bit(tp);
                right_bit * (&tp[3].next() - q_y_right.clone())
                    - tp[5].next() * (&tp[2].next() - q_x_right_1.clone())
            }),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| {
                let right_bit = get_right_bit(tp);
                tp[5].next().square()
                    - right_bit * (&tp[2].next() + q_x_right_2.clone() + tp[6].next())
            }),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| {
                let right_bit = get_right_bit(tp);
                &right_bit * (tp[3].next() + tp[7].next())
                    - tp[5].next() * (tp[2].next() - tp[6].next())
            }),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| {
                let right_bit = get_right_bit(tp);
                (SparsePolynomial::new(&[(FieldElement::ONE, 0)]) - &right_bit)
                    * (tp[2].next() - tp[6].next())
            }),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| {
                let right_bit = get_right_bit(tp);
                (SparsePolynomial::new(&[(FieldElement::ONE, 0)]) - &right_bit)
                    * (tp[3].next() - tp[7].next())
            }),
            numerator:   hash_end_rows.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| tp[4].clone()),
            numerator:   no_rows.clone(),
            denominator: field_element_end_rows.clone(),
        },
        Constraint {
            base:        Box::new(move |tp| tp[4].clone()),
            numerator:   no_rows.clone(),
            denominator: hash_end_rows.clone(),
        },
    ]
}
