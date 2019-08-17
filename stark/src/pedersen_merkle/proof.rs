#[cfg(test)]
mod tests {
    use crate::{
        merkle::make_tree,
        mmap_vec::MmapVec,
        pedersen_merkle::{
            constraints::get_pedersen_merkle_constraints,
            inputs::{starkware_private_input, STARKWARE_PUBLIC_INPUT},
            trace_table::get_trace_table,
        },
        proofs::{
            calculate_low_degree_extensions, get_constraint_polynomials, interpolate_trace_table,
            Merkleizable, ProofParams,
        },
    };
    use macros_decl::{field_element, hex, u256h};
    use primefield::FieldElement;
    use u256::U256;

    pub fn get_coefficients() -> Vec<FieldElement> {
        vec![
            field_element!("0636ad17759a0cc671e906ef94553c10f7a2c012d7a2aa599875506f874c136a"),
            field_element!("00ab929f48dee245d46548d9ce7b5c12809a489269702bede4a0f0beba6c96c3"),
            field_element!("032d059175506c780d44d30bf305b2e5cce87c2d10812aa4d19a4528a5906e97"),
            field_element!("062fc698139debf58aa475f18474829bce0ad224493570b723b254220774c0a4"),
            field_element!("07a316b3888038c223729c1ca14608dc3a536c62453f29facbb945faea4edc06"),
            field_element!("073ba8423c357d128709e1a1c45f5321483026f156fc58bc3f2f2fcd4e26112d"),
            field_element!("0215d0bcc49a30e0ca497c801a392300b3621d9c9be977c956d84a72db66ef50"),
            field_element!("03063ac609aed7c06323a4a46df08169cda8222d6c825c41495938acac23bd25"),
            field_element!("03b8f5b9dcb514cb0b72b96e507ee51ed5a90ce9887f9ba0ed132a78379f41bf"),
            field_element!("02cba94fa3a77dc4a6472998bb8c2d730f5bb538216172abec1feeaac28172f7"),
            field_element!("0329512d0cf95b0c90e3df8a6dbc965057738955b26d3ab7099fd2129a2733ad"),
            field_element!("0029b37fd38f7517cd35c29d1963f4c48bc53ca3ca6ae97d238107bdeb4587c0"),
            field_element!("05d12ac775d829842a492cb4b73edc1496349571d4b1cac0ca69626753b0c000"),
            field_element!("05d1a23dfb3b7a0d2def3dc025daa911876871c471f46ad7f445373a22b499d6"),
            field_element!("05442e604659a3c9f8fb27a9045f0298ff7864c310f1e332f1731741b417fdd3"),
            field_element!("07d9afbc5e50e96cb40ee87da8bf587782e682c1f6a3992d80baa06c3ba7869e"),
            field_element!("07e1ce86e3b58bae217e62f4f65a748109d19312cd9fffc21d076670360aacf9"),
            field_element!("035f9da854f2c57d45b02aea22d6b3a6032709a56e4fc97ec2091cd0dbd5914e"),
            field_element!("059a33bc0404c02913a7e3f86e649872772fa342eb5f012ad305eaa1118838ac"),
            field_element!("045d9748f52e5a9d978b691134cc96cdccc424ca2a680443ae7a55c08d7c4aa2"),
            field_element!("02d57682b21f33ff481a4acb4998ae61cd63af8b093aad8c1045daec53c6c187"),
            field_element!("054528769c7f4a197d9e4ee98cfb1fcd4693005abf6971f5b2d1094c35b6213d"),
            field_element!("036f9f941c351259092b93d06fd6fa04ead6f9c2bf689ba5dc493dc272b3e4e2"),
            field_element!("0622608ba33a72b440416448210531a4d01603e896c2eb0845031805ed9a5c74"),
            field_element!("01cbed1d58c1df62c5d6858493008de7e597431dc57350fb2c2943e2de1cc0c3"),
            field_element!("0781db13a07eec56a98fa4a1a7ff68003e5c16811926409de4b1f7ea2c624ead"),
            field_element!("021331547cc14840df44d41241a4da54be67df3f788f38dbd737c2bae6cd7838"),
            field_element!("07235529f0c22209c5f44c41c9d932b8c5744b63634567edb4d175cfdf25437f"),
            field_element!("04f99ffbba41cc2d8cdd9f13bbaf265e6f32deb4daac355f095c6f3c3a6762a2"),
            field_element!("042b86e961dd43e847d6278ba49870e0f04212b5ae38785cae336fba6eafcbe1"),
            field_element!("00f4a02801ac456e6ced57ea2814cb038881cb6de9487104fd2c76732485bdd8"),
            field_element!("06850c719229f42ea96a90dfaf75f248b45b9d896443adf29189e02c906fd27f"),
            field_element!("0116ee01cb9f6967ae360d3c38983ca38aa5c863e10c85ad77b04ad65a8adae7"),
            field_element!("0695eeb76a10a9c0398db1ebe391d2e25f6a80ba83855dc9a6b3ebe0698a4bcd"),
            field_element!("053d35cee3cf6e8b1f4406f8c9bc0f88d1e39facbc70eb19b7c1927b02934eaf"),
            field_element!("05c040858783b6a092ae756b1bd36a91e18bd92bdf4453b3580c535db22d12d9"),
            field_element!("06a2a83dcf1222a9972faa03aa45b5a03ea9995833c9dcef272f73a4dc6fb7d6"),
            field_element!("07537e90d5b2bab1c038fc6854267e7b2806d2f26c2fb7ee92bc65501903e6d2"),
            field_element!("050c83b136f235043250e31fdc262b8ff441686e8f11b29d3a7706b86095d128"),
            field_element!("00821f83891431a1cc871d9c4b74b212c5eb113acc1340088900205e7b8698b3"),
            field_element!("05897b09a49d1ae72f7845fb242db4a6c0f6f4aac9d63ab0f331f46332df4c82"),
            field_element!("00728f28f5309ddf5a3a9444bc2e97a084a9f4342f62a84da891ef0931a2147b"),
            field_element!("0381b768e7faa0361af12ae323ccb29f502d0ddc3964a90f4354ed5bb6ba34b4"),
            field_element!("05870d743173c27f92536909745a36ac31c6b5384e4d0127f8cf6a813e036e3b"),
            field_element!("0012ea1ebbd9e4ad0fe90a0444d90f8c8e4cab8650a5f0cfed6fce0dfbb604ce"),
            field_element!("0551212193e2ffe995afb9052c083eb6773b43dcc8df6e69e73591ff3ba411b5"),
            field_element!("04e0cc02bf5c6c4b572e455f76de37fcf38e35905d856ad6e086d4ed9bd1793c"),
            field_element!("0480de46109f40b539374cbc413e935be066a7296443cb8e4de05f654faadbd7"),
            field_element!("026a515d41b9f630302a52b80b60d6dfd08ff009e104570ba0537c8f5f8ec02d"),
            field_element!("01e3755bec6d69cb6ff4516b0cf43ee52466aafbe9ffe9a2f1296ef53421d7ed"),
            field_element!("03e97a0940ddd5c2ee158a97e6d29dc5129ec9c7a96e34a8237a464f6d51f6ab"),
            field_element!("018c45ab286ec38ef666ca02ba3484186270c23b54edc2bac749da3fe78ffc40"),
            field_element!("064e9cfd92cd6deb7cf8bd9929bdcc1b6161774432a12575338b829372bc9a8b"),
            field_element!("02224d4e3eee94168463684553d1a14d399bf81d3cab736b3bc58480f3832477"),
            field_element!("01c2bb2a80a57431bfab9636e98a6c73b24661a19077c2b56f3de44b0896b9f4"),
            field_element!("066b5653e399f0d37c44d7e05559098c96d8bec05824c4fb82f8474a8911df74"),
            field_element!("037f7c5048aa39d4a8b09861d91c7e7c8d560e7e6dd1da981febdb526b2305d0"),
            field_element!("01d7b36c4e979188ec71f7013ac4ff807aa77d379d6e8b9eee04ecfe8ceaa5b6"),
        ]
    }

