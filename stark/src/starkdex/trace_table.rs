use crate::TraceTable;
use ecc::Affine;
use primefield::FieldElement;
use starkdex::{PEDERSEN_POINTS, SHIFT_POINT};
use std::prelude::v1::*;
use u256::U256;

fn get_trace_table() -> TraceTable {
    let num_columns = 10;
    let num_rows = 2usize.pow(15);
    let mut trace_table = TraceTable::new(num_rows, num_columns);

    trace_table
}

struct SigConfig {
    pub alpha:       FieldElement,
    pub beta:        FieldElement,
    pub shift_point: Point,
}

struct Point {
    pub x: FieldElement,
    pub y: FieldElement,
}

fn test_trace_table() {
    let trace_table = get_trace_table();
    for i in 0..trace_table.num_rows() {
        let is_settlement = FieldElement::ZERO;
        let is_modification = FieldElement::ZERO;
        let amounts_range_check__bit_0 = FieldElement::ONE;
        let sig_verify__exponentiate_key__bit = FieldElement::ONE;
        let sig_verify__exponentiate_generator__bit = FieldElement::ONE;
        let hash_pool__hash__ec_subset_sum__bit = FieldElement::ONE;
        let hash_pool__hash__ec_subset_sum__bit_neg =
            FieldElement::ONE - &hash_pool__hash__ec_subset_sum__bit;
        let state_transition__merkle_update__side_bit_extraction__bit_1 = FieldElement::ONE;
        let state_transition__merkle_update__side_bit_extraction__bit_0 = FieldElement::ZERO; // I think this is the negation of the above?
        let state_transition__merkle_update__prev_authentication__leaf_0 = FieldElement::ONE;
        let state_transition__merkle_update__prev_authentication__sibling_0 = FieldElement::ZERO;
        let state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit =
            FieldElement::ZERO;
        let state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit_neg =
            FieldElement::ONE
                - &state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit;
        let state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit =
            FieldElement::ONE;
        let state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit_neg =
            FieldElement::ONE
                - &state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit;
        let state_transition__merkle_update__new_authentication__sibling_0 = FieldElement::ZERO;
        let initial_root = FieldElement::ONE;
        let sig_config = SigConfig {
            alpha:       FieldElement::ONE,
            beta:        FieldElement::ONE,
            shift_point: Point {
                x: FieldElement::ONE,
                y: FieldElement::ONE,
            },
        };
        let shift_point = Point {
            x: FieldElement::ONE,
            y: FieldElement::ONE,
        };
        let sig_verify__doubling_key__x_squared = FieldElement::ZERO;
        let final_root = FieldElement::ONE;
        let ecdsa_points__y = FieldElement::ONE;
        let ecdsa_points__x = FieldElement::ONE;
        let state_transition__merkle_update__new_authentication__leaf_0 = FieldElement::ZERO;

        let boundary_vault_id = FieldElement::ONE;
        let boundary_base = FieldElement::ONE;
        let boundary_amount0 = FieldElement::ONE;
        let boundary_amount1 = FieldElement::ONE;
        let boundary_token = FieldElement::ONE;
        let boundary_key = FieldElement::ONE;

        let vault_shift = FieldElement::ONE;

        let trade_shift = FieldElement::ZERO;
        let amount_shift = FieldElement::ZERO;

        let column4_row_expr0 = FieldElement::NEGATIVE_ONE;
        let column4_row_expr1 = FieldElement::NEGATIVE_ONE;
        let column0_row_expr2 = FieldElement::NEGATIVE_ONE;
        let column0_row_expr0 = FieldElement::NEGATIVE_ONE;

        let trace_length = 0;
        let path_length = 256;

        let sig_verify__exponentiate_key__bit_neg =
            FieldElement::ONE - &sig_verify__exponentiate_key__bit;
        let sig_verify__exponentiate_generator__bit_neg =
            FieldElement::ONE - &sig_verify__exponentiate_generator__bit;

        let merkle_hash_points__x = FieldElement::ONE;
        let merkle_hash_points__y = FieldElement::ONE;
        let hash_pool_points__y = FieldElement::ONE;
        let hash_pool_points__x = FieldElement::ONE;

        if (i % 4 == 0) && !(i % 1024 == 1024 / 256 * 255) {
            assert_eq!(
                hash_pool__hash__ec_subset_sum__bit
                    * (hash_pool__hash__ec_subset_sum__bit - FieldElement::ONE),
                FieldElement::ZERO
            );
        }
        if (i % 1024 == 4 * 251) && !(false) {
            assert_eq!(trace_table[(8, i + 3)], FieldElement::ZERO);
        }
        if (i % 1024 == 4 * 255) && !(false) {
            assert_eq!(trace_table[(8, i + 3)], FieldElement::ZERO);
        }
        if (i % 4 == 0) && !(i % 1024 == 1024 / 256 * 255) {
            assert_eq!(
                hash_pool__hash__ec_subset_sum__bit
                    * (trace_table[(8, i + 2)] - hash_pool_points__y)
                    - trace_table[(8, i + 1)] * (trace_table[(8, i + 0)] - hash_pool_points__x),
                FieldElement::ZERO
            );
        }
        if (i % 4 == 0) && !(i % 1024 == 1024 / 256 * 255) {
            assert_eq!(
                trace_table[(8, i + 1)] * trace_table[(8, i + 1)]
                    - hash_pool__hash__ec_subset_sum__bit
                        * (trace_table[(8, i + 0)] + hash_pool_points__x + trace_table[(8, i + 4)]),
                FieldElement::ZERO
            );
        }
        if (i % 4 == 0) && !(i % 1024 == 1024 / 256 * 255) {
            assert_eq!(
                hash_pool__hash__ec_subset_sum__bit
                    * (trace_table[(8, i + 2)] + trace_table[(8, i + 6)])
                    - trace_table[(8, i + 1)] * (trace_table[(8, i + 0)] - trace_table[(8, i + 4)]),
                FieldElement::ZERO
            );
        }
        if (i % 4 == 0) && !(i % 1024 == 1024 / 256 * 255) {
            assert_eq!(
                hash_pool__hash__ec_subset_sum__bit_neg
                    * (trace_table[(8, i + 4)] - trace_table[(8, i + 0)]),
                FieldElement::ZERO
            );
        }
        if (i % 4 == 0) && !(i % 1024 == 1024 / 256 * 255) {
            assert_eq!(
                hash_pool__hash__ec_subset_sum__bit_neg
                    * (trace_table[(8, i + 6)] - trace_table[(8, i + 2)]),
                FieldElement::ZERO
            );
        }
        if (i % 1024 == 0) && !(i % 2048 == 2048 / 2) {
            assert_eq!(
                trace_table[(8, i + 1024)] - trace_table[(8, i + 1020)],
                FieldElement::ZERO
            );
        }
        if (i % 1024 == 0) && !(i % 2048 == 2048 / 2) {
            assert_eq!(
                trace_table[(8, i + 1026)] - trace_table[(8, i + 1022)],
                FieldElement::ZERO
            );
        }
        if (i % 2048 == 0) && !(false) {
            assert_eq!(trace_table[(8, i + 0)] - shift_point.x, FieldElement::ZERO);
        }
        if (i % 2048 == 0) && !(false) {
            assert_eq!(trace_table[(8, i + 2)] - shift_point.y, FieldElement::ZERO);
        }
        if (i % 4096 == 0) && !(false) {
            assert_eq!(
                trace_table[(8, i + 2044)] - trace_table[(8, i + 2051)],
                FieldElement::ZERO
            );
        }
        if (i % 512 == 0) && !(i % 16384 == 16384 / 32 * 31) {
            assert_eq!(
                state_transition__merkle_update__side_bit_extraction__bit_0
                    * state_transition__merkle_update__side_bit_extraction__bit_0
                    - state_transition__merkle_update__side_bit_extraction__bit_0,
                FieldElement::ZERO
            );
        }
        if (i % 16384 == 16384 / 32 * path_length) && !(false) {
            assert_eq!(trace_table[(6, i + 255)], FieldElement::ZERO);
        }
        if (true) && !(i % 256 == 255) {
            assert_eq!( state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit * (state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit - FieldElement::ONE), FieldElement::ZERO);
        }
        if (i % 256 == 251) && !(false) {
            assert_eq!(trace_table[(3, i + 0)], FieldElement::ZERO);
        }
        if (i % 256 == 255) && !(false) {
            assert_eq!(trace_table[(3, i + 0)], FieldElement::ZERO);
        }
        if (true) && !(i % 256 == 255) {
            assert_eq!(
                state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit
                    * (trace_table[(1, i + 0)] - merkle_hash_points__y)
                    - trace_table[(2, i + 0)] * (trace_table[(0, i + 0)] - merkle_hash_points__x),
                FieldElement::ZERO
            );
        }
        if (true) && !(i % 256 == 255) {
            assert_eq!( trace_table[(2,i + 0)] * trace_table[(2,i + 0)] - state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit * (trace_table[(0,i + 0)] + merkle_hash_points__x + trace_table[(0,i + 1)]), FieldElement::ZERO);
        }
        if (true) && !(i % 256 == 255) {
            assert_eq!(
                state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit
                    * (trace_table[(1, i + 0)] + trace_table[(1, i + 1)])
                    - trace_table[(2, i + 0)] * (trace_table[(0, i + 0)] - trace_table[(0, i + 1)]),
                FieldElement::ZERO
            );
        }
        if (true) && !(i % 256 == 255) {
            assert_eq!(
                state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit_neg
                    * (trace_table[(0, i + 1)] - trace_table[(0, i + 0)]),
                FieldElement::ZERO
            );
        }
        if (true) && !(i % 256 == 255) {
            assert_eq!(
                state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit_neg
                    * (trace_table[(1, i + 1)] - trace_table[(1, i + 0)]),
                FieldElement::ZERO
            );
        }
        if (i % 256 == 0) && !(i % 512 == 512 / 2) {
            assert_eq!(
                trace_table[(0, i + 256)] - trace_table[(0, i + 255)],
                FieldElement::ZERO
            );
        }
        if (i % 256 == 0) && !(i % 512 == 512 / 2) {
            assert_eq!(
                trace_table[(1, i + 256)] - trace_table[(1, i + 255)],
                FieldElement::ZERO
            );
        }
        if (i % 512 == 0) && !(false) {
            assert_eq!(trace_table[(0, i + 0)] - shift_point.x, FieldElement::ZERO);
        }
        if (i % 512 == 0) && !(false) {
            assert_eq!(trace_table[(1, i + 0)] - shift_point.y, FieldElement::ZERO);
        }
        if (i % 512 == 0) && !() {
            assert_eq!(
                (FieldElement::ONE - state_transition__merkle_update__side_bit_extraction__bit_1)
                    * (trace_table[(0, i + 511)] - trace_table[(3, i + 512)]),
                FieldElement::ZERO
            );
        }
        if (i % 512 == 0) && !() {
            assert_eq!(
                state_transition__merkle_update__side_bit_extraction__bit_1
                    * (trace_table[(0, i + 511)] - trace_table[(3, i + 768)]),
                FieldElement::ZERO
            );
        }
        if (true) && !(i % 256 == 255) {
            assert_eq!( state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit * (state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit - FieldElement::ONE), FieldElement::ZERO);
        }
        if (i % 256 == 251) && !(false) {
            assert_eq!(trace_table[(7, i + 0)], FieldElement::ZERO);
        }
        if (i % 256 == 255) && !(false) {
            assert_eq!(trace_table[(7, i + 0)], FieldElement::ZERO);
        }
        if (true) && !(i % 256 == 255) {
            assert_eq!(
                state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit
                    * (trace_table[(5, i + 0)] - merkle_hash_points__y)
                    - trace_table[(6, i + 0)] * (trace_table[(4, i + 0)] - merkle_hash_points__x),
                FieldElement::ZERO
            );
        }
        if (true) && !(i % 256 == 255) {
            assert_eq!( trace_table[(6,i + 0)] * trace_table[(6,i + 0)] - state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit * (trace_table[(4,i + 0)] + merkle_hash_points__x + trace_table[(4,i + 1)]), FieldElement::ZERO);
        }
        if (true) && !(i % 256 == 255) {
            assert_eq!(
                state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit
                    * (trace_table[(5, i + 0)] + trace_table[(5, i + 1)])
                    - trace_table[(6, i + 0)] * (trace_table[(4, i + 0)] - trace_table[(4, i + 1)]),
                FieldElement::ZERO
            );
        }
        if (true) && !(i % 256 == 255) {
            assert_eq!(
                state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit_neg
                    * (trace_table[(4, i + 1)] - trace_table[(4, i + 0)]),
                FieldElement::ZERO
            );
        }
        if (true) && !(i % 256 == 255) {
            assert_eq!(
                state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit_neg
                    * (trace_table[(5, i + 1)] - trace_table[(5, i + 0)]),
                FieldElement::ZERO
            );
        }
        if (i % 256 == 0) && !(i % 512 == 512 / 2) {
            assert_eq!(
                trace_table[(4, i + 256)] - trace_table[(4, i + 255)],
                FieldElement::ZERO
            );
        }
        if (i % 256 == 0) && !(i % 512 == 512 / 2) {
            assert_eq!(
                trace_table[(5, i + 256)] - trace_table[(5, i + 255)],
                FieldElement::ZERO
            );
        }
        if (i % 512 == 0) && !(false) {
            assert_eq!(trace_table[(4, i + 0)] - shift_point.x, FieldElement::ZERO);
        }
        if (i % 512 == 0) && !(false) {
            assert_eq!(trace_table[(5, i + 0)] - shift_point.y, FieldElement::ZERO);
        }
        if (i % 512 == 0) && !() {
            assert_eq!(
                (FieldElement::ONE - state_transition__merkle_update__side_bit_extraction__bit_1)
                    * (trace_table[(4, i + 511)] - trace_table[(7, i + 512)]),
                FieldElement::ZERO
            );
        }
        if (i % 512 == 0) && !() {
            assert_eq!(
                state_transition__merkle_update__side_bit_extraction__bit_1
                    * (trace_table[(4, i + 511)] - trace_table[(7, i + 768)]),
                FieldElement::ZERO
            );
        }
        if (i % 512 == 0) && !(i % 16384 == 16384 / 32 * 31) {
            assert_eq!(
                state_transition__merkle_update__prev_authentication__sibling_0
                    - state_transition__merkle_update__new_authentication__sibling_0,
                FieldElement::ZERO
            );
        }
        if (i % 16384 == 0) && !(false) {
            assert_eq!(
                state_transition__merkle_update__prev_authentication__leaf_0
                    - trace_table[(8, i + 4092)],
                FieldElement::ZERO
            );
        }
        if (i % 16384 == 0) && !(false) {
            assert_eq!(
                state_transition__merkle_update__new_authentication__leaf_0
                    - trace_table[(8, i + 12284)],
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_modification * (trace_table[(9, i + 16376)] * boundary_base - boundary_key),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_modification * (trace_table[(9, i + 16360)] * boundary_base - boundary_token),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_modification * (trace_table[(8, i + 3075)] * boundary_base - boundary_amount0),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_modification * (trace_table[(8, i + 11267)] * boundary_base - boundary_amount1),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_modification * (trace_table[(6, i + 255)] * boundary_base - boundary_vault_id),
                FieldElement::ZERO
            );
        }
        if (i % 128 == 0) && !(i % 8192 == 8192 / 64 * 63) {
            assert_eq!(
                amounts_range_check__bit_0 * amounts_range_check__bit_0
                    - amounts_range_check__bit_0,
                FieldElement::ZERO
            );
        }
        if (i % 8192 == 8192 / 64 * 63) && !(false) {
            assert_eq!(trace_table[(9, i + 4)], FieldElement::ZERO);
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_settlement
                    * (trace_table[(8, i + 3075)]
                        - trace_table[(8, i + 11267)]
                        - (trace_table[(8, i + 27651)] - trace_table[(8, i + 19459)])),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_settlement
                    * (trace_table[(8, i + 35843)]
                        - trace_table[(8, i + 44035)]
                        - (trace_table[(8, i + 60419)] - trace_table[(8, i + 52227)])),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                (trace_table[(9, i + 4)]
                    - (trace_table[(8, i + 3075)] - trace_table[(8, i + 11267)]))
                    * is_settlement,
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                (trace_table[(9, i + 32772)]
                    - (trace_table[(8, i + 35843)] - trace_table[(8, i + 44035)]))
                    * is_settlement,
                FieldElement::ZERO
            );
        }
        if (i % 16384 == 0) && !(false) {
            assert_eq!(
                trace_table[(9, i + 8196)] - trace_table[(8, i + 11267)],
                FieldElement::ZERO
            );
        }
        if (i % 64 == 0) && !(i % 16384 == 16384 / 256 * 255) {
            assert_eq!(
                sig_verify__doubling_key__x_squared
                    + sig_verify__doubling_key__x_squared
                    + sig_verify__doubling_key__x_squared
                    + sig_config.alpha
                    - (trace_table[(9, i + 32)] + trace_table[(9, i + 32)])
                        * trace_table[(9, i + 16)],
                FieldElement::ZERO
            );
        }
        if (i % 64 == 0) && !(i % 16384 == 16384 / 256 * 255) {
            assert_eq!(
                trace_table[(9, i + 16)] * trace_table[(9, i + 16)]
                    - (trace_table[(9, i + 0)]
                        + trace_table[(9, i + 0)]
                        + trace_table[(9, i + 64)]),
                FieldElement::ZERO
            );
        }
        if (i % 64 == 0) && !(i % 16384 == 16384 / 256 * 255) {
            assert_eq!(
                trace_table[(9, i + 32)] + trace_table[(9, i + 96)]
                    - trace_table[(9, i + 16)]
                        * (trace_table[(9, i + 0)] - trace_table[(9, i + 64)]),
                FieldElement::ZERO
            );
        }
        if (i % 128 == 0) && !(i % 32768 == 32768 / 256 * 256) {
            assert_eq!(
                sig_verify__exponentiate_generator__bit
                    * (sig_verify__exponentiate_generator__bit - FieldElement::ONE),
                FieldElement::ZERO
            );
        }
        if (i % 32768 == 32768 / 256 * 251) && !(false) {
            assert_eq!(trace_table[(9, i + 20)], FieldElement::ZERO);
        }
        if (i % 32768 == 32768 / 256 * 255) && !(false) {
            assert_eq!(trace_table[(9, i + 20)], FieldElement::ZERO);
        }
        if (i % 128 == 0) && !(i % 32768 == 32768 / 256 * 256) {
            assert_eq!(
                sig_verify__exponentiate_generator__bit
                    * (trace_table[(9, i + 36)] - ecdsa_points__y)
                    - trace_table[(9, i + 100)] * (trace_table[(9, i + 68)] - ecdsa_points__x),
                FieldElement::ZERO
            );
        }
        if (i % 128 == 0) && !(i % 32768 == 32768 / 256 * 256) {
            assert_eq!(
                trace_table[(9, i + 100)] * trace_table[(9, i + 100)]
                    - sig_verify__exponentiate_generator__bit
                        * (trace_table[(9, i + 68)] + ecdsa_points__x + trace_table[(9, i + 196)]),
                FieldElement::ZERO
            );
        }
        if (i % 128 == 0) && !(i % 32768 == 32768 / 256 * 256) {
            assert_eq!(
                sig_verify__exponentiate_generator__bit
                    * (trace_table[(9, i + 36)] + trace_table[(9, i + 164)])
                    - trace_table[(9, i + 100)]
                        * (trace_table[(9, i + 68)] - trace_table[(9, i + 196)]),
                FieldElement::ZERO
            );
        }
        if (i % 128 == 0) && !(i % 32768 == 32768 / 256 * 256) {
            assert_eq!(
                trace_table[(9, i + 84)] * (trace_table[(9, i + 68)] - ecdsa_points__x)
                    - FieldElement::ONE,
                FieldElement::ZERO
            );
        }
        if (i % 128 == 0) && !(i % 32768 == 32768 / 256 * 256) {
            assert_eq!(
                sig_verify__exponentiate_generator__bit_neg
                    * (trace_table[(9, i + 196)] - trace_table[(9, i + 68)]),
                FieldElement::ZERO
            );
        }
        if (i % 128 == 0) && !(i % 32768 == 32768 / 256 * 256) {
            assert_eq!(
                sig_verify__exponentiate_generator__bit_neg
                    * (trace_table[(9, i + 164)] - trace_table[(9, i + 36)]),
                FieldElement::ZERO
            );
        }
        if (i % 64 == 0) && !(i % 16384 == 16384 / 256 * 255) {
            assert_eq!(
                sig_verify__exponentiate_key__bit
                    * (sig_verify__exponentiate_key__bit - FieldElement::ONE),
                FieldElement::ZERO
            );
        }
        if (i % 16384 == 16384 / 256 * 251) && !(false) {
            assert_eq!(trace_table[(9, i + 24)], FieldElement::ZERO);
        }
        if (i % 16384 == 16384 / 256 * 255) && !(false) {
            assert_eq!(trace_table[(9, i + 24)], FieldElement::ZERO);
        }
        if (i % 64 == 0) && !(i % 16384 == 16384 / 256 * 255) {
            assert_eq!(
                sig_verify__exponentiate_key__bit
                    * (trace_table[(9, i + 8)] - trace_table[(9, i + 32)])
                    - trace_table[(9, i + 40)]
                        * (trace_table[(9, i + 48)] - trace_table[(9, i + 0)]),
                FieldElement::ZERO
            );
        }
        if (i % 64 == 0) && !(i % 16384 == 16384 / 256 * 255) {
            assert_eq!(
                trace_table[(9, i + 40)] * trace_table[(9, i + 40)]
                    - sig_verify__exponentiate_key__bit
                        * (trace_table[(9, i + 48)]
                            + trace_table[(9, i + 0)]
                            + trace_table[(9, i + 112)]),
                FieldElement::ZERO
            );
        }
        if (i % 64 == 0) && !(i % 16384 == 16384 / 256 * 255) {
            assert_eq!(
                sig_verify__exponentiate_key__bit
                    * (trace_table[(9, i + 8)] + trace_table[(9, i + 72)])
                    - trace_table[(9, i + 40)]
                        * (trace_table[(9, i + 48)] - trace_table[(9, i + 112)]),
                FieldElement::ZERO
            );
        }
        if (i % 64 == 0) && !(i % 16384 == 16384 / 256 * 255) {
            assert_eq!(
                trace_table[(9, i + 56)] * (trace_table[(9, i + 48)] - trace_table[(9, i + 0)])
                    - FieldElement::ONE,
                FieldElement::ZERO
            );
        }
        if (i % 64 == 0) && !(i % 16384 == 16384 / 256 * 255) {
            assert_eq!(
                sig_verify__exponentiate_key__bit_neg
                    * (trace_table[(9, i + 112)] - trace_table[(9, i + 48)]),
                FieldElement::ZERO
            );
        }
        if (i % 64 == 0) && !(i % 16384 == 16384 / 256 * 255) {
            assert_eq!(
                sig_verify__exponentiate_key__bit_neg
                    * (trace_table[(9, i + 72)] - trace_table[(9, i + 8)]),
                FieldElement::ZERO
            );
        }
        if (i % 32768 == 0) && !(false) {
            assert_eq!(
                trace_table[(9, i + 68)] - sig_config.shift_point.x,
                FieldElement::ZERO
            );
        }
        if (i % 32768 == 0) && !(false) {
            assert_eq!(
                trace_table[(9, i + 36)] + sig_config.shift_point.y,
                FieldElement::ZERO
            );
        }
        if (i % 16384 == 0) && !(false) {
            assert_eq!(
                trace_table[(9, i + 48)] - sig_config.shift_point.x,
                FieldElement::ZERO
            );
        }
        if (i % 16384 == 0) && !(false) {
            assert_eq!(
                trace_table[(9, i + 8)] - sig_config.shift_point.y,
                FieldElement::ZERO
            );
        }
        if (i % 32768 == 0) && !(false) {
            assert_eq!(
                trace_table[(9, i + 32676)]
                    - trace_table[(9, i + 16328)]
                    - trace_table[(9, i + 32724)]
                        * (trace_table[(9, i + 32708)] - trace_table[(9, i + 16368)]),
                FieldElement::ZERO
            );
        }
        if (i % 32768 == 0) && !(false) {
            assert_eq!(
                trace_table[(9, i + 32724)] * trace_table[(9, i + 32724)]
                    - (trace_table[(9, i + 32708)]
                        + trace_table[(9, i + 16368)]
                        + trace_table[(9, i + 16384)]),
                FieldElement::ZERO
            );
        }
        if (i % 32768 == 0) && !(false) {
            assert_eq!(
                trace_table[(9, i + 32676)] + trace_table[(9, i + 16416)]
                    - trace_table[(9, i + 32724)]
                        * (trace_table[(9, i + 32708)] - trace_table[(9, i + 16384)]),
                FieldElement::ZERO
            );
        }
        if (i % 32768 == 0) && !(false) {
            assert_eq!(
                trace_table[(9, i + 32740)]
                    * (trace_table[(9, i + 32708)] - trace_table[(9, i + 16368)])
                    - FieldElement::ONE,
                FieldElement::ZERO
            );
        }
        if (i % 32768 == 0) && !(false) {
            assert_eq!(
                trace_table[(9, i + 32712)] + sig_config.shift_point.y
                    - trace_table[(8, i + 3069)]
                        * (trace_table[(9, i + 32752)] - sig_config.shift_point.x),
                FieldElement::ZERO
            );
        }
        if (i % 32768 == 0) && !(false) {
            assert_eq!(
                trace_table[(8, i + 3069)] * trace_table[(8, i + 3069)]
                    - (trace_table[(9, i + 32752)]
                        + sig_config.shift_point.x
                        + trace_table[(9, i + 24)]),
                FieldElement::ZERO
            );
        }
        if (i % 32768 == 0) && !(false) {
            assert_eq!(
                trace_table[(8, i + 19453)]
                    * (trace_table[(9, i + 32752)] - sig_config.shift_point.x)
                    - FieldElement::ONE,
                FieldElement::ZERO
            );
        }
        if (i % 32768 == 0) && !(false) {
            assert_eq!(
                trace_table[(9, i + 20)] * trace_table[(8, i + 11261)] - FieldElement::ONE,
                FieldElement::ZERO
            );
        }
        if (i % 16384 == 0) && !(false) {
            assert_eq!(
                trace_table[(9, i + 24)] * trace_table[(9, i + 16336)] - FieldElement::ONE,
                FieldElement::ZERO
            );
        }
        if (i % 32768 == 0) && !(false) {
            assert_eq!(
                trace_table[(8, i + 27645)] - trace_table[(9, i + 0)] * trace_table[(9, i + 0)],
                FieldElement::ZERO
            );
        }
        if (i % 32768 == 0) && !(false) {
            assert_eq!(
                trace_table[(9, i + 32)] * trace_table[(9, i + 32)]
                    - (trace_table[(9, i + 0)] * trace_table[(8, i + 27645)]
                        + sig_config.alpha * trace_table[(9, i + 0)]
                        + sig_config.beta),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_settlement
                    * (trace_table[(8, i + 7171)]
                        - (((trace_table[(6, i + 255)] * vault_shift
                            + trace_table[(6, i + 49407)])
                            * amount_shift
                            + trace_table[(9, i + 4)])
                            * amount_shift
                            + trace_table[(9, i + 32772)])
                            * trade_shift),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_settlement * (trace_table[(8, i + 36867)] - trace_table[(8, i + 8188)]),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_settlement * (trace_table[(8, i + 37891)] - trace_table[(6, i + 16639)]),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_settlement * (trace_table[(8, i + 39939)] - trace_table[(6, i + 33023)]),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_settlement * (trace_table[(8, i + 8188)] - trace_table[(9, i + 20)]),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_settlement * (trace_table[(8, i + 40956)] - trace_table[(9, i + 32788)]),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_settlement * (trace_table[(9, i + 0)] - trace_table[(9, i + 16376)]),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_settlement * (trace_table[(8, i + 4099)] - trace_table[(9, i + 16360)]),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_settlement * (trace_table[(9, i + 0)] - trace_table[(9, i + 65528)]),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_settlement * (trace_table[(8, i + 5123)] - trace_table[(9, i + 65512)]),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_settlement * (trace_table[(9, i + 32768)] - trace_table[(9, i + 32760)]),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_settlement * (trace_table[(8, i + 4099)] - trace_table[(9, i + 32744)]),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_settlement * (trace_table[(9, i + 32768)] - trace_table[(9, i + 49144)]),
                FieldElement::ZERO
            );
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_settlement * (trace_table[(8, i + 5123)] - trace_table[(9, i + 49128)]),
                FieldElement::ZERO
            );
        }
        if (i % 8192 == 0) && !(false) {
            assert_eq!(
                trace_table[(8, i + 1021)] * (FieldElement::ONE - trace_table[(8, i + 1021)]),
                FieldElement::ZERO
            );
        }
        if (i % 8192 == 0) && !(false) {
            assert_eq!(
                trace_table[(8, i + 1021)] * trace_table[(8, i + 3075)],
                FieldElement::ZERO
            );
        }
        if (i % 8192 == 0) && !(false) {
            assert_eq!(
                trace_table[(8, i + 1021)] * trace_table[(8, i + 5117)],
                FieldElement::ZERO
            );
        }
        if (i % 8192 == 0) && !(false) {
            assert_eq!(
                trace_table[(8, i + 3075)] * trace_table[(8, i + 5117)]
                    - (FieldElement::ONE - trace_table[(8, i + 1021)]),
                FieldElement::ZERO
            );
        }
        if (i % 16384 == 0) && !(false) {
            assert_eq!(
                (FieldElement::ONE - trace_table[(8, i + 1021)]) * trace_table[(9, i + 16376)]
                    - trace_table[(8, i + 3)],
                FieldElement::ZERO
            );
        }
        if (i % 16384 == 0) && !(false) {
            assert_eq!(
                (FieldElement::ONE - trace_table[(8, i + 1021)]) * trace_table[(9, i + 16360)]
                    - trace_table[(8, i + 1027)],
                FieldElement::ZERO
            );
        }
        if (i % 16384 == 0) && !(false) {
            assert_eq!(
                (FieldElement::ONE - trace_table[(8, i + 9213)]) * trace_table[(9, i + 16376)]
                    - trace_table[(8, i + 8195)],
                FieldElement::ZERO
            );
        }
        if (i % 16384 == 0) && !(false) {
            assert_eq!(
                (FieldElement::ONE - trace_table[(8, i + 9213)]) * trace_table[(9, i + 16360)]
                    - trace_table[(8, i + 9219)],
                FieldElement::ZERO
            );
        }
        if (i == 0) && !(false) {
            assert_eq!(column0_row_expr0 - initial_root, FieldElement::ZERO);
        }
        if (i == trace_length - 65536) && !(false) {
            assert_eq!(column4_row_expr1 - final_root, FieldElement::ZERO);
        }
        if (i % 16384 == 0) && !(i == trace_length - 65536 + 49152) {
            assert_eq!(column4_row_expr0 - column0_row_expr2, FieldElement::ZERO);
        }
        if (i % 65536 == 0) && !(false) {
            assert_eq!(
                is_modification * (column4_row_expr0 - column4_row_expr1),
                FieldElement::ZERO
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starkware_inputs_consistent() {
        test_trace_table()
    }
}
