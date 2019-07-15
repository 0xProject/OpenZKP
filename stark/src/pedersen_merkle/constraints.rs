use crate::{pedersen_merkle::input::get_public_input, polynomial::eval_poly};
use primefield::{invert_batch, FieldElement};
// use rayon::prelude::*;
use u256::U256;
// use u256::u256h;
use crate::pedersen_merkle::input::get_periodic_columns;
use ecc::Affine;
use itertools::izip;
use starkdex::SHIFT_POINT;

// pub fn eval_whole_loop(
//     LDEn: &[&[FieldElement]],
//     constraint_coefficients: &[FieldElement],
//     claim_index: usize,
//     claim_fib: &FieldElement,
// ) -> Vec<FieldElement> {
//     let eval_domain_size_usize = LDEn[0].len();
//     let eval_domain_size = eval_domain_size_usize as u64;
//     let beta = 2_u64.pow(4);
//     let trace_len = eval_domain_size / beta;
//
//     let omega = FieldElement::root(U256::from(trace_len * beta)).unwrap();
//     let g = omega.pow(U256::from(beta));
//     let gen = FieldElement::GENERATOR;
//
//     let mut CC = Vec::with_capacity(eval_domain_size_usize);
//     let g_trace = g.pow(U256::from(trace_len - 1));
//     let g_claim = g.pow(U256::from(claim_index as u64));
//     let x = gen.clone();
//     let x_trace = (&x).pow(U256::from(trace_len));
//     let x_1023 = (&x).pow(U256::from(trace_len - 1));
//     let omega_trace = (&omega).pow(U256::from(trace_len));
//     let omega_1023 = (&omega).pow(U256::from(trace_len - 1));
//
//     let x_omega_cycle = geometric_series(&x, &omega, eval_domain_size_usize);
//     let x_trace_cycle = geometric_series(&x_trace, &omega_trace,
// eval_domain_size_usize);     let x_1023_cycle = geometric_series(&x_1023,
// &omega_1023, eval_domain_size_usize);
//
//     let mut x_trace_sub_one: Vec<FieldElement> =
// Vec::with_capacity(eval_domain_size_usize);     let mut x_sub_one:
// Vec<FieldElement> = Vec::with_capacity(eval_domain_size_usize);     let mut
// x_g_claim_cycle: Vec<FieldElement> =
// Vec::with_capacity(eval_domain_size_usize);
//
//     x_omega_cycle
//         .par_iter()
//         .map(|i| (i - FieldElement::ONE, i - &g_claim))
//         .unzip_into_vecs(&mut x_sub_one, &mut x_g_claim_cycle);
//
//     x_trace_cycle
//         .par_iter()
//         .map(|i| i - FieldElement::ONE)
//         .collect_into_vec(&mut x_trace_sub_one);
//
//     let pool = vec![&x_trace_sub_one, &x_sub_one, &x_g_claim_cycle];
//
//     let mut held = Vec::with_capacity(3);
//     pool.par_iter()
//         .map(|i| invert_batch(i))
//         .collect_into_vec(&mut held);
//
//     x_g_claim_cycle = held.pop().unwrap();
//     x_sub_one = held.pop().unwrap();
//     x_trace_sub_one = held.pop().unwrap();
//
//     (0..eval_domain_size_usize)
//         .into_par_iter()
//         .map(|i| {
//             let j = ((i as u64) + beta) % eval_domain_size;
//
//             let P0 = LDEn[0][i as usize].clone();
//             let P1 = LDEn[1][i as usize].clone();
//             let P0n = LDEn[0][j as usize].clone();
//             let P1n = LDEn[1][j as usize].clone();
//
//             let A = x_trace_sub_one[i as usize].clone();
//             let C0 = (&P0n - &P1) * (&x_omega_cycle[i as usize] - &g_trace) *
// &A;             let C1 = (&P1n - &P0 - &P1) * (&x_omega_cycle[i as usize] -
// &g_trace) * &A;             let C2 = (&P0 - FieldElement::ONE) * &x_sub_one[i
// as usize];             let C3 = (&P0 - claim_fib) * &x_g_claim_cycle[i as
// usize];
//
//             let C0a = &C0 * &x_1023_cycle[i as usize];
//             let C1a = &C1 * &x_1023_cycle[i as usize];
//             let C2a = &C2 * &x_omega_cycle[i as usize];
//             let C3a = &C3 * &x_omega_cycle[i as usize];
//
//             let mut r = FieldElement::ZERO;
//             r += &constraint_coefficients[0] * C0;
//             r += &constraint_coefficients[1] * C0a;
//             r += &constraint_coefficients[2] * C1;
//             r += &constraint_coefficients[3] * C1a;
//             r += &constraint_coefficients[4] * C2;
//             r += &constraint_coefficients[5] * C2a;
//             r += &constraint_coefficients[6] * C3;
//             r += &constraint_coefficients[7] * C3a;
//
//             r
//         })
//         .collect_into_vec(&mut CC);
//     CC
// }