    #[test]
    fn pedersen_merkle_proof() {
        let proof_parameters = ProofParams {
            blowup:                   16,
            pow_bits:                 12,
            queries:                  20,
            fri_layout:               vec![3, 2],
            constraints_degree_bound: 1,
        };

        let trace_table = get_trace_table(&STARKWARE_PUBLIC_INPUT, &starkware_private_input());
        let trace_polynomials = interpolate_trace_table(&trace_table);
        let extended_trace_table =
            calculate_low_degree_extensions(&trace_polynomials, proof_parameters.blowup);

        let extended_trace_table_tree = extended_trace_table.merkleize();
        assert_eq!(
            extended_trace_table_tree[1].as_bytes(),
            hex!("b00a4c7f03959e01df2504fb73d2b238a8ab08b2000000000000000000000000")
        );

        let constraints = get_pedersen_merkle_constraints(&STARKWARE_PUBLIC_INPUT);
        let constraint_polynomials =
            get_constraint_polynomials(&trace_polynomials, &constraints, &get_coefficients(), 2);

        let constraint_lde =
            calculate_low_degree_extensions(&constraint_polynomials, proof_parameters.blowup);

        let extended_constraint_table =
            calculate_low_degree_extensions(&constraint_polynomials, proof_parameters.blowup);
    }
}

