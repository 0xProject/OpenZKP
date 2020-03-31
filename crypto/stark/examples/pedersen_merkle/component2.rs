use super::inputs::{Claim, Witness};
use zkp_primefield::FieldElement;
use zkp_stark::{component2::Component, RationalExpression, TraceTable};

struct MerkleTreeLayer;

impl MerkleTreeLayer {
    fn new() -> MerkleTreeLayer {
        MerkleTreeLayer {}
    }
}

impl Component for MerkleTreeLayer {
    type Claim = (FieldElement, FieldElement);
    type Witness = (FieldElement, bool);

    fn dimensions(&self) -> (usize, usize) {
        unimplemented!()
    }

    fn constraints(&self, _claim: &Self::Claim) -> Vec<RationalExpression> {
        unimplemented!()
    }

    fn trace(&self, _claim: &Self::Claim, _witness: &Self::Witness) -> TraceTable {
        unimplemented!()
    }
}

struct MerkleTree;

impl MerkleTree {
    fn new() -> MerkleTree {
        MerkleTree {}
    }
}

impl Component for MerkleTree {
    type Claim = Claim;
    type Witness = Witness;

    fn dimensions(&self) -> (usize, usize) {
        unimplemented!()
    }

    fn constraints(&self, _claim: &Self::Claim) -> Vec<RationalExpression> {
        unimplemented!()
    }

    fn trace(&self, _claim: &Self::Claim, _witness: &Self::Witness) -> TraceTable {
        unimplemented!()
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
            let claim = (leaf, hash);
            let witness = (sibling, direction);
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
            let component = MerkleTree::new();
            prop_assert_eq!(component.check(&claim, &witness), Ok(()));
        });
    }
}
