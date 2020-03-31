use super::inputs::{Claim, Witness};
use crate::pedersen_points::merkle_hash;
use itertools::izip;
use zkp_primefield::{FieldElement, One, Pow, Root};
use zkp_stark::{
    component2::{Component, Vertical},
    RationalExpression, TraceTable,
};

struct MerkleTreeLayer;

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

    fn dimensions(&self) -> (usize, usize) {
        (256, 8)
    }

    fn constraints(&self, _: &Self::Claim) -> Vec<RationalExpression> {
        use crate::{
            pedersen_points::SHIFT_POINT,
            periodic_columns::{
                LEFT_X_COEFFICIENTS_REF, LEFT_Y_COEFFICIENTS_REF, RIGHT_X_COEFFICIENTS_REF,
                RIGHT_Y_COEFFICIENTS_REF,
            },
        };
        use zkp_elliptic_curve::Affine;
        use zkp_stark::DensePolynomial;
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
                (Constant(FieldElement::one()) - left_bit.clone()) * (Trace(6, 0) - Trace(2, 1)),
            ),
            on_hash_loop_rows(
                (Constant(FieldElement::one()) - left_bit.clone()) * (Trace(7, 0) - Trace(3, 1)),
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
                (Constant(FieldElement::one()) - right_bit.clone()) * (Trace(2, 1) - Trace(6, 1)),
            ),
            on_hash_loop_rows(
                (Constant(FieldElement::one()) - right_bit.clone()) * (Trace(3, 1) - Trace(7, 1)),
            ),
            on_fe_end_rows(Trace(4, 0)),
            on_no_hash_rows(Trace(4, 0)),
        ];
        constraints
    }

    fn trace(&self, _: &Self::Claim, (leaf, sibling, direction): &Self::Witness) -> TraceTable {
        let mut data = vec!["a", "bcd", "ef", "ghij"];
        data.sort_by_key(|a| usize::max_value() - a.len());

        use crate::component::{get_coordinates, hash_next_bit, initialize_hash};

        let mut trace = TraceTable::new(256, 8);
        let mut row = if *direction {
            initialize_hash(sibling.into(), leaf.into())
        } else {
            initialize_hash(leaf.into(), sibling.into())
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
        // TODO: Check hash
        trace
    }
}

struct MerkleTree {
    layers: Vertical<MerkleTreeLayer>,
}

impl MerkleTree {
    fn new(depth: usize) -> MerkleTree {
        let layer = MerkleTreeLayer::new();
        MerkleTree {
            layers: Vertical::new(layer, depth),
        }
    }
}

impl Component for MerkleTree {
    type Claim = Claim;
    type Witness = Witness;

    fn dimensions(&self) -> (usize, usize) {
        self.layers.dimensions()
    }

    fn constraints(&self, claim: &Self::Claim) -> Vec<RationalExpression> {
        let claim = vec![(); self.layers.size()];
        self.layers.constraints(&claim)
    }

    fn trace(&self, claim: &Self::Claim, witness: &Self::Witness) -> TraceTable {
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
        self.layers.trace(&claim, &witness)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pedersen_points::merkle_hash;
    use proptest::{collection::vec as prop_vec, prelude::*};
    use zkp_primefield::FieldElement;

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
            prop_assert_eq!(component.check(&claim, &witness), Ok(()));
            // TODO:
            // assert_eq!(component.eval_label("hash"), hash);
        });
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
