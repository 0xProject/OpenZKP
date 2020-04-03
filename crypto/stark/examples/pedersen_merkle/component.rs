use super::inputs::{Claim, Witness};
use crate::{
    pedersen_points::{merkle_hash, PEDERSEN_POINTS, SHIFT_POINT},
    periodic_columns::{
        LEFT_X_COEFFICIENTS_REF, LEFT_Y_COEFFICIENTS_REF, RIGHT_X_COEFFICIENTS_REF,
        RIGHT_Y_COEFFICIENTS_REF,
    },
};
use itertools::izip;
use zkp_elliptic_curve::Affine;
use zkp_primefield::{FieldElement, One, Pow, Root, Zero};
use zkp_stark::{
    component2::{Component, PolyWriter, Vertical},
    DensePolynomial, RationalExpression, TraceTable,
};
use zkp_u256::{Binary, U256};

fn get_slope(p_1: &Affine, p_2: &Affine) -> FieldElement {
    let (x_1, y_1) = p_1.as_coordinates().unwrap();
    let (x_2, y_2) = p_2.as_coordinates().unwrap();
    (y_1 - y_2) / (x_1 - x_2)
}

struct MerkleTreeLayer;

// TODO: What convention do we want to follow for labels?
#[allow(clippy::unused_self)]
impl MerkleTreeLayer {
    fn new() -> MerkleTreeLayer {
        MerkleTreeLayer {}
    }

    fn left(&self) -> (usize, RationalExpression) {
        use RationalExpression::*;
        (0, Trace(0, 0))
    }

    fn right(&self) -> (usize, RationalExpression) {
        use RationalExpression::*;
        (0, Trace(4, 0))
    }

    fn hash(&self) -> (usize, RationalExpression) {
        use RationalExpression::*;
        (255, Trace(6, 0))
    }
}

impl Component for MerkleTreeLayer {
    type Claim = ();
    type Witness = (FieldElement, FieldElement, bool);

    fn dimensions2(&self) -> (usize, usize) {
        (8, 256)
    }

