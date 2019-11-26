use super::{
    inputs::Claim,
    pedersen_points::SHIFT_POINT,
    periodic_columns::{
        LEFT_X_COEFFICIENTS, LEFT_Y_COEFFICIENTS, RIGHT_X_COEFFICIENTS, RIGHT_Y_COEFFICIENTS,
    },
};
use std::{prelude::v1::*, vec};
use zkp_elliptic_curve::Affine;
use zkp_primefield::FieldElement;
use zkp_stark::{Constraints, DensePolynomial, RationalExpression};


#[cfg(test)]
mod tests {
    use super::{
        super::inputs::{short_witness, SHORT_CLAIM},
        *,
    };
    use zkp_stark::{prove, Provable, Verifiable};

    #[test]
    fn short_pedersen_merkle() {
        let claim = SHORT_CLAIM;
        let witness = short_witness();

        let mut constraints = claim.constraints();
        constraints.blowup = 16;
        constraints.pow_bits = 0;
        constraints.num_queries = 13;
        constraints.fri_layout = vec![3, 2];

        let trace = claim.trace(&witness);
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
}
