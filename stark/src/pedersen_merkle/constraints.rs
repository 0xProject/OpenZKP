use crate::{
    mmap_vec::MmapVec,
    pedersen_merkle::input::{get_periodic_columns, get_public_input, PublicInput},
    polynomial::Polynomial,
    proofs::{geometric_series, Constraint, TraceTable},
};
use ecc::Affine;
use itertools::izip;
use primefield::{invert_batch, FieldElement};
use rayon::prelude::*;
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

pub fn get_pedersen_merkle_constraints(public_input: &PublicInput) -> Vec<Constraint> {
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
                (tp[0].clone() - Polynomial::constant(leaf.clone())) // note that this is much more easily done in the frequency domain.
                    * (tp[4].clone() - Polynomial::constant(leaf.clone()))
            }),
            numerator:   no_rows.clone(),
            denominator: first_row.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(move |tp, _| Polynomial::constant(root.clone()) - tp[6].clone()),
            numerator:   no_rows.clone(),
            denominator: last_row.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length, FieldElement::ONE)]),
        },
        // Constraint {
        //     base:        Box::new(|tp, g| {
        //         (tp[5].clone() - tp[0].shift(g)) * (tp[4].clone() - tp[4].shift(g))
        //     }),
        //     numerator:   no_rows.clone(),
        //     denominator: every_hash_start_row.clone(),
        //     adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        // },
        // Constraint {
        //     base:        Box::new(move |tp, g| {
        //         tp[6].clone() - Polynomial::constant(shift_point_x.clone())
        //     }),
        //     numerator:   no_rows.clone(),
        //     denominator: last_row.clone(),
        //     adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        // },
        // Constraint {
        //     base:        Box::new(move |tp, g| {
        //         tp[7].clone() - Polynomial::constant(shift_point_y.clone())
        //     }),
        //     numerator:   no_rows.clone(),
        //     denominator: last_row.clone(),
        //     adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        // },
        // Constraint {
        //     base:        Box::new(|tp, g| {
        //         let left_bit = get_left_bit(tp, g);
        //         left_bit.clone() * (Polynomial::constant(FieldElement::ONE) - left_bit)
        //     }),
        //     numerator:   no_rows.clone(),
        //     denominator: every_row.clone(),
        //     adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        // },
        // Constraint {
        //     base:        Box::new(move |tp, g| {
        //         let left_bit = get_left_bit(tp, g);
        //         left_bit * (tp[7].clone() - q_y_left.clone())
        //             - tp[1].shift(g) * (tp[6].clone() - q_x_left_1.clone())
        //     }),
        //     numerator:   no_rows.clone(),
        //     denominator: every_row.clone(),
        //     adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        // },
        // Constraint {
        //     base:        Box::new(move |tp, g| {
        //         let left_bit = get_left_bit(tp, g);
        //         tp[1].clone() * tp[1].clone()
        //             - left_bit * (tp[6].clone() + q_x_left_2.clone() + tp[2].shift(g))
        //     }),
        //     numerator:   no_rows.clone(),
        //     denominator: every_row.clone(),
        //     adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        // },
    ]
}

// Constraint expression for left_add_points/y: left_bit * (right_pt__y_row0 +
// left_pt__y_row1) - left_slope_row1 * (right_pt__x_row0 - left_pt__x_row1).
// Constraint expression for left_no_add_x: left_bit_neg * (right_pt__x_row0 -
// left_pt__x_row1). Constraint expression for left_no_add_y: left_bit_neg *
// (right_pt__y_row0 - left_pt__y_row1). Constraint expression for
// left_src_vanish_start: left_src_row0. Constraint expression for
// left_src_vanish_end: left_src_row0.

