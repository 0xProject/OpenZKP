#[cfg(test)]
mod tests {
    use crate::{
        pedersen_merkle::{
            constraints::get_pedersen_merkle_constraints,
            inputs::{starkware_private_input, STARKWARE_PUBLIC_INPUT},
            trace_table::get_trace_table,
        },
        proofs::{stark_proof, ProofParams},
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
}