    fn constraints(&self, _: &Self::Claim) -> Vec<RationalExpression> {
        use RationalExpression::*;

        // Constraints
        let field_element_bits = 252;
        let (shift_point_x, shift_point_y) = match SHIFT_POINT {
            Affine::Zero => panic!(),
            Affine::Point { x, y } => (x, y),
        };

        // Periodic columns
        let periodic = |coefficients| Polynomial(DensePolynomial::new(coefficients), Box::new(X));
        let periodic_left_x = periodic(LEFT_X_COEFFICIENTS_REF);
        let periodic_left_y = periodic(LEFT_Y_COEFFICIENTS_REF);
        let periodic_right_x = periodic(RIGHT_X_COEFFICIENTS_REF);
        let periodic_right_y = periodic(RIGHT_Y_COEFFICIENTS_REF);

        // Repeating patterns
        let omega = FieldElement::root(256).unwrap();
        let omega_i = |i: usize| Constant(omega.pow(i));
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
            on_hash_start_rows(Trace(6, 0) - Constant(shift_point_x)),
            on_hash_start_rows(Trace(7, 0) - Constant(shift_point_y)),
            on_hash_loop_rows(left_bit.clone() * (left_bit.clone() - 1.into())),
            on_hash_loop_rows(
                left_bit.clone() * (Trace(7, 0) - periodic_left_y)
                    - Trace(1, 1) * (Trace(6, 0) - periodic_left_x.clone()),
            ),
            on_hash_loop_rows(
                Trace(1, 1) * Trace(1, 1)
                    - left_bit.clone() * (Trace(6, 0) + periodic_left_x + Trace(2, 1)),
            ),
            on_hash_loop_rows(
                left_bit.clone() * (Trace(7, 0) + Trace(3, 1))
                    - Trace(1, 1) * (Trace(6, 0) - Trace(2, 1)),
            ),
            on_hash_loop_rows(
                (Constant(FieldElement::one()) - left_bit.clone()) * (Trace(6, 0) - Trace(2, 1)),
            ),
            on_hash_loop_rows(
                (Constant(FieldElement::one()) - left_bit) * (Trace(7, 0) - Trace(3, 1)),
            ),
            on_fe_end_rows(Trace(0, 0)),
            on_no_hash_rows(Trace(0, 0)),
            on_hash_loop_rows(right_bit.clone() * (right_bit.clone() - 1.into())),
            on_hash_loop_rows(
                right_bit.clone() * (Trace(3, 1) - periodic_right_y)
                    - Trace(5, 1) * (Trace(2, 1) - periodic_right_x.clone()),
            ),
            on_hash_loop_rows(
                Trace(5, 1) * Trace(5, 1)
                    - right_bit.clone() * (Trace(2, 1) + periodic_right_x + Trace(6, 1)),
            ),
            on_hash_loop_rows(
                right_bit.clone() * (Trace(3, 1) + Trace(7, 1))
                    - Trace(5, 1) * (Trace(2, 1) - Trace(6, 1)),
            ),
            on_hash_loop_rows(
                (Constant(FieldElement::one()) - right_bit.clone()) * (Trace(2, 1) - Trace(6, 1)),
            ),
            on_hash_loop_rows(
                (Constant(FieldElement::one()) - right_bit) * (Trace(3, 1) - Trace(7, 1)),
            ),
            on_fe_end_rows(Trace(4, 0)),
            on_no_hash_rows(Trace(4, 0)),
        ];
        constraints
    }

    fn trace2<P: PolyWriter>(
        &self,
        trace: &mut P,
        _: &Self::Claim,
        (leaf, sibling, direction): &Self::Witness,
    ) {
        let (left, right) = if *direction {
            (sibling, leaf)
        } else {
            (leaf, sibling)
        };
        let mut left_source: U256 = left.into();
        let mut right_source: U256 = right.into();
        let mut left_point = Affine::ZERO;
        let mut right_point = SHIFT_POINT;
        for bit_index in 0..256 {
            let mut left_slope = FieldElement::zero();
            let mut right_slope = FieldElement::zero();
            if bit_index > 0 {
                left_point = right_point.clone();
                if left_source.bit(0) {
                    let p = &PEDERSEN_POINTS[bit_index];
                    left_slope = get_slope(&left_point, &p);
                    left_point += p;
                }
                right_point = left_point.clone();
                if right_source.bit(0) {
                    let p = &PEDERSEN_POINTS[bit_index + 252];
                    right_slope = get_slope(&right_point, &p);
                    right_point += p;
                }
                left_source >>= 1;
                right_source >>= 1;
            }
            trace.write(0, bit_index, FieldElement::from(left_source.clone()));
            trace.write(1, bit_index, left_slope.clone());
            trace.write(
                2,
                bit_index,
                left_point.x().cloned().unwrap_or_else(FieldElement::zero),
            );
            trace.write(
                3,
                bit_index,
                left_point.y().cloned().unwrap_or_else(FieldElement::zero),
            );
            trace.write(4, bit_index, FieldElement::from(right_source.clone()));
            trace.write(5, bit_index, right_slope.clone());
            trace.write(
                6,
                bit_index,
                right_point.x().cloned().unwrap_or_else(FieldElement::zero),
            );
            trace.write(
                7,
                bit_index,
                right_point.y().cloned().unwrap_or_else(FieldElement::zero),
            );
        }
    }
}

pub(crate) struct MerkleTree {
    layers: Vertical<MerkleTreeLayer>,
}

impl MerkleTree {
    pub(crate) fn new(depth: usize) -> MerkleTree {
        let layer = MerkleTreeLayer::new();
        MerkleTree {
            layers: Vertical::new(layer, depth),
        }
    }
}

impl Component for MerkleTree {
    type Claim = Claim;
    type Witness = Witness;

    fn dimensions2(&self) -> (usize, usize) {
        self.layers.dimensions2()
    }

    fn constraints(&self, claim: &Self::Claim) -> Vec<RationalExpression> {
        use RationalExpression::*;
        let fake_claim = vec![(); self.layers.size()];
        let mut constraints = self.layers.constraints(&fake_claim);

        // Construct constraints
        let (rows, columns) = self.dimensions();
        let path_length = self.layers.size();
        let trace_length = rows;
        let root = claim.root.clone();
        let leaf = claim.leaf.clone();

        // Repeating patterns
        let omega = FieldElement::root(trace_length).unwrap();
        let omega_i = |i| Constant(omega.pow(i));
        let row = |i| X - omega_i(i);

        // Connect components together
        // TODO: How do we do this cleanly using labels?
        constraints.insert(
            0,
            (Trace(6, 0) - Trace(0, 1)) * (Trace(6, 0) - Trace(4, 1)) * row(trace_length - 1)
                / (X.pow(path_length) - omega_i(trace_length - path_length)),
        );

        // Add boundary constraints
        // `leaf` is equals either left or right, they should be on the same row
        let left = self.layers.element().left();
        let right = self.layers.element().right();
        assert_eq!(left.0, right.0);
        constraints.insert(
            0,
            (Constant(leaf.clone()) - left.1.clone()) * (Constant(leaf) - right.1) / row(left.0),
        );

        // The final hash equals `root`
        let hash = self.layers.element().hash();
        let row_index = hash.0 + (path_length - 1) * self.layers.element().dimensions().0;
        constraints.insert(1, (Constant(root) - hash.1) / row(row_index));

        // Add column constraints
        for i in 0..columns {
            constraints.insert(i, Trace(i, 0));
        }

        constraints
    }

