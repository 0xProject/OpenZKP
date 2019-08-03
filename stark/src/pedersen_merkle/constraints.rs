use crate::{
    pedersen_merkle::input::{get_periodic_columns, PublicInput},
    polynomial::Polynomial,
    proofs::{geometric_series, Constraint, TraceTable},
};
use ecc::Affine;
use primefield::FieldElement;
use starkdex::SHIFT_POINT;
use u256::U256;

pub fn get_trace_table(length: usize, witness: FieldElement) -> TraceTable {
    let mut elements = vec![FieldElement::ONE, witness];
    for i in 1..length {
        elements.push(elements[2 * i - 1].clone());
        elements.push(&elements[2 * i - 2] + &elements[2 * i - 1]);
    }
    TraceTable::new(length, 2, elements)
}

pub fn get_fibonacci_constraints(public_input: &PublicInput) -> Vec<Constraint> {
    let path_length = public_input.path_length;
    let trace_length = path_length * 256;
    let root = public_input.root.clone();
    let leaf = public_input.leaf.clone();
    let field_element_bits = U256::from(252u64);

    let g = FieldElement::root(U256::from(trace_length as u64)).unwrap();
    let no_rows = Polynomial::new(&[FieldElement::ONE]);
    let first_row = Polynomial::new(&[-&FieldElement::ONE, FieldElement::ONE]);
    let last_row = Polynomial::new(&[
        -&g.pow(U256::from(trace_length as u64 - 1)),
        FieldElement::ONE,
    ]);
    let every_hash_start_row = Polynomial::from_sparse(&[
        (path_length, FieldElement::ONE),
        (
            0,
            -&g.pow(U256::from((path_length * (trace_length - 1)) as u64)),
        ),
    ]);
    let field_element_end_rows = Polynomial::from_sparse(&[
        (
            0,
            -&g.pow(U256::from(field_element_bits * path_length as u64)),
        ),
        (path_length, FieldElement::ONE),
    ]);
    let hash_end_rows = Polynomial::from_sparse(&[
        (path_length, FieldElement::ONE),
        (
            0,
            -&g.pow(U256::from((path_length * (trace_length - 1)) as u64)),
        ),
    ]);
    let every_row =
        Polynomial::from_sparse(&[(trace_length, FieldElement::ONE), (0, -&FieldElement::ONE)]);

    let (shift_point_x, shift_point_y) = match SHIFT_POINT {
        Affine::Zero => panic!(),
        Affine::Point { x, y } => (x, y),
    };

    let periodic_columns = get_periodic_columns();
    let q_x_left_1 = Polynomial::periodic(&periodic_columns.left_x_coefficients, path_length);
    let q_x_left_2 = Polynomial::periodic(&periodic_columns.left_x_coefficients, path_length);
    let q_y_left = Polynomial::periodic(&periodic_columns.left_y_coefficients, path_length);
    let q_x_right = Polynomial::periodic(&periodic_columns.right_x_coefficients, path_length);
    let q_y_right = Polynomial::periodic(&periodic_columns.right_y_coefficients, path_length);

    fn get_left_bit(
        trace_polynomials: &[Polynomial],
        trace_generator: &FieldElement,
    ) -> Polynomial {
        trace_polynomials[0].clone()
            - &FieldElement::from(U256::from(2u64)) * &trace_polynomials[0].shift(trace_generator)
    }

    vec![
        Constraint {
            base:        Box::new(|tp, _| tp[0].clone()),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(|tp, _| tp[1].clone()),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(|tp, _| tp[2].clone()),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(|tp, _| tp[3].clone()),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(|tp, _| tp[4].clone()),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(|tp, _| tp[5].clone()),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(|tp, _| tp[6].clone()),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(|tp, _| tp[7].clone()),
            numerator:   no_rows.clone(),
            denominator: no_rows.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(move |tp, _| {
                (tp[0].clone() - Polynomial::constant(leaf.clone()))
                    * (tp[4].clone() - Polynomial::constant(leaf.clone()))
            }),
            numerator:   no_rows.clone(),
            denominator: first_row.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(move |tp, _| Polynomial::constant(root.clone()) - tp[4].clone()),
            numerator:   no_rows.clone(),
            denominator: last_row.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(|tp, g| {
                (tp[4].clone() - tp[0].shift(g)) * (tp[4].clone() - tp[4].shift(g))
            }),
            numerator:   no_rows.clone(),
            denominator: last_row.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(move |tp, g| {
                tp[6].clone() - Polynomial::constant(shift_point_x.clone())
            }),
            numerator:   no_rows.clone(),
            denominator: last_row.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(move |tp, g| {
                tp[7].clone() - Polynomial::constant(shift_point_y.clone())
            }),
            numerator:   no_rows.clone(),
            denominator: last_row.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(|tp, g| {
                let left_bit = get_left_bit(tp, g);
                left_bit.clone() * (Polynomial::constant(FieldElement::ONE) - left_bit)
            }),
            numerator:   no_rows.clone(),
            denominator: every_row.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(move |tp, g| {
                let left_bit = get_left_bit(tp, g);
                left_bit * (tp[7].clone() - q_y_left.clone())
                    - tp[1].shift(g) * (tp[6].clone() - q_x_left_1.clone())
            }),
            numerator:   no_rows.clone(),
            denominator: every_row.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(move |tp, g| {
                let left_bit = get_left_bit(tp, g);
                tp[1].clone() * tp[1].clone()
                    - left_bit * (tp[6].clone() + q_x_left_2.clone() + tp[2].shift(g))
            }),
            numerator:   no_rows.clone(),
            denominator: every_row.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
    ]
}

// Constraint expression for left_add_points/y: left_bit * (right_pt__y_row0 + left_pt__y_row1) - left_slope_row1 * (right_pt__x_row0 - left_pt__x_row1).
// Constraint expression for left_no_add_x: left_bit_neg * (right_pt__x_row0 - left_pt__x_row1).
// Constraint expression for left_no_add_y: left_bit_neg * (right_pt__y_row0 - left_pt__y_row1).
// Constraint expression for left_src_vanish_start: left_src_row0.
// Constraint expression for left_src_vanish_end: left_src_row0.

// Constraint expression for right_src_bits: right_bit * (right_bit - 1).
// Constraint expression for right_add_points/slope: right_bit * (left_pt__y_row1 - q_y_right) - right_slope_row1 * (left_pt__x_row1 - q_x_right).
// Constraint expression for right_add_points/x: right_slope_row1 * right_slope_row1 - right_bit * (left_pt__x_row1 + q_x_right + right_pt__x_row1).
// Constraint expression for right_add_points/y: right_bit * (left_pt__y_row1 + right_pt__y_row1) - right_slope_row1 * (left_pt__x_row1 - right_pt__x_row1).
// Constraint expression for right_no_add_x: right_bit_neg * (left_pt__x_row1 - right_pt__x_row1).
// Constraint expression for right_no_add_y: right_bit_neg * (left_pt__y_row1 - right_pt__y_row1).
// Constraint expression for right_src_vanish_start: right_src_row0.
// Constraint expression for right_src_vanish_end: right_src_row0.
