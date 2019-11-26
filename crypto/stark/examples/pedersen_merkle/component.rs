use crate::{
    constraints::get_pedersen_merkle_constraints,
    inputs::{Claim, Witness},
};
use log::info;
use std::collections::HashMap;
use zkp_stark::{Component, Provable};

pub fn pedersen_merkle(claim: &Claim, witness: &Witness) -> Component {
    info!("Constructing constraint system...");
    let constraints = get_pedersen_merkle_constraints(&claim)
        .expressions()
        .to_vec();
    let trace = claim.trace(witness);
    let labels = HashMap::default();
    Component {
        trace,
        constraints,
        labels,
    }
}
