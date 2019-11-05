// use super::{constraints::get_pedersen_merkle_constraints,
// trace_table::get_trace_table};
use std::{collections::BTreeMap, prelude::v1::*};
use zkp_elliptic_curve::Affine;
use zkp_hash::Hash;
use zkp_primefield::FieldElement;
use zkp_stark::{Constraints, Provable, TraceTable, Verifiable};

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Claim {
    pub modifications:       Vec<Modification>,
    pub initial_vaults_root: Hash,
    pub final_vaults_root:   Hash,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Witness {
    pub initial_vaults: Vaults,
    pub settlements:    Vec<Settlement>,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Parameters {
    pub signature:        SignatureParameters,
    pub hash_shift_point: Affine,
    pub n_vaults:         usize,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct SignatureParameters {
    pub shift_point: Affine,
    pub alpha:       FieldElement,
    pub beta:        FieldElement,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Settlement {
    maker: Modification,
    taker: Modification,
    index: usize,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Modification {
    initial_amount: u32,
    final_amount:   u32,
    index:          usize,
    key:            FieldElement,
    token:          FieldElement,
    vault:          u32,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Vault {
    key:    FieldElement,
    token:  FieldElement,
    amount: usize,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Vaults {
    tree:   Tree,
    vaults: BTreeMap<u32, Vault>,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Tree {
    height: usize,
    hash:   FieldElement,
    left:   Option<Box<Tree>>,
    right:  Option<Box<Tree>>,
}

impl Vaults {
    pub fn new() -> Self {
        Self {
            tree:   Tree::new(31),
            vaults: BTreeMap::new(),
        }
    }
}

impl Tree {
    pub fn new(height: usize) -> Self {
        Self {
            height,
            hash:   Self::empty_hash(height),
            left:   None,
            right:  None,
        }
    }

    fn empty_hash(height: usize) -> FieldElement {
        FieldElement::ZERO
    }
}