// Constraint expression for right_src_bits: right_bit * (right_bit - 1).
// Constraint expression for right_add_points/slope: right_bit *
// (left_pt__y_row1 - q_y_right) - right_slope_row1 * (left_pt__x_row1 -
// q_x_right). Constraint expression for right_add_points/x: right_slope_row1 *
// right_slope_row1 - right_bit * (left_pt__x_row1 + q_x_right +
// right_pt__x_row1). Constraint expression for right_add_points/y: right_bit *
// (left_pt__y_row1 + right_pt__y_row1) - right_slope_row1 * (left_pt__x_row1 -
// right_pt__x_row1). Constraint expression for right_no_add_x: right_bit_neg *
// (left_pt__x_row1 - right_pt__x_row1). Constraint expression for
// right_no_add_y: right_bit_neg * (left_pt__y_row1 - right_pt__y_row1).
// Constraint expression for right_src_vanish_start: right_src_row0.
// Constraint expression for right_src_vanish_end: right_src_row0.

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
    path_length: &U256,
) -> (FieldElement, FieldElement, FieldElement, FieldElement) {
    let periodic_columns = get_periodic_columns();

    let q_x_left = Polynomial::new(&periodic_columns.left_x_coefficients)
        .evaluate(&x.pow(path_length.clone()));
    let q_y_left = Polynomial::new(&periodic_columns.left_y_coefficients)
        .evaluate(&x.pow(path_length.clone()));

    let q_x_right = Polynomial::new(&periodic_columns.right_x_coefficients)
        .evaluate(&x.pow(path_length.clone()));

    let q_y_right = Polynomial::new(&periodic_columns.right_y_coefficients)
        .evaluate(&x.pow(path_length.clone()));

    (q_x_left, q_y_left, q_x_right, q_y_right)
}

