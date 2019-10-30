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

fn signature_verification_constraints() -> Vec<RationalExpression> {
    fn exponentiate_generator_constraints() -> Vec<RationalExpression> {
        use RationalExpression::*;

        let trace_length = 10;
        let trace_generator = Constant(FieldElement::ZERO);

        let ecdsa_points_x = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
        let ecdsa_points_y = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));

        let bit = Trace(9, 0) - (Trace(9, 8) + Trace(9, 8));
        let bit_neg = Constant(1.into()) - bit.clone();

        vec![
        (bit.clone() * (bit.clone() - 1.into())) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - 1.into()), // booleanity_test
        (Trace(9, 0)) / (X.pow(trace_length / 32768) - trace_generator.pow(251 * trace_length / 256)), // bit_extraction_end
        (Trace(9, 0)) / (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)), // zeros_tail
        (bit.clone() * (Trace(9, 6) - ecdsa_points_y) - Trace(9, 0) * (Trace(9, 8) - ecdsa_points_x.clone())) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - 1.into()), // add_points/slope
        (Trace(9, 0) * Trace(9, 0) - bit.clone() * (Trace(9, 8) + ecdsa_points_x.clone() + Trace(9, 6))) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - 1.into()), // add_points/x
        (bit.clone() * (Trace(9, 6) + Trace(9, 4)) - Trace(9, 0) * (Trace(9, 8) - Trace(9, 6))) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - 1.into()), // add_points/y
        (Trace(9, 4) * (Trace(9, 8) - ecdsa_points_x.clone()) - 1.into()) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - 1.into()), // add_points/x_diff_inv
        (bit_neg.clone() * (Trace(9, 6) - Trace(9, 8))) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - 1.into()), // copy_point/x
        (bit_neg.clone() * (Trace(9, 4) - Trace(9, 6))) * (X.pow(trace_length / 32768) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 128) - 1.into()), // copy_point/y
        ]
    }

    fn exponentiate_key_constraints() -> Vec<RationalExpression> {
        use RationalExpression::*;

        let trace_length = 10;
        let trace_generator = Constant(FieldElement::ZERO);

        let exponentiate_key_bit = Trace(9, 4) - (Trace(9, 8) + Trace(9, 8));
        let exponentiate_key_bit_neg = Constant(1.into()) - exponentiate_key_bit.clone();

        vec![
        (exponentiate_key_bit.clone() * (exponentiate_key_bit.clone() - 1.into())) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // exponentiate_key/booleanity_test
        (Trace(9, 4)) / (X.pow(trace_length / 16384) - trace_generator.pow(251 * trace_length / 256)), // exponentiate_key/bit_extraction_end
        (Trace(9, 4)) / (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)), // exponentiate_key/zeros_tail
        (exponentiate_key_bit.clone() * (Trace(9, 8) - Trace(9, 2)) - Trace(9, 0) * (Trace(9, 8) - Trace(9, 0))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // exponentiate_key/add_points/slope
        (Trace(9, 0) * Trace(9, 0) - exponentiate_key_bit.clone() * (Trace(9, 8) + Trace(9, 0) + Trace(9, 2))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // exponentiate_key/add_points/x
        (exponentiate_key_bit.clone() * (Trace(9, 8) + Trace(9, 2)) - Trace(9, 0) * (Trace(9, 8) - Trace(9, 2))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // exponentiate_key/add_points/y
        (Trace(9, 6) * (Trace(9, 8) - Trace(9, 0)) - 1.into()) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // exponentiate_key/add_points/x_diff_inv
        (exponentiate_key_bit_neg.clone() * (Trace(9, 2) - Trace(9, 8))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // exponentiate_key/copy_point/x
        (exponentiate_key_bit_neg.clone() * (Trace(9, 2) - Trace(9, 8))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // exponentiate_key/copy_point/y
        ]
    }

    use RationalExpression::*;

    let trace_length = 10;
    let trace_generator = Constant(FieldElement::ZERO);

    let sig_config = SignatureConfig {
        shift_point: Point {
            x: Constant(FieldElement::ONE),
            y: Constant(FieldElement::ONE),
        },
        alpha: Constant(FieldElement::ONE),
        beta: Constant(FieldElement::ONE),
    };

    let doubling_key_x_squared = Trace(9, 0) * Trace(9, 0);

    let mut result = vec![
    (doubling_key_x_squared.clone() + doubling_key_x_squared.clone() + doubling_key_x_squared.clone() + sig_config.alpha.clone() - (Trace(9, 2) + Trace(9, 2)) * Trace(9, 6)) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // doubling_key/slope
    (Trace(9, 6) * Trace(9, 6) - (Trace(9, 0) + Trace(9, 0) + Trace(9, 4))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // doubling_key/x
    (Trace(9, 2) + Trace(9, 6) - Trace(9, 6) * (Trace(9, 0) - Trace(9, 4))) * (X.pow(trace_length / 16384) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 64) - 1.into()), // doubling_key/y
    (Trace(9, 8) - sig_config.shift_point.x.clone()) / (X.pow(trace_length / 32768) - 1.into()), // init_gen/x
    (Trace(9, 6) + sig_config.shift_point.y.clone()) / (X.pow(trace_length / 32768) - 1.into()), // init_gen/y
    (Trace(9, 8) - sig_config.shift_point.x.clone()) / (X.pow(trace_length / 16384) - 1.into()), // init_key/x
    (Trace(9, 8) - sig_config.shift_point.y.clone()) / (X.pow(trace_length / 16384) - 1.into()), // init_key/y
    (Trace(9, 6) - Trace(9, 8) - Trace(9, 4) * (Trace(9, 8) - Trace(9, 8))) / (X.pow(trace_length / 32768) - 1.into()), // add_results/slope
    (Trace(9, 4) * Trace(9, 4) - (Trace(9, 8) + Trace(9, 8) + Trace(9, 4))) / (X.pow(trace_length / 32768) - 1.into()), // add_results/x
    (Trace(9, 6) + Trace(9, 6) - Trace(9, 4) * (Trace(9, 8) - Trace(9, 4))) / (X.pow(trace_length / 32768) - 1.into()), // add_results/y
    (Trace(9, 0) * (Trace(9, 8) - Trace(9, 8)) - 1.into()) / (X.pow(trace_length / 32768) - 1.into()), // add_results/x_diff_inv
    (Trace(9, 2) + sig_config.shift_point.y.clone() - Trace(8, 9) * (Trace(9, 2) - sig_config.shift_point.x.clone())) / (X.pow(trace_length / 32768) - 1.into()), // extract_r/slope
    (Trace(8, 9) * Trace(8, 9) - (Trace(9, 2) + sig_config.shift_point.x.clone() + Trace(9, 4))) / (X.pow(trace_length / 32768) - 1.into()), // extract_r/x
    (Trace(8, 3) * (Trace(9, 2) - sig_config.shift_point.x.clone()) - 1.into()) / (X.pow(trace_length / 32768) - 1.into()), // extract_r/x_diff_inv
    (Trace(9, 0) * Trace(8, 1) - 1.into()) / (X.pow(trace_length / 32768) - 1.into()), // z_nonzero
    (Trace(9, 4) * Trace(9, 6) - 1.into()) / (X.pow(trace_length / 16384) - 1.into()), // r_and_w_nonzero
    (Trace(8, 5) - Trace(9, 0) * Trace(9, 0)) / (X.pow(trace_length / 32768) - 1.into()), // q_on_curve/x_squared
    (Trace(9, 2) * Trace(9, 2) - (Trace(9, 0) * Trace(8, 5) + sig_config.alpha.clone() * Trace(9, 0) + sig_config.beta)) / (X.pow(trace_length / 32768) - 1.into()), // q_on_curve/on_curve
    ];

    result.extend(exponentiate_key_constraints());
    result.extend(exponentiate_generator_constraints());
    result
}

fn state_transition_constraints() -> Vec<RationalExpression> {
    use RationalExpression::*;

    let trace_length = 10;
    let path_length = 256;
    let trace_generator = Constant(FieldElement::ZERO);

    let merkle_hash_points_x = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let merkle_hash_points_y = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));

    let shift_point = Point {
        x: Constant(FieldElement::ONE),
        y: Constant(FieldElement::ONE),
    };

    let side_bit_extraction_bit_0 = Trace(6, 5) - (Trace(6, 7) + Trace(6, 7));
    let prev_authentication_hashes_ec_subset_sum_bit = Trace(3, 0) - (Trace(3, 1) + Trace(3, 1));
    let prev_authentication_hashes_ec_subset_sum_bit_neg = Constant(1.into()) - prev_authentication_hashes_ec_subset_sum_bit.clone();
    let side_bit_extraction_bit_1 = Trace(6, 7) - (Trace(6, 9) + Trace(6, 9));
    let new_authentication_hashes_ec_subset_sum_bit = Trace(7, 0) - (Trace(7, 1) + Trace(7, 1));
    let new_authentication_hashes_ec_subset_sum_bit_neg = Constant(1.into()) - new_authentication_hashes_ec_subset_sum_bit.clone();
    let prev_authentication_sibling_0 = side_bit_extraction_bit_0.clone() * Trace(3, 0) + (Constant(1.into()) - side_bit_extraction_bit_0.clone()) * Trace(3, 6);
    let new_authentication_sibling_0 = side_bit_extraction_bit_0.clone() * Trace(7, 0) + (Constant(1.into()) - side_bit_extraction_bit_0.clone()) * Trace(7, 6);
    let prev_authentication_leaf_0 = (Constant(1.into()) - side_bit_extraction_bit_0.clone()) * Trace(3, 0) + side_bit_extraction_bit_0.clone() * Trace(3, 6);
    let new_authentication_leaf_0 = (Constant(1.into()) - side_bit_extraction_bit_0.clone()) * Trace(7, 0) + side_bit_extraction_bit_0.clone() * Trace(7, 6);

    vec![
    (side_bit_extraction_bit_0.clone() * side_bit_extraction_bit_0.clone() - side_bit_extraction_bit_0.clone()) * (X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) / (X.pow(trace_length / 512) - 1.into()), // side_bit_extraction/bit
    (Trace(6, 5)) / (X.pow(trace_length / 16384) - trace_generator.pow(path_length * trace_length / 32)), // side_bit_extraction/zero
    (prev_authentication_hashes_ec_subset_sum_bit.clone() * (prev_authentication_hashes_ec_subset_sum_bit.clone() - 1.into())) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // prev_authentication/hashes/ec_subset_sum/booleanity_test
    (Trace(3, 0)) / (X.pow(trace_length / 256) - trace_generator.pow(251 * trace_length / 256)), // prev_authentication/hashes/ec_subset_sum/bit_extraction_end
    (Trace(3, 0)) / (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)), // prev_authentication/hashes/ec_subset_sum/zeros_tail
    (prev_authentication_hashes_ec_subset_sum_bit.clone() * (Trace(1, 0) - merkle_hash_points_y.clone()) - Trace(2, 0) * (Trace(0, 0) - merkle_hash_points_x.clone())) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // prev_authentication/hashes/ec_subset_sum/add_points/slope
    (Trace(2, 0) * Trace(2, 0) - prev_authentication_hashes_ec_subset_sum_bit.clone() * (Trace(0, 0) + merkle_hash_points_x.clone() + Trace(0, 1))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // prev_authentication/hashes/ec_subset_sum/add_points/x
    (prev_authentication_hashes_ec_subset_sum_bit.clone() * (Trace(1, 0) + Trace(1, 1)) - Trace(2, 0) * (Trace(0, 0) - Trace(0, 1))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // prev_authentication/hashes/ec_subset_sum/add_points/y
    (prev_authentication_hashes_ec_subset_sum_bit_neg.clone() * (Trace(0, 1) - Trace(0, 0))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // prev_authentication/hashes/ec_subset_sum/copy_point/x
    (prev_authentication_hashes_ec_subset_sum_bit_neg.clone() * (Trace(1, 1) - Trace(1, 0))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // prev_authentication/hashes/ec_subset_sum/copy_point/y
    (Trace(0, 6) - Trace(0, 5)) * (X.pow(trace_length / 512) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 256) - 1.into()), // prev_authentication/hashes/copy_point/x
    (Trace(1, 6) - Trace(1, 5)) * (X.pow(trace_length / 512) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 256) - 1.into()), // prev_authentication/hashes/copy_point/y
    (Trace(0, 0) - shift_point.x.clone()) / (X.pow(trace_length / 512) - 1.into()), // prev_authentication/hashes/init/x
    (Trace(1, 0) - shift_point.y.clone()) / (X.pow(trace_length / 512) - 1.into()), // prev_authentication/hashes/init/y
    ((Constant(1.into()) - side_bit_extraction_bit_1.clone()) * (Trace(0, 1) - Trace(3, 2))) * ((X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) * (X.pow(trace_length / 16384) - trace_generator.pow(15 * trace_length / 16))) / (X.pow(trace_length / 512) - 1.into()), // prev_authentication/copy_prev_to_left
    (side_bit_extraction_bit_1.clone() * (Trace(0, 1) - Trace(3, 8))) * ((X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) * (X.pow(trace_length / 16384) - trace_generator.pow(15 * trace_length / 16))) / (X.pow(trace_length / 512) - 1.into()), // prev_authentication/copy_prev_to_right
    (new_authentication_hashes_ec_subset_sum_bit.clone() * (new_authentication_hashes_ec_subset_sum_bit.clone() - 1.into())) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // new_authentication/hashes/ec_subset_sum/booleanity_test
    (Trace(7, 0)) / (X.pow(trace_length / 256) - trace_generator.pow(251 * trace_length / 256)), // new_authentication/hashes/ec_subset_sum/bit_extraction_end
    (Trace(7, 0)) / (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)), // new_authentication/hashes/ec_subset_sum/zeros_tail
    (new_authentication_hashes_ec_subset_sum_bit.clone() * (Trace(5, 0) - merkle_hash_points_y.clone()) - Trace(6, 0) * (Trace(4, 0) - merkle_hash_points_x.clone())) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // new_authentication/hashes/ec_subset_sum/add_points/slope
    (Trace(6, 0) * Trace(6, 0) - new_authentication_hashes_ec_subset_sum_bit.clone() * (Trace(4, 0) + merkle_hash_points_x.clone() + Trace(4, 1))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // new_authentication/hashes/ec_subset_sum/add_points/x
    (new_authentication_hashes_ec_subset_sum_bit.clone() * (Trace(5, 0) + Trace(5, 1)) - Trace(6, 0) * (Trace(4, 0) - Trace(4, 1))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // new_authentication/hashes/ec_subset_sum/add_points/y
    (new_authentication_hashes_ec_subset_sum_bit_neg.clone() * (Trace(4, 1) - Trace(4, 0))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // new_authentication/hashes/ec_subset_sum/copy_point/x
    (new_authentication_hashes_ec_subset_sum_bit_neg.clone() * (Trace(5, 1) - Trace(5, 0))) * (X.pow(trace_length / 256) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length) - 1.into()), // new_authentication/hashes/ec_subset_sum/copy_point/y
    (Trace(4, 6) - Trace(4, 5)) * (X.pow(trace_length / 512) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 256) - 1.into()), // new_authentication/hashes/copy_point/x
    (Trace(5, 6) - Trace(5, 5)) * (X.pow(trace_length / 512) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 256) - 1.into()), // new_authentication/hashes/copy_point/y
    (Trace(4, 0) - shift_point.x.clone()) / (X.pow(trace_length / 512) - 1.into()), // new_authentication/hashes/init/x
    (Trace(5, 0) - shift_point.y.clone()) / (X.pow(trace_length / 512) - 1.into()), // new_authentication/hashes/init/y
    ((Constant(1.into()) - side_bit_extraction_bit_1.clone()) * (Trace(4, 1) - Trace(7, 2))) * ((X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) * (X.pow(trace_length / 16384) - trace_generator.pow(15 * trace_length / 16))) / (X.pow(trace_length / 512) - 1.into()), // new_authentication/copy_prev_to_left
    (side_bit_extraction_bit_1.clone() * (Trace(4, 1) - Trace(7, 8))) * ((X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) * (X.pow(trace_length / 16384) - trace_generator.pow(15 * trace_length / 16))) / (X.pow(trace_length / 512) - 1.into()), // new_authentication/copy_prev_to_right
    (prev_authentication_sibling_0 - new_authentication_sibling_0) * (X.pow(trace_length / 16384) - trace_generator.pow(31 * trace_length / 32)) / (X.pow(trace_length / 512) - 1.into()), // same_siblings
    (prev_authentication_leaf_0 - Trace(8, 2)) / (X.pow(trace_length / 16384) - 1.into()), // merkle_set_prev_leaf
    (new_authentication_leaf_0 - Trace(8, 4)) / (X.pow(trace_length / 16384) - 1.into()), // merkle_set_new_leaf
    ]
}

fn hash_pool_constraints() -> Vec<RationalExpression> {
    use RationalExpression::*;

    let trace_length = 10;
    let trace_generator = Constant(FieldElement::ZERO);

    let hash_pool_points_x = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));
    let hash_pool_points_y = Polynomial(DensePolynomial::new(&[FieldElement::ZERO]), Box::new(X.pow(20)));

    let shift_point = Point {
        x: Constant(FieldElement::ONE),
        y: Constant(FieldElement::ONE),
    };

    let ec_subset_sum_bit = Trace(8, 3) - (Trace(8, 7) + Trace(8, 7));
    let ec_subset_sum_bit_neg = Constant(1.into()) - ec_subset_sum_bit.clone();
    vec![
    (ec_subset_sum_bit.clone() * (ec_subset_sum_bit.clone() - 1.into())) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - 1.into()), // ec_subset_sum/booleanity_test
    (Trace(8, 3)) / (X.pow(trace_length / 1024) - trace_generator.pow(251 * trace_length / 256)), // ec_subset_sum/bit_extraction_end
    (Trace(8, 3)) / (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)), // ec_subset_sum/zeros_tail
    (ec_subset_sum_bit.clone() * (Trace(8, 2) - hash_pool_points_y) - Trace(8, 1) * (Trace(8, 0) - hash_pool_points_x.clone())) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - 1.into()), // ec_subset_sum/add_points/slope
    (Trace(8, 1) * Trace(8, 1) - ec_subset_sum_bit.clone() * (Trace(8, 0) + hash_pool_points_x.clone() + Trace(8, 4))) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - 1.into()), // ec_subset_sum/add_points/x
    (ec_subset_sum_bit.clone() * (Trace(8, 2) + Trace(8, 6)) - Trace(8, 1) * (Trace(8, 0) - Trace(8, 4))) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - 1.into()), // ec_subset_sum/add_points/y
    (ec_subset_sum_bit_neg.clone() * (Trace(8, 4) - Trace(8, 0))) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - 1.into()), // ec_subset_sum/copy_point/x
    (ec_subset_sum_bit_neg.clone() * (Trace(8, 6) - Trace(8, 2))) * (X.pow(trace_length / 1024) - trace_generator.pow(255 * trace_length / 256)) / (X.pow(trace_length / 4) - 1.into()), // ec_subset_sum/copy_point/y
    (Trace(8, 4) - Trace(8, 0)) * (X.pow(trace_length / 2048) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 1024) - 1.into()), // copy_point/x
    (Trace(8, 6) - Trace(8, 2)) * (X.pow(trace_length / 2048) - trace_generator.pow(trace_length / 2)) / (X.pow(trace_length / 1024) - 1.into()), // copy_point/y
    (Trace(8, 0) - shift_point.x.clone()) / (X.pow(trace_length / 2048) - 1.into()), // init/x
    (Trace(8, 2) - shift_point.y.clone()) / (X.pow(trace_length / 2048) - 1.into()), // init/y
    (Trace(8, 4) - Trace(8, 1)) / (X.pow(trace_length / 4096) - 1.into()), // output_to_input
    ]
}

