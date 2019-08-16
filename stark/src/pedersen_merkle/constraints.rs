use crate::{
    pedersen_merkle::{
        inputs::{starkware_private_input, PublicInput, STARKWARE_PUBLIC_INPUT},
        periodic_columns::{
            LEFT_X_COEFFICIENTS, LEFT_Y_COEFFICIENTS, RIGHT_X_COEFFICIENTS, RIGHT_Y_COEFFICIENTS,
        },
    },
    polynomial::{DensePolynomial, SparsePolynomial},
    proofs::{geometric_series, Constraint},
};
use ecc::Affine;
use itertools::izip;
use macros_decl::{field_element, u256h};
use primefield::{invert_batch, FieldElement};
use rayon::prelude::*;
use starkdex::SHIFT_POINT;
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

struct Row {
    left:  Subrow,
    right: Subrow,
}

struct Subrow {
    source: FieldElement,
    slope:  FieldElement,
    x:      FieldElement,
    y:      FieldElement,
}

fn get_pedersen_coordinates(
    x: &FieldElement,
    path_length: usize,
) -> (FieldElement, FieldElement, FieldElement, FieldElement) {
    let q_x_left = SparsePolynomial::periodic(&LEFT_X_COEFFICIENTS, path_length).evaluate(&x);
    let q_y_left = SparsePolynomial::periodic(&LEFT_Y_COEFFICIENTS, path_length).evaluate(&x);
    let q_x_right = SparsePolynomial::periodic(&RIGHT_X_COEFFICIENTS, path_length).evaluate(&x);
    let q_y_right = SparsePolynomial::periodic(&RIGHT_Y_COEFFICIENTS, path_length).evaluate(&x);
    (q_x_left, q_y_left, q_x_right, q_y_right)
}

