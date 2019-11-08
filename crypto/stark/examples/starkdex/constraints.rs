use zkp_primefield::FieldElement;
use zkp_elliptic_curve::Affine;
use zkp_stark::{RationalExpression, DensePolynomial};
use super::inputs::{Parameters};
use super::periodic_columns::{PEDERSEN_POINTS_X, PEDERSEN_POINTS_Y, ECDSA_POINTS_X, ECDSA_POINTS_Y};

fn get_coordinates(p: &Affine) -> (RationalExpression, RationalExpression) {
    match p {
        Affine::Zero => panic!(),
        Affine::Point { x, y } => (RationalExpression::Constant(x.clone()), RationalExpression::Constant(y.clone())),
    }
}

#[allow(dead_code)]
fn constraints(parameters: &Parameters) -> Vec<RationalExpression> {
    use RationalExpression::*;

    let trace_length = 10_000_000;
    let trace_generator = Constant(FieldElement::ONE);
    let path_length = 165;

    let ecdsa_points_x = Polynomial(DensePolynomial::new(&ECDSA_POINTS_X), Box::new(X.pow(20)));
    let ecdsa_points_y = Polynomial(DensePolynomial::new(&ECDSA_POINTS_Y), Box::new(X.pow(20)));

    let alpha = Constant(parameters.signature.alpha.clone());
    let beta = Constant(parameters.signature.beta.clone());
    let (signature_shift_x, signature_shift_y) = get_coordinates(&parameters.signature.shift_point);

    let merkle_hash_points_x = Polynomial(DensePolynomial::new(&PEDERSEN_POINTS_X), Box::new(X.pow(20)));
    let merkle_hash_points_y = Polynomial(DensePolynomial::new(&PEDERSEN_POINTS_Y), Box::new(X.pow(20)));
    let (hash_shift_x, hash_shift_y) = get_coordinates(&parameters.hash_shift_point);

    let hash_pool_points_x = Polynomial(DensePolynomial::new(&PEDERSEN_POINTS_X), Box::new(X.pow(20)));
    let hash_pool_points_y = Polynomial(DensePolynomial::new(&PEDERSEN_POINTS_Y), Box::new(X.pow(20)));

    let column0_row_expr0 = Trace(0, -3); // 0x2240
    let column0_row_expr2 = Trace(0, -2); // 0x2260
    let column4_row_expr1 = Trace(4, -1); // 0x2460
    let column4_row_expr0 = Trace(4, -3); // 0x2480

    let is_settlement = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(5)));
    let is_modification = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let boundary_base = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let boundary_key = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let boundary_token = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let boundary_amount0 = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let boundary_amount1 = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let boundary_vault_id = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));

    // ctx[mm_vault_shift] = 2**32;
    // ctx[mm_amount_shift] = 2**63; // 2**kRangeCheckBits
    // ctx[mm_trade_shift] = 2**32;
    let trade_shift = Constant(FieldElement::ONE);
    let amount_shift = Constant(FieldElement::ONE);
    let vault_shift = Constant(FieldElement::ONE);

    let initial_root = Constant(FieldElement::ONE);
    let final_root = Constant(FieldElement::ONE);

    let hash_pool_hash_ec_subset_sum_bit = Trace(8, 3) - (Trace(8, 7) + Trace(8, 7));
    let hash_pool_hash_ec_subset_sum_bit_neg = Constant(1.into()) - hash_pool_hash_ec_subset_sum_bit.clone();
    let state_transition_merkle_update_side_bit_extraction_bit_0 = Trace(6, 255) - (Trace(6, 767) + Trace(6, 767));
    let state_transition_merkle_update_prev_authentication_hashes_ec_subset_sum_bit = Trace(3, 0) - (Trace(3, 1) + Trace(3, 1));
    let state_transition_merkle_update_prev_authentication_hashes_ec_subset_sum_bit_neg = Constant(1.into()) - state_transition_merkle_update_prev_authentication_hashes_ec_subset_sum_bit.clone();
    let state_transition_merkle_update_side_bit_extraction_bit_1 = Trace(6, 767) - (Trace(6, 1279) + Trace(6, 1279));
    let state_transition_merkle_update_new_authentication_hashes_ec_subset_sum_bit = Trace(7, 0) - (Trace(7, 1) + Trace(7, 1));
    let state_transition_merkle_update_new_authentication_hashes_ec_subset_sum_bit_neg = Constant(1.into()) - state_transition_merkle_update_new_authentication_hashes_ec_subset_sum_bit.clone();
    let state_transition_merkle_update_prev_authentication_sibling_0 = state_transition_merkle_update_side_bit_extraction_bit_0.clone() * Trace(3, 0) + (Constant(1.into()) - state_transition_merkle_update_side_bit_extraction_bit_0.clone()) * Trace(3, 256);
    let state_transition_merkle_update_new_authentication_sibling_0 = state_transition_merkle_update_side_bit_extraction_bit_0.clone() * Trace(7, 0) + (Constant(1.into()) - state_transition_merkle_update_side_bit_extraction_bit_0.clone()) * Trace(7, 256);
    let state_transition_merkle_update_prev_authentication_leaf_0 = (Constant(1.into()) - state_transition_merkle_update_side_bit_extraction_bit_0.clone()) * Trace(3, 0) + state_transition_merkle_update_side_bit_extraction_bit_0.clone() * Trace(3, 256);
    let state_transition_merkle_update_new_authentication_leaf_0 = (Constant(1.into()) - state_transition_merkle_update_side_bit_extraction_bit_0.clone()) * Trace(7, 0) + state_transition_merkle_update_side_bit_extraction_bit_0.clone() * Trace(7, 256);
    let amounts_range_check_bit_0 = Trace(9, 4) - (Trace(9, 132) + Trace(9, 132));
    let sig_verify_doubling_key_x_squared = Trace(9, 0) * Trace(9, 0);
    let sig_verify_exponentiate_generator_bit = Trace(9, 20) - (Trace(9, 148) + Trace(9, 148));
    let sig_verify_exponentiate_generator_bit_neg = Constant(1.into()) - sig_verify_exponentiate_generator_bit.clone();
    let sig_verify_exponentiate_key_bit = Trace(9, 24) - (Trace(9, 88) + Trace(9, 88));
    let sig_verify_exponentiate_key_bit_neg = Constant(1.into()) - sig_verify_exponentiate_key_bit.clone();

    vec![
    (hash_pool_hash_ec_subset_sum_bit.clone() * (hash_pool_hash_ec_subset_sum_bit.clone() - 1.into())) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - 1.into()), // hash_pool/hash/ec_subset_sum/booleanity_test
    (Trace(8, 3)) / (X.pow(trace_length / 1024) - trace_generator.pow(251 * trace_length / 256)), // hash_pool/hash/ec_subset_sum/bit_extraction_end
    (Trace(8, 3)) / (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)), // hash_pool/hash/ec_subset_sum/zeros_tail
    (hash_pool_hash_ec_subset_sum_bit.clone() * (Trace(8, 2) - hash_pool_points_y) - Trace(8, 1) * (Trace(8, 0) - hash_pool_points_x.clone())) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - 1.into()), // hash_pool/hash/ec_subset_sum/add_points/slope
    (Trace(8, 1) * Trace(8, 1) - hash_pool_hash_ec_subset_sum_bit.clone() * (Trace(8, 0) + hash_pool_points_x + Trace(8, 4))) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - 1.into()), // hash_pool/hash/ec_subset_sum/add_points/x
    (hash_pool_hash_ec_subset_sum_bit.clone() * (Trace(8, 2) + Trace(8, 6)) - Trace(8, 1) * (Trace(8, 0) - Trace(8, 4))) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - 1.into()), // hash_pool/hash/ec_subset_sum/add_points/y
    (hash_pool_hash_ec_subset_sum_bit_neg.clone() * (Trace(8, 4) - Trace(8, 0))) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - 1.into()), // hash_pool/hash/ec_subset_sum/copy_point/x
    (hash_pool_hash_ec_subset_sum_bit_neg.clone() * (Trace(8, 6) - Trace(8, 2))) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - 1.into()), // hash_pool/hash/ec_subset_sum/copy_point/y
    (Trace(8, 1024) - Trace(8, 1020)) * (X.pow(trace_length / 2048) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 1024) - 1.into()), // hash_pool/hash/copy_point/x
    (Trace(8, 1026) - Trace(8, 1022)) * (X.pow(trace_length / 2048) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 1024) - 1.into()), // hash_pool/hash/copy_point/y
    (Trace(8, 0) - hash_shift_x.clone()) / (X.pow(trace_length / 2048) - 1.into()), // hash_pool/hash/init/x
    (Trace(8, 2) - hash_shift_y.clone()) / (X.pow(trace_length / 2048) - 1.into()), // hash_pool/hash/init/y
    (Trace(8, 2044) - Trace(8, 2051)) / (X.pow(trace_length / 4096) - 1.into()), // hash_pool/output_to_input
    (state_transition_merkle_update_side_bit_extraction_bit_0.clone() * state_transition_merkle_update_side_bit_extraction_bit_0.clone() - state_transition_merkle_update_side_bit_extraction_bit_0.clone()) * (X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) / (X.pow(trace_length / 512) - 1.into()), // state_transition/merkle_update/side_bit_extraction/bit
    (Trace(6, 255)) / (X.pow(trace_length / 16384) - trace_generator.pow(path_length * trace_length / 32)), // state_transition/merkle_update/side_bit_extraction/zero
    (state_transition_merkle_update_prev_authentication_hashes_ec_subset_sum_bit.clone() * (state_transition_merkle_update_prev_authentication_hashes_ec_subset_sum_bit.clone() - 1.into())) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/booleanity_test
    (Trace(3, 0)) / (X.pow(trace_length / 256) - trace_generator.pow(251 * trace_length / 256)), // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit_extraction_end
    (Trace(3, 0)) / (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)), // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/zeros_tail
    (state_transition_merkle_update_prev_authentication_hashes_ec_subset_sum_bit.clone() * (Trace(1, 0) - merkle_hash_points_y.clone()) - Trace(2, 0) * (Trace(0, 0) - merkle_hash_points_x.clone())) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/slope
    (Trace(2, 0) * Trace(2, 0) - state_transition_merkle_update_prev_authentication_hashes_ec_subset_sum_bit.clone() * (Trace(0, 0) + merkle_hash_points_x.clone() + Trace(0, 1))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/x
    (state_transition_merkle_update_prev_authentication_hashes_ec_subset_sum_bit.clone() * (Trace(1, 0) + Trace(1, 1)) - Trace(2, 0) * (Trace(0, 0) - Trace(0, 1))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/y
    (state_transition_merkle_update_prev_authentication_hashes_ec_subset_sum_bit_neg.clone() * (Trace(0, 1) - Trace(0, 0))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/copy_point/x
    (state_transition_merkle_update_prev_authentication_hashes_ec_subset_sum_bit_neg.clone() * (Trace(1, 1) - Trace(1, 0))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/copy_point/y
    (Trace(0, 256) - Trace(0, 255)) * (X.pow(trace_length / 512) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 256) - 1.into()), // state_transition/merkle_update/prev_authentication/hashes/copy_point/x
    (Trace(1, 256) - Trace(1, 255)) * (X.pow(trace_length / 512) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 256) - 1.into()), // state_transition/merkle_update/prev_authentication/hashes/copy_point/y
    (Trace(0, 0) - hash_shift_x.clone()) / (X.pow(trace_length / 512) - 1.into()), // state_transition/merkle_update/prev_authentication/hashes/init/x
    (Trace(1, 0) - hash_shift_y.clone()) / (X.pow(trace_length / 512) - 1.into()), // state_transition/merkle_update/prev_authentication/hashes/init/y
    ((Constant(1.into()) - state_transition_merkle_update_side_bit_extraction_bit_1.clone()) * (Trace(0, 511) - Trace(3, 512))) * ((X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) * (X.pow(trace_length / 16384) - trace_generator.pow(15 * trace_length / 16))) / (X.pow(trace_length / 512) - 1.into()), // state_transition/merkle_update/prev_authentication/copy_prev_to_left
    (state_transition_merkle_update_side_bit_extraction_bit_1.clone() * (Trace(0, 511) - Trace(3, 768))) * ((X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) * (X.pow(trace_length / 16384) - trace_generator.pow(15 * trace_length / 16))) / (X.pow(trace_length / 512) - 1.into()), // state_transition/merkle_update/prev_authentication/copy_prev_to_right
    (state_transition_merkle_update_new_authentication_hashes_ec_subset_sum_bit.clone() * (state_transition_merkle_update_new_authentication_hashes_ec_subset_sum_bit.clone() - 1.into())) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/booleanity_test
    (Trace(7, 0)) / (X.pow(trace_length / 256) - trace_generator.pow(251 * trace_length / 256)), // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit_extraction_end
    (Trace(7, 0)) / (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)), // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/zeros_tail
    (state_transition_merkle_update_new_authentication_hashes_ec_subset_sum_bit.clone() * (Trace(5, 0) - merkle_hash_points_y.clone()) - Trace(6, 0) * (Trace(4, 0) - merkle_hash_points_x.clone())) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/slope
    (Trace(6, 0) * Trace(6, 0) - state_transition_merkle_update_new_authentication_hashes_ec_subset_sum_bit.clone() * (Trace(4, 0) + merkle_hash_points_x.clone() + Trace(4, 1))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/x
    (state_transition_merkle_update_new_authentication_hashes_ec_subset_sum_bit.clone() * (Trace(5, 0) + Trace(5, 1)) - Trace(6, 0) * (Trace(4, 0) - Trace(4, 1))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/y
    (state_transition_merkle_update_new_authentication_hashes_ec_subset_sum_bit_neg.clone() * (Trace(4, 1) - Trace(4, 0))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/copy_point/x
    (state_transition_merkle_update_new_authentication_hashes_ec_subset_sum_bit_neg.clone() * (Trace(5, 1) - Trace(5, 0))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/copy_point/y
    (Trace(4, 256) - Trace(4, 255)) * (X.pow(trace_length / 512) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 256) - 1.into()), // state_transition/merkle_update/new_authentication/hashes/copy_point/x
    (Trace(5, 256) - Trace(5, 255)) * (X.pow(trace_length / 512) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 256) - 1.into()), // state_transition/merkle_update/new_authentication/hashes/copy_point/y
    (Trace(4, 0) - hash_shift_x.clone()) / (X.pow(trace_length / 512) - 1.into()), // state_transition/merkle_update/new_authentication/hashes/init/x
    (Trace(5, 0) - hash_shift_y.clone()) / (X.pow(trace_length / 512) - 1.into()), // state_transition/merkle_update/new_authentication/hashes/init/y
    ((Constant(1.into()) - state_transition_merkle_update_side_bit_extraction_bit_1.clone()) * (Trace(4, 511) - Trace(7, 512))) * ((X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) * (X.pow(trace_length / 16384) - trace_generator.pow(15 * trace_length / 16))) / (X.pow(trace_length / 512) - 1.into()), // state_transition/merkle_update/new_authentication/copy_prev_to_left
    (state_transition_merkle_update_side_bit_extraction_bit_1.clone() * (Trace(4, 511) - Trace(7, 768))) * ((X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) * (X.pow(trace_length / 16384) - trace_generator.pow(15 * trace_length / 16))) / (X.pow(trace_length / 512) - 1.into()), // state_transition/merkle_update/new_authentication/copy_prev_to_right
    (state_transition_merkle_update_prev_authentication_sibling_0 - state_transition_merkle_update_new_authentication_sibling_0) * (X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) / (X.pow(trace_length / 512) - 1.into()), // state_transition/merkle_update/same_siblings
    (state_transition_merkle_update_prev_authentication_leaf_0 - Trace(8, 4092)) / (X.pow(trace_length / 16384) - 1.into()), // state_transition/merkle_set_prev_leaf
    (state_transition_merkle_update_new_authentication_leaf_0 - Trace(8, 12284)) / (X.pow(trace_length / 16384) - 1.into()), // state_transition/merkle_set_new_leaf
    (is_modification.clone() * (Trace(9, 16376) * boundary_base.clone() - boundary_key.clone())) / (X.pow(trace_length / 65536) - 1.into()), // modification_boundary_key.clone()
    (is_modification.clone() * (Trace(9, 16360) * boundary_base.clone() - boundary_token.clone())) / (X.pow(trace_length / 65536) - 1.into()), // modification_boundary_token.clone()
    (is_modification.clone() * (Trace(8, 3075) * boundary_base.clone() - boundary_amount0.clone())) / (X.pow(trace_length / 65536) - 1.into()), // modification_boundary_amount0.clone()
    (is_modification.clone() * (Trace(8, 11267) * boundary_base.clone() - boundary_amount1.clone())) / (X.pow(trace_length / 65536) - 1.into()), // modification_boundary_amount1.clone()
    (is_modification.clone() * (Trace(6, 255) * boundary_base.clone() - boundary_vault_id.clone())) / (X.pow(trace_length / 65536) - 1.into()), // modification_boundary_vault_id.clone()
    (amounts_range_check_bit_0.clone() * amounts_range_check_bit_0.clone() - amounts_range_check_bit_0.clone()) * (X.pow(trace_length / 8192) - trace_generator.pow(63 * trace_length / 64)) / (X.pow(trace_length / 128) - 1.into()), // amounts_range_check/bit
    (Trace(9, 4)) / (X.pow(trace_length / 8192) - trace_generator.pow(63 * trace_length / 64)), // amounts_range_check/zero
    (is_settlement.clone() * (Trace(8, 3075) - Trace(8, 11267) - (Trace(8, 27651) - Trace(8, 19459)))) / (X.pow(trace_length / 65536) - 1.into()), // total_token_a_not_changed
    (is_settlement.clone() * (Trace(8, 35843) - Trace(8, 44035) - (Trace(8, 60419) - Trace(8, 52227)))) / (X.pow(trace_length / 65536) - 1.into()), // total_token_b_not_changed
    ((Trace(9, 4) - (Trace(8, 3075) - Trace(8, 11267))) * is_settlement.clone()) / (X.pow(trace_length / 65536) - 1.into()), // diff_a_range_check_input
    ((Trace(9, 32772) - (Trace(8, 35843) - Trace(8, 44035))) * is_settlement.clone()) / (X.pow(trace_length / 65536) - 1.into()), // diff_b_range_check_input
    (Trace(9, 8196) - Trace(8, 11267)) / (X.pow(trace_length / 16384) - 1.into()), // amounts_range_check_inputs
    (sig_verify_doubling_key_x_squared.clone() + sig_verify_doubling_key_x_squared.clone() + sig_verify_doubling_key_x_squared.clone() + alpha.clone() - (Trace(9, 32) + Trace(9, 32)) * Trace(9, 16)) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // sig_verify/doubling_key/slope
    (Trace(9, 16) * Trace(9, 16) - (Trace(9, 0) + Trace(9, 0) + Trace(9, 64))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // sig_verify/doubling_key/x
    (Trace(9, 32) + Trace(9, 96) - Trace(9, 16) * (Trace(9, 0) - Trace(9, 64))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // sig_verify/doubling_key/y
    (sig_verify_exponentiate_generator_bit.clone() * (sig_verify_exponentiate_generator_bit.clone() - 1.into())) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - 1.into()), // sig_verify/exponentiate_generator/booleanity_test
    (Trace(9, 20)) / (X.pow(trace_length / 32768) - trace_generator.pow(251 * trace_length / 256)), // sig_verify/exponentiate_generator/bit_extraction_end
    (Trace(9, 20)) / (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)), // sig_verify/exponentiate_generator/zeros_tail
    (sig_verify_exponentiate_generator_bit.clone() * (Trace(9, 36) - ecdsa_points_y) - Trace(9, 100) * (Trace(9, 68) - ecdsa_points_x.clone())) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - 1.into()), // sig_verify/exponentiate_generator/add_points/slope
    (Trace(9, 100) * Trace(9, 100) - sig_verify_exponentiate_generator_bit.clone() * (Trace(9, 68) + ecdsa_points_x.clone() + Trace(9, 196))) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - 1.into()), // sig_verify/exponentiate_generator/add_points/x
    (sig_verify_exponentiate_generator_bit.clone() * (Trace(9, 36) + Trace(9, 164)) - Trace(9, 100) * (Trace(9, 68) - Trace(9, 196))) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - 1.into()), // sig_verify/exponentiate_generator/add_points/y
    (Trace(9, 84) * (Trace(9, 68) - ecdsa_points_x.clone()) - 1.into()) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - 1.into()), // sig_verify/exponentiate_generator/add_points/x_diff_inv
    (sig_verify_exponentiate_generator_bit_neg.clone() * (Trace(9, 196) - Trace(9, 68))) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - 1.into()), // sig_verify/exponentiate_generator/copy_point/x
    (sig_verify_exponentiate_generator_bit_neg.clone() * (Trace(9, 164) - Trace(9, 36))) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - 1.into()), // sig_verify/exponentiate_generator/copy_point/y
    (sig_verify_exponentiate_key_bit.clone() * (sig_verify_exponentiate_key_bit.clone() - 1.into())) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // sig_verify/exponentiate_key/booleanity_test
    (Trace(9, 24)) / (X.pow(trace_length / 16384) - trace_generator.pow(251 * trace_length / 256)), // sig_verify/exponentiate_key/bit_extraction_end
    (Trace(9, 24)) / (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)), // sig_verify/exponentiate_key/zeros_tail
    (sig_verify_exponentiate_key_bit.clone() * (Trace(9, 8) - Trace(9, 32)) - Trace(9, 40) * (Trace(9, 48) - Trace(9, 0))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // sig_verify/exponentiate_key/add_points/slope
    (Trace(9, 40) * Trace(9, 40) - sig_verify_exponentiate_key_bit.clone() * (Trace(9, 48) + Trace(9, 0) + Trace(9, 112))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // sig_verify/exponentiate_key/add_points/x
    (sig_verify_exponentiate_key_bit.clone() * (Trace(9, 8) + Trace(9, 72)) - Trace(9, 40) * (Trace(9, 48) - Trace(9, 112))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // sig_verify/exponentiate_key/add_points/y
    (Trace(9, 56) * (Trace(9, 48) - Trace(9, 0)) - 1.into()) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // sig_verify/exponentiate_key/add_points/x_diff_inv
    (sig_verify_exponentiate_key_bit_neg.clone() * (Trace(9, 112) - Trace(9, 48))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // sig_verify/exponentiate_key/copy_point/x
    (sig_verify_exponentiate_key_bit_neg.clone() * (Trace(9, 72) - Trace(9, 8))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // sig_verify/exponentiate_key/copy_point/y
    (Trace(9, 68) - signature_shift_x.clone()) / (X.pow(trace_length / 32768) - 1.into()), // sig_verify/init_gen/x
    (Trace(9, 36) + signature_shift_y.clone()) / (X.pow(trace_length / 32768) - 1.into()), // sig_verify/init_gen/y
    (Trace(9, 48) - signature_shift_x.clone()) / (X.pow(trace_length / 16384) - 1.into()), // sig_verify/init_key/x
    (Trace(9, 8) - signature_shift_y.clone()) / (X.pow(trace_length / 16384) - 1.into()), // sig_verify/init_key/y
    (Trace(9, 32676) - Trace(9, 16328) - Trace(9, 32724) * (Trace(9, 32708) - Trace(9, 16368))) / (X.pow(trace_length / 32768) - 1.into()), // sig_verify/add_results/slope
    (Trace(9, 32724) * Trace(9, 32724) - (Trace(9, 32708) + Trace(9, 16368) + Trace(9, 16384))) / (X.pow(trace_length / 32768) - 1.into()), // sig_verify/add_results/x
    (Trace(9, 32676) + Trace(9, 16416) - Trace(9, 32724) * (Trace(9, 32708) - Trace(9, 16384))) / (X.pow(trace_length / 32768) - 1.into()), // sig_verify/add_results/y
    (Trace(9, 32740) * (Trace(9, 32708) - Trace(9, 16368)) - 1.into()) / (X.pow(trace_length / 32768) - 1.into()), // sig_verify/add_results/x_diff_inv
    (Trace(9, 32712) + signature_shift_y.clone() - Trace(8, 3069) * (Trace(9, 32752) - signature_shift_x.clone())) / (X.pow(trace_length / 32768) - 1.into()), // sig_verify/extract_r/slope
    (Trace(8, 3069) * Trace(8, 3069) - (Trace(9, 32752) + signature_shift_x.clone() + Trace(9, 24))) / (X.pow(trace_length / 32768) - 1.into()), // sig_verify/extract_r/x
    (Trace(8, 19453) * (Trace(9, 32752) - signature_shift_x.clone()) - 1.into()) / (X.pow(trace_length / 32768) - 1.into()), // sig_verify/extract_r/x_diff_inv
    (Trace(9, 20) * Trace(8, 11261) - 1.into()) / (X.pow(trace_length / 32768) - 1.into()), // sig_verify/z_nonzero
    (Trace(9, 24) * Trace(9, 16336) - 1.into()) / (X.pow(trace_length / 16384) - 1.into()), // sig_verify/r_and_w_nonzero
    (Trace(8, 27645) - Trace(9, 0) * Trace(9, 0)) / (X.pow(trace_length / 32768) - 1.into()), // sig_verify/q_on_curve/x_squared
    (Trace(9, 32) * Trace(9, 32) - (Trace(9, 0) * Trace(8, 27645) + alpha.clone() * Trace(9, 0) + beta)) / (X.pow(trace_length / 32768) - 1.into()), // sig_verify/q_on_curve/on_curve
    (is_settlement.clone() * (Trace(8, 7171) - (((Trace(6, 255) * vault_shift.clone() + Trace(6, 49407)) * amount_shift.clone() + Trace(9, 4)) * amount_shift.clone() + Trace(9, 32772)) * trade_shift)) / (X.pow(trace_length / 65536) - 1.into()), // maker_sig_input_packed
    (is_settlement.clone() * (Trace(8, 36867) - Trace(8, 8188))) / (X.pow(trace_length / 65536) - 1.into()), // taker_sig_input_maker_hash
    (is_settlement.clone() * (Trace(8, 37891) - Trace(6, 16639))) / (X.pow(trace_length / 65536) - 1.into()), // taker_sig_input_vault_a
    (is_settlement.clone() * (Trace(8, 39939) - Trace(6, 33023))) / (X.pow(trace_length / 65536) - 1.into()), // taker_sig_input_vault_b
    (is_settlement.clone() * (Trace(8, 8188) - Trace(9, 20))) / (X.pow(trace_length / 65536) - 1.into()), // copy_signature_input_maker
    (is_settlement.clone() * (Trace(8, 40956) - Trace(9, 32788))) / (X.pow(trace_length / 65536) - 1.into()), // copy_signature_input_taker
    (is_settlement.clone() * (Trace(9, 0) - Trace(9, 16376))) / (X.pow(trace_length / 65536) - 1.into()), // handle_empty_vault/consistency_key_change0
    (is_settlement.clone() * (Trace(8, 4099) - Trace(9, 16360))) / (X.pow(trace_length / 65536) - 1.into()), // handle_empty_vault/consistency_token_change0
    (is_settlement.clone() * (Trace(9, 0) - Trace(9, 65528))) / (X.pow(trace_length / 65536) - 1.into()), // handle_empty_vault/consistency_key_change3
    (is_settlement.clone() * (Trace(8, 5123) - Trace(9, 65512))) / (X.pow(trace_length / 65536) - 1.into()), // handle_empty_vault/consistency_token_change3
    (is_settlement.clone() * (Trace(9, 32768) - Trace(9, 32760))) / (X.pow(trace_length / 65536) - 1.into()), // handle_empty_vault/consistency_key_change1
    (is_settlement.clone() * (Trace(8, 4099) - Trace(9, 32744))) / (X.pow(trace_length / 65536) - 1.into()), // handle_empty_vault/consistency_token_change1
    (is_settlement.clone() * (Trace(9, 32768) - Trace(9, 49144))) / (X.pow(trace_length / 65536) - 1.into()), // handle_empty_vault/consistency_key_change2
    (is_settlement.clone() * (Trace(8, 5123) - Trace(9, 49128))) / (X.pow(trace_length / 65536) - 1.into()), // handle_empty_vault/consistency_token_change2
    (Trace(8, 1021) * (Constant(1.into()) - Trace(8, 1021))) / (X.pow(trace_length / 8192) - 1.into()), // handle_empty_vault/vault_empty/empty_vault_booleanity
    (Trace(8, 1021) * Trace(8, 3075)) / (X.pow(trace_length / 8192) - 1.into()), // handle_empty_vault/vault_empty/amount_zero_when_empty
    (Trace(8, 1021) * Trace(8, 5117)) / (X.pow(trace_length / 8192) - 1.into()), // handle_empty_vault/vault_empty/amount_inv_zero_when_empty
    (Trace(8, 3075) * Trace(8, 5117) - (Constant(1.into()) - Trace(8, 1021))) / (X.pow(trace_length / 8192) - 1.into()), // handle_empty_vault/vault_empty/empty_when_amount_zero
    ((Constant(1.into()) - Trace(8, 1021)) * Trace(9, 16376) - Trace(8, 3)) / (X.pow(trace_length / 16384) - 1.into()), // handle_empty_vault/consistency_key_stage0
    ((Constant(1.into()) - Trace(8, 1021)) * Trace(9, 16360) - Trace(8, 1027)) / (X.pow(trace_length / 16384) - 1.into()), // handle_empty_vault/consistency_token_stage0
    ((Constant(1.into()) - Trace(8, 9213)) * Trace(9, 16376) - Trace(8, 8195)) / (X.pow(trace_length / 16384) - 1.into()), // handle_empty_vault/consistency_key_stage1
    ((Constant(1.into()) - Trace(8, 9213)) * Trace(9, 16360) - Trace(8, 9219)) / (X.pow(trace_length / 16384) - 1.into()), // handle_empty_vault/consistency_token_stage1
    (column0_row_expr0 - initial_root) / (X - 1.into()), // initial_root
    (column4_row_expr1.clone() - final_root) / (X - trace_generator.pow(65536 * (trace_length / 65536 - 1))), // final_root
    (column4_row_expr0.clone() - column0_row_expr2) * (X - trace_generator.pow(65536 * (trace_length / 65536 - 1) + 49152)) / (X.pow(trace_length / 16384) - 1.into()), // copy_merkle_roots
    (is_modification.clone() * (column4_row_expr0 - column4_row_expr1)) / (X.pow(trace_length / 65536) - 1.into()), // copy_merkle_roots_modification
    ]
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::SignatureParameters;
    use crate::pedersen_points::SHIFT_POINT;
    // use zkp_elliptic_curve::Affine;
    // use zkp_primefield::geometric_series::root_series;
    // use zkp_stark::DensePolynomial;
    use std::{collections::BTreeSet};

    #[test]
    fn sanity_check() {
        let parameters = Parameters {
            signature: SignatureParameters {
                shift_point: SHIFT_POINT,
                alpha: FieldElement::ONE,
                beta: FieldElement::ZERO,
            },
            hash_shift_point: SHIFT_POINT,
            n_vaults: 30,
        };

        let constraints = constraints(&parameters);
        assert_eq!(constraints.len(), 120);

        let trace_arguments: Vec<_> = constraints.iter()
                    .map(RationalExpression::trace_arguments)
                    .fold(BTreeSet::new(), |x, y| &x | &y)
                    .into_iter()
                    .collect();
        assert_eq!(trace_arguments.len(), 124);
    }
}