    fn trace2<P: PolyWriter>(&self, trace: &mut P, claim: &Self::Claim, witness: &Self::Witness) {
        let witness = izip!(witness.directions.iter(), witness.path.iter())
            .scan(claim.leaf.clone(), |leaf, (direction, sibling)| {
                let layer: <MerkleTreeLayer as Component>::Witness =
                    (leaf.clone(), sibling.clone(), *direction);
                *leaf = if *direction {
                    merkle_hash(&sibling, &leaf)
                } else {
                    merkle_hash(&leaf, &sibling)
                };
                Some(layer)
            })
            .collect::<Vec<_>>();
        let claim = vec![(); self.layers.size()];
        self.layers.trace2(trace, &claim, &witness)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        inputs::{short_witness, SHORT_CLAIM},
        pedersen_points::merkle_hash,
    };
    use proptest::{collection::vec as prop_vec, prelude::*};
    use zkp_macros_decl::field_element;
    use zkp_primefield::FieldElement;
    use zkp_stark::{prove, Constraints};
    use zkp_u256::U256;

    // TODO: Move to TraceTable or RationalExpression?
    fn eval(trace: &TraceTable, label: (usize, RationalExpression)) -> FieldElement {
        label.1.evaluate(
            &FieldElement::root(trace.num_rows()).unwrap().pow(label.0),
            &|column, row_offset| {
                let row = ((label.0 as isize) + row_offset) as usize;
                trace[(row, column)].clone()
            },
        )
    }

    #[test]
    fn test_tree_layer_example() {
        let component = MerkleTreeLayer::new();
        let leaf =
            field_element!("061af4ecd745b4c67e476860ce382ae8696dc1e258d02c59557bb7abcf66f1e8");
        let direction = true;
        let sibling =
            field_element!("0465da90a0487ff6d4ea63658db7439f4023957b750f3ae8a5e0a18edef453b1");
        let hash =
            field_element!("02fe7d53bedb42fbc905d7348bd5d61302882ba48a27377b467a9005d6e8d3fd");
        let claim = ();
        let witness = (leaf.clone(), sibling.clone(), direction);
        let trace = component.trace(&claim, &witness);
        assert_eq!(component.check(&claim, &witness), Ok(()));
        assert_eq!(&eval(&trace, component.left()), &sibling);
        assert_eq!(&eval(&trace, component.right()), &leaf);
        assert_eq!(&eval(&trace, component.hash()), &hash);
    }

    #[test]
    fn test_tree_layer() {
        let config = ProptestConfig::with_cases(10);
        proptest!(config, |(leaf: FieldElement, direction: bool, sibling: FieldElement)| {
            let hash = if direction {
                merkle_hash(&sibling, &leaf)
            } else {
                merkle_hash(&leaf, &sibling)
            };
            let component = MerkleTreeLayer::new();
            let claim = ();
            let witness = (leaf, sibling, direction);
            let trace = component.trace(&claim, &witness);
            prop_assert_eq!(component.check(&claim, &witness), Ok(()));
            prop_assert_eq!(&eval(&trace, component.hash()), &hash);
        });
    }

    #[test]
    fn test_pedersen_merkle_small_proof() {
        let claim = SHORT_CLAIM;
        let witness = short_witness();
        let component = MerkleTree::new(witness.path.len());
        let constraints = component.constraints(&claim);
        let trace = component.trace(&claim, &witness);
        let mut constraints = Constraints::from_expressions(
            (trace.num_rows(), trace.num_columns()),
            (&claim).into(),
            constraints,
        )
        .unwrap();
        constraints.blowup = 16;
        constraints.pow_bits = 0;
        constraints.num_queries = 13;
        constraints.fri_layout = vec![3, 2];
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

    #[test]
    fn test_pedersen_merkle() {
        let config = ProptestConfig::with_cases(10);
        let witness = (0_usize..4)
            .prop_flat_map(|log_size| {
                let size = 1 << log_size;
                (
                    prop_vec(bool::arbitrary(), size),
                    prop_vec(FieldElement::arbitrary(), size),
                )
            })
            .prop_map(|(directions, path)| Witness { directions, path });
        proptest!(config, |(witness in witness, claim: FieldElement)| {
            let claim = Claim::from_leaf_witness(claim, &witness);
            let component = MerkleTree::new(witness.path.len());
            prop_assert_eq!(component.check(&claim, &witness), Ok(()));
        });
    }
}