fn other_constraints() -> Vec<RationalExpression> {
    use RationalExpression::*;

    let trace_length = 10;
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

    let trade_shift = Constant(FieldElement::ONE);
    let amount_shift = Constant(FieldElement::ONE);
    let vault_shift = Constant(FieldElement::ONE);

    let initial_root = Constant(FieldElement::ONE);
    let final_root = Constant(FieldElement::ONE);

    let amounts_range_check_bit_0 = Trace(9, 4) - (Trace(9, 2) + Trace(9, 2));
    vec![
    (is_modification.clone() * (Trace(9, 6) * boundary_base.clone() - boundary_key)) / (X.pow(trace_length / 65536) - 1.into()), // modification_boundary_key
    (is_modification.clone() * (Trace(9, 0) * boundary_base.clone() - boundary_token.clone() )) / (X.pow(trace_length / 65536) - 1.into()), // modification_boundary_token.clone()
    (is_modification.clone() * (Trace(8, 5) * boundary_base.clone() - boundary_amount0)) / (X.pow(trace_length / 65536) - 1.into()), // modification_boundary_amount0
    (is_modification.clone() * (Trace(8, 7) * boundary_base.clone() - boundary_amount1)) / (X.pow(trace_length / 65536) - 1.into()), // modification_boundary_amount1
    (is_modification.clone() * (Trace(6, 5) * boundary_base.clone() - boundary_vault_id)) / (X.pow(trace_length / 65536) - 1.into()), // modification_boundary_vault_id
    (amounts_range_check_bit_0.clone() * amounts_range_check_bit_0.clone() - amounts_range_check_bit_0.clone()) * (X.pow(trace_length / 8192) - trace_generator.pow(63 * trace_length / 64)) / (X.pow(trace_length / 128) - 1.into()), // amounts_range_check/bit
    (Trace(9, 4)) / (X.pow(trace_length / 8192) - trace_generator.pow(63 * trace_length / 64)), // amounts_range_check/zero
    (is_settlement.clone() * (Trace(8, 5) - Trace(8, 7) - (Trace(8, 1) - Trace(8, 9)))) / (X.pow(trace_length / 65536) - 1.into()), // total_token_a_not_changed
    (is_settlement.clone() * (Trace(8, 3) - Trace(8, 5) - (Trace(8, 9) - Trace(8, 7)))) / (X.pow(trace_length / 65536) - 1.into()), // total_token_b_not_changed
    ((Trace(9, 4) - (Trace(8, 5) - Trace(8, 7))) * is_settlement.clone()) / (X.pow(trace_length / 65536) - 1.into()), // diff_a_range_check_input
    ((Trace(9, 2) - (Trace(8, 3) - Trace(8, 5))) * is_settlement.clone()) / (X.pow(trace_length / 65536) - 1.into()), // diff_b_range_check_input
    (Trace(9, 6) - Trace(8, 7)) / (X.pow(trace_length / 16384) - 1.into()), // amounts_range_check_inputs
    (is_settlement.clone() * (Trace(8, 1) - (((Trace(6, 5) * vault_shift + Trace(6, 7)) * amount_shift.clone() + Trace(9, 4)) * amount_shift.clone() + Trace(9, 2)) * trade_shift)) / (X.pow(trace_length / 65536) - 1.into()), // maker_sig_input_packed
    (is_settlement.clone() * (Trace(8, 7) - Trace(8, 8))) / (X.pow(trace_length / 65536) - 1.into()), // taker_sig_input_maker_hash
    (is_settlement.clone() * (Trace(8, 1) - Trace(6, 9))) / (X.pow(trace_length / 65536) - 1.into()), // taker_sig_input_vault_a
    (is_settlement.clone() * (Trace(8, 9) - Trace(6, 3))) / (X.pow(trace_length / 65536) - 1.into()), // taker_sig_input_vault_b
    (is_settlement.clone() * (Trace(8, 8) - Trace(9, 0))) / (X.pow(trace_length / 65536) - 1.into()), // copy_signature_input_maker
    (is_settlement.clone() * (Trace(8, 6) - Trace(9, 8))) / (X.pow(trace_length / 65536) - 1.into()), // copy_signature_input_taker
    (is_settlement.clone() * (Trace(9, 0) - Trace(9, 6))) / (X.pow(trace_length / 65536) - 1.into()), // handle_empty_vault/consistency_key_change0
    (is_settlement.clone() * (Trace(8, 9) - Trace(9, 0))) / (X.pow(trace_length / 65536) - 1.into()), // handle_empty_vault/consistency_token_change0
    (is_settlement.clone() * (Trace(9, 0) - Trace(9, 8))) / (X.pow(trace_length / 65536) - 1.into()), // handle_empty_vault/consistency_key_change3
    (is_settlement.clone() * (Trace(8, 3) - Trace(9, 2))) / (X.pow(trace_length / 65536) - 1.into()), // handle_empty_vault/consistency_token_change3
    (is_settlement.clone() * (Trace(9, 8) - Trace(9, 0))) / (X.pow(trace_length / 65536) - 1.into()), // handle_empty_vault/consistency_key_change1
    (is_settlement.clone() * (Trace(8, 9) - Trace(9, 4))) / (X.pow(trace_length / 65536) - 1.into()), // handle_empty_vault/consistency_token_change1
    (is_settlement.clone() * (Trace(9, 8) - Trace(9, 4))) / (X.pow(trace_length / 65536) - 1.into()), // handle_empty_vault/consistency_key_change2
    (is_settlement.clone() * (Trace(8, 3) - Trace(9, 8))) / (X.pow(trace_length / 65536) - 1.into()), // handle_empty_vault/consistency_token_change2
    (Trace(8, 1) * (Constant(1.into()) - Trace(8, 1))) / (X.pow(trace_length / 8192) - 1.into()), // handle_empty_vault/vault_empty/empty_vault_booleanity
    (Trace(8, 1) * Trace(8, 5)) / (X.pow(trace_length / 8192) - 1.into()), // handle_empty_vault/vault_empty/amount_zero_when_empty
    (Trace(8, 1) * Trace(8, 7)) / (X.pow(trace_length / 8192) - 1.into()), // handle_empty_vault/vault_empty/amount_inv_zero_when_empty
    (Trace(8, 5) * Trace(8, 7) - (Constant(1.into()) - Trace(8, 1))) / (X.pow(trace_length / 8192) - 1.into()), // handle_empty_vault/vault_empty/empty_when_amount_zero
    ((Constant(1.into()) - Trace(8, 1)) * Trace(9, 6) - Trace(8, 3)) / (X.pow(trace_length / 16384) - 1.into()), // handle_empty_vault/consistency_key_stage0
    ((Constant(1.into()) - Trace(8, 1)) * Trace(9, 0) - Trace(8, 7)) / (X.pow(trace_length / 16384) - 1.into()), // handle_empty_vault/consistency_token_stage0
    ((Constant(1.into()) - Trace(8, 3)) * Trace(9, 6) - Trace(8, 5)) / (X.pow(trace_length / 16384) - 1.into()), // handle_empty_vault/consistency_key_stage1
    ((Constant(1.into()) - Trace(8, 3)) * Trace(9, 0) - Trace(8, 9)) / (X.pow(trace_length / 16384) - 1.into()), // handle_empty_vault/consistency_token_stage1
    (column0_row_expr0 - initial_root.clone()) / (X - 1.into()), // initial_root.clone()
    (column4_row_expr1.clone() - final_root.clone()) / (X - trace_generator.pow(65536 * (trace_length / 65536 - 1))), // final_root.clone()
    (column4_row_expr0.clone() - column0_row_expr2) * (X - trace_generator.pow(65536 * (trace_length / 65536 - 1) + 49152)) / (X.pow(trace_length / 16384) - 1.into()), // copy_merkle_roots
    (is_modification.clone() * (column4_row_expr0.clone() - column4_row_expr1.clone())) / (X.pow(trace_length / 65536) - 1.into()), // copy_merkle_roots_modification
    ]
}

#[allow(dead_code)]
fn constraints() -> Vec<RationalExpression> {
    let mut result = state_transition_constraints();
    result.extend(signature_verification_constraints());
    result.extend(hash_pool_constraints());
    result.extend(other_constraints());
    result
}
