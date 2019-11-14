#[cfg(test)]
mod tests {
    use crate::{
        constraints::constraints,
        inputs::{Claim, Modification, Parameters, SignatureParameters},
        pedersen_points::SHIFT_POINT,
    };
    use zkp_macros_decl::field_element;
    use zkp_primefield::FieldElement;
    use zkp_stark::{check_constraints, Constraints, TraceTable};
    use zkp_u256::U256;

    #[test]
    fn test_trace() {
        let parameters = Parameters {
            signature:        SignatureParameters {
                shift_point: SHIFT_POINT,
                alpha:       FieldElement::ONE,
                beta:        field_element!(
                    "06f21413efbe40de150e596d72f7a8c5609ad26c15c915c1f4cdfcb99cee9e89"
                ),
            },
            hash_shift_point: SHIFT_POINT,
            n_vaults:         30,
        };

        let claim = Claim {
            n_transactions:      4,
            modifications:       vec![
                Modification {
                    initial_amount: 0,
                    final_amount:   1000,
                    index:          0,
                    key:            field_element!(
                        "057d5d2e5da7409db60d64ae4e79443fedfd5eb925b5e54523eaf42cc1978169"
                    ),
                    token:          field_element!(
                        "03e7aa5d1a9b180d6a12d451d3ae6fb95e390f722280f1ea383bb49d11828d"
                    ),
                    vault:          1,
                },
                Modification {
                    initial_amount: 0,
                    final_amount:   1000,
                    index:          1,
                    key:            field_element!(
                        "024dca9f8032c9c8d1a2aae85b49df5dded9bb8da46d32284e339f5a9b30e820"
                    ),
                    token:          field_element!(
                        "03e7aa5d1a9b180d6a12d451d3ae6fb95e390f722280f1ea383bb49d11828d"
                    ),
                    vault:          2,
                },
                Modification {
                    initial_amount: 0,
                    final_amount:   1000,
                    index:          2,
                    key:            field_element!(
                        "03be0fef73793139380d0d5c27a33d6b1a67c29eb3bbe24e5635bc13b3439542"
                    ),
                    token:          field_element!(
                        "03e7aa5d1a9b180d6a12d451d3ae6fb95e390f722280f1ea383bb49d11828d"
                    ),
                    vault:          3,
                },
                Modification {
                    initial_amount: 0,
                    final_amount:   1000,
                    index:          3,
                    key:            field_element!(
                        "03f0f302fdf6ba1a4669ce4fc9bd2b4ba17bdc088ae32984f40c26e7006d2f9b"
                    ),
                    token:          field_element!(
                        "03e7aa5d1a9b180d6a12d451d3ae6fb95e390f722280f1ea383bb49d11828d"
                    ),
                    vault:          4,
                },
            ],
            initial_vaults_root: field_element!(
                "00156823f988424670b3a750156e77068328aa496ff883106ccc78ff85ea1dc1"
            ),
            final_vaults_root:   field_element!(
                "0181ae03ea55029827c08a70034df9861bc6c86689205155d966f28bf2cfb20a"
            ),
        };

        let constraints = constraints(&claim, &parameters);
        let trace_length = claim.n_transactions * 65536;

        let system =
            Constraints::from_expressions((trace_length, 10), vec![], constraints).unwrap();

        let trace_table = TraceTable::new(trace_length, 10);

        assert!(check_constraints(&system, &trace_table).is_ok());
    }
}
