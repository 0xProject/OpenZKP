#[cfg(test)]
mod tests {
    use crate::{
        fft::{bit_reversal_fft_cofactor, ifft},
        merkle::make_tree,
        mmap_vec::MmapVec,
        pedersen_merkle::{
            input::{get_private_input, get_public_input},
            trace_table::get_trace,
        },
        polynomial::eval_poly,
        utils::Reversible,
    };
    use hex_literal::*;
    use primefield::FieldElement;
    use rayon::prelude::*;
    use u256::U256;

    #[test]
    fn pedersen_merkle_proof() {
        let public_input = get_public_input();
        let trace_table = get_trace(
            public_input.path_length,
            public_input.leaf,
            &get_private_input(),
        );

        let mut columns: [Vec<FieldElement>; 8] = Default::default();
        for (i, value) in trace_table.iter().enumerate() {
            if i % 4 == 0 {
                columns[i % 8].push(value.clone());
            } else {
                columns[i % 8].push(value.clone());
            }
        }

        let trace_length: usize = public_input.path_length * 256;
        let trace_generator =
            FieldElement::GENERATOR.pow(FieldElement::MODULUS / U256::from(trace_length as u64));

        let mut trace_polynomials: Vec<Vec<FieldElement>> =
            vec![Vec::with_capacity(trace_length); 8];
        columns
            .into_par_iter()
            .map(|c| ifft(&c))
            .collect_into_vec(&mut trace_polynomials);

        let oods_point = FieldElement::from_hex_str(
            "0x273966fc4697d1762d51fe633f941e92f87bdda124cf7571007a4681b140c05",
        );
        let shifted_oods_point = &trace_generator * oods_point.clone();

        let expected_field_elements = vec![
            (
                "0x1c55a628c340086e7b03b833483a41e49232f2eb3cf7efe399af42d36026793",
                "0x6a5fac2d52aad81e922c8e21515d3b93e2184137af76cec9ee16428bb3d8742",
            ),
            (
                "0x37166910df8fec267b29d203031fb13e7f6da72863d9fe77e8a735d6a1e79a5",
                "0xb9a28b911c2aaef882a6dfb7ff291cc98afe46d39c04cc7add60167d28320f",
            ),
            (
                "0x221a5558fb6b1bcc8a61ba4aae7e0646ff4d7690e58a64cc53fdff836a3bc18",
                "0x336b6efed32a340ec120f4eb8124a70df35548e8a0f71d207cd746bcc815606",
            ),
            (
                "0x1ab3ec5448b6246fca3274aae40db371d6ab1d2d1ff2d32cfc598393d0458a2",
                "0x71499fe5c6d16e0de24de83ee50f2d7068b636dd6a5ae6faaa83549b50348ba",
            ),
            (
                "0x602e311233369f3f2e214f1e07345ce5a57b1c281e99a55d75966a29cb241e5",
                "0x4f22d2b9e7fad2a83e220be43671604520d2b2e83d986061409b304ae5ac0ad",
            ),
            (
                "0x227b8ac64b82ff81f247c523e63c8fe2dd23f198fb5da96209d465f9ad9a13b",
                "0x4cfb2bfb81f724710fe4e80489dff8757835c157495c8257fe9283395b10bc5",
            ),
            (
                "0x76bbb9f25fc2dec3d5ae212c754289e4ada4bf192d3b76ecdb8708e25f1b474",
                "0x549a1f9ac0513424ac4311e8bc8830c527a375776cde770247d06389f74e895",
            ),
            (
                "0x601aa2e1927d2b8c37b29e7a82d0e44fbfb9598c3cdef28beb8a9c31f3ebf8a",
                "0x544e59775ac2833e4c353ec09dd296cbc7b2c9cbd6642da40859d64c534ce79",
            ),
        ];

        for (i, (f_1, f_2)) in expected_field_elements.iter().enumerate() {
            assert_eq!(
                eval_poly(oods_point.clone(), &trace_polynomials[i]),
                FieldElement::from_hex_str(*f_1)
            );
            assert_eq!(
                eval_poly(shifted_oods_point.clone(), &trace_polynomials[i]),
                FieldElement::from_hex_str(*f_2)
            );
        }

        let beta = 16usize;
        let evaluation_length = trace_length * beta;
        let evaluation_generator =
            FieldElement::root(U256::from(evaluation_length as u64)).unwrap();
        let evaluation_offset = FieldElement::GENERATOR;

        let mut extended_trace_table: MmapVec<[FieldElement; 8]> =
            MmapVec::with_capacity(evaluation_length);

        for i in 0..beta {
            let mut coset_leaves: Vec<Vec<FieldElement>> =
                vec![Vec::with_capacity(trace_length); 8];
            trace_polynomials
                .clone()
                .into_par_iter()
                .map(|p| {
                    let reverse_i = i.bit_reverse() >> (64 - 4);
                    let cofactor =
                        &evaluation_offset * evaluation_generator.pow(U256::from(reverse_i as u64));
                    bit_reversal_fft_cofactor(&p, &cofactor)
                })
                .collect_into_vec(&mut coset_leaves);

            for j in 0..trace_length {
                extended_trace_table.push([
                    coset_leaves[0][j].clone(),
                    coset_leaves[1][j].clone(),
                    coset_leaves[2][j].clone(),
                    coset_leaves[3][j].clone(),
                    coset_leaves[4][j].clone(),
                    coset_leaves[5][j].clone(),
                    coset_leaves[6][j].clone(),
                    coset_leaves[7][j].clone(),
                ])
            }
        }

        let merkle_tree = make_tree(&extended_trace_table);
        assert_eq!(
            merkle_tree[1],
            hex!("b00a4c7f03959e01df2504fb73d2b238a8ab08b2000000000000000000000000")
        );
    }
}
