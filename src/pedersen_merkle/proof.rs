#[cfg(test)]
mod tests {
    use rayon::prelude::*;
    use std::iter;

    use super::*;
    use crate::fft::{fft, fft_cofactor, ifft};
    use crate::field::{FieldElement, MODULUS};
    use crate::merkle::{hash_node, Hashable};
    use crate::montgomery::R1;
    use crate::pedersen_merkle::input::{get_private_input, get_public_input};
    use crate::pedersen_merkle::trace_table::get_trace_table;
    use crate::polynomial::eval_poly;
    use crate::u256::U256;
    use hex_literal::*;

    fn pedersen_merkle_proof() {
        let public_input = get_public_input();
        let trace_table = get_trace_table(&public_input, &get_private_input());

        let mut columns: [Vec<FieldElement>; 8] = Default::default();
        for (i, value) in trace_table.iter().enumerate() {
            if i % 4 == 0 {
                columns[i % 8].push(FieldElement::from(R1) * value.clone());
            } else {
                columns[i % 8].push(value.clone());
            }
        }

        let trace_length: usize = public_input.path_length * 256;
        let trace_generator = FieldElement::GENERATOR
            .pow(MODULUS / U256::from(trace_length as u128))
            .unwrap();
        let trace_polynomials: Vec<_> = (0..8)
            .map(|i| ifft(trace_generator.clone(), columns[i].as_slice()))
            .collect();

        assert_eq!(columns[6][columns[6].len() - 1], public_input.root);
        assert_eq!(
            columns[6][columns[6].len() - 1],
            FieldElement::from_hex_str(
                "0x779aed4d3452b88d754ff4eed01b257e63384752782b7efde2e0a9e6eb03423"
            )
        );

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

        let beta: usize = 16;
        let evaluation_length = trace_length * beta;
        let evaluation_generator =
            FieldElement::root(U256::from(evaluation_length as u64)).unwrap();
        let evaluation_offset = FieldElement::GENERATOR;

        // let index = 3671035u64;
        // let evaluation_point = &evaluation_offset
        //     * evaluation_generator
        //         .pow(reverse(index, 25) as u128)
        //         .unwrap();
        // let expected_values = vec![
        //     FieldElement::from_hex_str(
        //         "0x191dd69283475ddd3b21e70a2f33ac1ddc57c94d94372c91b4dc165470cd16d",
        //     ),
        //     FieldElement::from_hex_str(
        //         "0x540b97a03b8932df6a5ad25f7e575cfa54024094ea4a8bbd3c491b81b83fe4b",
        //     ),
        //     FieldElement::from_hex_str(
        //         "0x77fc9484e5f4e5dff43420c0ed32ec8a082f530800f50e073f83b97f4f000b8",
        //     ),
        //     FieldElement::from_hex_str(
        //         "0x46bc71b42bd78c76e4669ccfa1fa85c4bd8112b10c78535b7c113782ae410f3",
        //     ),
        //     FieldElement::from_hex_str(
        //         "0x0acbed66102168104f8c9c8a536d11f0fd3d3865fa637fd8088fe5b8125b2f8",
        //     ),
        //     FieldElement::from_hex_str(
        //         "0x1f84d70300430f209e89bff935c1bd588b34207a010eb113a35639483e152a7",
        //     ),
        //     FieldElement::from_hex_str(
        //         "0x3101db85628661e0002ff9769e5ec8292173e6645bdba61925313da30a3989a",
        //     ),
        //     FieldElement::from_hex_str(
        //         "0x626d82c055ce4c31c9e61d32c64293bab8e0973b4d9e9b627a785c1e7a17d67",
        //     ),
        // ];
        // for (i, expected_value) in expected_values.iter().enumerate() {
        //     assert_eq!(
        //         eval_poly(evaluation_point.clone(), &trace_polynomials[i]),
        //         *expected_value
        //     );
        // }
        //
        // let partner_index: u64 = index - 1;
        // let partner_point = &evaluation_offset
        //     * evaluation_generator
        //         .pow(reverse(partner_index, 25) as u128)
        //         .unwrap();
        // let partner_row: Vec<_> = (0..8)
        //     .map(|i| eval_poly(partner_point.clone(), &trace_polynomials[i]).0)
        //     .collect();
        // assert_eq!(
        //     partner_row.as_slice().hash(),
        //     hex!("9e7fc484305af8dc7171beac83e61b009d2d6f91000000000000000000000000")
        // );
        //
        // let left_partner_index: u64 = index - 3;
        // let left_partner_point = &evaluation_offset
        //     * evaluation_generator
        //         .pow(reverse(left_partner_index, 25) as u128)
        //         .unwrap();
        // let left_partner_row: Vec<_> = (0..8)
        //     .map(|i| eval_poly(left_partner_point.clone(), &trace_polynomials[i]).0)
        //     .collect();
        //
        // let right_partner_index: u64 = index - 2;
        // let right_partner_point = &evaluation_offset
        //     * evaluation_generator
        //         .pow(reverse(right_partner_index, 25) as u128)
        //         .unwrap();
        // let right_partner_row: Vec<_> = (0..8)
        //     .map(|i| eval_poly(right_partner_point.clone(), &trace_polynomials[i]).0)
        //     .collect();
        //
        // let left = left_partner_row.as_slice().hash();
        // let right = right_partner_row.as_slice().hash();
        // assert_eq!(
        //     hash_node(&left, &right),
        //     hex!("a05377347d88d46b9d68b3c4082bfc7fab4d822a000000000000000000000000")
        // );

        // let low_degree_extensions: Vec<Vec<FieldElement>> = trace_polynomials
        //     .into_par_iter()
        //     .map(|trace_polynomial| {
        //         (0..beta as u64)
        //             .map(|j| {
        //                 let cofactor = &evaluation_offset * evaluation_generator.pow(j).unwrap();
        //                 fft_cofactor(trace_generator.clone(), &trace_polynomial, cofactor)
        //             })
        //             .flatten()
        //             .collect()
        //     })
        //     .collect();
    }
}
