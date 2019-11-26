use super::{
    inputs::{Claim, Witness},
    pedersen_points::{PEDERSEN_POINTS, SHIFT_POINT},
    periodic_columns::{
        LEFT_X_COEFFICIENTS, LEFT_Y_COEFFICIENTS, RIGHT_X_COEFFICIENTS, RIGHT_Y_COEFFICIENTS,
    },
};
use log::info;
use std::collections::HashMap;
use zkp_elliptic_curve::Affine;
use zkp_primefield::FieldElement;
use zkp_stark::{Component, Constraints, DensePolynomial, RationalExpression, TraceTable};
use zkp_u256::U256;

pub fn get_trace_table(claim: &Claim, witness: &Witness) -> TraceTable {
    let num_columns = 8;
    let mut trace = TraceTable::new(claim.path_length * 256, num_columns);

    let mut row: Row = Row::default();
    row.right.point = Affine::Point {
        x: claim.leaf.clone(),
        y: FieldElement::ZERO,
    };

    for path_index in 0..claim.path_length {
        for bit_index in 0..256 {
            if bit_index % 256 == 0 {
                let other_hash = U256::from(&witness.path[path_index]);
                let (x, _) = get_coordinates(&row.right.point);
                if witness.directions[path_index] {
                    row = initialize_hash(other_hash, U256::from(x));
                } else {
                    row = initialize_hash(U256::from(x), other_hash);
                }
            } else {
                row = hash_next_bit(&row, bit_index);
            }
            let row_index = path_index * 256 + bit_index;

            let (left_x, left_y) = get_coordinates(&row.left.point);
            trace[(row_index, 0)] = FieldElement::from(row.left.source.clone());
            trace[(row_index, 1)] = row.left.slope.clone();
            trace[(row_index, 2)] = left_x.clone();
            trace[(row_index, 3)] = left_y.clone();

            let (right_x, right_y) = get_coordinates(&row.right.point);
            trace[(row_index, 4)] = FieldElement::from(row.right.source.clone());
            trace[(row_index, 5)] = row.right.slope.clone();
            trace[(row_index, 6)] = right_x.clone();
            trace[(row_index, 7)] = right_y.clone();
        }
    }
    trace
}

fn initialize_hash(left_source: U256, right_source: U256) -> Row {
    let mut row: Row = Row::default();
    row.left.source = left_source;
    row.right.source = right_source;
    row.right.point = SHIFT_POINT;
    row
}

fn hash_next_bit(row: &Row, bit_index: usize) -> Row {
    let mut next_row = Row {
        left:  Subrow {
            source: row.left.source.clone() >> 1,
            point: row.right.point.clone(),
            ..Subrow::default()
        },
        right: Subrow {
            source: row.right.source.clone() >> 1,
            ..Subrow::default()
        },
    };
    if row.left.source.bit(0) {
        let p = &PEDERSEN_POINTS[bit_index];
        next_row.left.slope = get_slope(&next_row.left.point, &p);
        next_row.left.point += p;
    }

    next_row.right.point = next_row.left.point.clone();
    if row.right.source.bit(0) {
        let p = &PEDERSEN_POINTS[bit_index + 252];
        next_row.right.slope = get_slope(&next_row.right.point, &p);
        next_row.right.point += p;
    }
    next_row
}

#[derive(Default)]
struct Row {
    left:  Subrow,
    right: Subrow,
}

struct Subrow {
    source: U256,
    slope:  FieldElement,
    point:  Affine,
}

impl Default for Subrow {
    fn default() -> Self {
        Self {
            source: U256::ZERO,
            slope:  FieldElement::ZERO,
            point:  Affine::Point {
                x: FieldElement::ZERO,
                y: FieldElement::ZERO,
            },
        }
    }
}

fn get_slope(p_1: &Affine, p_2: &Affine) -> FieldElement {
    let (x_1, y_1) = get_coordinates(p_1);
    let (x_2, y_2) = get_coordinates(p_2);
    (y_1 - y_2) / (x_1 - x_2)
}

fn get_coordinates(p: &Affine) -> (&FieldElement, &FieldElement) {
    match p {
        Affine::Zero => panic!(),
        Affine::Point { x, y } => (x, y),
    }
}

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

    Constraints::from_expressions((trace_length, 8), claim.into(), vec![
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
    ])
    .unwrap()
}

pub fn pedersen_merkle(claim: &Claim, witness: &Witness) -> Component {
    info!("Constructing constraint system...");
    let constraints = get_pedersen_merkle_constraints(&claim)
        .expressions()
        .to_vec();
    let trace = get_trace_table(claim, witness);
    let labels = HashMap::default();
    Component {
        trace,
        constraints,
        labels,
    }
}
