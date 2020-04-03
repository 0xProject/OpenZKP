use super::pedersen_points::merkle_hash;
use std::{prelude::v1::*, vec};
use zkp_primefield::FieldElement;

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Claim {
    pub path_length: usize,
    pub leaf:        FieldElement,
    pub root:        FieldElement,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Witness {
    pub path: Vec<(bool, FieldElement)>,
    pub leaf: FieldElement,
    pub root: FieldElement,
}

impl Witness {
    pub fn new(leaf: FieldElement, path: Vec<(bool, FieldElement)>) -> Witness {
        let root = path
            .iter()
            .fold(leaf.clone(), |leaf, (direction, sibling)| {
                if *direction {
                    merkle_hash(sibling, &leaf)
                } else {
                    merkle_hash(&leaf, sibling)
                }
            });
        Witness { leaf, path, root }
    }
}

impl From<&Witness> for Claim {
    fn from(witness: &Witness) -> Self {
        Claim {
            path_length: witness.path.len(),
            leaf:        witness.leaf.clone(),
            root:        witness.root.clone(),
        }
    }
}

impl From<&Claim> for Vec<u8> {
    fn from(claim: &Claim) -> Self {
        let mut bytes: Self = vec![];
        bytes.extend_from_slice(&claim.path_length.to_be_bytes());
        bytes.extend_from_slice(&claim.root.as_montgomery().to_bytes_be());
        bytes.extend_from_slice(&claim.leaf.as_montgomery().to_bytes_be());
        bytes
    }
}

impl Claim {
    // TODO: Remove
    pub fn from_leaf_witness(leaf: FieldElement, witness: &Witness) -> Self {
        let mut root = leaf.clone();
        for (direction, sibling) in witness.path.iter() {
            root = if *direction {
                merkle_hash(sibling, &root)
            } else {
                merkle_hash(&root, sibling)
            }
        }
        Claim {
            path_length: witness.path.len(),
            leaf,
            root,
        }
    }

    pub fn verify(&self, witness: &Witness) {
        assert_eq!(self, &Claim::from_leaf_witness(self.leaf.clone(), witness));
    }
}

#[cfg(test)]
use zkp_macros_decl::field_element;

#[cfg(test)]
use zkp_u256::U256;

#[cfg(test)]
pub(crate) const SHORT_CLAIM: Claim = Claim {
    path_length: 4,
    leaf:        field_element!("00"),
    root:        field_element!("0720d51348b23cb2ca2c3c279ad338b759cbe85aa986f1e3e6e5dad5fff30255"),
};

#[cfg(test)]
const SHORT_DIRECTIONS: [bool; 4] = [true, false, true, true];

#[cfg(test)]
const SHORT_PATH: [FieldElement; 4] = [
    field_element!("01"),
    field_element!("02"),
    field_element!("03"),
    field_element!("04"),
];

#[cfg(test)]
pub(crate) fn short_witness() -> Witness {
    let path = SHORT_DIRECTIONS
        .iter()
        .copied()
        .zip(SHORT_PATH.iter().cloned())
        .collect::<Vec<_>>();
    Witness::new(field_element!("00"), path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use zkp_macros_decl::hex;

    #[test]
    fn claim_writable_correct() {
        assert_eq!(Vec::from(&SHORT_CLAIM), hex!("0000000000000004062b7c2734c31d5b73119a5bfdb460c0411af12fafd42af8ca041fea5ec464d00000000000000000000000000000000000000000000000000000000000000000").to_vec());
    }
}
