use super::{
    inputs::{Claim, Witness},
    pedersen_points::{PEDERSEN_POINTS, SHIFT_POINT},
    periodic_columns::{
        LEFT_X_COEFFICIENTS, LEFT_Y_COEFFICIENTS, RIGHT_X_COEFFICIENTS, RIGHT_Y_COEFFICIENTS,
    },
};
use itertools::Itertools;
use log::info;
use std::collections::HashMap;
use zkp_elliptic_curve::Affine;
use zkp_primefield::FieldElement;
use zkp_stark::{compose_vertical, Component, DensePolynomial, RationalExpression, TraceTable};
use zkp_u256::U256;

pub fn tree_layer(leaf: &FieldElement, direction: bool, sibling: &FieldElement) -> Component {
    use RationalExpression::*;

    // Compute trace table
    let mut trace = TraceTable::new(256, 8);
    let leaf = U256::from(leaf);
    let other_hash = U256::from(sibling);
    let mut row = if direction {
        initialize_hash(other_hash, leaf)
    } else {
        initialize_hash(leaf, other_hash)
    };
    for bit_index in 0..256 {
        if bit_index > 0 {
            row = hash_next_bit(&row, bit_index);
        }

        let (left_x, left_y) = get_coordinates(&row.left.point);
        trace[(bit_index, 0)] = FieldElement::from(row.left.source.clone());
        trace[(bit_index, 1)] = row.left.slope.clone();
        trace[(bit_index, 2)] = left_x.clone();
        trace[(bit_index, 3)] = left_y.clone();

        let (right_x, right_y) = get_coordinates(&row.right.point);
        trace[(bit_index, 4)] = FieldElement::from(row.right.source.clone());
        trace[(bit_index, 5)] = row.right.slope.clone();
        trace[(bit_index, 6)] = right_x.clone();
        trace[(bit_index, 7)] = right_y.clone();
    }

    // Constraints
    let field_element_bits = 252;
    let (shift_point_x, shift_point_y) = match SHIFT_POINT {
        Affine::Zero => panic!(),
        Affine::Point { x, y } => (x, y),
    };

    // Periodic columns
    let periodic = |coefficients| Polynomial(DensePolynomial::new(coefficients), Box::new(X));
    let periodic_left_x = periodic(&LEFT_X_COEFFICIENTS);
    let periodic_left_y = periodic(&LEFT_Y_COEFFICIENTS);
    let periodic_right_x = periodic(&RIGHT_X_COEFFICIENTS);
    let periodic_right_y = periodic(&RIGHT_Y_COEFFICIENTS);

    // Repeating patterns
    let omega = FieldElement::root(256).unwrap();
    let omega_i = |i| Constant(omega.pow(i));
    let row = |i| X - omega_i(i);
    let all_rows = || X.pow(256) - 1.into();
    let on_no_hash_rows = |a: RationalExpression| a / row(255);
    let on_hash_start_rows = |a: RationalExpression| a / row(0);
    let on_hash_loop_rows = |a: RationalExpression| a * row(255) / all_rows();
    let on_fe_end_rows = |a: RationalExpression| a / row(field_element_bits);

    // Common sub-expressions
    let left_bit = Trace(0, 0) - Trace(0, 1) * 2.into();
    let right_bit = Trace(4, 0) - Trace(4, 1) * 2.into();

    let constraints = vec![
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

    // Labels
    let mut labels = HashMap::default();
    labels.insert("left".to_owned(), (0, Trace(0, 0)));
    labels.insert("right".to_owned(), (0, Trace(4, 0)));
    labels.insert("hash".to_owned(), (255, Trace(6, 0)));

    Component {
        trace,
        constraints,
        labels,
    }
}

pub fn pedersen_merkle(claim: &Claim, witness: &Witness) -> Component {
    use RationalExpression::*;
    info!("Constructing constraint system...");

    // Vertically compose tree_levels
    let mut hash = claim.leaf.clone();
    let mut components = Vec::default();
    for (direction, sibling) in witness.directions.iter().zip(witness.path.iter()) {
        let component = tree_layer(&hash, *direction, sibling);
        hash = component.eval_label("hash");
        components.push(component);
    }
    while components.len() > 1 {
        components = components
            .into_iter()
            .tuples()
            .map(|(top, bottom)| {
                let mut composite = compose_vertical(top, bottom);
                composite.rename_label("top_left", "left");
                composite.rename_label("top_right", "right");
                composite.remove_label("top_hash");
                composite.remove_label("bottom_left");
                composite.remove_label("bottom_right");
                composite.rename_label("bottom_hash", "hash");
                composite
            })
            .collect()
    }
    let mut component = components[0].clone();

    // Construct constraints
    let path_length = claim.path_length;
    let trace_length = component.trace.num_rows();
    let root = claim.root.clone();
    let leaf = claim.leaf.clone();

    // Repeating patterns
    let omega = FieldElement::root(trace_length).unwrap();
    let omega_i = |i| Constant(omega.pow(i));
    let row = |i| X - omega_i(i);

    // Connect components together
    // TODO: How do we do this cleanly using labels?
    component.constraints.insert(
        0,
        (Trace(6, 0) - Trace(0, 1)) * (Trace(6, 0) - Trace(4, 1)) * row(trace_length - 1)
            / (X.pow(path_length) - omega_i(trace_length - path_length)),
    );

    // Add boundary constraints
    // `leaf` is equals either left or right, they should be on the same row
    assert_eq!(component.labels["left"].0, component.labels["left"].0);
    component.constraints.insert(
        0,
        (Constant(leaf.clone()) - component.labels["left"].1.clone())
            * (Constant(leaf.clone()) - component.labels["right"].1.clone())
            / row(component.labels["left"].0),
    );
    // The final hash equals `root`
    component.constraints.insert(
        1,
        (Constant(root.clone()) - component.labels["hash"].1.clone())
            / row(component.labels["hash"].0),
    );

    // Add column constraints
    for i in 0..component.trace.num_columns() {
        component.constraints.insert(i, Trace(i, 0));
    }

    component
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

#[cfg(test)]
mod test {
    use super::{
        super::{
            inputs::{short_witness, SHORT_CLAIM},
            pedersen_points::merkle_hash,
        },
        *,
    };
    use quickcheck_macros::quickcheck;
    use rand::{
        distributions::{Distribution, Uniform},
        Rng, SeedableRng,
    };
    use rand_xoshiro::Xoshiro256PlusPlus;
    use zkp_macros_decl::field_element;
    use zkp_stark::prove;
    use zkp_u256::U256;

    #[test]
    fn test_tree_layer_example() {
        let leaf =
            field_element!("061af4ecd745b4c67e476860ce382ae8696dc1e258d02c59557bb7abcf66f1e8");
        let direction = true;
        let sibling =
            field_element!("0465da90a0487ff6d4ea63658db7439f4023957b750f3ae8a5e0a18edef453b1");
        let hash =
            field_element!("02fe7d53bedb42fbc905d7348bd5d61302882ba48a27377b467a9005d6e8d3fd");
        let component = tree_layer(&leaf, direction, &sibling);
        assert!(component.check());
        assert_eq!(&component.eval_label("left"), &sibling);
        assert_eq!(&component.eval_label("right"), &leaf);
        assert_eq!(&component.eval_label("hash"), &hash);
    }

    #[quickcheck]
    fn test_tree_layer(leaf: FieldElement, direction: bool, sibling: FieldElement) {
        let component = tree_layer(&leaf, direction, &sibling);
        let hash = if direction {
            merkle_hash(&sibling, &leaf)
        } else {
            merkle_hash(&leaf, &sibling)
        };
        assert!(component.check());
        assert_eq!(component.eval_label("hash"), hash);
    }

    #[test]
    fn test_pedersen_merkle_small_proof() {
        let claim = SHORT_CLAIM;
        let witness = short_witness();
        let component = pedersen_merkle(&claim, &witness);
        let mut constraints = Constraints::from_expressions(
            (component.trace.num_rows(), component.trace.num_columns()),
            (&claim).into(),
            component.constraints,
        )
        .unwrap();
        constraints.blowup = 16;
        constraints.pow_bits = 0;
        constraints.num_queries = 13;
        constraints.fri_layout = vec![3, 2];
        let proof = prove(&constraints, &component.trace).unwrap();

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

    #[quickcheck]
    fn test_pedersen_merkle(seed: u64) {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        let size = 1 << Uniform::from(0..8).sample(&mut rng);
        let witness = Witness {
            directions: (0..size).map(|_| rng.gen()).collect(),
            path:       (0..size).map(|_| rng.gen()).collect(),
        };
        let claim = Claim::from_leaf_witness(rng.gen(), &witness);
        let component = pedersen_merkle(&claim, &witness);
        assert!(component.check());
    }
}
