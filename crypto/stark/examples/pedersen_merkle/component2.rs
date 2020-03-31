use super::{
    inputs::{Claim, Witness},
    pedersen_points::{PEDERSEN_POINTS, SHIFT_POINT},
    periodic_columns::{
        LEFT_X_COEFFICIENTS_REF, LEFT_Y_COEFFICIENTS_REF, RIGHT_X_COEFFICIENTS_REF,
        RIGHT_Y_COEFFICIENTS_REF,
    },
};
use itertools::Itertools;
use log::info;
use std::collections::HashMap;
use zkp_elliptic_curve::Affine;
use zkp_primefield::{FieldElement, One, Pow, Root, Zero};
use zkp_stark::{component2::Component, DensePolynomial, RationalExpression, TraceTable};
use zkp_u256::{Binary, U256};

struct MerkleTree;

impl Component for MerkleTree {
    type Claim = Claim;
    type Witness = Witness;

    fn dimensions(&self) -> (usize, usize) {
        unimplemented!()
    }

    fn constraints(&self, claim: &Self::Claim) -> Vec<RationalExpression> {
        unimplemented!()
    }

    fn trace(&self, claim: &Self::Claim, witness: &Self::Witness) -> TraceTable {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::{
        *,
    };
    use proptest::{collection::vec as prop_vec, prelude::*};

    
}
