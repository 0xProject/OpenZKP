#[cfg(feature = "prover")]
use crate::constraint_system::Provable;
use crate::{
    constraint_system::Verifiable,
    constraints::Constraints,
    pedersen_merkle::{constraints::get_pedersen_merkle_constraints, trace_table::get_trace_table},
    trace_table::TraceTable,
};
use primefield::FieldElement;
use std::{prelude::v1::*, vec};

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
    pub directions: Vec<bool>,
    pub path:       Vec<FieldElement>,
}

impl Verifiable for Claim {
    fn constraints(&self) -> Constraints {
        get_pedersen_merkle_constraints(self)
    }

    fn trace_length(&self) -> usize {
        256 * self.path_length
    }

    fn trace_columns(&self) -> usize {
        8
    }
}

#[cfg(feature = "prover")]
impl Provable<Claim> for Witness {
    fn trace(&self, claim: &Claim) -> TraceTable {
        get_trace_table(claim, self)
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

#[cfg(test)]
use macros_decl::field_element;

#[cfg(test)]
use u256::U256;

#[cfg(test)]
pub const SHORT_CLAIM: Claim = Claim {
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
pub fn short_witness() -> Witness {
    Witness {
        directions: SHORT_DIRECTIONS.to_vec(),
        path:       SHORT_PATH.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::channel::{ProverChannel, RandomGenerator, Writable};
    use hash::Hash;
    use macros_decl::hex;
    use u256::U256;

    #[test]
    fn claim_writable_matches_starkware() {
        // Test that our implementation of Writable for Claim matches StarkWare's
        // by checking that the first random element we generate for the proof (the
        // first constraint coefficient) matches the the one in
        // pedersen_merkle_proof_annotations.txt.
        let mut proof = ProverChannel::new();
        proof.initialize(&Vec::from(&SHORT_CLAIM));

        // This is /pedersen merkle/STARK/Original/Commit on Trace
        proof.write(&Hash::new(hex!(
            "b00a4c7f03959e01df2504fb73d2b238a8ab08b2000000000000000000000000"
        )));

        let first_random: FieldElement = proof.get_random();
        let first_constraint_coefficient =
            field_element!("0458928c6aa01a8aa95f4ece0cd405277e9966231ee2defa4d817eeb8391cb36");
        assert_eq!(first_random, first_constraint_coefficient);
    }
}
