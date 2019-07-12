use crate::{pedersen_merkle::input::get_public_input, polynomial::eval_poly};
use primefield::{invert_batch, FieldElement};
// use rayon::prelude::*;
use u256::U256;
// use u256::u256h;
use ecc::Affine;
use starkdex::SHIFT_POINT;

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

// TODO: Naming
#[allow(non_snake_case)]
pub fn eval_c_direct(
    x: &FieldElement,
    polynomials: &[&[FieldElement]],
    _claim_index: usize,
    _claim: FieldElement,
    _coefficients: &[FieldElement],
) -> FieldElement {
    let public_input = get_public_input();
    let path_length = U256::from(public_input.path_length as u64);
    let trace_length = U256::from(256u64) * &path_length;
    let beta = 16u64;
    let evaluation_length = U256::from(beta) * &trace_length;

    let trace_generator = FieldElement::root(trace_length.clone()).unwrap();
    let evaluation_generator = FieldElement::root(evaluation_length.clone()).unwrap();

    let _numerators = vec![
        x - trace_generator.pow(&trace_length - U256::from(1u64)),
        x.pow(path_length.clone())
            - trace_generator.pow((&trace_length - U256::from(1u64)) * &path_length),
        FieldElement::ONE,
    ];
    let _denominators = invert_batch(&[
        x - FieldElement::ONE,
        x - trace_generator.pow(&trace_length - U256::from(1u64)),
        x.pow(path_length.clone())
            - trace_generator.pow(&path_length * (&trace_length - U256::from(1u64))),
        x.pow(path_length.clone()) - FieldElement::ONE,
        x.pow(trace_length.clone()) - FieldElement::ONE,
        x.pow(path_length.clone()) - trace_generator.pow(U256::from(252u64) * &path_length),
        FieldElement::ONE,
    ]);

    let mut this_row: Vec<FieldElement> = Vec::with_capacity(8);
    for polynomial in polynomials {
        this_row.push(eval_poly(x.clone(), polynomial));
    }
    let mut next_row: Vec<FieldElement> = Vec::with_capacity(8);
    for polynomial in polynomials {
        next_row.push(eval_poly(x.clone() * &evaluation_generator, polynomial));
    }

    let this = Row {
        left:  Subrow {
            source: this_row[0].clone(),
            slope:  this_row[1].clone(),
            x:      this_row[2].clone(),
            y:      this_row[2].clone(),
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
            y:      next_row[2].clone(),
        },
        right: Subrow {
            source: next_row[4].clone(),
            slope:  next_row[5].clone(),
            x:      next_row[6].clone(),
            y:      next_row[7].clone(),
        },
    };
    let left_bit = &this.left.source - next.left.source.double();
    let _right_bit = &this.right.source - next.right.source.double();

    let (shift_point_x, shift_point_y) = match SHIFT_POINT {
        Affine::Zero => panic!(),
        Affine::Point { x, y } => (x, y),
    };

    let q_x_left = FieldElement::ONE;
    let q_y_left = FieldElement::ZERO;
    let q_x_right = FieldElement::ONE;
    let q_y_right = FieldElement::ZERO;

    let _constraints = vec![
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

    FieldElement::ZERO
    // let eval_C0 = |x: FieldElement| -> FieldElement {
    //     ((eval_P0(&x * &g) - eval_P1(x.clone())) * (&x -
    // &g.pow(U256::from(trace_len - 1))))         / (&x.pow(U256::
    // from(trace_len)) - FieldElement::ONE) };
    // let eval_C1 = |x: FieldElement| -> FieldElement {
    //     ((eval_P1(&x * &g) - eval_P0(x.clone()) - eval_P1(x.clone()))
    //         * (&x - (&g.pow(U256::from(trace_len - 1)))))
    //         / (&x.pow(U256::from(trace_len)) - FieldElement::ONE)
    // };
    // let eval_C2 = |x: FieldElement| -> FieldElement {
    //     ((eval_P0(x.clone()) - FieldElement::ONE) * FieldElement::ONE) / (&x -
    // FieldElement::ONE) };
    // let eval_C3 = |x: FieldElement| -> FieldElement {
    //     (eval_P0(x.clone()) - claim) / (&x - &g.pow(U256::from(claim_index as
    // u64))) };
    //
    // fn degree_adjustment_factor(
    //     constraint_degree: u64,
    //     numerator_degree: u64,
    //     denominator_degree: u64,
    // ) -> u64 {
    //     trace_len + denominator_degree - 1 - constraint_degree - numerator_degree
    // };
    //
    // let eval_C = |x: FieldElement| -> FieldElement {
    //     let composition_degree_bound = trace_len;
    //     let mut r = FieldElement::ZERO;
    //     r += &constraint_coefficients[0] * &eval_C0(x.clone());
    //     r += &constraint_coefficients[1]
    //         * &eval_C0(x.clone())
    //         * (&x).pow(U256::from(degree_adjustment_factor(
    //           composition_degree_bound, trace_len - 1, 1, trace_len,
    //         )));
    //     r += &constraint_coefficients[2] * &eval_C1(x.clone());
    //     r += &constraint_coefficients[3]
    //         * &eval_C1(x.clone())
    //         * (&x).pow(U256::from(degree_adjustment_factor(
    //           composition_degree_bound, trace_len - 1, 1, trace_len,
    //         )));
    //     r += &constraint_coefficients[4] * &eval_C2(x.clone());
    //     r += &constraint_coefficients[5]
    //         * &eval_C2(x.clone())
    //         * x.pow(U256::from(degree_adjustment_factor(
    //           composition_degree_bound, trace_len - 1, 0, 1,
    //         )));
    //     r += &constraint_coefficients[6] * (eval_C3.clone())(x.clone());
    //     r += &constraint_coefficients[7]
    //         * &eval_C3(x.clone())
    //         * x.pow(U256::from(degree_adjustment_factor(
    //           composition_degree_bound, trace_len - 1, 0, 1,
    //         )));
    //     r
    // };
    // eval_C(x.clone())
}