pub fn eval_c_direct(
    x: &FieldElement,
    polynomials: &[Polynomial],
    _claim_index: usize,
    _claim: FieldElement,
    coefficients: &[FieldElement],
) -> FieldElement {
    let public_input = get_public_input();
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

    let (q_x_left, q_y_left, q_x_right, q_y_right) = get_pedersen_coordinates(&x, &path_length);

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
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 2, 2,
    ];
    let denominator_indices = vec![
        6, 6, 6, 6, 6, 6, 6, 6, 0, 1, 2, 3, 3, 4, 4, 4, 4, 4, 4, 5, 2, 4, 4, 4, 4, 4, 4, 5, 2,
    ];
    let adjustment_indices = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 4, 5, 5, 5, 5, 5, 5, 4, 4, 5, 5, 5, 5, 5, 5, 4, 4,
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
        pedersen_merkle::proof::{get_extended_trace_table, get_trace_polynomials},
        proofs::get_constraint_polynomial,
    };
    use hex_literal::*;
    use u256::u256h;

    #[test]
    fn pedersen_coordinates_are_correct() {
        let oods_point = FieldElement::from_hex_str(
            "0x273966fc4697d1762d51fe633f941e92f87bdda124cf7571007a4681b140c05",
        );
        let path_length = U256::from(8192u64);

        let (q_x_left, q_y_left, q_x_right, q_y_right) =
            get_pedersen_coordinates(&oods_point, &path_length);

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

    fn get_coefficients() -> Vec<FieldElement> {
        vec![
            FieldElement::from(u256h!(
                "0636ad17759a0cc671e906ef94553c10f7a2c012d7a2aa599875506f874c136a"
            )),
            FieldElement::from(u256h!(
                "00ab929f48dee245d46548d9ce7b5c12809a489269702bede4a0f0beba6c96c3"
            )),
            FieldElement::from(u256h!(
                "032d059175506c780d44d30bf305b2e5cce87c2d10812aa4d19a4528a5906e97"
            )),
            FieldElement::from(u256h!(
                "062fc698139debf58aa475f18474829bce0ad224493570b723b254220774c0a4"
            )),
            FieldElement::from(u256h!(
                "07a316b3888038c223729c1ca14608dc3a536c62453f29facbb945faea4edc06"
            )),
            FieldElement::from(u256h!(
                "073ba8423c357d128709e1a1c45f5321483026f156fc58bc3f2f2fcd4e26112d"
            )),
            FieldElement::from(u256h!(
                "0215d0bcc49a30e0ca497c801a392300b3621d9c9be977c956d84a72db66ef50"
            )),
            FieldElement::from(u256h!(
                "03063ac609aed7c06323a4a46df08169cda8222d6c825c41495938acac23bd25"
            )),
            FieldElement::from(u256h!(
                "03b8f5b9dcb514cb0b72b96e507ee51ed5a90ce9887f9ba0ed132a78379f41bf"
            )),
            FieldElement::from(u256h!(
                "02cba94fa3a77dc4a6472998bb8c2d730f5bb538216172abec1feeaac28172f7"
            )),
            FieldElement::from(u256h!(
                "0329512d0cf95b0c90e3df8a6dbc965057738955b26d3ab7099fd2129a2733ad"
            )),
            FieldElement::from(u256h!(
                "0029b37fd38f7517cd35c29d1963f4c48bc53ca3ca6ae97d238107bdeb4587c0"
            )),
            FieldElement::from(u256h!(
                "05d12ac775d829842a492cb4b73edc1496349571d4b1cac0ca69626753b0c000"
            )),
            FieldElement::from(u256h!(
                "05d1a23dfb3b7a0d2def3dc025daa911876871c471f46ad7f445373a22b499d6"
            )),
            FieldElement::from(u256h!(
                "05442e604659a3c9f8fb27a9045f0298ff7864c310f1e332f1731741b417fdd3"
            )),
            FieldElement::from(u256h!(
                "07d9afbc5e50e96cb40ee87da8bf587782e682c1f6a3992d80baa06c3ba7869e"
            )),
            FieldElement::from(u256h!(
                "07e1ce86e3b58bae217e62f4f65a748109d19312cd9fffc21d076670360aacf9"
            )),
            FieldElement::from(u256h!(
                "035f9da854f2c57d45b02aea22d6b3a6032709a56e4fc97ec2091cd0dbd5914e"
            )),
            FieldElement::from(u256h!(
                "059a33bc0404c02913a7e3f86e649872772fa342eb5f012ad305eaa1118838ac"
            )),
            FieldElement::from(u256h!(
                "045d9748f52e5a9d978b691134cc96cdccc424ca2a680443ae7a55c08d7c4aa2"
            )),
            FieldElement::from(u256h!(
                "02d57682b21f33ff481a4acb4998ae61cd63af8b093aad8c1045daec53c6c187"
            )),
            FieldElement::from(u256h!(
                "054528769c7f4a197d9e4ee98cfb1fcd4693005abf6971f5b2d1094c35b6213d"
            )),
            FieldElement::from(u256h!(
                "036f9f941c351259092b93d06fd6fa04ead6f9c2bf689ba5dc493dc272b3e4e2"
            )),
            FieldElement::from(u256h!(
                "0622608ba33a72b440416448210531a4d01603e896c2eb0845031805ed9a5c74"
            )),
            FieldElement::from(u256h!(
                "01cbed1d58c1df62c5d6858493008de7e597431dc57350fb2c2943e2de1cc0c3"
            )),
            FieldElement::from(u256h!(
                "0781db13a07eec56a98fa4a1a7ff68003e5c16811926409de4b1f7ea2c624ead"
            )),
            FieldElement::from(u256h!(
                "021331547cc14840df44d41241a4da54be67df3f788f38dbd737c2bae6cd7838"
            )),
            FieldElement::from(u256h!(
                "07235529f0c22209c5f44c41c9d932b8c5744b63634567edb4d175cfdf25437f"
            )),
            FieldElement::from(u256h!(
                "04f99ffbba41cc2d8cdd9f13bbaf265e6f32deb4daac355f095c6f3c3a6762a2"
            )),
            FieldElement::from(u256h!(
                "042b86e961dd43e847d6278ba49870e0f04212b5ae38785cae336fba6eafcbe1"
            )),
            FieldElement::from(u256h!(
                "00f4a02801ac456e6ced57ea2814cb038881cb6de9487104fd2c76732485bdd8"
            )),
            FieldElement::from(u256h!(
                "06850c719229f42ea96a90dfaf75f248b45b9d896443adf29189e02c906fd27f"
            )),
            FieldElement::from(u256h!(
                "0116ee01cb9f6967ae360d3c38983ca38aa5c863e10c85ad77b04ad65a8adae7"
            )),
            FieldElement::from(u256h!(
                "0695eeb76a10a9c0398db1ebe391d2e25f6a80ba83855dc9a6b3ebe0698a4bcd"
            )),
            FieldElement::from(u256h!(
                "053d35cee3cf6e8b1f4406f8c9bc0f88d1e39facbc70eb19b7c1927b02934eaf"
            )),
            FieldElement::from(u256h!(
                "05c040858783b6a092ae756b1bd36a91e18bd92bdf4453b3580c535db22d12d9"
            )),
            FieldElement::from(u256h!(
                "06a2a83dcf1222a9972faa03aa45b5a03ea9995833c9dcef272f73a4dc6fb7d6"
            )),
            FieldElement::from(u256h!(
                "07537e90d5b2bab1c038fc6854267e7b2806d2f26c2fb7ee92bc65501903e6d2"
            )),
            FieldElement::from(u256h!(
                "050c83b136f235043250e31fdc262b8ff441686e8f11b29d3a7706b86095d128"
            )),
            FieldElement::from(u256h!(
                "00821f83891431a1cc871d9c4b74b212c5eb113acc1340088900205e7b8698b3"
            )),
            FieldElement::from(u256h!(
                "05897b09a49d1ae72f7845fb242db4a6c0f6f4aac9d63ab0f331f46332df4c82"
            )),
            FieldElement::from(u256h!(
                "00728f28f5309ddf5a3a9444bc2e97a084a9f4342f62a84da891ef0931a2147b"
            )),
            FieldElement::from(u256h!(
                "0381b768e7faa0361af12ae323ccb29f502d0ddc3964a90f4354ed5bb6ba34b4"
            )),
            FieldElement::from(u256h!(
                "05870d743173c27f92536909745a36ac31c6b5384e4d0127f8cf6a813e036e3b"
            )),
            FieldElement::from(u256h!(
                "0012ea1ebbd9e4ad0fe90a0444d90f8c8e4cab8650a5f0cfed6fce0dfbb604ce"
            )),
            FieldElement::from(u256h!(
                "0551212193e2ffe995afb9052c083eb6773b43dcc8df6e69e73591ff3ba411b5"
            )),
            FieldElement::from(u256h!(
                "04e0cc02bf5c6c4b572e455f76de37fcf38e35905d856ad6e086d4ed9bd1793c"
            )),
            FieldElement::from(u256h!(
                "0480de46109f40b539374cbc413e935be066a7296443cb8e4de05f654faadbd7"
            )),
            FieldElement::from(u256h!(
                "026a515d41b9f630302a52b80b60d6dfd08ff009e104570ba0537c8f5f8ec02d"
            )),
            FieldElement::from(u256h!(
                "01e3755bec6d69cb6ff4516b0cf43ee52466aafbe9ffe9a2f1296ef53421d7ed"
            )),
            FieldElement::from(u256h!(
                "03e97a0940ddd5c2ee158a97e6d29dc5129ec9c7a96e34a8237a464f6d51f6ab"
            )),
            FieldElement::from(u256h!(
                "018c45ab286ec38ef666ca02ba3484186270c23b54edc2bac749da3fe78ffc40"
            )),
            FieldElement::from(u256h!(
                "064e9cfd92cd6deb7cf8bd9929bdcc1b6161774432a12575338b829372bc9a8b"
            )),
            FieldElement::from(u256h!(
                "02224d4e3eee94168463684553d1a14d399bf81d3cab736b3bc58480f3832477"
            )),
            FieldElement::from(u256h!(
                "01c2bb2a80a57431bfab9636e98a6c73b24661a19077c2b56f3de44b0896b9f4"
            )),
            FieldElement::from(u256h!(
                "066b5653e399f0d37c44d7e05559098c96d8bec05824c4fb82f8474a8911df74"
            )),
            FieldElement::from(u256h!(
                "037f7c5048aa39d4a8b09861d91c7e7c8d560e7e6dd1da981febdb526b2305d0"
            )),
            FieldElement::from(u256h!(
                "01d7b36c4e979188ec71f7013ac4ff807aa77d379d6e8b9eee04ecfe8ceaa5b6"
            )),
        ]
    }

    // #[test]
    // fn evals_match() {
    //     let coefficients = get_coefficients();
    //
    //     let trace_polynomials = get_trace_polynomials();
    //     let trace_polynomial_references: Vec<&[FieldElement]> =
    //         trace_polynomials.iter().map(|x| x.as_slice()).collect();
    //
    //     let direct_result = eval_c_direct(
    //         &FieldElement::GENERATOR,
    //         &trace_polynomial_references,
    //         0usize,             // not used
    //         FieldElement::ZERO, // not used
    //         &coefficients,
    //     );
    //
    //     let extended_trace_table = get_extended_trace_table();
    //     let extended_trace_table_references: Vec<&[FieldElement]> =
    //         extended_trace_table.iter().map(|x| x.as_slice()).collect();
    //     let whole_loop_result = eval_whole_loop(
    //         &extended_trace_table_references,
    //         &coefficients,
    //         0usize,              // unused
    //         &FieldElement::ZERO, // unused
    //     );
    //
    //     assert_eq!(whole_loop_result[0], direct_result);
    // }

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
    fn wayne() {
        let trace_polynomials = get_trace_polynomials();

        let mut constraint_coefficients = vec![FieldElement::ZERO; 100];
        for i in 0..20 {
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
            &get_pedersen_merkle_constraints(&get_public_input()),
            &constraint_coefficients,
        );
        let new = constraint_polynomial.evaluate(&x);

        assert_eq!(old, new);
    }
}