// #[test]
// fn constraint_merkle_root_is_correct() {
//     let constraint_polynomial_on_extended_domain =
//         evaluate_constraint_polynomial_on_extended_domain(&
// get_coefficients());     let trace_length =
// constraint_polynomial_on_extended_domain[0].len();
//
//     let mut leaves: MmapVec<[FieldElement; 2]> =
// MmapVec::with_capacity(trace_length);     for i in 0..trace_length {
//         leaves.push([
//             constraint_polynomial_on_extended_domain[0][i].clone(),
//             constraint_polynomial_on_extended_domain[1][i].clone(),
//         ]);
//     }
//
//     let merkle_tree = make_tree(&leaves);
//
//     assert_eq!(
//         merkle_tree[1],
//         hex!("
// 2e821fe1f3062acdbd3a4bd0be2293f4264abc7b000000000000000000000000")     );
// }

// pub fn evaluate_constraint_polynomial_on_extended_domain(
//     constraint_coefficients: &[FieldElement],
// ) -> Vec<MmapVec<FieldElement>> {
//     let constraint_polynomial = get_constraint_polynomial(
//         &get_trace_polynomials(),
//         &get_pedersen_merkle_constraints(&get_public_input()),
//         constraint_coefficients,
//     );
//     println!("constraint polynomial!");
//     let even_polynomial = constraint_polynomial.even();
//     let odd_polynomial = constraint_polynomial.odd();
//     let trace_length = even_polynomial.len();
//
//     let polynomials = vec![even_polynomial, odd_polynomial];
//
//     let beta = 16usize;
//     let evaluation_length = trace_length * beta;
//     let evaluation_generator =
// FieldElement::root(U256::from(evaluation_length as u64)).unwrap();
//     let evaluation_offset = FieldElement::GENERATOR;
//
//     let mut constraint_polynomial_on_extended_domain =
//         vec![MmapVec::with_capacity(evaluation_length); 2];
//     for i in 0..beta {
//         let mut cosets: Vec<Vec<FieldElement>> =
// vec![Vec::with_capacity(trace_length); 2];         polynomials
//             .par_iter()
//             .map(|p| {
//                 let reverse_i = i.bit_reverse() >> (64 - 4);
//                 let cofactor =
//                     &evaluation_offset *
// evaluation_generator.pow(U256::from(reverse_i as u64));                 
// fft_cofactor_bit_reversed(&p.reverse_coefficients(), &cofactor)             
// })             .collect_into_vec(&mut cosets);
//         for (extended_trace_column, coset) in
// constraint_polynomial_on_extended_domain             .iter_mut()
//             .zip(cosets)
//         {
//             extended_trace_column.extend(&coset);
//         }
//         println!("{}", { i });
//     }
//     constraint_polynomial_on_extended_domain
// }
