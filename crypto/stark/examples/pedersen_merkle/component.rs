use log::info;
use std::time::Instant;
use zkp_macros_decl::{field_element, hex};
use zkp_primefield::FieldElement;
use zkp_stark::{prove, Provable, Component, Constraints};
use zkp_u256::U256;
use std::collections::HashMap;
use crate::inputs::{Claim, Witness};
use crate::constraints::get_pedersen_merkle_constraints;

pub fn pedersen_merkle(claim: &Claim, witness: &Witness) -> Component {
    info!("Constructing constraint system...");
    let constraints = get_pedersen_merkle_constraints(&claim).expressions().to_vec();
    let trace = claim.trace(witness);
    let labels = HashMap::default();
    Component { trace, constraints, labels }
}
