// use env_logger;
// use log::info;
// use std::{env, time::Instant};
// use zkp_macros_decl::field_element;
use zkp_primefield::FieldElement;
use zkp_stark::{RationalExpression, DensePolynomial};

struct SignatureConfig {
    pub shift_point: Point,
    pub alpha: RationalExpression,
    pub beta: RationalExpression,
}

struct Point {
    pub x:  RationalExpression,
    pub y:  RationalExpression,
}

fn constraints() -> Vec<RationalExpression> {
    use RationalExpression::*;

    let trace_length = 10;
    let path_length = 256;
    let trace_generator = Constant(FieldElement::ZERO);

    let column4_row_expr0 = Trace(4, 10);
    let column0_row_expr2 = Trace(4, 10);
    let column4_row_expr1 = Trace(4, 1);
    let column0_row_expr0 = Trace(4, 1);


    let is_settlement = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(5)));
    let is_modification = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let boundary_base = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let boundary_key = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let boundary_token = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let boundary_amount0 = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let boundary_amount1 = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let boundary_vault_id = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));

    let ecdsa_points__x = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let ecdsa_points__y = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let merkle_hash_points__x = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let merkle_hash_points__y = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let hash_pool_points__x = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let hash_pool_points__y = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));

    let trade_shift = Constant(FieldElement::ONE);
    let amount_shift = Constant(FieldElement::ONE);
    let vault_shift = Constant(FieldElement::ONE);

    let initial_root = Constant(FieldElement::ONE);
    let final_root = Constant(FieldElement::ONE);

    let sig_config = SignatureConfig {
        shift_point: Point {
            x: Constant(FieldElement::ONE),
            y: Constant(FieldElement::ONE),
        },
        alpha: Constant(FieldElement::ONE),
        beta: Constant(FieldElement::ONE),
    };

    let shift_point = Point {
        x: Constant(FieldElement::ONE),
        y: Constant(FieldElement::ONE),
    };

    let hash_pool__hash__ec_subset_sum__bit = Trace(8, 3) - (Trace(8, 7) + Trace(8, 7));
    let hash_pool__hash__ec_subset_sum__bit_neg = Constant(1.into()) - hash_pool__hash__ec_subset_sum__bit;
    let state_transition__merkle_update__side_bit_extraction__bit_0 = Trace(6, 5) - (Trace(6, 7) + Trace(6, 7));
    let state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit = Trace(3, 0) - (Trace(3, 1) + Trace(3, 1));
    let state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit_neg = Constant(1.into()) - state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit.clone();
    let state_transition__merkle_update__side_bit_extraction__bit_1 = Trace(6, 7) - (Trace(6, 9) + Trace(6, 9));
    let state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit = Trace(7, 0) - (Trace(7, 1) + Trace(7, 1));
    let state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit_neg = Constant(1.into()) - state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit.clone();
    let state_transition__merkle_update__prev_authentication__sibling_0 = state_transition__merkle_update__side_bit_extraction__bit_0 * Trace(3, 0) + (Constant(1.into()) - state_transition__merkle_update__side_bit_extraction__bit_0) * Trace(3, 6);
    let state_transition__merkle_update__new_authentication__sibling_0 = state_transition__merkle_update__side_bit_extraction__bit_0 * Trace(7, 0) + (Constant(1.into()) - state_transition__merkle_update__side_bit_extraction__bit_0) * Trace(7, 6);
    let state_transition__merkle_update__prev_authentication__leaf_0 = (Constant(1.into()) - state_transition__merkle_update__side_bit_extraction__bit_0) * Trace(3, 0) + state_transition__merkle_update__side_bit_extraction__bit_0 * Trace(3, 6);
    let state_transition__merkle_update__new_authentication__leaf_0 = (Constant(1.into()) - state_transition__merkle_update__side_bit_extraction__bit_0) * Trace(7, 0) + state_transition__merkle_update__side_bit_extraction__bit_0 * Trace(7, 6);
    let amounts_range_check__bit_0 = Trace(9, 4) - (Trace(9, 2) + Trace(9, 2));
    let sig_verify__doubling_key__x_squared = Trace(9, 0) * Trace(9, 0);
    let sig_verify__exponentiate_generator__bit = Trace(9, 0) - (Trace(9, 8) + Trace(9, 8));
    let sig_verify__exponentiate_generator__bit_neg = Constant(1.into()) - sig_verify__exponentiate_generator__bit.clone();
    let sig_verify__exponentiate_key__bit = Trace(9, 4) - (Trace(9, 8) + Trace(9, 8));
    let sig_verify__exponentiate_key__bit_neg = Constant(1.into()) - sig_verify__exponentiate_key__bit.clone();
    vec![
    (hash_pool__hash__ec_subset_sum__bit.clone() * (hash_pool__hash__ec_subset_sum__bit.clone() - Constant(1.into()))) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - Constant(1.into())), // hash_pool/hash/ec_subset_sum/booleanity_test
    (Trace(8, 3)) / (X.pow(trace_length / 1024) - trace_generator.pow(251 * trace_length / 256)), // hash_pool/hash/ec_subset_sum/bit_extraction_end
    (Trace(8, 3)) / (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)), // hash_pool/hash/ec_subset_sum/zeros_tail
    (hash_pool__hash__ec_subset_sum__bit.clone() * (Trace(8, 2) - hash_pool_points__y) - Trace(8, 1) * (Trace(8, 0) - hash_pool_points__x)) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - Constant(1.into())), // hash_pool/hash/ec_subset_sum/add_points/slope
    (Trace(8, 1) * Trace(8, 1) - hash_pool__hash__ec_subset_sum__bit.clone() * (Trace(8, 0) + hash_pool_points__x + Trace(8, 4))) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - Constant(1.into())), // hash_pool/hash/ec_subset_sum/add_points/x
    (hash_pool__hash__ec_subset_sum__bit.clone() * (Trace(8, 2) + Trace(8, 6)) - Trace(8, 1) * (Trace(8, 0) - Trace(8, 4))) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - Constant(1.into())), // hash_pool/hash/ec_subset_sum/add_points/y
    (hash_pool__hash__ec_subset_sum__bit_neg.clone() * (Trace(8, 4) - Trace(8, 0))) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - Constant(1.into())), // hash_pool/hash/ec_subset_sum/copy_point/x
    (hash_pool__hash__ec_subset_sum__bit_neg.clone() * (Trace(8, 6) - Trace(8, 2))) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - Constant(1.into())), // hash_pool/hash/ec_subset_sum/copy_point/y
    (Trace(8, 4) - Trace(8, 0)) * (X.pow(trace_length / 2048) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 1024) - Constant(1.into())), // hash_pool/hash/copy_point/x
    (Trace(8, 6) - Trace(8, 2)) * (X.pow(trace_length / 2048) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 1024) - Constant(1.into())), // hash_pool/hash/copy_point/y
    (Trace(8, 0) - shift_point.x.clone()) / (X.pow(trace_length / 2048) - Constant(1.into())), // hash_pool/hash/init/x
    (Trace(8, 2) - shift_point.y.clone()) / (X.pow(trace_length / 2048) - Constant(1.into())), // hash_pool/hash/init/y
    (Trace(8, 4) - Trace(8, 1)) / (X.pow(trace_length / 4096) - Constant(1.into())), // hash_pool/output_to_input
    (state_transition__merkle_update__side_bit_extraction__bit_0 * state_transition__merkle_update__side_bit_extraction__bit_0 - state_transition__merkle_update__side_bit_extraction__bit_0) * (X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) / (X.pow(trace_length / 512) - Constant(1.into())), // state_transition/merkle_update/side_bit_extraction/bit
    (Trace(6, 5)) / (X.pow(trace_length / 16384) - trace_generator.pow(path_length * trace_length / 32)), // state_transition/merkle_update/side_bit_extraction/zero
    (state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit.clone() * (state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit.clone() - Constant(1.into()))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - Constant(1.into())), // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/booleanity_test
    (Trace(3, 0)) / (X.pow(trace_length / 256) - trace_generator.pow(251 * trace_length / 256)), // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit_extraction_end
    (Trace(3, 0)) / (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)), // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/zeros_tail
    (state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit.clone() * (Trace(1, 0) - merkle_hash_points__y.clone()) - Trace(2, 0) * (Trace(0, 0) - merkle_hash_points__x.clone())) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - Constant(1.into())), // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/slope
    (Trace(2, 0) * Trace(2, 0) - state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit.clone() * (Trace(0, 0) + merkle_hash_points__x.clone() + Trace(0, 1))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - Constant(1.into())), // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/x
    (state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit.clone() * (Trace(1, 0) + Trace(1, 1)) - Trace(2, 0) * (Trace(0, 0) - Trace(0, 1))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - Constant(1.into())), // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/y
    (state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit_neg.clone().clone() * (Trace(0, 1) - Trace(0, 0))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - Constant(1.into())), // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/copy_point/x
    (state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit_neg.clone().clone() * (Trace(1, 1) - Trace(1, 0))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - Constant(1.into())), // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/copy_point/y
    (Trace(0, 6) - Trace(0, 5)) * (X.pow(trace_length / 512) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 256) - Constant(1.into())), // state_transition/merkle_update/prev_authentication/hashes/copy_point/x
    (Trace(1, 6) - Trace(1, 5)) * (X.pow(trace_length / 512) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 256) - Constant(1.into())), // state_transition/merkle_update/prev_authentication/hashes/copy_point/y
    (Trace(0, 0) - shift_point.x.clone()) / (X.pow(trace_length / 512) - Constant(1.into())), // state_transition/merkle_update/prev_authentication/hashes/init/x
    (Trace(1, 0) - shift_point.y.clone()) / (X.pow(trace_length / 512) - Constant(1.into())), // state_transition/merkle_update/prev_authentication/hashes/init/y
    ((Constant(1.into()) - state_transition__merkle_update__side_bit_extraction__bit_1.clone()) * (Trace(0, 1) - Trace(3, 2))) * ((X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) * (X.pow(trace_length / 16384) - trace_generator.pow(15 * trace_length / 16))) / (X.pow(trace_length / 512) - Constant(1.into())), // state_transition/merkle_update/prev_authentication/copy_prev_to_left
    (state_transition__merkle_update__side_bit_extraction__bit_1.clone() * (Trace(0, 1) - Trace(3, 8))) * ((X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) * (X.pow(trace_length / 16384) - trace_generator.pow(15 * trace_length / 16))) / (X.pow(trace_length / 512) - Constant(1.into())), // state_transition/merkle_update/prev_authentication/copy_prev_to_right
    (state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit.clone() * (state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit.clone() - Constant(1.into()))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - Constant(1.into())), // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/booleanity_test
    (Trace(7, 0)) / (X.pow(trace_length / 256) - trace_generator.pow(251 * trace_length / 256)), // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit_extraction_end
    (Trace(7, 0)) / (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)), // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/zeros_tail
    (state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit.clone() * (Trace(5, 0) - merkle_hash_points__y.clone()) - Trace(6, 0) * (Trace(4, 0) - merkle_hash_points__x.clone())) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - Constant(1.into())), // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/slope
    (Trace(6, 0) * Trace(6, 0) - state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit.clone() * (Trace(4, 0) + merkle_hash_points__x.clone() + Trace(4, 1))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - Constant(1.into())), // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/x
    (state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit.clone() * (Trace(5, 0) + Trace(5, 1)) - Trace(6, 0) * (Trace(4, 0) - Trace(4, 1))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - Constant(1.into())), // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/y
    (state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit_neg.clone() * (Trace(4, 1) - Trace(4, 0))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - Constant(1.into())), // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/copy_point/x
    (state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit_neg.clone() * (Trace(5, 1) - Trace(5, 0))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - Constant(1.into())), // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/copy_point/y
    (Trace(4, 6) - Trace(4, 5)) * (X.pow(trace_length / 512) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 256) - Constant(1.into())), // state_transition/merkle_update/new_authentication/hashes/copy_point/x
    (Trace(5, 6) - Trace(5, 5)) * (X.pow(trace_length / 512) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 256) - Constant(1.into())), // state_transition/merkle_update/new_authentication/hashes/copy_point/y
    (Trace(4, 0) - shift_point.x.clone()) / (X.pow(trace_length / 512) - Constant(1.into())), // state_transition/merkle_update/new_authentication/hashes/init/x
    (Trace(5, 0) - shift_point.y.clone()) / (X.pow(trace_length / 512) - Constant(1.into())), // state_transition/merkle_update/new_authentication/hashes/init/y
    ((Constant(1.into()) - state_transition__merkle_update__side_bit_extraction__bit_1.clone()) * (Trace(4, 1) - Trace(7, 2))) * ((X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) * (X.pow(trace_length / 16384) - trace_generator.pow(15 * trace_length / 16))) / (X.pow(trace_length / 512) - Constant(1.into())), // state_transition/merkle_update/new_authentication/copy_prev_to_left
    (state_transition__merkle_update__side_bit_extraction__bit_1.clone() * (Trace(4, 1) - Trace(7, 8))) * ((X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) * (X.pow(trace_length / 16384) - trace_generator.pow(15 * trace_length / 16))) / (X.pow(trace_length / 512) - Constant(1.into())), // state_transition/merkle_update/new_authentication/copy_prev_to_right
    (state_transition__merkle_update__prev_authentication__sibling_0 - state_transition__merkle_update__new_authentication__sibling_0) * (X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) / (X.pow(trace_length / 512) - Constant(1.into())), // state_transition/merkle_update/same_siblings
    (state_transition__merkle_update__prev_authentication__leaf_0 - Trace(8, 2)) / (X.pow(trace_length / 16384) - Constant(1.into())), // state_transition/merkle_set_prev_leaf
    (state_transition__merkle_update__new_authentication__leaf_0 - Trace(8, 4)) / (X.pow(trace_length / 16384) - Constant(1.into())), // state_transition/merkle_set_new_leaf
    (is_modification.clone() * (Trace(9, 6) * boundary_base.clone() - boundary_key)) / (X.pow(trace_length / 65536) - Constant(1.into())), // modification_boundary_key
    (is_modification.clone() * (Trace(9, 0) * boundary_base.clone() - boundary_token.clone() )) / (X.pow(trace_length / 65536) - Constant(1.into())), // modification_boundary_token.clone()
    (is_modification.clone() * (Trace(8, 5) * boundary_base.clone() - boundary_amount0)) / (X.pow(trace_length / 65536) - Constant(1.into())), // modification_boundary_amount0
    (is_modification.clone() * (Trace(8, 7) * boundary_base.clone() - boundary_amount1)) / (X.pow(trace_length / 65536) - Constant(1.into())), // modification_boundary_amount1
    (is_modification.clone() * (Trace(6, 5) * boundary_base.clone() - boundary_vault_id)) / (X.pow(trace_length / 65536) - Constant(1.into())), // modification_boundary_vault_id
    (amounts_range_check__bit_0.clone() * amounts_range_check__bit_0.clone() - amounts_range_check__bit_0.clone()) * (X.pow(trace_length / 8192) - trace_generator.pow(63 * trace_length / 64)) / (X.pow(trace_length / 128) - Constant(1.into())), // amounts_range_check/bit
    (Trace(9, 4)) / (X.pow(trace_length / 8192) - trace_generator.pow(63 * trace_length / 64)), // amounts_range_check/zero
    (is_settlement.clone() * (Trace(8, 5) - Trace(8, 7) - (Trace(8, 1) - Trace(8, 9)))) / (X.pow(trace_length / 65536) - Constant(1.into())), // total_token_a_not_changed
    (is_settlement.clone() * (Trace(8, 3) - Trace(8, 5) - (Trace(8, 9) - Trace(8, 7)))) / (X.pow(trace_length / 65536) - Constant(1.into())), // total_token_b_not_changed
    ((Trace(9, 4) - (Trace(8, 5) - Trace(8, 7))) * is_settlement.clone()) / (X.pow(trace_length / 65536) - Constant(1.into())), // diff_a_range_check_input
    ((Trace(9, 2) - (Trace(8, 3) - Trace(8, 5))) * is_settlement.clone()) / (X.pow(trace_length / 65536) - Constant(1.into())), // diff_b_range_check_input
    (Trace(9, 6) - Trace(8, 7)) / (X.pow(trace_length / 16384) - Constant(1.into())), // amounts_range_check_inputs
    (sig_verify__doubling_key__x_squared.clone() + sig_verify__doubling_key__x_squared.clone() + sig_verify__doubling_key__x_squared.clone() + sig_config.alpha.clone() - (Trace(9, 2) + Trace(9, 2)) * Trace(9, 6)) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - Constant(1.into())), // sig_verify/doubling_key/slope
    (Trace(9, 6) * Trace(9, 6) - (Trace(9, 0) + Trace(9, 0) + Trace(9, 4))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - Constant(1.into())), // sig_verify/doubling_key/x
    (Trace(9, 2) + Trace(9, 6) - Trace(9, 6) * (Trace(9, 0) - Trace(9, 4))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - Constant(1.into())), // sig_verify/doubling_key/y
    (sig_verify__exponentiate_generator__bit.clone() * (sig_verify__exponentiate_generator__bit.clone() - Constant(1.into()))) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - Constant(1.into())), // sig_verify/exponentiate_generator/booleanity_test
    (Trace(9, 0)) / (X.pow(trace_length / 32768) - trace_generator.pow(251 * trace_length / 256)), // sig_verify/exponentiate_generator/bit_extraction_end
    (Trace(9, 0)) / (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)), // sig_verify/exponentiate_generator/zeros_tail
    (sig_verify__exponentiate_generator__bit.clone() * (Trace(9, 6) - ecdsa_points__y) - Trace(9, 0) * (Trace(9, 8) - ecdsa_points__x.clone())) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - Constant(1.into())), // sig_verify/exponentiate_generator/add_points/slope
    (Trace(9, 0) * Trace(9, 0) - sig_verify__exponentiate_generator__bit.clone() * (Trace(9, 8) + ecdsa_points__x.clone() + Trace(9, 6))) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - Constant(1.into())), // sig_verify/exponentiate_generator/add_points/x
    (sig_verify__exponentiate_generator__bit.clone() * (Trace(9, 6) + Trace(9, 4)) - Trace(9, 0) * (Trace(9, 8) - Trace(9, 6))) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - Constant(1.into())), // sig_verify/exponentiate_generator/add_points/y
    (Trace(9, 4) * (Trace(9, 8) - ecdsa_points__x.clone()) - Constant(1.into())) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - Constant(1.into())), // sig_verify/exponentiate_generator/add_points/x_diff_inv
    (sig_verify__exponentiate_generator__bit_neg.clone() * (Trace(9, 6) - Trace(9, 8))) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - Constant(1.into())), // sig_verify/exponentiate_generator/copy_point/x
    (sig_verify__exponentiate_generator__bit_neg.clone() * (Trace(9, 4) - Trace(9, 6))) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - Constant(1.into())), // sig_verify/exponentiate_generator/copy_point/y
    (sig_verify__exponentiate_key__bit.clone() * (sig_verify__exponentiate_key__bit.clone() - Constant(1.into()))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - Constant(1.into())), // sig_verify/exponentiate_key/booleanity_test
    (Trace(9, 4)) / (X.pow(trace_length / 16384) - trace_generator.pow(251 * trace_length / 256)), // sig_verify/exponentiate_key/bit_extraction_end
    (Trace(9, 4)) / (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)), // sig_verify/exponentiate_key/zeros_tail
    (sig_verify__exponentiate_key__bit.clone() * (Trace(9, 8) - Trace(9, 2)) - Trace(9, 0) * (Trace(9, 8) - Trace(9, 0))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - Constant(1.into())), // sig_verify/exponentiate_key/add_points/slope
    (Trace(9, 0) * Trace(9, 0) - sig_verify__exponentiate_key__bit.clone() * (Trace(9, 8) + Trace(9, 0) + Trace(9, 2))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - Constant(1.into())), // sig_verify/exponentiate_key/add_points/x
    (sig_verify__exponentiate_key__bit.clone() * (Trace(9, 8) + Trace(9, 2)) - Trace(9, 0) * (Trace(9, 8) - Trace(9, 2))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - Constant(1.into())), // sig_verify/exponentiate_key/add_points/y
    (Trace(9, 6) * (Trace(9, 8) - Trace(9, 0)) - Constant(1.into())) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - Constant(1.into())), // sig_verify/exponentiate_key/add_points/x_diff_inv
    (sig_verify__exponentiate_key__bit_neg.clone() * (Trace(9, 2) - Trace(9, 8))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - Constant(1.into())), // sig_verify/exponentiate_key/copy_point/x
    (sig_verify__exponentiate_key__bit_neg.clone() * (Trace(9, 2) - Trace(9, 8))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - Constant(1.into())), // sig_verify/exponentiate_key/copy_point/y
    (Trace(9, 8) - sig_config.shift_point.x.clone()) / (X.pow(trace_length / 32768) - Constant(1.into())), // sig_verify/init_gen/x
    (Trace(9, 6) + sig_config.shift_point.y.clone()) / (X.pow(trace_length / 32768) - Constant(1.into())), // sig_verify/init_gen/y
    (Trace(9, 8) - sig_config.shift_point.x.clone()) / (X.pow(trace_length / 16384) - Constant(1.into())), // sig_verify/init_key/x
    (Trace(9, 8) - sig_config.shift_point.y.clone()) / (X.pow(trace_length / 16384) - Constant(1.into())), // sig_verify/init_key/y
    (Trace(9, 6) - Trace(9, 8) - Trace(9, 4) * (Trace(9, 8) - Trace(9, 8))) / (X.pow(trace_length / 32768) - Constant(1.into())), // sig_verify/add_results/slope
    (Trace(9, 4) * Trace(9, 4) - (Trace(9, 8) + Trace(9, 8) + Trace(9, 4))) / (X.pow(trace_length / 32768) - Constant(1.into())), // sig_verify/add_results/x
    (Trace(9, 6) + Trace(9, 6) - Trace(9, 4) * (Trace(9, 8) - Trace(9, 4))) / (X.pow(trace_length / 32768) - Constant(1.into())), // sig_verify/add_results/y
    (Trace(9, 0) * (Trace(9, 8) - Trace(9, 8)) - Constant(1.into())) / (X.pow(trace_length / 32768) - Constant(1.into())), // sig_verify/add_results/x_diff_inv
    (Trace(9, 2) + sig_config.shift_point.y.clone() - Trace(8, 9) * (Trace(9, 2) - sig_config.shift_point.x.clone())) / (X.pow(trace_length / 32768) - Constant(1.into())), // sig_verify/extract_r/slope
    (Trace(8, 9) * Trace(8, 9) - (Trace(9, 2) + sig_config.shift_point.x.clone() + Trace(9, 4))) / (X.pow(trace_length / 32768) - Constant(1.into())), // sig_verify/extract_r/x
    (Trace(8, 3) * (Trace(9, 2) - sig_config.shift_point.x.clone()) - Constant(1.into())) / (X.pow(trace_length / 32768) - Constant(1.into())), // sig_verify/extract_r/x_diff_inv
    (Trace(9, 0) * Trace(8, 1) - Constant(1.into())) / (X.pow(trace_length / 32768) - Constant(1.into())), // sig_verify/z_nonzero
    (Trace(9, 4) * Trace(9, 6) - Constant(1.into())) / (X.pow(trace_length / 16384) - Constant(1.into())), // sig_verify/r_and_w_nonzero
    (Trace(8, 5) - Trace(9, 0) * Trace(9, 0)) / (X.pow(trace_length / 32768) - Constant(1.into())), // sig_verify/q_on_curve/x_squared
    (Trace(9, 2) * Trace(9, 2) - (Trace(9, 0) * Trace(8, 5) + sig_config.alpha.clone() * Trace(9, 0) + sig_config.beta)) / (X.pow(trace_length / 32768) - Constant(1.into())), // sig_verify/q_on_curve/on_curve
    (is_settlement.clone() * (Trace(8, 1) - (((Trace(6, 5) * vault_shift + Trace(6, 7)) * amount_shift.clone() + Trace(9, 4)) * amount_shift.clone() + Trace(9, 2)) * trade_shift)) / (X.pow(trace_length / 65536) - Constant(1.into())), // maker_sig_input_packed
    (is_settlement.clone() * (Trace(8, 7) - Trace(8, 8))) / (X.pow(trace_length / 65536) - Constant(1.into())), // taker_sig_input_maker_hash
    (is_settlement.clone() * (Trace(8, 1) - Trace(6, 9))) / (X.pow(trace_length / 65536) - Constant(1.into())), // taker_sig_input_vault_a
    (is_settlement.clone() * (Trace(8, 9) - Trace(6, 3))) / (X.pow(trace_length / 65536) - Constant(1.into())), // taker_sig_input_vault_b
    (is_settlement.clone() * (Trace(8, 8) - Trace(9, 0))) / (X.pow(trace_length / 65536) - Constant(1.into())), // copy_signature_input_maker
    (is_settlement.clone() * (Trace(8, 6) - Trace(9, 8))) / (X.pow(trace_length / 65536) - Constant(1.into())), // copy_signature_input_taker
    (is_settlement.clone() * (Trace(9, 0) - Trace(9, 6))) / (X.pow(trace_length / 65536) - Constant(1.into())), // handle_empty_vault/consistency_key_change0
    (is_settlement.clone() * (Trace(8, 9) - Trace(9, 0))) / (X.pow(trace_length / 65536) - Constant(1.into())), // handle_empty_vault/consistency_token_change0
    (is_settlement.clone() * (Trace(9, 0) - Trace(9, 8))) / (X.pow(trace_length / 65536) - Constant(1.into())), // handle_empty_vault/consistency_key_change3
    (is_settlement.clone() * (Trace(8, 3) - Trace(9, 2))) / (X.pow(trace_length / 65536) - Constant(1.into())), // handle_empty_vault/consistency_token_change3
    (is_settlement.clone() * (Trace(9, 8) - Trace(9, 0))) / (X.pow(trace_length / 65536) - Constant(1.into())), // handle_empty_vault/consistency_key_change1
    (is_settlement.clone() * (Trace(8, 9) - Trace(9, 4))) / (X.pow(trace_length / 65536) - Constant(1.into())), // handle_empty_vault/consistency_token_change1
    (is_settlement.clone() * (Trace(9, 8) - Trace(9, 4))) / (X.pow(trace_length / 65536) - Constant(1.into())), // handle_empty_vault/consistency_key_change2
    (is_settlement.clone() * (Trace(8, 3) - Trace(9, 8))) / (X.pow(trace_length / 65536) - Constant(1.into())), // handle_empty_vault/consistency_token_change2
    (Trace(8, 1) * (Constant(1.into()) - Trace(8, 1))) / (X.pow(trace_length / 8192) - Constant(1.into())), // handle_empty_vault/vault_empty/empty_vault_booleanity
    (Trace(8, 1) * Trace(8, 5)) / (X.pow(trace_length / 8192) - Constant(1.into())), // handle_empty_vault/vault_empty/amount_zero_when_empty
    (Trace(8, 1) * Trace(8, 7)) / (X.pow(trace_length / 8192) - Constant(1.into())), // handle_empty_vault/vault_empty/amount_inv_zero_when_empty
    (Trace(8, 5) * Trace(8, 7) - (Constant(1.into()) - Trace(8, 1))) / (X.pow(trace_length / 8192) - Constant(1.into())), // handle_empty_vault/vault_empty/empty_when_amount_zero
    ((Constant(1.into()) - Trace(8, 1)) * Trace(9, 6) - Trace(8, 3)) / (X.pow(trace_length / 16384) - Constant(1.into())), // handle_empty_vault/consistency_key_stage0
    ((Constant(1.into()) - Trace(8, 1)) * Trace(9, 0) - Trace(8, 7)) / (X.pow(trace_length / 16384) - Constant(1.into())), // handle_empty_vault/consistency_token_stage0
    ((Constant(1.into()) - Trace(8, 3)) * Trace(9, 6) - Trace(8, 5)) / (X.pow(trace_length / 16384) - Constant(1.into())), // handle_empty_vault/consistency_key_stage1
    ((Constant(1.into()) - Trace(8, 3)) * Trace(9, 0) - Trace(8, 9)) / (X.pow(trace_length / 16384) - Constant(1.into())), // handle_empty_vault/consistency_token_stage1
    (column0_row_expr0 - initial_root.clone()) / (X - Constant(1.into())), // initial_root.clone()
    (column4_row_expr1.clone() - final_root.clone()) / (X - trace_generator.pow(65536 * (trace_length / 65536 - 1))), // final_root.clone()
    (column4_row_expr0.clone() - column0_row_expr2) * (X - trace_generator.pow(65536 * (trace_length / 65536 - 1) + 49152)) / (X.pow(trace_length / 16384) - Constant(1.into())), // copy_merkle_roots
    (is_modification.clone() * (column4_row_expr0.clone() - column4_row_expr1.clone())) / (X.pow(trace_length / 65536) - Constant(1.into())), // copy_merkle_roots_modification
    ]
}