pub fn eval_c_direct(
    x: &FieldElement,
    polynomials: &[DensePolynomial],
    _claim_index: usize,
    _claim: FieldElement,
    coefficients: &[FieldElement],
) -> FieldElement {
    let public_input = STARKWARE_PUBLIC_INPUT;
    let path_length = U256::from(public_input.path_length as u64);
    let trace_length = U256::from(256u64) * &path_length;

    let trace_generator = FieldElement::root(trace_length.clone()).unwrap();

    let numerators = vec![
        x - trace_generator.pow(&trace_length - U256::ONE),
        x.pow(path_length.clone())
            - trace_generator.pow((&trace_length - U256::ONE) * &path_length),
        FieldElement::ONE,
    ];
    let denominators = invert_batch(&[
        x - FieldElement::ONE,
        x - trace_generator.pow(&trace_length - U256::from(1u64)),
        x.pow(path_length.clone())
            - trace_generator.pow(&path_length * (&trace_length - U256::ONE)),
        x.pow(path_length.clone()) - FieldElement::ONE,
        x.pow(trace_length.clone()) - FieldElement::ONE,
        x.pow(path_length.clone()) - trace_generator.pow(U256::from(252u64) * &path_length),
        FieldElement::ONE,
    ]);

    let mut this_row: Vec<FieldElement> = Vec::with_capacity(8);
    for polynomial in polynomials {
        this_row.push(polynomial.evaluate(&x.clone()));
    }
    let mut next_row: Vec<FieldElement> = Vec::with_capacity(8);
    for polynomial in polynomials {
        next_row.push(polynomial.evaluate(&(x * &trace_generator)));
    }

    let this = Row {
        left:  Subrow {
            source: this_row[0].clone(),
            slope:  this_row[1].clone(),
            x:      this_row[2].clone(),
            y:      this_row[3].clone(),
        },
        right: Subrow {
            source: this_row[4].clone(),
            slope:  this_row[5].clone(),
            x:      this_row[6].clone(),
            y:      this_row[7].clone(),
        },
    };

    let next = Row {
        left:  Subrow {
            source: next_row[0].clone(),
            slope:  next_row[1].clone(),
            x:      next_row[2].clone(),
            y:      next_row[3].clone(),
        },
        right: Subrow {
            source: next_row[4].clone(),
            slope:  next_row[5].clone(),
            x:      next_row[6].clone(),
            y:      next_row[7].clone(),
        },
    };
    let left_bit = &this.left.source - next.left.source.double();
    let right_bit = &this.right.source - next.right.source.double();

    let (shift_point_x, shift_point_y) = match SHIFT_POINT {
        Affine::Zero => panic!(),
        Affine::Point { x, y } => (x, y),
    };

    let (q_x_left, q_y_left, q_x_right, q_y_right) = get_pedersen_coordinates(&x, 8192);

    let constraints = vec![
        this.left.source.clone(),
        this.left.slope.clone(),
        this.left.x.clone(),
        this.left.y.clone(),
        this.right.source.clone(),
        this.right.slope.clone(),
        this.right.x.clone(),
        this.right.y.clone(),
        (&public_input.leaf - &this.left.source) * (&public_input.leaf - &this.right.source),
        &public_input.root - &this.right.x,
        (&this.right.x - &next.left.source) * (&this.right.x - &next.right.source),
        &this.right.x - shift_point_x,
        &this.right.y - shift_point_y,
        &left_bit * (&left_bit - FieldElement::ONE),
        &left_bit * (&this.right.y - &q_y_left) - &next.left.slope * (&this.right.x - &q_x_left),
        next.left.slope.square() - &left_bit * (&this.right.x + &q_x_left + &next.left.x),
        &left_bit * (&this.right.y + &next.left.y)
            - &next.left.slope * (&this.right.x - &next.left.x),
        (FieldElement::ONE - &left_bit) * (&this.right.x - &next.left.x),
        (FieldElement::ONE - &left_bit) * (&this.right.y - &next.left.y),
        this.left.source.clone(),
        this.left.source.clone(),
        &right_bit * (&right_bit - FieldElement::ONE),
        &right_bit * (&next.left.y - &q_y_right) - &next.right.slope * (&next.left.x - &q_x_right),
        next.right.slope.square() - &right_bit * (&next.left.x + &q_x_right + &next.right.x),
        &right_bit * (&next.left.y + &next.right.y)
            - &next.right.slope * (&next.left.x - &next.right.x),
        (FieldElement::ONE - &right_bit) * (&next.left.x - &next.right.x),
        (FieldElement::ONE - &right_bit) * (&next.left.y - &next.right.y),
        this.right.source.clone(),
        this.right.source.clone(),
    ];

    let degree_adjustment =
        |constraint_degree: U256, numerator_degree: U256, denominator_degree: U256| -> U256 {
            2u64 * trace_length.clone() + denominator_degree
                - U256::ONE
                - constraint_degree
                - numerator_degree
        };

    let adjustments = vec![
        x.pow(degree_adjustment(
            &trace_length - U256::ONE,
            U256::ZERO,
            U256::ZERO,
        )),
        x.pow(degree_adjustment(
            2u64 * (&trace_length - U256::ONE),
            U256::ZERO,
            U256::ONE,
        )),
        x.pow(degree_adjustment(
            &trace_length - U256::ONE,
            U256::ZERO,
            U256::ONE,
        )),
        x.pow(degree_adjustment(
            2u64 * (&trace_length - U256::ONE),
            U256::ONE,
            path_length.clone(),
        )),
        x.pow(degree_adjustment(
            &trace_length - U256::ONE,
            U256::ZERO,
            path_length.clone(),
        )),
        x.pow(degree_adjustment(
            2u64 * (&trace_length - U256::ONE),
            path_length.clone(),
            trace_length.clone(),
        )),
    ];

    let numerator_indices = vec![
        2, 2, 2, 2, 2, 2, 2, 2, // asdfasdf
        2, 2, 0, 2, 2, // asdfasdf
        1, 1, 1, 1, 1, 1, 2, 2, // asdfasdf
        1, 1, 1, 1, 1, 1, 2, 2, // asdfasdf
    ];
    let denominator_indices = vec![
        6, 6, 6, 6, 6, 6, 6, 6, // asdfa
        0, 1, 2, 3, 3, // asdfasdf
        4, 4, 4, 4, 4, 4, 5, 2, // asdfasdf
        4, 4, 4, 4, 4, 4, 5, 2, // asdfasdf
    ];
    let adjustment_indices = vec![
        0, 0, 0, 0, 0, 0, 0, 0, // asfasdf
        1, 2, 3, 4, 4, 5, 5, 5, 5, 5, 5, 4, 4, 5, 5, 5, 5, 5, 5, 4, 4,
    ];

    let mut result = FieldElement::ZERO;
    for (i, (numerator_index, denominator_index, adjustment_index)) in
        izip!(numerator_indices, denominator_indices, adjustment_indices).enumerate()
    {
        let value =
            &constraints[i] * &numerators[numerator_index] * &denominators[denominator_index];
        result += value
            * (&coefficients[2 * i] + &coefficients[2 * i + 1] * &adjustments[adjustment_index]);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        pedersen_merkle::{
            inputs::{starkware_private_input, STARKWARE_PUBLIC_INPUT},
            trace_table::get_trace_table,
        },
        proofs::{
            calculate_low_degree_extensions, get_constraint_polynomial, interpolate_trace_table,
            Merkleizable, ProofParams,
        },
    };
    use macros_decl::{hex, u256h};

    fn get_trace_polynomials() -> Vec<DensePolynomial> {
        let trace_table = get_trace_table(&STARKWARE_PUBLIC_INPUT, &starkware_private_input());
        interpolate_trace_table(&trace_table)
    }

    #[test]
    fn pedersen_coordinates_are_correct() {
        let oods_point = FieldElement::from_hex_str(
            "0x273966fc4697d1762d51fe633f941e92f87bdda124cf7571007a4681b140c05",
        );

        let (q_x_left, q_y_left, q_x_right, q_y_right) =
            get_pedersen_coordinates(&oods_point, 8192);

        assert_eq!(
            q_x_left,
            FieldElement::from_hex_str(
                "0x4ea59d2fe0379a2e1a2ef80fb7c9ff326f32d1e4194dfffd22077ecc82e8072"
            )
        );
        assert_eq!(
            q_y_left,
            FieldElement::from_hex_str(
                "0x395b0c1bdd514cad5718e7cfc7fb1b65493f49bbada576a505a426e9231abb9"
            )
        );
        assert_eq!(
            q_x_right,
            FieldElement::from_hex_str(
                "0x40b16f2290963858584758e12b1f2da3c0e9c81ed45f69875554c0ca45ad104"
            )
        );
        assert_eq!(
            q_y_right,
            FieldElement::from_hex_str(
                "0x1d9e6d4e31f8278a249701bdb397de10d87b3a93ca7dcb71b38f9fda87119bc"
            )
        );
    }

    #[test]
    fn eval_c_direct_is_correct() {
        let trace_polynomials = get_trace_polynomials();

        let oods_point = FieldElement::from_hex_str(
            "0x273966fc4697d1762d51fe633f941e92f87bdda124cf7571007a4681b140c05",
        );

        let result = eval_c_direct(
            &oods_point,
            &trace_polynomials,
            0usize,             // not used
            FieldElement::ZERO, // not used
            &get_coefficients(),
        );

        let expected = FieldElement::from_hex_str(
            "0x77d10d22df8a41ee56095fc18c0d02dcd101c2e5749ff65458828bbd3c820db",
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn constraint_oods_values_are_correct() {
        let trace_polynomials = get_trace_polynomials();

        let oods_point = FieldElement::from_hex_str(
            "0x273966fc4697d1762d51fe633f941e92f87bdda124cf7571007a4681b140c05",
        );

        let positive_oods = eval_c_direct(
            &oods_point,
            &trace_polynomials,
            0usize,             // not used
            FieldElement::ZERO, // not used
            &get_coefficients(),
        );

        let negative_oods = eval_c_direct(
            &(FieldElement::ZERO - &oods_point),
            &trace_polynomials,
            0usize,             // not used
            FieldElement::ZERO, // not used
            &get_coefficients(),
        );

        let even_oods_value = FieldElement::from_hex_str(
            "0x7370f59cb5af66e4183bc0c5d206e7f6c2be944366ad42a4d8bccd5417499f",
        );
        let odd_oods_value = FieldElement::from_hex_str(
            "0x4b32254637e364a6649ed013dd993dc0acd08ba4d360ddac758e931dcc531d",
        );

        assert_eq!(
            (&positive_oods + &negative_oods) / FieldElement::from_hex_str("2"),
            even_oods_value
        );
        assert_eq!(
            (&positive_oods - &negative_oods) / oods_point.double(),
            odd_oods_value
        );
    }

    #[test]
    fn new_matches_old_constraints() {
        let trace_polynomials = get_trace_polynomials();

        let mut constraints = get_pedersen_merkle_constraints(&STARKWARE_PUBLIC_INPUT);
        // constraints.truncate(25);
        let mut constraint_coefficients = vec![FieldElement::ZERO; 100];
        for i in 0..2 * constraints.len() {
            constraint_coefficients[i] = FieldElement::ONE;
        }

        let x = FieldElement::GENERATOR;
        let old = eval_c_direct(
            &x,
            &trace_polynomials,
            0usize,
            FieldElement::ZERO,
            &constraint_coefficients,
        );

        let constraint_polynomial = get_constraint_polynomial(
            &trace_polynomials,
            &constraints,
            &constraint_coefficients,
            2,
        );
        let new = constraint_polynomial.evaluate(&x);

        assert_eq!(old, new);
    }

    #[test]
    fn wayne() {
        let constraint_polynomial = get_constraint_polynomial(
            &get_trace_polynomials(),
            &get_pedersen_merkle_constraints(&STARKWARE_PUBLIC_INPUT),
            &get_coefficients(),
            2,
        );

        let oods_point = FieldElement::from_hex_str(
            "0x273966fc4697d1762d51fe633f941e92f87bdda124cf7571007a4681b140c05",
        );
        let oods_value = constraint_polynomial.evaluate(&oods_point);

        let expected = FieldElement::from_hex_str(
            "0x77d10d22df8a41ee56095fc18c0d02dcd101c2e5749ff65458828bbd3c820db",
        );

        assert_eq!(oods_value, expected);
    }

    #[test]
    fn oods_2() {
        let constraint_polynomial = get_constraint_polynomial(
            &get_trace_polynomials(),
            &get_pedersen_merkle_constraints(&STARKWARE_PUBLIC_INPUT),
            &get_coefficients(),
            2,
        );

        let oods_point = FieldElement::from_hex_str(
            "0x273966fc4697d1762d51fe633f941e92f87bdda124cf7571007a4681b140c05",
        );
        let negative_oods_point = -&oods_point;

        let even_oods_value = (constraint_polynomial.evaluate(&oods_point)
            + constraint_polynomial.evaluate(&negative_oods_point))
            / FieldElement::from_hex_str("2");
        let odd_oods_value = (constraint_polynomial.evaluate(&oods_point)
            - constraint_polynomial.evaluate(&negative_oods_point))
            / oods_point.double();

        assert_eq!(
            even_oods_value,
            FieldElement::from_hex_str(
                "0x7370f59cb5af66e4183bc0c5d206e7f6c2be944366ad42a4d8bccd5417499f",
            )
        );

        assert_eq!(
            odd_oods_value,
            FieldElement::from_hex_str(
                "0x4b32254637e364a6649ed013dd993dc0acd08ba4d360ddac758e931dcc531d",
            )
        );
    }
}