struct Row {
    left:  Subrow,
    right: Subrow,
}

struct Subrow {
    source: FieldElement,
    slope:  FieldElement,
    x:      FieldElement,
    y:      FieldElement,
}

pub fn eval_c_direct(
    x: &FieldElement,
    polynomials: &[&[FieldElement]],
    _claim_index: usize,
    _claim: FieldElement,
    coefficients: &[FieldElement],
) -> FieldElement {
    let public_input = get_public_input();
    let path_length = U256::from(public_input.path_length as u64);
    let trace_length = U256::from(256u64) * &path_length;
    let beta = 16u64;
    let evaluation_length = U256::from(beta) * &trace_length;

    let trace_generator = FieldElement::root(trace_length.clone()).unwrap();
    let evaluation_generator = FieldElement::root(evaluation_length.clone()).unwrap();

    let numerators = vec![
        x - trace_generator.pow(&trace_length - U256::ONE),
        x.pow(path_length.clone())
            - trace_generator.pow((&trace_length - U256::ONE) * &path_length),
        FieldElement::ONE,
    ];
    let denominators = invert_batch(&[
        x - FieldElement::ONE,
        x - trace_generator.pow(&trace_length - U256::from(1u64)),
        x.pow(path_length.clone())
            - trace_generator.pow(&path_length * (&trace_length - U256::ONE)),
        x.pow(path_length.clone()) - FieldElement::ONE,
        x.pow(trace_length.clone()) - FieldElement::ONE,
        x.pow(path_length.clone()) - trace_generator.pow(U256::from(252u64) * &path_length),
        FieldElement::ONE,
    ]);

    let mut this_row: Vec<FieldElement> = Vec::with_capacity(8);
    for polynomial in polynomials {
        this_row.push(eval_poly(x.clone(), polynomial));
    }
    let mut next_row: Vec<FieldElement> = Vec::with_capacity(8);
    for polynomial in polynomials {
        next_row.push(eval_poly(x.clone() * &evaluation_generator, polynomial));
    }

    let this = Row {
        left:  Subrow {
            source: this_row[0].clone(),
            slope:  this_row[1].clone(),
            x:      this_row[2].clone(),
            y:      this_row[2].clone(),
        },
        right: Subrow {
            source: this_row[4].clone(),
            slope:  this_row[5].clone(),
            x:      this_row[6].clone(),
            y:      this_row[7].clone(),
        },
    };

    let next = Row {
        left:  Subrow {
            source: next_row[0].clone(),
            slope:  next_row[1].clone(),
            x:      next_row[2].clone(),
            y:      next_row[2].clone(),
        },
        right: Subrow {
            source: next_row[4].clone(),
            slope:  next_row[5].clone(),
            x:      next_row[6].clone(),
            y:      next_row[7].clone(),
        },
    };
    let left_bit = &this.left.source - next.left.source.double();
    let right_bit = &this.right.source - next.right.source.double();

    let (shift_point_x, shift_point_y) = match SHIFT_POINT {
        Affine::Zero => panic!(),
        Affine::Point { x, y } => (x, y),
    };

    let periodic_columns = get_periodic_columns();
    let q_x_left = eval_poly(x.clone(), &periodic_columns.left_x_coefficients);
    let q_y_left = eval_poly(x.clone(), &periodic_columns.left_y_coefficients);
    let q_x_right = eval_poly(x.clone(), &periodic_columns.right_x_coefficients);
    let q_y_right = eval_poly(x.clone(), &periodic_columns.right_y_coefficients);

    let constraints = vec![
        this.left.source.clone(),
        this.left.slope.clone(),
        this.left.x.clone(),
        this.left.y.clone(),
        this.right.source.clone(),
        this.right.slope.clone(),
        this.right.x.clone(),
        this.right.y.clone(),
        (&public_input.leaf - &this.left.source) * (&public_input.leaf - &this.right.source),
        &public_input.root - &this.right.x,
        (&this.right.x - &next.left.source) * (&this.right.x - &next.right.source),
        &this.right.x - shift_point_x,
        &this.right.y - shift_point_y,
        &left_bit * (&left_bit - FieldElement::ONE),
        &left_bit * (&this.right.y - &q_y_left) - &next.left.slope * (&this.right.x - &q_x_left),
        next.left.slope.square() - &left_bit * (&this.right.x + &q_x_left + &next.left.x),
        &left_bit * (&this.right.y + &next.left.y)
            - &next.left.slope * (&this.right.x - &next.left.x),
        (FieldElement::ONE - &left_bit) * (&this.right.x - &next.left.x),
        (FieldElement::ONE - &left_bit) * (&this.right.y - &next.left.y),
        this.left.source.clone(),
        this.left.source.clone(),
        &right_bit * (&right_bit - FieldElement::ONE),
        &right_bit * (&next.left.y - &q_y_right) - &next.right.slope * (&next.left.x - &q_x_right),
        next.right.slope.square() - &right_bit * (&next.left.x + &q_x_right + &next.right.x),
        &right_bit * (&next.left.y + &next.right.y)
            - &next.right.slope * (&next.left.x - &next.right.x),
        (FieldElement::ONE - &right_bit) * (&next.left.x - &next.right.x),
        (FieldElement::ONE - &right_bit) * (&next.left.y - &next.right.y),
        this.right.source.clone(),
        this.right.source.clone(),
    ];

    let degree_adjustment = |constraint_degree: U256,
                             numerator_degree: U256,
                             denominator_degree: U256|
     -> U256 {
        trace_length.clone() + denominator_degree - U256::ONE - constraint_degree - numerator_degree
    };

    let adjustments = vec![
        x.pow(degree_adjustment(
            &trace_length - U256::ONE,
            U256::ZERO,
            U256::ZERO,
        )),
        x.pow(degree_adjustment(
            2u64 * (&trace_length - U256::ONE),
            U256::ZERO,
            U256::ONE,
        )),
        x.pow(degree_adjustment(
            &trace_length - U256::ONE,
            U256::ZERO,
            U256::ONE,
        )),
        x.pow(degree_adjustment(
            2u64 * (&trace_length - U256::ONE),
            U256::ONE,
            path_length.clone(),
        )),
        x.pow(degree_adjustment(
            &trace_length - U256::ONE,
            U256::ZERO,
            path_length.clone(),
        )),
        x.pow(degree_adjustment(
            2u64 * (&trace_length - U256::ONE),
            path_length.clone(),
            trace_length.clone(),
        )),
    ];

    // There are 58 coefficients, so each of these should be length 29.
    let numerator_indices = vec![
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 2, 2,
    ];
    let denominator_indices = vec![
        6, 6, 6, 6, 6, 6, 6, 6, 0, 1, 2, 3, 3, 4, 4, 4, 4, 4, 4, 5, 2, 4, 4, 4, 4, 4, 4, 5, 2,
    ];
    let adjustment_indices = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 4, 5, 5, 5, 5, 5, 5, 4, 4, 5, 5, 5, 5, 5, 5, 4, 4,
    ];

    let mut result = FieldElement::ZERO;
    for (i, (numerator_index, denominator_index, adjustment_index)) in
        izip!(numerator_indices, denominator_indices, adjustment_indices).enumerate()
    {
        let value =
            &constraints[i] * &numerators[numerator_index] * &denominators[denominator_index];
        result += value
            * (&coefficients[2 * i] + &coefficients[2 * i + 1] * &adjustments[adjustment_index]);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use hex_literal::*;
    use u256::u256h;

    #[test]
    fn trace_is_correct() {
        let _coefficients = vec![
            FieldElement::from(u256h!(
                "0636ad17759a0cc671e906ef94553c10f7a2c012d7a2aa599875506f874c136a"
            )),
            FieldElement::from(u256h!(
                "00ab929f48dee245d46548d9ce7b5c12809a489269702bede4a0f0beba6c96c3"
            )),
            FieldElement::from(u256h!(
                "032d059175506c780d44d30bf305b2e5cce87c2d10812aa4d19a4528a5906e97"
            )),
            FieldElement::from(u256h!(
                "062fc698139debf58aa475f18474829bce0ad224493570b723b254220774c0a4"
            )),
            FieldElement::from(u256h!(
                "07a316b3888038c223729c1ca14608dc3a536c62453f29facbb945faea4edc06"
            )),
            FieldElement::from(u256h!(
                "073ba8423c357d128709e1a1c45f5321483026f156fc58bc3f2f2fcd4e26112d"
            )),
            FieldElement::from(u256h!(
                "0215d0bcc49a30e0ca497c801a392300b3621d9c9be977c956d84a72db66ef50"
            )),
            FieldElement::from(u256h!(
                "03063ac609aed7c06323a4a46df08169cda8222d6c825c41495938acac23bd25"
            )),
            FieldElement::from(u256h!(
                "03b8f5b9dcb514cb0b72b96e507ee51ed5a90ce9887f9ba0ed132a78379f41bf"
            )),
            FieldElement::from(u256h!(
                "02cba94fa3a77dc4a6472998bb8c2d730f5bb538216172abec1feeaac28172f7"
            )),
            FieldElement::from(u256h!(
                "0329512d0cf95b0c90e3df8a6dbc965057738955b26d3ab7099fd2129a2733ad"
            )),
            FieldElement::from(u256h!(
                "0029b37fd38f7517cd35c29d1963f4c48bc53ca3ca6ae97d238107bdeb4587c0"
            )),
            FieldElement::from(u256h!(
                "05d12ac775d829842a492cb4b73edc1496349571d4b1cac0ca69626753b0c000"
            )),
            FieldElement::from(u256h!(
                "05d1a23dfb3b7a0d2def3dc025daa911876871c471f46ad7f445373a22b499d6"
            )),
            FieldElement::from(u256h!(
                "05442e604659a3c9f8fb27a9045f0298ff7864c310f1e332f1731741b417fdd3"
            )),
            FieldElement::from(u256h!(
                "07d9afbc5e50e96cb40ee87da8bf587782e682c1f6a3992d80baa06c3ba7869e"
            )),
            FieldElement::from(u256h!(
                "07e1ce86e3b58bae217e62f4f65a748109d19312cd9fffc21d076670360aacf9"
            )),
            FieldElement::from(u256h!(
                "035f9da854f2c57d45b02aea22d6b3a6032709a56e4fc97ec2091cd0dbd5914e"
            )),
            FieldElement::from(u256h!(
                "059a33bc0404c02913a7e3f86e649872772fa342eb5f012ad305eaa1118838ac"
            )),
            FieldElement::from(u256h!(
                "045d9748f52e5a9d978b691134cc96cdccc424ca2a680443ae7a55c08d7c4aa2"
            )),
            FieldElement::from(u256h!(
                "02d57682b21f33ff481a4acb4998ae61cd63af8b093aad8c1045daec53c6c187"
            )),
            FieldElement::from(u256h!(
                "054528769c7f4a197d9e4ee98cfb1fcd4693005abf6971f5b2d1094c35b6213d"
            )),
            FieldElement::from(u256h!(
                "036f9f941c351259092b93d06fd6fa04ead6f9c2bf689ba5dc493dc272b3e4e2"
            )),
            FieldElement::from(u256h!(
                "0622608ba33a72b440416448210531a4d01603e896c2eb0845031805ed9a5c74"
            )),
            FieldElement::from(u256h!(
                "01cbed1d58c1df62c5d6858493008de7e597431dc57350fb2c2943e2de1cc0c3"
            )),
            FieldElement::from(u256h!(
                "0781db13a07eec56a98fa4a1a7ff68003e5c16811926409de4b1f7ea2c624ead"
            )),
            FieldElement::from(u256h!(
                "021331547cc14840df44d41241a4da54be67df3f788f38dbd737c2bae6cd7838"
            )),
            FieldElement::from(u256h!(
                "07235529f0c22209c5f44c41c9d932b8c5744b63634567edb4d175cfdf25437f"
            )),
            FieldElement::from(u256h!(
                "04f99ffbba41cc2d8cdd9f13bbaf265e6f32deb4daac355f095c6f3c3a6762a2"
            )),
            FieldElement::from(u256h!(
                "042b86e961dd43e847d6278ba49870e0f04212b5ae38785cae336fba6eafcbe1"
            )),
            FieldElement::from(u256h!(
                "00f4a02801ac456e6ced57ea2814cb038881cb6de9487104fd2c76732485bdd8"
            )),
            FieldElement::from(u256h!(
                "06850c719229f42ea96a90dfaf75f248b45b9d896443adf29189e02c906fd27f"
            )),
            FieldElement::from(u256h!(
                "0116ee01cb9f6967ae360d3c38983ca38aa5c863e10c85ad77b04ad65a8adae7"
            )),
            FieldElement::from(u256h!(
                "0695eeb76a10a9c0398db1ebe391d2e25f6a80ba83855dc9a6b3ebe0698a4bcd"
            )),
            FieldElement::from(u256h!(
                "053d35cee3cf6e8b1f4406f8c9bc0f88d1e39facbc70eb19b7c1927b02934eaf"
            )),
            FieldElement::from(u256h!(
                "05c040858783b6a092ae756b1bd36a91e18bd92bdf4453b3580c535db22d12d9"
            )),
            FieldElement::from(u256h!(
                "06a2a83dcf1222a9972faa03aa45b5a03ea9995833c9dcef272f73a4dc6fb7d6"
            )),
            FieldElement::from(u256h!(
                "07537e90d5b2bab1c038fc6854267e7b2806d2f26c2fb7ee92bc65501903e6d2"
            )),
            FieldElement::from(u256h!(
                "050c83b136f235043250e31fdc262b8ff441686e8f11b29d3a7706b86095d128"
            )),
            FieldElement::from(u256h!(
                "00821f83891431a1cc871d9c4b74b212c5eb113acc1340088900205e7b8698b3"
            )),
            FieldElement::from(u256h!(
                "05897b09a49d1ae72f7845fb242db4a6c0f6f4aac9d63ab0f331f46332df4c82"
            )),
            FieldElement::from(u256h!(
                "00728f28f5309ddf5a3a9444bc2e97a084a9f4342f62a84da891ef0931a2147b"
            )),
            FieldElement::from(u256h!(
                "0381b768e7faa0361af12ae323ccb29f502d0ddc3964a90f4354ed5bb6ba34b4"
            )),
            FieldElement::from(u256h!(
                "05870d743173c27f92536909745a36ac31c6b5384e4d0127f8cf6a813e036e3b"
            )),
            FieldElement::from(u256h!(
                "0012ea1ebbd9e4ad0fe90a0444d90f8c8e4cab8650a5f0cfed6fce0dfbb604ce"
            )),
            FieldElement::from(u256h!(
                "0551212193e2ffe995afb9052c083eb6773b43dcc8df6e69e73591ff3ba411b5"
            )),
            FieldElement::from(u256h!(
                "04e0cc02bf5c6c4b572e455f76de37fcf38e35905d856ad6e086d4ed9bd1793c"
            )),
            FieldElement::from(u256h!(
                "0480de46109f40b539374cbc413e935be066a7296443cb8e4de05f654faadbd7"
            )),
            FieldElement::from(u256h!(
                "026a515d41b9f630302a52b80b60d6dfd08ff009e104570ba0537c8f5f8ec02d"
            )),
            FieldElement::from(u256h!(
                "01e3755bec6d69cb6ff4516b0cf43ee52466aafbe9ffe9a2f1296ef53421d7ed"
            )),
            FieldElement::from(u256h!(
                "03e97a0940ddd5c2ee158a97e6d29dc5129ec9c7a96e34a8237a464f6d51f6ab"
            )),
            FieldElement::from(u256h!(
                "018c45ab286ec38ef666ca02ba3484186270c23b54edc2bac749da3fe78ffc40"
            )),
            FieldElement::from(u256h!(
                "064e9cfd92cd6deb7cf8bd9929bdcc1b6161774432a12575338b829372bc9a8b"
            )),
            FieldElement::from(u256h!(
                "02224d4e3eee94168463684553d1a14d399bf81d3cab736b3bc58480f3832477"
            )),
            FieldElement::from(u256h!(
                "01c2bb2a80a57431bfab9636e98a6c73b24661a19077c2b56f3de44b0896b9f4"
            )),
            FieldElement::from(u256h!(
                "066b5653e399f0d37c44d7e05559098c96d8bec05824c4fb82f8474a8911df74"
            )),
            FieldElement::from(u256h!(
                "037f7c5048aa39d4a8b09861d91c7e7c8d560e7e6dd1da981febdb526b2305d0"
            )),
            FieldElement::from(u256h!(
                "01d7b36c4e979188ec71f7013ac4ff807aa77d379d6e8b9eee04ecfe8ceaa5b6"
            )),
        ];

        // assert_eq!(eval_c_direct(FieldElement::ONE), FieldElement::ONE);
    }

}
