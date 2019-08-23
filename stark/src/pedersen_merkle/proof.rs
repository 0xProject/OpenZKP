#[cfg(test)]
mod tests {
    use crate::{
        pedersen_merkle::{
            constraints::get_pedersen_merkle_constraints,
            inputs::{
                short_private_input, starkware_private_input, SHORT_PUBLIC_INPUT,
                STARKWARE_PUBLIC_INPUT,
            },
            trace_table::get_trace_table,
        },
        proof_params::ProofParams,
        proofs::stark_proof,
    };
    use macros_decl::hex;

    #[test]
    #[ignore]
    fn starkware_pedersen_merkle() {
        let public_input = STARKWARE_PUBLIC_INPUT;
        let private_input = starkware_private_input();
        let trace_table = get_trace_table(&public_input, &private_input);

        let constraints = &get_pedersen_merkle_constraints(&public_input);

        let proof = stark_proof(&trace_table, &constraints, &public_input, &ProofParams {
            blowup:                   16,
            pow_bits:                 28,
            queries:                  13,
            fri_layout:               vec![3, 3, 3, 3, 2],
            constraints_degree_bound: 2,
        });

        assert_eq!(
            proof.proof[0..32],
            hex!("b00a4c7f03959e01df2504fb73d2b238a8ab08b2000000000000000000000000")
        );
        assert_eq!(
            proof.proof[32..64],
            hex!("2e821fe1f3062acdbd3a4bd0be2293f4264abc7b000000000000000000000000")
        );

        // FRI commitments
        assert_eq!(
            proof.proof[640..672],
            hex!("b5ae7a8389c7de33f08f79c7dca057e5db5c0d65000000000000000000000000")
        );
        assert_eq!(
            proof.proof[672..704],
            hex!("83f4858900e1519c1b788333f55b54762485e5d6000000000000000000000000")
        );
        assert_eq!(
            proof.proof[704..736],
            hex!("be090ca452f0affe901588d522960b7b92d8882c000000000000000000000000")
        );
        assert_eq!(
            proof.proof[736..768],
            hex!("3cc9adaad436cfab60978d57f13d5f22e6a8791f000000000000000000000000")
        );
        assert_eq!(
            proof.proof[768..800],
            hex!("8af79c56d74b9252c3c542fc2b56d4692c608c98000000000000000000000000")
        );
    }

    #[test]
    fn short_pedersen_merkle() {
        let public_input = SHORT_PUBLIC_INPUT;
        let private_input = short_private_input();
        let trace_table = get_trace_table(&public_input, &private_input);

        let constraints = &get_pedersen_merkle_constraints(&public_input);

        let proof = stark_proof(&trace_table, &constraints, &public_input, &ProofParams {
            blowup:                   16,
            pow_bits:                 0,
            queries:                  13,
            fri_layout:               vec![3, 2],
            constraints_degree_bound: 2,
        });

        assert_eq!(
            hex::encode(proof.proof[0..32].to_vec()),
            "e2c4e35c37e33aa3b439592f2f3c5c82f464f026000000000000000000000000"
        );
        assert_eq!(
            hex::encode(proof.proof[32..64].to_vec()),
            "c5df989253ac4c3eff4fdb4130f832db1d2a9826000000000000000000000000"
        );

        // FRI commitments
        assert_eq!(
            hex::encode(proof.proof[640..672].to_vec()),
            "744f04f8bcd9c5aafb8907586428fbe9dd81b976000000000000000000000000"
        );
        assert_eq!(
            hex::encode(proof.proof[672..704].to_vec()),
            "ce329839a5eccb8009ffebf029312989e68f1cde000000000000000000000000"
        );
    }
}
