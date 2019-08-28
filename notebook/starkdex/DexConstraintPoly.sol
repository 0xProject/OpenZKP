/*
  Copyright 2019 StarkWare Industries Ltd.

  Licensed under the Apache License, Version 2.0 (the "License").
  You may not use this file except in compliance with the License.
  You may obtain a copy of the License at

  https://www.starkware.co/open-source-license/

  Unless required by applicable law or agreed to in writing,
  software distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions
  and limitations under the License.
*/

// ---------- The following code was auto-generated. ----------

pragma solidity ^0.5.2;

contract DexConstraintPoly {
    // The Memory map during the execution of this contract is as follows:
    // [0x0, 0x20) - periodic_column/hash_pool_points/x
    // [0x20, 0x40) - periodic_column/hash_pool_points/y
    // [0x40, 0x60) - periodic_column/merkle_hash_points/x
    // [0x60, 0x80) - periodic_column/merkle_hash_points/y
    // [0x80, 0xa0) - periodic_column/boundary_base
    // [0xa0, 0xc0) - periodic_column/is_modification
    // [0xc0, 0xe0) - periodic_column/is_settlement
    // [0xe0, 0x100) - periodic_column/boundary_key
    // [0x100, 0x120) - periodic_column/boundary_token
    // [0x120, 0x140) - periodic_column/boundary_amount0
    // [0x140, 0x160) - periodic_column/boundary_amount1
    // [0x160, 0x180) - periodic_column/boundary_vault_id
    // [0x180, 0x1a0) - periodic_column/ecdsa_points/x
    // [0x1a0, 0x1c0) - periodic_column/ecdsa_points/y
    // [0x1c0, 0x1e0) - trace_length
    // [0x1e0, 0x200) - shift_point.x
    // [0x200, 0x220) - shift_point.y
    // [0x220, 0x240) - vaults_path_length
    // [0x240, 0x260) - sig_config.alpha
    // [0x260, 0x280) - sig_config.beta
    // [0x280, 0x2a0) - n_modifications
    // [0x2a0, 0x2c0) - initial_vaults_root
    // [0x2c0, 0x2e0) - final_vaults_root
    // [0x2e0, 0x300) - n_settlements
    // [0x300, 0x320) - vault_shift
    // [0x320, 0x340) - amount_shift
    // [0x340, 0x360) - trade_shift
    // [0x360, 0x380) - trace_generator
    // [0x380, 0x3a0) - oods_point
    // [0x3a0, 0x2220) - coefficients
    // [0x2220, 0x3240) - oods_values
    // ----------------------- end of input data - -------------------------
    // [0x3240, 0x3260) - composition_degree_bound
    // [0x3260, 0x3280) - intermediate_value/hash_pool/hash/ec_subset_sum/bit
    // [0x3280, 0x32a0) - intermediate_value/hash_pool/hash/ec_subset_sum/bit_neg
    // [0x32a0, 0x32c0) - intermediate_value/state_transition/merkle_update/side_bit_extraction/bit_0
    // [0x32c0, 0x32e0) - intermediate_value/state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit
    // [0x32e0, 0x3300) - intermediate_value/state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit_neg
    // [0x3300, 0x3320) - intermediate_value/state_transition/merkle_update/side_bit_extraction/bit_1
    // [0x3320, 0x3340) - intermediate_value/state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit
    // [0x3340, 0x3360) - intermediate_value/state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit_neg
    // [0x3360, 0x3380) - intermediate_value/state_transition/merkle_update/prev_authentication/sibling_0
    // [0x3380, 0x33a0) - intermediate_value/state_transition/merkle_update/new_authentication/sibling_0
    // [0x33a0, 0x33c0) - intermediate_value/state_transition/merkle_update/prev_authentication/leaf_0
    // [0x33c0, 0x33e0) - intermediate_value/state_transition/merkle_update/new_authentication/leaf_0
    // [0x33e0, 0x3400) - intermediate_value/amounts_range_check/bit_0
    // [0x3400, 0x3420) - intermediate_value/settlement_id_range_check/bit_0
    // [0x3420, 0x3440) - intermediate_value/sig_verify/doubling_key/x_squared
    // [0x3440, 0x3460) - intermediate_value/sig_verify/exponentiate_generator/bit
    // [0x3460, 0x3480) - intermediate_value/sig_verify/exponentiate_generator/bit_neg
    // [0x3480, 0x34a0) - intermediate_value/sig_verify/exponentiate_key/bit
    // [0x34a0, 0x34c0) - intermediate_value/sig_verify/exponentiate_key/bit_neg
    // [0x34c0, 0x34e0) - intermediate_value/packed_message
    // [0x34e0, 0x37a0) - expmods
    // [0x37a0, 0x3ae0) - denominator_invs
    // [0x3ae0, 0x3e20) - denominators
    // [0x3e20, 0x3f80) - numerators
    // [0x3f80, 0x4300) - adjustments
    // [0x4300, 0x43c0) - expmod_context

    function() external {
        uint256 res;
        assembly {
            let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001
            // Copy input from calldata to memory.
            calldatacopy(0x0, 0x0, /*inputDataSize*/ 0x3240)
            let point := /*oods_point*/ mload(0x380)
            // Initialize compositionDegreeBound to 2 * trace_length.
            mstore(0x3240, mul(2, /*trace_length*/ mload(0x1c0)))
            function expmod(base, exponent, modulus) -> res {
              let p := /*expmod_context*/ 0x4300
              mstore(p, 0x20)                 // Length of Base
              mstore(add(p, 0x20), 0x20)      // Length of Exponent
              mstore(add(p, 0x40), 0x20)      // Length of Modulus
              mstore(add(p, 0x60), base)      // Base
              mstore(add(p, 0x80), exponent)  // Exponent
              mstore(add(p, 0xa0), modulus)   // Modulus
              // Call modexp precompile.
              if iszero(staticcall(not(0), 0x05, p, 0xc0, p, 0x20)) {
                revert(0, 0)
              }
              res := mload(p)
            }

            function degreeAdjustment(compositionPolynomialDegreeBound, constraintDegree, numeratorDegree,
                                       denominatorDegree) -> res {
              res := sub(sub(compositionPolynomialDegreeBound, 1),
                         sub(add(constraintDegree, numeratorDegree), denominatorDegree))
            }

            {
              // Prepare expmods for denominators and numerators.

              // expmods[0] = point^(trace_length / 4)
              mstore(0x34e0, expmod(point, div(/*trace_length*/ mload(0x1c0), 4), PRIME))

              // expmods[1] = point^(trace_length / 1024)
              mstore(0x3500, expmod(point, div(/*trace_length*/ mload(0x1c0), 1024), PRIME))

              // expmods[2] = point^(trace_length / 2048)
              mstore(0x3520, expmod(point, div(/*trace_length*/ mload(0x1c0), 2048), PRIME))

              // expmods[3] = point^(trace_length / 4096)
              mstore(0x3540, expmod(point, div(/*trace_length*/ mload(0x1c0), 4096), PRIME))

              // expmods[4] = point^(trace_length / 512)
              mstore(0x3560, expmod(point, div(/*trace_length*/ mload(0x1c0), 512), PRIME))

              // expmods[5] = point^(trace_length / 16384)
              mstore(0x3580, expmod(point, div(/*trace_length*/ mload(0x1c0), 16384), PRIME))

              // expmods[6] = point^trace_length
              mstore(0x35a0, expmod(point, /*trace_length*/ mload(0x1c0), PRIME))

              // expmods[7] = point^(trace_length / 256)
              mstore(0x35c0, expmod(point, div(/*trace_length*/ mload(0x1c0), 256), PRIME))

              // expmods[8] = point^(trace_length / 65536)
              mstore(0x35e0, expmod(point, div(/*trace_length*/ mload(0x1c0), 65536), PRIME))

              // expmods[9] = point^(trace_length / 128)
              mstore(0x3600, expmod(point, div(/*trace_length*/ mload(0x1c0), 128), PRIME))

              // expmods[10] = point^(trace_length / 8192)
              mstore(0x3620, expmod(point, div(/*trace_length*/ mload(0x1c0), 8192), PRIME))

              // expmods[11] = point^(trace_length / 64)
              mstore(0x3640, expmod(point, div(/*trace_length*/ mload(0x1c0), 64), PRIME))

              // expmods[12] = point^(trace_length / 32768)
              mstore(0x3660, expmod(point, div(/*trace_length*/ mload(0x1c0), 32768), PRIME))

              // expmods[13] = trace_generator^(255 * trace_length / 256)
              mstore(0x3680, expmod(/*trace_generator*/ mload(0x360), div(mul(255, /*trace_length*/ mload(0x1c0)), 256), PRIME))

              // expmods[14] = trace_generator^(63 * trace_length / 64)
              mstore(0x36a0, expmod(/*trace_generator*/ mload(0x360), div(mul(63, /*trace_length*/ mload(0x1c0)), 64), PRIME))

              // expmods[15] = trace_generator^(trace_length / 2)
              mstore(0x36c0, expmod(/*trace_generator*/ mload(0x360), div(/*trace_length*/ mload(0x1c0), 2), PRIME))

              // expmods[16] = trace_generator^(31 * trace_length / 32)
              mstore(0x36e0, expmod(/*trace_generator*/ mload(0x360), div(mul(31, /*trace_length*/ mload(0x1c0)), 32), PRIME))

              // expmods[17] = trace_generator^(vaults_path_length * trace_length / 32)
              mstore(0x3700, expmod(/*trace_generator*/ mload(0x360), div(mul(/*vaults_path_length*/ mload(0x220), /*trace_length*/ mload(0x1c0)), 32), PRIME))

              // expmods[18] = trace_generator^(15 * trace_length / 16)
              mstore(0x3720, expmod(/*trace_generator*/ mload(0x360), div(mul(15, /*trace_length*/ mload(0x1c0)), 16), PRIME))

              // expmods[19] = trace_generator^(251 * trace_length / 256)
              mstore(0x3740, expmod(/*trace_generator*/ mload(0x360), div(mul(251, /*trace_length*/ mload(0x1c0)), 256), PRIME))

              // expmods[20] = trace_generator^(65536 * (trace_length / 65536 - 1))
              mstore(0x3760, expmod(/*trace_generator*/ mload(0x360), mul(65536, sub(div(/*trace_length*/ mload(0x1c0), 65536), 1)), PRIME))

              // expmods[21] = trace_generator^(65536 * (trace_length / 65536 - 1) + 49152)
              mstore(0x3780, expmod(/*trace_generator*/ mload(0x360), add(mul(65536, sub(div(/*trace_length*/ mload(0x1c0), 65536), 1)), 49152), PRIME))
            }

            {
              // Prepare denominators for batch inverse.

              // Denominator for constraints: 'hash_pool/hash/ec_subset_sum/booleanity_test', 'hash_pool/hash/ec_subset_sum/add_points/slope', 'hash_pool/hash/ec_subset_sum/add_points/x', 'hash_pool/hash/ec_subset_sum/add_points/y', 'hash_pool/hash/ec_subset_sum/copy_point/x', 'hash_pool/hash/ec_subset_sum/copy_point/y'.
              // denominators[0] = point^(trace_length / 4) - 1
              mstore(0x3ae0,
                     addmod(/*point^(trace_length / 4)*/ mload(0x34e0), sub(PRIME, 1), PRIME))

              // Denominator for constraints: 'hash_pool/hash/ec_subset_sum/bit_extraction_end'.
              // denominators[1] = point^(trace_length / 1024) - trace_generator^(63 * trace_length / 64)
              mstore(0x3b00,
                     addmod(
                       /*point^(trace_length / 1024)*/ mload(0x3500),
                       sub(PRIME, /*trace_generator^(63 * trace_length / 64)*/ mload(0x36a0)),
                       PRIME))

              // Denominator for constraints: 'hash_pool/hash/ec_subset_sum/zeros_tail'.
              // denominators[2] = point^(trace_length / 1024) - trace_generator^(255 * trace_length / 256)
              mstore(0x3b20,
                     addmod(
                       /*point^(trace_length / 1024)*/ mload(0x3500),
                       sub(PRIME, /*trace_generator^(255 * trace_length / 256)*/ mload(0x3680)),
                       PRIME))

              // Denominator for constraints: 'hash_pool/hash/copy_point/x', 'hash_pool/hash/copy_point/y'.
              // denominators[3] = point^(trace_length / 1024) - 1
              mstore(0x3b40,
                     addmod(/*point^(trace_length / 1024)*/ mload(0x3500), sub(PRIME, 1), PRIME))

              // Denominator for constraints: 'hash_pool/hash/init/x', 'hash_pool/hash/init/y', 'settlement_id_range_check/bit'.
              // denominators[4] = point^(trace_length / 2048) - 1
              mstore(0x3b60,
                     addmod(/*point^(trace_length / 2048)*/ mload(0x3520), sub(PRIME, 1), PRIME))

              // Denominator for constraints: 'hash_pool/output_to_input'.
              // denominators[5] = point^(trace_length / 4096) - 1
              mstore(0x3b80,
                     addmod(/*point^(trace_length / 4096)*/ mload(0x3540), sub(PRIME, 1), PRIME))

              // Denominator for constraints: 'state_transition/merkle_update/side_bit_extraction/bit', 'state_transition/merkle_update/prev_authentication/hashes/init/x', 'state_transition/merkle_update/prev_authentication/hashes/init/y', 'state_transition/merkle_update/prev_authentication/copy_prev_to_left', 'state_transition/merkle_update/prev_authentication/copy_prev_to_right', 'state_transition/merkle_update/new_authentication/hashes/init/x', 'state_transition/merkle_update/new_authentication/hashes/init/y', 'state_transition/merkle_update/new_authentication/copy_prev_to_left', 'state_transition/merkle_update/new_authentication/copy_prev_to_right', 'state_transition/merkle_update/same_siblings'.
              // denominators[6] = point^(trace_length / 512) - 1
              mstore(0x3ba0,
                     addmod(/*point^(trace_length / 512)*/ mload(0x3560), sub(PRIME, 1), PRIME))

              // Denominator for constraints: 'state_transition/merkle_update/side_bit_extraction/zero'.
              // denominators[7] = point^(trace_length / 16384) - trace_generator^(vaults_path_length * trace_length / 32)
              mstore(0x3bc0,
                     addmod(
                       /*point^(trace_length / 16384)*/ mload(0x3580),
                       sub(PRIME, /*trace_generator^(vaults_path_length * trace_length / 32)*/ mload(0x3700)),
                       PRIME))

              // Denominator for constraints: 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/booleanity_test', 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/slope', 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/x', 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/y', 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/copy_point/x', 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/copy_point/y', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/booleanity_test', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/slope', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/x', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/y', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/copy_point/x', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/copy_point/y'.
              // denominators[8] = point^trace_length - 1
              mstore(0x3be0,
                     addmod(/*point^trace_length*/ mload(0x35a0), sub(PRIME, 1), PRIME))

              // Denominator for constraints: 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit_extraction_end', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit_extraction_end'.
              // denominators[9] = point^(trace_length / 256) - trace_generator^(63 * trace_length / 64)
              mstore(0x3c00,
                     addmod(
                       /*point^(trace_length / 256)*/ mload(0x35c0),
                       sub(PRIME, /*trace_generator^(63 * trace_length / 64)*/ mload(0x36a0)),
                       PRIME))

              // Denominator for constraints: 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/zeros_tail', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/zeros_tail'.
              // denominators[10] = point^(trace_length / 256) - trace_generator^(255 * trace_length / 256)
              mstore(0x3c20,
                     addmod(
                       /*point^(trace_length / 256)*/ mload(0x35c0),
                       sub(PRIME, /*trace_generator^(255 * trace_length / 256)*/ mload(0x3680)),
                       PRIME))

              // Denominator for constraints: 'state_transition/merkle_update/prev_authentication/hashes/copy_point/x', 'state_transition/merkle_update/prev_authentication/hashes/copy_point/y', 'state_transition/merkle_update/new_authentication/hashes/copy_point/x', 'state_transition/merkle_update/new_authentication/hashes/copy_point/y'.
              // denominators[11] = point^(trace_length / 256) - 1
              mstore(0x3c40,
                     addmod(/*point^(trace_length / 256)*/ mload(0x35c0), sub(PRIME, 1), PRIME))

              // Denominator for constraints: 'state_transition/merkle_set_prev_leaf', 'state_transition/merkle_set_new_leaf', 'amounts_range_check_inputs', 'sig_verify/init_key/x', 'sig_verify/init_key/y', 'sig_verify/r_and_w_nonzero', 'handle_empty_vault/consistency_key_stage0', 'handle_empty_vault/consistency_token_stage0', 'handle_empty_vault/consistency_key_stage1', 'handle_empty_vault/consistency_token_stage1', 'copy_merkle_roots'.
              // denominators[12] = point^(trace_length / 16384) - 1
              mstore(0x3c60,
                     addmod(/*point^(trace_length / 16384)*/ mload(0x3580), sub(PRIME, 1), PRIME))

              // Denominator for constraints: 'modification_boundary_key', 'modification_boundary_token', 'modification_boundary_amount0', 'modification_boundary_amount1', 'modification_boundary_vault_id', 'total_token_a_not_changed', 'total_token_b_not_changed', 'diff_a_range_check_input', 'diff_b_range_check_input', 'maker_sig_input_packed', 'taker_sig_input_maker_hash', 'taker_sig_input_vault_a', 'taker_sig_input_vault_b', 'copy_signature_input_maker', 'copy_signature_input_taker', 'handle_empty_vault/consistency_key_change0', 'handle_empty_vault/consistency_token_change0', 'handle_empty_vault/consistency_key_change3', 'handle_empty_vault/consistency_token_change3', 'handle_empty_vault/consistency_key_change1', 'handle_empty_vault/consistency_token_change1', 'handle_empty_vault/consistency_key_change2', 'handle_empty_vault/consistency_token_change2', 'copy_merkle_roots_modification'.
              // denominators[13] = point^(trace_length / 65536) - 1
              mstore(0x3c80,
                     addmod(/*point^(trace_length / 65536)*/ mload(0x35e0), sub(PRIME, 1), PRIME))

              // Denominator for constraints: 'amounts_range_check/bit', 'sig_verify/exponentiate_generator/booleanity_test', 'sig_verify/exponentiate_generator/add_points/slope', 'sig_verify/exponentiate_generator/add_points/x', 'sig_verify/exponentiate_generator/add_points/y', 'sig_verify/exponentiate_generator/add_points/x_diff_inv', 'sig_verify/exponentiate_generator/copy_point/x', 'sig_verify/exponentiate_generator/copy_point/y'.
              // denominators[14] = point^(trace_length / 128) - 1
              mstore(0x3ca0,
                     addmod(/*point^(trace_length / 128)*/ mload(0x3600), sub(PRIME, 1), PRIME))

              // Denominator for constraints: 'amounts_range_check/zero'.
              // denominators[15] = point^(trace_length / 8192) - trace_generator^(63 * trace_length / 64)
              mstore(0x3cc0,
                     addmod(
                       /*point^(trace_length / 8192)*/ mload(0x3620),
                       sub(PRIME, /*trace_generator^(63 * trace_length / 64)*/ mload(0x36a0)),
                       PRIME))

              // Denominator for constraints: 'settlement_id_range_check/zero'.
              // denominators[16] = point^(trace_length / 65536) - trace_generator^(31 * trace_length / 32)
              mstore(0x3ce0,
                     addmod(
                       /*point^(trace_length / 65536)*/ mload(0x35e0),
                       sub(PRIME, /*trace_generator^(31 * trace_length / 32)*/ mload(0x36e0)),
                       PRIME))

              // Denominator for constraints: 'sig_verify/doubling_key/slope', 'sig_verify/doubling_key/x', 'sig_verify/doubling_key/y', 'sig_verify/exponentiate_key/booleanity_test', 'sig_verify/exponentiate_key/add_points/slope', 'sig_verify/exponentiate_key/add_points/x', 'sig_verify/exponentiate_key/add_points/y', 'sig_verify/exponentiate_key/add_points/x_diff_inv', 'sig_verify/exponentiate_key/copy_point/x', 'sig_verify/exponentiate_key/copy_point/y'.
              // denominators[17] = point^(trace_length / 64) - 1
              mstore(0x3d00,
                     addmod(/*point^(trace_length / 64)*/ mload(0x3640), sub(PRIME, 1), PRIME))

              // Denominator for constraints: 'sig_verify/exponentiate_generator/bit_extraction_end'.
              // denominators[18] = point^(trace_length / 32768) - trace_generator^(251 * trace_length / 256)
              mstore(0x3d20,
                     addmod(
                       /*point^(trace_length / 32768)*/ mload(0x3660),
                       sub(PRIME, /*trace_generator^(251 * trace_length / 256)*/ mload(0x3740)),
                       PRIME))

              // Denominator for constraints: 'sig_verify/exponentiate_generator/zeros_tail'.
              // denominators[19] = point^(trace_length / 32768) - trace_generator^(255 * trace_length / 256)
              mstore(0x3d40,
                     addmod(
                       /*point^(trace_length / 32768)*/ mload(0x3660),
                       sub(PRIME, /*trace_generator^(255 * trace_length / 256)*/ mload(0x3680)),
                       PRIME))

              // Denominator for constraints: 'sig_verify/exponentiate_key/bit_extraction_end'.
              // denominators[20] = point^(trace_length / 16384) - trace_generator^(251 * trace_length / 256)
              mstore(0x3d60,
                     addmod(
                       /*point^(trace_length / 16384)*/ mload(0x3580),
                       sub(PRIME, /*trace_generator^(251 * trace_length / 256)*/ mload(0x3740)),
                       PRIME))

              // Denominator for constraints: 'sig_verify/exponentiate_key/zeros_tail'.
              // denominators[21] = point^(trace_length / 16384) - trace_generator^(255 * trace_length / 256)
              mstore(0x3d80,
                     addmod(
                       /*point^(trace_length / 16384)*/ mload(0x3580),
                       sub(PRIME, /*trace_generator^(255 * trace_length / 256)*/ mload(0x3680)),
                       PRIME))

              // Denominator for constraints: 'sig_verify/init_gen/x', 'sig_verify/init_gen/y', 'sig_verify/add_results/slope', 'sig_verify/add_results/x', 'sig_verify/add_results/y', 'sig_verify/add_results/x_diff_inv', 'sig_verify/extract_r/slope', 'sig_verify/extract_r/x', 'sig_verify/extract_r/x_diff_inv', 'sig_verify/z_nonzero', 'sig_verify/q_on_curve/x_squared', 'sig_verify/q_on_curve/on_curve'.
              // denominators[22] = point^(trace_length / 32768) - 1
              mstore(0x3da0,
                     addmod(/*point^(trace_length / 32768)*/ mload(0x3660), sub(PRIME, 1), PRIME))

              // Denominator for constraints: 'handle_empty_vault/vault_empty/empty_vault_booleanity', 'handle_empty_vault/vault_empty/amount_zero_when_empty', 'handle_empty_vault/vault_empty/amount_inv_zero_when_empty', 'handle_empty_vault/vault_empty/empty_when_amount_zero'.
              // denominators[23] = point^(trace_length / 8192) - 1
              mstore(0x3dc0,
                     addmod(/*point^(trace_length / 8192)*/ mload(0x3620), sub(PRIME, 1), PRIME))

              // Denominator for constraints: 'initial_vaults_root'.
              // denominators[24] = point - 1
              mstore(0x3de0,
                     addmod(point, sub(PRIME, 1), PRIME))

              // Denominator for constraints: 'final_vaults_root'.
              // denominators[25] = point - trace_generator^(65536 * (trace_length / 65536 - 1))
              mstore(0x3e00,
                     addmod(
                       point,
                       sub(PRIME, /*trace_generator^(65536 * (trace_length / 65536 - 1))*/ mload(0x3760)),
                       PRIME))
            }

            {
              // Compute the inverses of the denominators into denominatorInvs using batch inverse.

              // Start by computing the cumulative product.
              // Let (d_0, d_1, d_2, ..., d_{n-1}) be the values in denominators. After this loop
              // denominatorInvs will be (1, d_0, d_0 * d_1, ...) and prod will contain the value of
              // d_0 * ... * d_{n-1}.
              // Compute the offset between the partialProducts array and the input values array.
              let productsToValuesOffset := 0x340
              let prod := 1
              let partialProductEndPtr := 0x3ae0
              for { let partialProductPtr := 0x37a0 }
                  lt(partialProductPtr, partialProductEndPtr)
                  { partialProductPtr := add(partialProductPtr, 0x20) } {
                  mstore(partialProductPtr, prod)
                  // prod *= d_{i}.
                  prod := mulmod(prod,
                                 mload(add(partialProductPtr, productsToValuesOffset)),
                                 PRIME)
              }

              let firstPartialProductPtr := 0x37a0
              // Compute the inverse of the product.
              let prodInv := expmod(prod, sub(PRIME, 2), PRIME)

              if eq(prodInv, 0) {
                  // Solidity generates reverts with reason that look as follows:
                  // 1. 4 bytes with the constant 0x08c379a0 (== Keccak256(b'Error(string)')[:4]).
                  // 2. 32 bytes offset bytes (always 0x20 as far as i can tell).
                  // 3. 32 bytes with the length of the revert reason.
                  // 4. Revert reason string.

                  mstore(0, 0x08c379a000000000000000000000000000000000000000000000000000000000)
                  mstore(0x4, 0x20)
                  mstore(0x24, 0x1e)
                  mstore(0x44, "Batch inverse product is zero.")
                  revert(0, 0x62)
              }

              // Compute the inverses.
              // Loop over denominator_invs in reverse order.
              // currentPartialProductPtr is initialized to one past the end.
              let currentPartialProductPtr := 0x3ae0
              for { } gt(currentPartialProductPtr, firstPartialProductPtr) { } {
                  currentPartialProductPtr := sub(currentPartialProductPtr, 0x20)
                  // Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
                  mstore(currentPartialProductPtr,
                         mulmod(mload(currentPartialProductPtr), prodInv, PRIME))
                  // Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
                  prodInv := mulmod(prodInv,
                                     mload(add(currentPartialProductPtr, productsToValuesOffset)),
                                     PRIME)
              }
            }

            {
              // Compute numerators and adjustment polynomials.

              // Numerator for constraints 'hash_pool/hash/ec_subset_sum/booleanity_test', 'hash_pool/hash/ec_subset_sum/add_points/slope', 'hash_pool/hash/ec_subset_sum/add_points/x', 'hash_pool/hash/ec_subset_sum/add_points/y', 'hash_pool/hash/ec_subset_sum/copy_point/x', 'hash_pool/hash/ec_subset_sum/copy_point/y'.
              // numerators[0] = point^(trace_length / 1024) - trace_generator^(255 * trace_length / 256)
              mstore(0x3e20,
                     addmod(
                       /*point^(trace_length / 1024)*/ mload(0x3500),
                       sub(PRIME, /*trace_generator^(255 * trace_length / 256)*/ mload(0x3680)),
                       PRIME))

              // Numerator for constraints 'hash_pool/hash/copy_point/x', 'hash_pool/hash/copy_point/y'.
              // numerators[1] = point^(trace_length / 2048) - trace_generator^(trace_length / 2)
              mstore(0x3e40,
                     addmod(
                       /*point^(trace_length / 2048)*/ mload(0x3520),
                       sub(PRIME, /*trace_generator^(trace_length / 2)*/ mload(0x36c0)),
                       PRIME))

              // Numerator for constraints 'state_transition/merkle_update/side_bit_extraction/bit', 'state_transition/merkle_update/same_siblings'.
              // numerators[2] = point^(trace_length / 16384) - trace_generator^(31 * trace_length / 32)
              mstore(0x3e60,
                     addmod(
                       /*point^(trace_length / 16384)*/ mload(0x3580),
                       sub(PRIME, /*trace_generator^(31 * trace_length / 32)*/ mload(0x36e0)),
                       PRIME))

              // Numerator for constraints 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/booleanity_test', 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/slope', 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/x', 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/y', 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/copy_point/x', 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/copy_point/y', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/booleanity_test', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/slope', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/x', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/y', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/copy_point/x', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/copy_point/y'.
              // numerators[3] = point^(trace_length / 256) - trace_generator^(255 * trace_length / 256)
              mstore(0x3e80,
                     addmod(
                       /*point^(trace_length / 256)*/ mload(0x35c0),
                       sub(PRIME, /*trace_generator^(255 * trace_length / 256)*/ mload(0x3680)),
                       PRIME))

              // Numerator for constraints 'state_transition/merkle_update/prev_authentication/hashes/copy_point/x', 'state_transition/merkle_update/prev_authentication/hashes/copy_point/y', 'state_transition/merkle_update/new_authentication/hashes/copy_point/x', 'state_transition/merkle_update/new_authentication/hashes/copy_point/y'.
              // numerators[4] = point^(trace_length / 512) - trace_generator^(trace_length / 2)
              mstore(0x3ea0,
                     addmod(
                       /*point^(trace_length / 512)*/ mload(0x3560),
                       sub(PRIME, /*trace_generator^(trace_length / 2)*/ mload(0x36c0)),
                       PRIME))

              // Numerator for constraints 'state_transition/merkle_update/prev_authentication/copy_prev_to_left', 'state_transition/merkle_update/prev_authentication/copy_prev_to_right', 'state_transition/merkle_update/new_authentication/copy_prev_to_left', 'state_transition/merkle_update/new_authentication/copy_prev_to_right'.
              // numerators[5] = (point^(trace_length / 16384) - trace_generator^(31 * trace_length / 32)) * (point^(trace_length / 16384) - trace_generator^(15 * trace_length / 16))
              mstore(0x3ec0,
                     mulmod(
                       addmod(
                         /*point^(trace_length / 16384)*/ mload(0x3580),
                         sub(PRIME, /*trace_generator^(31 * trace_length / 32)*/ mload(0x36e0)),
                         PRIME),
                       addmod(
                         /*point^(trace_length / 16384)*/ mload(0x3580),
                         sub(PRIME, /*trace_generator^(15 * trace_length / 16)*/ mload(0x3720)),
                         PRIME),
                       PRIME))

              // Numerator for constraints 'amounts_range_check/bit'.
              // numerators[6] = point^(trace_length / 8192) - trace_generator^(63 * trace_length / 64)
              mstore(0x3ee0,
                     addmod(
                       /*point^(trace_length / 8192)*/ mload(0x3620),
                       sub(PRIME, /*trace_generator^(63 * trace_length / 64)*/ mload(0x36a0)),
                       PRIME))

              // Numerator for constraints 'settlement_id_range_check/bit'.
              // numerators[7] = point^(trace_length / 65536) - trace_generator^(31 * trace_length / 32)
              mstore(0x3f00,
                     addmod(
                       /*point^(trace_length / 65536)*/ mload(0x35e0),
                       sub(PRIME, /*trace_generator^(31 * trace_length / 32)*/ mload(0x36e0)),
                       PRIME))

              // Numerator for constraints 'sig_verify/doubling_key/slope', 'sig_verify/doubling_key/x', 'sig_verify/doubling_key/y', 'sig_verify/exponentiate_key/booleanity_test', 'sig_verify/exponentiate_key/add_points/slope', 'sig_verify/exponentiate_key/add_points/x', 'sig_verify/exponentiate_key/add_points/y', 'sig_verify/exponentiate_key/add_points/x_diff_inv', 'sig_verify/exponentiate_key/copy_point/x', 'sig_verify/exponentiate_key/copy_point/y'.
              // numerators[8] = point^(trace_length / 16384) - trace_generator^(255 * trace_length / 256)
              mstore(0x3f20,
                     addmod(
                       /*point^(trace_length / 16384)*/ mload(0x3580),
                       sub(PRIME, /*trace_generator^(255 * trace_length / 256)*/ mload(0x3680)),
                       PRIME))

              // Numerator for constraints 'sig_verify/exponentiate_generator/booleanity_test', 'sig_verify/exponentiate_generator/add_points/slope', 'sig_verify/exponentiate_generator/add_points/x', 'sig_verify/exponentiate_generator/add_points/y', 'sig_verify/exponentiate_generator/add_points/x_diff_inv', 'sig_verify/exponentiate_generator/copy_point/x', 'sig_verify/exponentiate_generator/copy_point/y'.
              // numerators[9] = point^(trace_length / 32768) - trace_generator^(255 * trace_length / 256)
              mstore(0x3f40,
                     addmod(
                       /*point^(trace_length / 32768)*/ mload(0x3660),
                       sub(PRIME, /*trace_generator^(255 * trace_length / 256)*/ mload(0x3680)),
                       PRIME))

              // Numerator for constraints 'copy_merkle_roots'.
              // numerators[10] = point - trace_generator^(65536 * (trace_length / 65536 - 1) + 49152)
              mstore(0x3f60,
                     addmod(
                       point,
                       sub(
                         PRIME,
                         /*trace_generator^(65536 * (trace_length / 65536 - 1) + 49152)*/ mload(0x3780)),
                       PRIME))

              // Adjustment polynomial for constraints 'hash_pool/hash/ec_subset_sum/booleanity_test', 'hash_pool/hash/ec_subset_sum/add_points/slope', 'hash_pool/hash/ec_subset_sum/add_points/x', 'hash_pool/hash/ec_subset_sum/add_points/y', 'hash_pool/hash/ec_subset_sum/copy_point/x', 'hash_pool/hash/ec_subset_sum/copy_point/y'.
              // adjustments[0] = point^degreeAdjustment(composition_degree_bound, 2 * (trace_length - 1), trace_length / 1024, trace_length / 4)
              mstore(0x3f80,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), mul(2, sub(/*trace_length*/ mload(0x1c0), 1)), div(/*trace_length*/ mload(0x1c0), 1024), div(/*trace_length*/ mload(0x1c0), 4)), PRIME))

              // Adjustment polynomial for constraints 'hash_pool/hash/ec_subset_sum/bit_extraction_end', 'hash_pool/hash/ec_subset_sum/zeros_tail'.
              // adjustments[1] = point^degreeAdjustment(composition_degree_bound, trace_length - 1, 0, trace_length / 1024)
              mstore(0x3fa0,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), sub(/*trace_length*/ mload(0x1c0), 1), 0, div(/*trace_length*/ mload(0x1c0), 1024)), PRIME))

              // Adjustment polynomial for constraints 'hash_pool/hash/copy_point/x', 'hash_pool/hash/copy_point/y'.
              // adjustments[2] = point^degreeAdjustment(composition_degree_bound, trace_length - 1, trace_length / 2048, trace_length / 1024)
              mstore(0x3fc0,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), sub(/*trace_length*/ mload(0x1c0), 1), div(/*trace_length*/ mload(0x1c0), 2048), div(/*trace_length*/ mload(0x1c0), 1024)), PRIME))

              // Adjustment polynomial for constraints 'hash_pool/hash/init/x', 'hash_pool/hash/init/y'.
              // adjustments[3] = point^degreeAdjustment(composition_degree_bound, trace_length - 1, 0, trace_length / 2048)
              mstore(0x3fe0,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), sub(/*trace_length*/ mload(0x1c0), 1), 0, div(/*trace_length*/ mload(0x1c0), 2048)), PRIME))

              // Adjustment polynomial for constraints 'hash_pool/output_to_input'.
              // adjustments[4] = point^degreeAdjustment(composition_degree_bound, trace_length - 1, 0, trace_length / 4096)
              mstore(0x4000,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), sub(/*trace_length*/ mload(0x1c0), 1), 0, div(/*trace_length*/ mload(0x1c0), 4096)), PRIME))

              // Adjustment polynomial for constraints 'state_transition/merkle_update/side_bit_extraction/bit', 'state_transition/merkle_update/same_siblings'.
              // adjustments[5] = point^degreeAdjustment(composition_degree_bound, 2 * (trace_length - 1), trace_length / 16384, trace_length / 512)
              mstore(0x4020,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), mul(2, sub(/*trace_length*/ mload(0x1c0), 1)), div(/*trace_length*/ mload(0x1c0), 16384), div(/*trace_length*/ mload(0x1c0), 512)), PRIME))

              // Adjustment polynomial for constraints 'state_transition/merkle_update/side_bit_extraction/zero', 'amounts_range_check_inputs', 'sig_verify/exponentiate_key/bit_extraction_end', 'sig_verify/exponentiate_key/zeros_tail', 'sig_verify/init_key/x', 'sig_verify/init_key/y'.
              // adjustments[6] = point^degreeAdjustment(composition_degree_bound, trace_length - 1, 0, trace_length / 16384)
              mstore(0x4040,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), sub(/*trace_length*/ mload(0x1c0), 1), 0, div(/*trace_length*/ mload(0x1c0), 16384)), PRIME))

              // Adjustment polynomial for constraints 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/booleanity_test', 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/slope', 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/x', 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/y', 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/copy_point/x', 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/copy_point/y', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/booleanity_test', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/slope', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/x', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/y', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/copy_point/x', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/copy_point/y'.
              // adjustments[7] = point^degreeAdjustment(composition_degree_bound, 2 * (trace_length - 1), trace_length / 256, trace_length)
              mstore(0x4060,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), mul(2, sub(/*trace_length*/ mload(0x1c0), 1)), div(/*trace_length*/ mload(0x1c0), 256), /*trace_length*/ mload(0x1c0)), PRIME))

              // Adjustment polynomial for constraints 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit_extraction_end', 'state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/zeros_tail', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit_extraction_end', 'state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/zeros_tail'.
              // adjustments[8] = point^degreeAdjustment(composition_degree_bound, trace_length - 1, 0, trace_length / 256)
              mstore(0x4080,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), sub(/*trace_length*/ mload(0x1c0), 1), 0, div(/*trace_length*/ mload(0x1c0), 256)), PRIME))

              // Adjustment polynomial for constraints 'state_transition/merkle_update/prev_authentication/hashes/copy_point/x', 'state_transition/merkle_update/prev_authentication/hashes/copy_point/y', 'state_transition/merkle_update/new_authentication/hashes/copy_point/x', 'state_transition/merkle_update/new_authentication/hashes/copy_point/y'.
              // adjustments[9] = point^degreeAdjustment(composition_degree_bound, trace_length - 1, trace_length / 512, trace_length / 256)
              mstore(0x40a0,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), sub(/*trace_length*/ mload(0x1c0), 1), div(/*trace_length*/ mload(0x1c0), 512), div(/*trace_length*/ mload(0x1c0), 256)), PRIME))

              // Adjustment polynomial for constraints 'state_transition/merkle_update/prev_authentication/hashes/init/x', 'state_transition/merkle_update/prev_authentication/hashes/init/y', 'state_transition/merkle_update/new_authentication/hashes/init/x', 'state_transition/merkle_update/new_authentication/hashes/init/y'.
              // adjustments[10] = point^degreeAdjustment(composition_degree_bound, trace_length - 1, 0, trace_length / 512)
              mstore(0x40c0,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), sub(/*trace_length*/ mload(0x1c0), 1), 0, div(/*trace_length*/ mload(0x1c0), 512)), PRIME))

              // Adjustment polynomial for constraints 'state_transition/merkle_update/prev_authentication/copy_prev_to_left', 'state_transition/merkle_update/prev_authentication/copy_prev_to_right', 'state_transition/merkle_update/new_authentication/copy_prev_to_left', 'state_transition/merkle_update/new_authentication/copy_prev_to_right'.
              // adjustments[11] = point^degreeAdjustment(composition_degree_bound, 2 * (trace_length - 1), trace_length / 16384 + trace_length / 16384, trace_length / 512)
              mstore(0x40e0,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), mul(2, sub(/*trace_length*/ mload(0x1c0), 1)), add(
                       div(/*trace_length*/ mload(0x1c0), 16384),
                       div(/*trace_length*/ mload(0x1c0), 16384)), div(/*trace_length*/ mload(0x1c0), 512)), PRIME))

              // Adjustment polynomial for constraints 'state_transition/merkle_set_prev_leaf', 'state_transition/merkle_set_new_leaf', 'sig_verify/r_and_w_nonzero', 'handle_empty_vault/consistency_key_stage0', 'handle_empty_vault/consistency_token_stage0', 'handle_empty_vault/consistency_key_stage1', 'handle_empty_vault/consistency_token_stage1'.
              // adjustments[12] = point^degreeAdjustment(composition_degree_bound, 2 * (trace_length - 1), 0, trace_length / 16384)
              mstore(0x4100,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), mul(2, sub(/*trace_length*/ mload(0x1c0), 1)), 0, div(/*trace_length*/ mload(0x1c0), 16384)), PRIME))

              // Adjustment polynomial for constraints 'modification_boundary_key', 'modification_boundary_token', 'modification_boundary_amount0', 'modification_boundary_amount1', 'modification_boundary_vault_id'.
              // adjustments[13] = point^degreeAdjustment(composition_degree_bound, trace_length - 1 + trace_length / 65536 - 1, 0, trace_length / 65536)
              mstore(0x4120,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), add(
                       sub(/*trace_length*/ mload(0x1c0), 1),
                       sub(div(/*trace_length*/ mload(0x1c0), 65536), 1)), 0, div(/*trace_length*/ mload(0x1c0), 65536)), PRIME))

              // Adjustment polynomial for constraints 'amounts_range_check/bit'.
              // adjustments[14] = point^degreeAdjustment(composition_degree_bound, 2 * (trace_length - 1), trace_length / 8192, trace_length / 128)
              mstore(0x4140,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), mul(2, sub(/*trace_length*/ mload(0x1c0), 1)), div(/*trace_length*/ mload(0x1c0), 8192), div(/*trace_length*/ mload(0x1c0), 128)), PRIME))

              // Adjustment polynomial for constraints 'amounts_range_check/zero'.
              // adjustments[15] = point^degreeAdjustment(composition_degree_bound, trace_length - 1, 0, trace_length / 8192)
              mstore(0x4160,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), sub(/*trace_length*/ mload(0x1c0), 1), 0, div(/*trace_length*/ mload(0x1c0), 8192)), PRIME))

              // Adjustment polynomial for constraints 'total_token_a_not_changed', 'total_token_b_not_changed', 'diff_a_range_check_input', 'diff_b_range_check_input', 'maker_sig_input_packed', 'taker_sig_input_maker_hash', 'taker_sig_input_vault_a', 'taker_sig_input_vault_b', 'copy_signature_input_maker', 'copy_signature_input_taker'.
              // adjustments[16] = point^degreeAdjustment(composition_degree_bound, 2 * (trace_length - 1), 0, trace_length / 65536)
              mstore(0x4180,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), mul(2, sub(/*trace_length*/ mload(0x1c0), 1)), 0, div(/*trace_length*/ mload(0x1c0), 65536)), PRIME))

              // Adjustment polynomial for constraints 'settlement_id_range_check/bit'.
              // adjustments[17] = point^degreeAdjustment(composition_degree_bound, 2 * (trace_length - 1), trace_length / 65536, trace_length / 2048)
              mstore(0x41a0,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), mul(2, sub(/*trace_length*/ mload(0x1c0), 1)), div(/*trace_length*/ mload(0x1c0), 65536), div(/*trace_length*/ mload(0x1c0), 2048)), PRIME))

              // Adjustment polynomial for constraints 'settlement_id_range_check/zero'.
              // adjustments[18] = point^degreeAdjustment(composition_degree_bound, trace_length - 1, 0, trace_length / 65536)
              mstore(0x41c0,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), sub(/*trace_length*/ mload(0x1c0), 1), 0, div(/*trace_length*/ mload(0x1c0), 65536)), PRIME))

              // Adjustment polynomial for constraints 'sig_verify/doubling_key/slope', 'sig_verify/doubling_key/x', 'sig_verify/doubling_key/y', 'sig_verify/exponentiate_key/booleanity_test', 'sig_verify/exponentiate_key/add_points/slope', 'sig_verify/exponentiate_key/add_points/x', 'sig_verify/exponentiate_key/add_points/y', 'sig_verify/exponentiate_key/add_points/x_diff_inv', 'sig_verify/exponentiate_key/copy_point/x', 'sig_verify/exponentiate_key/copy_point/y'.
              // adjustments[19] = point^degreeAdjustment(composition_degree_bound, 2 * (trace_length - 1), trace_length / 16384, trace_length / 64)
              mstore(0x41e0,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), mul(2, sub(/*trace_length*/ mload(0x1c0), 1)), div(/*trace_length*/ mload(0x1c0), 16384), div(/*trace_length*/ mload(0x1c0), 64)), PRIME))

              // Adjustment polynomial for constraints 'sig_verify/exponentiate_generator/booleanity_test', 'sig_verify/exponentiate_generator/add_points/slope', 'sig_verify/exponentiate_generator/add_points/x', 'sig_verify/exponentiate_generator/add_points/y', 'sig_verify/exponentiate_generator/add_points/x_diff_inv', 'sig_verify/exponentiate_generator/copy_point/x', 'sig_verify/exponentiate_generator/copy_point/y'.
              // adjustments[20] = point^degreeAdjustment(composition_degree_bound, 2 * (trace_length - 1), trace_length / 32768, trace_length / 128)
              mstore(0x4200,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), mul(2, sub(/*trace_length*/ mload(0x1c0), 1)), div(/*trace_length*/ mload(0x1c0), 32768), div(/*trace_length*/ mload(0x1c0), 128)), PRIME))

              // Adjustment polynomial for constraints 'sig_verify/exponentiate_generator/bit_extraction_end', 'sig_verify/exponentiate_generator/zeros_tail', 'sig_verify/init_gen/x', 'sig_verify/init_gen/y'.
              // adjustments[21] = point^degreeAdjustment(composition_degree_bound, trace_length - 1, 0, trace_length / 32768)
              mstore(0x4220,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), sub(/*trace_length*/ mload(0x1c0), 1), 0, div(/*trace_length*/ mload(0x1c0), 32768)), PRIME))

              // Adjustment polynomial for constraints 'sig_verify/add_results/slope', 'sig_verify/add_results/x', 'sig_verify/add_results/y', 'sig_verify/add_results/x_diff_inv', 'sig_verify/extract_r/slope', 'sig_verify/extract_r/x', 'sig_verify/extract_r/x_diff_inv', 'sig_verify/z_nonzero', 'sig_verify/q_on_curve/x_squared', 'sig_verify/q_on_curve/on_curve'.
              // adjustments[22] = point^degreeAdjustment(composition_degree_bound, 2 * (trace_length - 1), 0, trace_length / 32768)
              mstore(0x4240,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), mul(2, sub(/*trace_length*/ mload(0x1c0), 1)), 0, div(/*trace_length*/ mload(0x1c0), 32768)), PRIME))

              // Adjustment polynomial for constraints 'handle_empty_vault/consistency_key_change0', 'handle_empty_vault/consistency_token_change0', 'handle_empty_vault/consistency_key_change3', 'handle_empty_vault/consistency_token_change3', 'handle_empty_vault/consistency_key_change1', 'handle_empty_vault/consistency_token_change1', 'handle_empty_vault/consistency_key_change2', 'handle_empty_vault/consistency_token_change2'.
              // adjustments[23] = point^degreeAdjustment(composition_degree_bound, trace_length - 1 + n_modifications, 0, trace_length / 65536)
              mstore(0x4260,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), add(sub(/*trace_length*/ mload(0x1c0), 1), /*n_modifications*/ mload(0x280)), 0, div(/*trace_length*/ mload(0x1c0), 65536)), PRIME))

              // Adjustment polynomial for constraints 'handle_empty_vault/vault_empty/empty_vault_booleanity', 'handle_empty_vault/vault_empty/amount_zero_when_empty', 'handle_empty_vault/vault_empty/amount_inv_zero_when_empty', 'handle_empty_vault/vault_empty/empty_when_amount_zero'.
              // adjustments[24] = point^degreeAdjustment(composition_degree_bound, 2 * (trace_length - 1), 0, trace_length / 8192)
              mstore(0x4280,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), mul(2, sub(/*trace_length*/ mload(0x1c0), 1)), 0, div(/*trace_length*/ mload(0x1c0), 8192)), PRIME))

              // Adjustment polynomial for constraints 'initial_vaults_root', 'final_vaults_root'.
              // adjustments[25] = point^degreeAdjustment(composition_degree_bound, trace_length - 1, 0, 1)
              mstore(0x42a0,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), sub(/*trace_length*/ mload(0x1c0), 1), 0, 1), PRIME))

              // Adjustment polynomial for constraints 'copy_merkle_roots'.
              // adjustments[26] = point^degreeAdjustment(composition_degree_bound, trace_length - 1, 1, trace_length / 16384)
              mstore(0x42c0,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), sub(/*trace_length*/ mload(0x1c0), 1), 1, div(/*trace_length*/ mload(0x1c0), 16384)), PRIME))

              // Adjustment polynomial for constraints 'copy_merkle_roots_modification'.
              // adjustments[27] = point^degreeAdjustment(composition_degree_bound, trace_length - 1 + n_settlements, 0, trace_length / 65536)
              mstore(0x42e0,
                     expmod(point, degreeAdjustment(/*composition_degree_bound*/ mload(0x3240), add(sub(/*trace_length*/ mload(0x1c0), 1), /*n_settlements*/ mload(0x2e0)), 0, div(/*trace_length*/ mload(0x1c0), 65536)), PRIME))
            }

            {
              // Compute the result of the composition polynomial.

              {
              // hash_pool/hash/ec_subset_sum/bit = column8_row3 - (column8_row7 + column8_row7)
              let val := addmod(
                /*column8_row3*/ mload(0x2780),
                sub(
                  PRIME,
                  addmod(/*column8_row7*/ mload(0x27e0), /*column8_row7*/ mload(0x27e0), PRIME)),
                PRIME)
              mstore(0x3260, val)
              }

              {
              // Constraint expression for hash_pool/hash/ec_subset_sum/booleanity_test: hash_pool__hash__ec_subset_sum__bit * (hash_pool__hash__ec_subset_sum__bit - 1).
              let val := mulmod(
                /*intermediate_value/hash_pool/hash/ec_subset_sum/bit*/ mload(0x3260),
                addmod(
                  /*intermediate_value/hash_pool/hash/ec_subset_sum/bit*/ mload(0x3260),
                  sub(PRIME, 1),
                  PRIME),
                PRIME)

              // Numerator: point^(trace_length / 1024) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[0]
              val := mulmod(val, mload(0x3e20), PRIME)
              // Denominator: point^(trace_length / 4) - 1
              // val *= denominator_invs[0]
              val := mulmod(val, mload(0x37a0), PRIME)

              // res += val * (coefficients[0] + coefficients[1] * adjustments[0])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[0]*/ mload(0x3a0),
                                       mulmod(/*coefficients[1]*/ mload(0x3c0),
                                              /*adjustments[0]*/mload(0x3f80),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for hash_pool/hash/ec_subset_sum/bit_extraction_end: column8_row3.
              let val := /*column8_row3*/ mload(0x2780)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 1024) - trace_generator^(63 * trace_length / 64)
              // val *= denominator_invs[1]
              val := mulmod(val, mload(0x37c0), PRIME)

              // res += val * (coefficients[2] + coefficients[3] * adjustments[1])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[2]*/ mload(0x3e0),
                                       mulmod(/*coefficients[3]*/ mload(0x400),
                                              /*adjustments[1]*/mload(0x3fa0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for hash_pool/hash/ec_subset_sum/zeros_tail: column8_row3.
              let val := /*column8_row3*/ mload(0x2780)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 1024) - trace_generator^(255 * trace_length / 256)
              // val *= denominator_invs[2]
              val := mulmod(val, mload(0x37e0), PRIME)

              // res += val * (coefficients[4] + coefficients[5] * adjustments[1])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[4]*/ mload(0x420),
                                       mulmod(/*coefficients[5]*/ mload(0x440),
                                              /*adjustments[1]*/mload(0x3fa0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for hash_pool/hash/ec_subset_sum/add_points/slope: hash_pool__hash__ec_subset_sum__bit * (column8_row2 - hash_pool_points__y) - column8_row1 * (column8_row0 - hash_pool_points__x).
              let val := addmod(
                mulmod(
                  /*intermediate_value/hash_pool/hash/ec_subset_sum/bit*/ mload(0x3260),
                  addmod(
                    /*column8_row2*/ mload(0x2760),
                    sub(PRIME, /*periodic_column/hash_pool_points/y*/ mload(0x20)),
                    PRIME),
                  PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*column8_row1*/ mload(0x2740),
                    addmod(
                      /*column8_row0*/ mload(0x2720),
                      sub(PRIME, /*periodic_column/hash_pool_points/x*/ mload(0x0)),
                      PRIME),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 1024) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[0]
              val := mulmod(val, mload(0x3e20), PRIME)
              // Denominator: point^(trace_length / 4) - 1
              // val *= denominator_invs[0]
              val := mulmod(val, mload(0x37a0), PRIME)

              // res += val * (coefficients[6] + coefficients[7] * adjustments[0])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[6]*/ mload(0x460),
                                       mulmod(/*coefficients[7]*/ mload(0x480),
                                              /*adjustments[0]*/mload(0x3f80),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for hash_pool/hash/ec_subset_sum/add_points/x: column8_row1 * column8_row1 - hash_pool__hash__ec_subset_sum__bit * (column8_row0 + hash_pool_points__x + column8_row4).
              let val := addmod(
                mulmod(/*column8_row1*/ mload(0x2740), /*column8_row1*/ mload(0x2740), PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*intermediate_value/hash_pool/hash/ec_subset_sum/bit*/ mload(0x3260),
                    addmod(
                      addmod(
                        /*column8_row0*/ mload(0x2720),
                        /*periodic_column/hash_pool_points/x*/ mload(0x0),
                        PRIME),
                      /*column8_row4*/ mload(0x27a0),
                      PRIME),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 1024) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[0]
              val := mulmod(val, mload(0x3e20), PRIME)
              // Denominator: point^(trace_length / 4) - 1
              // val *= denominator_invs[0]
              val := mulmod(val, mload(0x37a0), PRIME)

              // res += val * (coefficients[8] + coefficients[9] * adjustments[0])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[8]*/ mload(0x4a0),
                                       mulmod(/*coefficients[9]*/ mload(0x4c0),
                                              /*adjustments[0]*/mload(0x3f80),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for hash_pool/hash/ec_subset_sum/add_points/y: hash_pool__hash__ec_subset_sum__bit * (column8_row2 + column8_row6) - column8_row1 * (column8_row0 - column8_row4).
              let val := addmod(
                mulmod(
                  /*intermediate_value/hash_pool/hash/ec_subset_sum/bit*/ mload(0x3260),
                  addmod(/*column8_row2*/ mload(0x2760), /*column8_row6*/ mload(0x27c0), PRIME),
                  PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*column8_row1*/ mload(0x2740),
                    addmod(/*column8_row0*/ mload(0x2720), sub(PRIME, /*column8_row4*/ mload(0x27a0)), PRIME),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 1024) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[0]
              val := mulmod(val, mload(0x3e20), PRIME)
              // Denominator: point^(trace_length / 4) - 1
              // val *= denominator_invs[0]
              val := mulmod(val, mload(0x37a0), PRIME)

              // res += val * (coefficients[10] + coefficients[11] * adjustments[0])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[10]*/ mload(0x4e0),
                                       mulmod(/*coefficients[11]*/ mload(0x500),
                                              /*adjustments[0]*/mload(0x3f80),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // hash_pool/hash/ec_subset_sum/bit_neg = 1 - hash_pool__hash__ec_subset_sum__bit
              let val := addmod(
                1,
                sub(PRIME, /*intermediate_value/hash_pool/hash/ec_subset_sum/bit*/ mload(0x3260)),
                PRIME)
              mstore(0x3280, val)
              }

              {
              // Constraint expression for hash_pool/hash/ec_subset_sum/copy_point/x: hash_pool__hash__ec_subset_sum__bit_neg * (column8_row4 - column8_row0).
              let val := mulmod(
                /*intermediate_value/hash_pool/hash/ec_subset_sum/bit_neg*/ mload(0x3280),
                addmod(/*column8_row4*/ mload(0x27a0), sub(PRIME, /*column8_row0*/ mload(0x2720)), PRIME),
                PRIME)

              // Numerator: point^(trace_length / 1024) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[0]
              val := mulmod(val, mload(0x3e20), PRIME)
              // Denominator: point^(trace_length / 4) - 1
              // val *= denominator_invs[0]
              val := mulmod(val, mload(0x37a0), PRIME)

              // res += val * (coefficients[12] + coefficients[13] * adjustments[0])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[12]*/ mload(0x520),
                                       mulmod(/*coefficients[13]*/ mload(0x540),
                                              /*adjustments[0]*/mload(0x3f80),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for hash_pool/hash/ec_subset_sum/copy_point/y: hash_pool__hash__ec_subset_sum__bit_neg * (column8_row6 - column8_row2).
              let val := mulmod(
                /*intermediate_value/hash_pool/hash/ec_subset_sum/bit_neg*/ mload(0x3280),
                addmod(/*column8_row6*/ mload(0x27c0), sub(PRIME, /*column8_row2*/ mload(0x2760)), PRIME),
                PRIME)

              // Numerator: point^(trace_length / 1024) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[0]
              val := mulmod(val, mload(0x3e20), PRIME)
              // Denominator: point^(trace_length / 4) - 1
              // val *= denominator_invs[0]
              val := mulmod(val, mload(0x37a0), PRIME)

              // res += val * (coefficients[14] + coefficients[15] * adjustments[0])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[14]*/ mload(0x560),
                                       mulmod(/*coefficients[15]*/ mload(0x580),
                                              /*adjustments[0]*/mload(0x3f80),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for hash_pool/hash/copy_point/x: column8_row1024 - column8_row1020.
              let val := addmod(
                /*column8_row1024*/ mload(0x2860),
                sub(PRIME, /*column8_row1020*/ mload(0x2800)),
                PRIME)

              // Numerator: point^(trace_length / 2048) - trace_generator^(trace_length / 2)
              // val *= numerators[1]
              val := mulmod(val, mload(0x3e40), PRIME)
              // Denominator: point^(trace_length / 1024) - 1
              // val *= denominator_invs[3]
              val := mulmod(val, mload(0x3800), PRIME)

              // res += val * (coefficients[16] + coefficients[17] * adjustments[2])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[16]*/ mload(0x5a0),
                                       mulmod(/*coefficients[17]*/ mload(0x5c0),
                                              /*adjustments[2]*/mload(0x3fc0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for hash_pool/hash/copy_point/y: column8_row1026 - column8_row1022.
              let val := addmod(
                /*column8_row1026*/ mload(0x2880),
                sub(PRIME, /*column8_row1022*/ mload(0x2840)),
                PRIME)

              // Numerator: point^(trace_length / 2048) - trace_generator^(trace_length / 2)
              // val *= numerators[1]
              val := mulmod(val, mload(0x3e40), PRIME)
              // Denominator: point^(trace_length / 1024) - 1
              // val *= denominator_invs[3]
              val := mulmod(val, mload(0x3800), PRIME)

              // res += val * (coefficients[18] + coefficients[19] * adjustments[2])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[18]*/ mload(0x5e0),
                                       mulmod(/*coefficients[19]*/ mload(0x600),
                                              /*adjustments[2]*/mload(0x3fc0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for hash_pool/hash/init/x: column8_row0 - shift_point.x.
              let val := addmod(/*column8_row0*/ mload(0x2720), sub(PRIME, /*shift_point.x*/ mload(0x1e0)), PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 2048) - 1
              // val *= denominator_invs[4]
              val := mulmod(val, mload(0x3820), PRIME)

              // res += val * (coefficients[20] + coefficients[21] * adjustments[3])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[20]*/ mload(0x620),
                                       mulmod(/*coefficients[21]*/ mload(0x640),
                                              /*adjustments[3]*/mload(0x3fe0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for hash_pool/hash/init/y: column8_row2 - shift_point.y.
              let val := addmod(/*column8_row2*/ mload(0x2760), sub(PRIME, /*shift_point.y*/ mload(0x200)), PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 2048) - 1
              // val *= denominator_invs[4]
              val := mulmod(val, mload(0x3820), PRIME)

              // res += val * (coefficients[22] + coefficients[23] * adjustments[3])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[22]*/ mload(0x660),
                                       mulmod(/*coefficients[23]*/ mload(0x680),
                                              /*adjustments[3]*/mload(0x3fe0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for hash_pool/output_to_input: column8_row2044 - column8_row2051.
              let val := addmod(
                /*column8_row2044*/ mload(0x28c0),
                sub(PRIME, /*column8_row2051*/ mload(0x2900)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 4096) - 1
              // val *= denominator_invs[5]
              val := mulmod(val, mload(0x3840), PRIME)

              // res += val * (coefficients[24] + coefficients[25] * adjustments[4])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[24]*/ mload(0x6a0),
                                       mulmod(/*coefficients[25]*/ mload(0x6c0),
                                              /*adjustments[4]*/mload(0x4000),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // state_transition/merkle_update/side_bit_extraction/bit_0 = column6_row255 - (column6_row767 + column6_row767)
              let val := addmod(
                /*column6_row255*/ mload(0x25c0),
                sub(
                  PRIME,
                  addmod(/*column6_row767*/ mload(0x25e0), /*column6_row767*/ mload(0x25e0), PRIME)),
                PRIME)
              mstore(0x32a0, val)
              }

              {
              // Constraint expression for state_transition/merkle_update/side_bit_extraction/bit: state_transition__merkle_update__side_bit_extraction__bit_0 * state_transition__merkle_update__side_bit_extraction__bit_0 - state_transition__merkle_update__side_bit_extraction__bit_0.
              let val := addmod(
                mulmod(
                  /*intermediate_value/state_transition/merkle_update/side_bit_extraction/bit_0*/ mload(0x32a0),
                  /*intermediate_value/state_transition/merkle_update/side_bit_extraction/bit_0*/ mload(0x32a0),
                  PRIME),
                sub(
                  PRIME,
                  /*intermediate_value/state_transition/merkle_update/side_bit_extraction/bit_0*/ mload(0x32a0)),
                PRIME)

              // Numerator: point^(trace_length / 16384) - trace_generator^(31 * trace_length / 32)
              // val *= numerators[2]
              val := mulmod(val, mload(0x3e60), PRIME)
              // Denominator: point^(trace_length / 512) - 1
              // val *= denominator_invs[6]
              val := mulmod(val, mload(0x3860), PRIME)

              // res += val * (coefficients[26] + coefficients[27] * adjustments[5])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[26]*/ mload(0x6e0),
                                       mulmod(/*coefficients[27]*/ mload(0x700),
                                              /*adjustments[5]*/mload(0x4020),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/side_bit_extraction/zero: column6_row255.
              let val := /*column6_row255*/ mload(0x25c0)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 16384) - trace_generator^(vaults_path_length * trace_length / 32)
              // val *= denominator_invs[7]
              val := mulmod(val, mload(0x3880), PRIME)

              // res += val * (coefficients[28] + coefficients[29] * adjustments[6])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[28]*/ mload(0x720),
                                       mulmod(/*coefficients[29]*/ mload(0x740),
                                              /*adjustments[6]*/mload(0x4040),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit = column3_row0 - (column3_row1 + column3_row1)
              let val := addmod(
                /*column3_row0*/ mload(0x23a0),
                sub(
                  PRIME,
                  addmod(/*column3_row1*/ mload(0x23c0), /*column3_row1*/ mload(0x23c0), PRIME)),
                PRIME)
              mstore(0x32c0, val)
              }

              {
              // Constraint expression for state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/booleanity_test: state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit * (state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit - 1).
              let val := mulmod(
                /*intermediate_value/state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit*/ mload(0x32c0),
                addmod(
                  /*intermediate_value/state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit*/ mload(0x32c0),
                  sub(PRIME, 1),
                  PRIME),
                PRIME)

              // Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[3]
              val := mulmod(val, mload(0x3e80), PRIME)
              // Denominator: point^trace_length - 1
              // val *= denominator_invs[8]
              val := mulmod(val, mload(0x38a0), PRIME)

              // res += val * (coefficients[30] + coefficients[31] * adjustments[7])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[30]*/ mload(0x760),
                                       mulmod(/*coefficients[31]*/ mload(0x780),
                                              /*adjustments[7]*/mload(0x4060),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit_extraction_end: column3_row0.
              let val := /*column3_row0*/ mload(0x23a0)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 256) - trace_generator^(63 * trace_length / 64)
              // val *= denominator_invs[9]
              val := mulmod(val, mload(0x38c0), PRIME)

              // res += val * (coefficients[32] + coefficients[33] * adjustments[8])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[32]*/ mload(0x7a0),
                                       mulmod(/*coefficients[33]*/ mload(0x7c0),
                                              /*adjustments[8]*/mload(0x4080),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/zeros_tail: column3_row0.
              let val := /*column3_row0*/ mload(0x23a0)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256)
              // val *= denominator_invs[10]
              val := mulmod(val, mload(0x38e0), PRIME)

              // res += val * (coefficients[34] + coefficients[35] * adjustments[8])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[34]*/ mload(0x7e0),
                                       mulmod(/*coefficients[35]*/ mload(0x800),
                                              /*adjustments[8]*/mload(0x4080),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/slope: state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit * (column1_row0 - merkle_hash_points__y) - column2_row0 * (column0_row0 - merkle_hash_points__x).
              let val := addmod(
                mulmod(
                  /*intermediate_value/state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit*/ mload(0x32c0),
                  addmod(
                    /*column1_row0*/ mload(0x2300),
                    sub(PRIME, /*periodic_column/merkle_hash_points/y*/ mload(0x60)),
                    PRIME),
                  PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*column2_row0*/ mload(0x2380),
                    addmod(
                      /*column0_row0*/ mload(0x2220),
                      sub(PRIME, /*periodic_column/merkle_hash_points/x*/ mload(0x40)),
                      PRIME),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[3]
              val := mulmod(val, mload(0x3e80), PRIME)
              // Denominator: point^trace_length - 1
              // val *= denominator_invs[8]
              val := mulmod(val, mload(0x38a0), PRIME)

              // res += val * (coefficients[36] + coefficients[37] * adjustments[7])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[36]*/ mload(0x820),
                                       mulmod(/*coefficients[37]*/ mload(0x840),
                                              /*adjustments[7]*/mload(0x4060),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/x: column2_row0 * column2_row0 - state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit * (column0_row0 + merkle_hash_points__x + column0_row1).
              let val := addmod(
                mulmod(/*column2_row0*/ mload(0x2380), /*column2_row0*/ mload(0x2380), PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*intermediate_value/state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit*/ mload(0x32c0),
                    addmod(
                      addmod(
                        /*column0_row0*/ mload(0x2220),
                        /*periodic_column/merkle_hash_points/x*/ mload(0x40),
                        PRIME),
                      /*column0_row1*/ mload(0x2240),
                      PRIME),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[3]
              val := mulmod(val, mload(0x3e80), PRIME)
              // Denominator: point^trace_length - 1
              // val *= denominator_invs[8]
              val := mulmod(val, mload(0x38a0), PRIME)

              // res += val * (coefficients[38] + coefficients[39] * adjustments[7])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[38]*/ mload(0x860),
                                       mulmod(/*coefficients[39]*/ mload(0x880),
                                              /*adjustments[7]*/mload(0x4060),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/add_points/y: state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit * (column1_row0 + column1_row1) - column2_row0 * (column0_row0 - column0_row1).
              let val := addmod(
                mulmod(
                  /*intermediate_value/state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit*/ mload(0x32c0),
                  addmod(/*column1_row0*/ mload(0x2300), /*column1_row1*/ mload(0x2320), PRIME),
                  PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*column2_row0*/ mload(0x2380),
                    addmod(/*column0_row0*/ mload(0x2220), sub(PRIME, /*column0_row1*/ mload(0x2240)), PRIME),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[3]
              val := mulmod(val, mload(0x3e80), PRIME)
              // Denominator: point^trace_length - 1
              // val *= denominator_invs[8]
              val := mulmod(val, mload(0x38a0), PRIME)

              // res += val * (coefficients[40] + coefficients[41] * adjustments[7])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[40]*/ mload(0x8a0),
                                       mulmod(/*coefficients[41]*/ mload(0x8c0),
                                              /*adjustments[7]*/mload(0x4060),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit_neg = 1 - state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit
              let val := addmod(
                1,
                sub(
                  PRIME,
                  /*intermediate_value/state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit*/ mload(0x32c0)),
                PRIME)
              mstore(0x32e0, val)
              }

              {
              // Constraint expression for state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/copy_point/x: state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit_neg * (column0_row1 - column0_row0).
              let val := mulmod(
                /*intermediate_value/state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit_neg*/ mload(0x32e0),
                addmod(/*column0_row1*/ mload(0x2240), sub(PRIME, /*column0_row0*/ mload(0x2220)), PRIME),
                PRIME)

              // Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[3]
              val := mulmod(val, mload(0x3e80), PRIME)
              // Denominator: point^trace_length - 1
              // val *= denominator_invs[8]
              val := mulmod(val, mload(0x38a0), PRIME)

              // res += val * (coefficients[42] + coefficients[43] * adjustments[7])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[42]*/ mload(0x8e0),
                                       mulmod(/*coefficients[43]*/ mload(0x900),
                                              /*adjustments[7]*/mload(0x4060),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/copy_point/y: state_transition__merkle_update__prev_authentication__hashes__ec_subset_sum__bit_neg * (column1_row1 - column1_row0).
              let val := mulmod(
                /*intermediate_value/state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit_neg*/ mload(0x32e0),
                addmod(/*column1_row1*/ mload(0x2320), sub(PRIME, /*column1_row0*/ mload(0x2300)), PRIME),
                PRIME)

              // Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[3]
              val := mulmod(val, mload(0x3e80), PRIME)
              // Denominator: point^trace_length - 1
              // val *= denominator_invs[8]
              val := mulmod(val, mload(0x38a0), PRIME)

              // res += val * (coefficients[44] + coefficients[45] * adjustments[7])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[44]*/ mload(0x920),
                                       mulmod(/*coefficients[45]*/ mload(0x940),
                                              /*adjustments[7]*/mload(0x4060),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/prev_authentication/hashes/copy_point/x: column0_row256 - column0_row255.
              let val := addmod(
                /*column0_row256*/ mload(0x2280),
                sub(PRIME, /*column0_row255*/ mload(0x2260)),
                PRIME)

              // Numerator: point^(trace_length / 512) - trace_generator^(trace_length / 2)
              // val *= numerators[4]
              val := mulmod(val, mload(0x3ea0), PRIME)
              // Denominator: point^(trace_length / 256) - 1
              // val *= denominator_invs[11]
              val := mulmod(val, mload(0x3900), PRIME)

              // res += val * (coefficients[46] + coefficients[47] * adjustments[9])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[46]*/ mload(0x960),
                                       mulmod(/*coefficients[47]*/ mload(0x980),
                                              /*adjustments[9]*/mload(0x40a0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/prev_authentication/hashes/copy_point/y: column1_row256 - column1_row255.
              let val := addmod(
                /*column1_row256*/ mload(0x2360),
                sub(PRIME, /*column1_row255*/ mload(0x2340)),
                PRIME)

              // Numerator: point^(trace_length / 512) - trace_generator^(trace_length / 2)
              // val *= numerators[4]
              val := mulmod(val, mload(0x3ea0), PRIME)
              // Denominator: point^(trace_length / 256) - 1
              // val *= denominator_invs[11]
              val := mulmod(val, mload(0x3900), PRIME)

              // res += val * (coefficients[48] + coefficients[49] * adjustments[9])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[48]*/ mload(0x9a0),
                                       mulmod(/*coefficients[49]*/ mload(0x9c0),
                                              /*adjustments[9]*/mload(0x40a0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/prev_authentication/hashes/init/x: column0_row0 - shift_point.x.
              let val := addmod(/*column0_row0*/ mload(0x2220), sub(PRIME, /*shift_point.x*/ mload(0x1e0)), PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 512) - 1
              // val *= denominator_invs[6]
              val := mulmod(val, mload(0x3860), PRIME)

              // res += val * (coefficients[50] + coefficients[51] * adjustments[10])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[50]*/ mload(0x9e0),
                                       mulmod(/*coefficients[51]*/ mload(0xa00),
                                              /*adjustments[10]*/mload(0x40c0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/prev_authentication/hashes/init/y: column1_row0 - shift_point.y.
              let val := addmod(/*column1_row0*/ mload(0x2300), sub(PRIME, /*shift_point.y*/ mload(0x200)), PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 512) - 1
              // val *= denominator_invs[6]
              val := mulmod(val, mload(0x3860), PRIME)

              // res += val * (coefficients[52] + coefficients[53] * adjustments[10])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[52]*/ mload(0xa20),
                                       mulmod(/*coefficients[53]*/ mload(0xa40),
                                              /*adjustments[10]*/mload(0x40c0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // state_transition/merkle_update/side_bit_extraction/bit_1 = column6_row767 - (column6_row1279 + column6_row1279)
              let val := addmod(
                /*column6_row767*/ mload(0x25e0),
                sub(
                  PRIME,
                  addmod(/*column6_row1279*/ mload(0x2600), /*column6_row1279*/ mload(0x2600), PRIME)),
                PRIME)
              mstore(0x3300, val)
              }

              {
              // Constraint expression for state_transition/merkle_update/prev_authentication/copy_prev_to_left: (1 - state_transition__merkle_update__side_bit_extraction__bit_1) * (column0_row511 - column3_row512).
              let val := mulmod(
                addmod(
                  1,
                  sub(
                    PRIME,
                    /*intermediate_value/state_transition/merkle_update/side_bit_extraction/bit_1*/ mload(0x3300)),
                  PRIME),
                addmod(
                  /*column0_row511*/ mload(0x22a0),
                  sub(PRIME, /*column3_row512*/ mload(0x2400)),
                  PRIME),
                PRIME)

              // Numerator: (point^(trace_length / 16384) - trace_generator^(31 * trace_length / 32)) * (point^(trace_length / 16384) - trace_generator^(15 * trace_length / 16))
              // val *= numerators[5]
              val := mulmod(val, mload(0x3ec0), PRIME)
              // Denominator: point^(trace_length / 512) - 1
              // val *= denominator_invs[6]
              val := mulmod(val, mload(0x3860), PRIME)

              // res += val * (coefficients[54] + coefficients[55] * adjustments[11])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[54]*/ mload(0xa60),
                                       mulmod(/*coefficients[55]*/ mload(0xa80),
                                              /*adjustments[11]*/mload(0x40e0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/prev_authentication/copy_prev_to_right: state_transition__merkle_update__side_bit_extraction__bit_1 * (column0_row511 - column3_row768).
              let val := mulmod(
                /*intermediate_value/state_transition/merkle_update/side_bit_extraction/bit_1*/ mload(0x3300),
                addmod(
                  /*column0_row511*/ mload(0x22a0),
                  sub(PRIME, /*column3_row768*/ mload(0x2420)),
                  PRIME),
                PRIME)

              // Numerator: (point^(trace_length / 16384) - trace_generator^(31 * trace_length / 32)) * (point^(trace_length / 16384) - trace_generator^(15 * trace_length / 16))
              // val *= numerators[5]
              val := mulmod(val, mload(0x3ec0), PRIME)
              // Denominator: point^(trace_length / 512) - 1
              // val *= denominator_invs[6]
              val := mulmod(val, mload(0x3860), PRIME)

              // res += val * (coefficients[56] + coefficients[57] * adjustments[11])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[56]*/ mload(0xaa0),
                                       mulmod(/*coefficients[57]*/ mload(0xac0),
                                              /*adjustments[11]*/mload(0x40e0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit = column7_row0 - (column7_row1 + column7_row1)
              let val := addmod(
                /*column7_row0*/ mload(0x2680),
                sub(
                  PRIME,
                  addmod(/*column7_row1*/ mload(0x26a0), /*column7_row1*/ mload(0x26a0), PRIME)),
                PRIME)
              mstore(0x3320, val)
              }

              {
              // Constraint expression for state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/booleanity_test: state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit * (state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit - 1).
              let val := mulmod(
                /*intermediate_value/state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit*/ mload(0x3320),
                addmod(
                  /*intermediate_value/state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit*/ mload(0x3320),
                  sub(PRIME, 1),
                  PRIME),
                PRIME)

              // Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[3]
              val := mulmod(val, mload(0x3e80), PRIME)
              // Denominator: point^trace_length - 1
              // val *= denominator_invs[8]
              val := mulmod(val, mload(0x38a0), PRIME)

              // res += val * (coefficients[58] + coefficients[59] * adjustments[7])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[58]*/ mload(0xae0),
                                       mulmod(/*coefficients[59]*/ mload(0xb00),
                                              /*adjustments[7]*/mload(0x4060),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit_extraction_end: column7_row0.
              let val := /*column7_row0*/ mload(0x2680)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 256) - trace_generator^(63 * trace_length / 64)
              // val *= denominator_invs[9]
              val := mulmod(val, mload(0x38c0), PRIME)

              // res += val * (coefficients[60] + coefficients[61] * adjustments[8])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[60]*/ mload(0xb20),
                                       mulmod(/*coefficients[61]*/ mload(0xb40),
                                              /*adjustments[8]*/mload(0x4080),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/zeros_tail: column7_row0.
              let val := /*column7_row0*/ mload(0x2680)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256)
              // val *= denominator_invs[10]
              val := mulmod(val, mload(0x38e0), PRIME)

              // res += val * (coefficients[62] + coefficients[63] * adjustments[8])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[62]*/ mload(0xb60),
                                       mulmod(/*coefficients[63]*/ mload(0xb80),
                                              /*adjustments[8]*/mload(0x4080),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/slope: state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit * (column5_row0 - merkle_hash_points__y) - column6_row0 * (column4_row0 - merkle_hash_points__x).
              let val := addmod(
                mulmod(
                  /*intermediate_value/state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit*/ mload(0x3320),
                  addmod(
                    /*column5_row0*/ mload(0x2520),
                    sub(PRIME, /*periodic_column/merkle_hash_points/y*/ mload(0x60)),
                    PRIME),
                  PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*column6_row0*/ mload(0x25a0),
                    addmod(
                      /*column4_row0*/ mload(0x2440),
                      sub(PRIME, /*periodic_column/merkle_hash_points/x*/ mload(0x40)),
                      PRIME),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[3]
              val := mulmod(val, mload(0x3e80), PRIME)
              // Denominator: point^trace_length - 1
              // val *= denominator_invs[8]
              val := mulmod(val, mload(0x38a0), PRIME)

              // res += val * (coefficients[64] + coefficients[65] * adjustments[7])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[64]*/ mload(0xba0),
                                       mulmod(/*coefficients[65]*/ mload(0xbc0),
                                              /*adjustments[7]*/mload(0x4060),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/x: column6_row0 * column6_row0 - state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit * (column4_row0 + merkle_hash_points__x + column4_row1).
              let val := addmod(
                mulmod(/*column6_row0*/ mload(0x25a0), /*column6_row0*/ mload(0x25a0), PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*intermediate_value/state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit*/ mload(0x3320),
                    addmod(
                      addmod(
                        /*column4_row0*/ mload(0x2440),
                        /*periodic_column/merkle_hash_points/x*/ mload(0x40),
                        PRIME),
                      /*column4_row1*/ mload(0x2460),
                      PRIME),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[3]
              val := mulmod(val, mload(0x3e80), PRIME)
              // Denominator: point^trace_length - 1
              // val *= denominator_invs[8]
              val := mulmod(val, mload(0x38a0), PRIME)

              // res += val * (coefficients[66] + coefficients[67] * adjustments[7])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[66]*/ mload(0xbe0),
                                       mulmod(/*coefficients[67]*/ mload(0xc00),
                                              /*adjustments[7]*/mload(0x4060),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/add_points/y: state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit * (column5_row0 + column5_row1) - column6_row0 * (column4_row0 - column4_row1).
              let val := addmod(
                mulmod(
                  /*intermediate_value/state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit*/ mload(0x3320),
                  addmod(/*column5_row0*/ mload(0x2520), /*column5_row1*/ mload(0x2540), PRIME),
                  PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*column6_row0*/ mload(0x25a0),
                    addmod(/*column4_row0*/ mload(0x2440), sub(PRIME, /*column4_row1*/ mload(0x2460)), PRIME),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[3]
              val := mulmod(val, mload(0x3e80), PRIME)
              // Denominator: point^trace_length - 1
              // val *= denominator_invs[8]
              val := mulmod(val, mload(0x38a0), PRIME)

              // res += val * (coefficients[68] + coefficients[69] * adjustments[7])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[68]*/ mload(0xc20),
                                       mulmod(/*coefficients[69]*/ mload(0xc40),
                                              /*adjustments[7]*/mload(0x4060),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit_neg = 1 - state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit
              let val := addmod(
                1,
                sub(
                  PRIME,
                  /*intermediate_value/state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit*/ mload(0x3320)),
                PRIME)
              mstore(0x3340, val)
              }

              {
              // Constraint expression for state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/copy_point/x: state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit_neg * (column4_row1 - column4_row0).
              let val := mulmod(
                /*intermediate_value/state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit_neg*/ mload(0x3340),
                addmod(/*column4_row1*/ mload(0x2460), sub(PRIME, /*column4_row0*/ mload(0x2440)), PRIME),
                PRIME)

              // Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[3]
              val := mulmod(val, mload(0x3e80), PRIME)
              // Denominator: point^trace_length - 1
              // val *= denominator_invs[8]
              val := mulmod(val, mload(0x38a0), PRIME)

              // res += val * (coefficients[70] + coefficients[71] * adjustments[7])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[70]*/ mload(0xc60),
                                       mulmod(/*coefficients[71]*/ mload(0xc80),
                                              /*adjustments[7]*/mload(0x4060),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/copy_point/y: state_transition__merkle_update__new_authentication__hashes__ec_subset_sum__bit_neg * (column5_row1 - column5_row0).
              let val := mulmod(
                /*intermediate_value/state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit_neg*/ mload(0x3340),
                addmod(/*column5_row1*/ mload(0x2540), sub(PRIME, /*column5_row0*/ mload(0x2520)), PRIME),
                PRIME)

              // Numerator: point^(trace_length / 256) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[3]
              val := mulmod(val, mload(0x3e80), PRIME)
              // Denominator: point^trace_length - 1
              // val *= denominator_invs[8]
              val := mulmod(val, mload(0x38a0), PRIME)

              // res += val * (coefficients[72] + coefficients[73] * adjustments[7])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[72]*/ mload(0xca0),
                                       mulmod(/*coefficients[73]*/ mload(0xcc0),
                                              /*adjustments[7]*/mload(0x4060),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/new_authentication/hashes/copy_point/x: column4_row256 - column4_row255.
              let val := addmod(
                /*column4_row256*/ mload(0x24a0),
                sub(PRIME, /*column4_row255*/ mload(0x2480)),
                PRIME)

              // Numerator: point^(trace_length / 512) - trace_generator^(trace_length / 2)
              // val *= numerators[4]
              val := mulmod(val, mload(0x3ea0), PRIME)
              // Denominator: point^(trace_length / 256) - 1
              // val *= denominator_invs[11]
              val := mulmod(val, mload(0x3900), PRIME)

              // res += val * (coefficients[74] + coefficients[75] * adjustments[9])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[74]*/ mload(0xce0),
                                       mulmod(/*coefficients[75]*/ mload(0xd00),
                                              /*adjustments[9]*/mload(0x40a0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/new_authentication/hashes/copy_point/y: column5_row256 - column5_row255.
              let val := addmod(
                /*column5_row256*/ mload(0x2580),
                sub(PRIME, /*column5_row255*/ mload(0x2560)),
                PRIME)

              // Numerator: point^(trace_length / 512) - trace_generator^(trace_length / 2)
              // val *= numerators[4]
              val := mulmod(val, mload(0x3ea0), PRIME)
              // Denominator: point^(trace_length / 256) - 1
              // val *= denominator_invs[11]
              val := mulmod(val, mload(0x3900), PRIME)

              // res += val * (coefficients[76] + coefficients[77] * adjustments[9])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[76]*/ mload(0xd20),
                                       mulmod(/*coefficients[77]*/ mload(0xd40),
                                              /*adjustments[9]*/mload(0x40a0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/new_authentication/hashes/init/x: column4_row0 - shift_point.x.
              let val := addmod(/*column4_row0*/ mload(0x2440), sub(PRIME, /*shift_point.x*/ mload(0x1e0)), PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 512) - 1
              // val *= denominator_invs[6]
              val := mulmod(val, mload(0x3860), PRIME)

              // res += val * (coefficients[78] + coefficients[79] * adjustments[10])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[78]*/ mload(0xd60),
                                       mulmod(/*coefficients[79]*/ mload(0xd80),
                                              /*adjustments[10]*/mload(0x40c0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/new_authentication/hashes/init/y: column5_row0 - shift_point.y.
              let val := addmod(/*column5_row0*/ mload(0x2520), sub(PRIME, /*shift_point.y*/ mload(0x200)), PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 512) - 1
              // val *= denominator_invs[6]
              val := mulmod(val, mload(0x3860), PRIME)

              // res += val * (coefficients[80] + coefficients[81] * adjustments[10])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[80]*/ mload(0xda0),
                                       mulmod(/*coefficients[81]*/ mload(0xdc0),
                                              /*adjustments[10]*/mload(0x40c0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/new_authentication/copy_prev_to_left: (1 - state_transition__merkle_update__side_bit_extraction__bit_1) * (column4_row511 - column7_row512).
              let val := mulmod(
                addmod(
                  1,
                  sub(
                    PRIME,
                    /*intermediate_value/state_transition/merkle_update/side_bit_extraction/bit_1*/ mload(0x3300)),
                  PRIME),
                addmod(
                  /*column4_row511*/ mload(0x24c0),
                  sub(PRIME, /*column7_row512*/ mload(0x26e0)),
                  PRIME),
                PRIME)

              // Numerator: (point^(trace_length / 16384) - trace_generator^(31 * trace_length / 32)) * (point^(trace_length / 16384) - trace_generator^(15 * trace_length / 16))
              // val *= numerators[5]
              val := mulmod(val, mload(0x3ec0), PRIME)
              // Denominator: point^(trace_length / 512) - 1
              // val *= denominator_invs[6]
              val := mulmod(val, mload(0x3860), PRIME)

              // res += val * (coefficients[82] + coefficients[83] * adjustments[11])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[82]*/ mload(0xde0),
                                       mulmod(/*coefficients[83]*/ mload(0xe00),
                                              /*adjustments[11]*/mload(0x40e0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for state_transition/merkle_update/new_authentication/copy_prev_to_right: state_transition__merkle_update__side_bit_extraction__bit_1 * (column4_row511 - column7_row768).
              let val := mulmod(
                /*intermediate_value/state_transition/merkle_update/side_bit_extraction/bit_1*/ mload(0x3300),
                addmod(
                  /*column4_row511*/ mload(0x24c0),
                  sub(PRIME, /*column7_row768*/ mload(0x2700)),
                  PRIME),
                PRIME)

              // Numerator: (point^(trace_length / 16384) - trace_generator^(31 * trace_length / 32)) * (point^(trace_length / 16384) - trace_generator^(15 * trace_length / 16))
              // val *= numerators[5]
              val := mulmod(val, mload(0x3ec0), PRIME)
              // Denominator: point^(trace_length / 512) - 1
              // val *= denominator_invs[6]
              val := mulmod(val, mload(0x3860), PRIME)

              // res += val * (coefficients[84] + coefficients[85] * adjustments[11])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[84]*/ mload(0xe20),
                                       mulmod(/*coefficients[85]*/ mload(0xe40),
                                              /*adjustments[11]*/mload(0x40e0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // state_transition/merkle_update/prev_authentication/sibling_0 = state_transition__merkle_update__side_bit_extraction__bit_0 * column3_row0 + (1 - state_transition__merkle_update__side_bit_extraction__bit_0) * column3_row256
              let val := addmod(
                mulmod(
                  /*intermediate_value/state_transition/merkle_update/side_bit_extraction/bit_0*/ mload(0x32a0),
                  /*column3_row0*/ mload(0x23a0),
                  PRIME),
                mulmod(
                  addmod(
                    1,
                    sub(
                      PRIME,
                      /*intermediate_value/state_transition/merkle_update/side_bit_extraction/bit_0*/ mload(0x32a0)),
                    PRIME),
                  /*column3_row256*/ mload(0x23e0),
                  PRIME),
                PRIME)
              mstore(0x3360, val)
              }

              {
              // state_transition/merkle_update/new_authentication/sibling_0 = state_transition__merkle_update__side_bit_extraction__bit_0 * column7_row0 + (1 - state_transition__merkle_update__side_bit_extraction__bit_0) * column7_row256
              let val := addmod(
                mulmod(
                  /*intermediate_value/state_transition/merkle_update/side_bit_extraction/bit_0*/ mload(0x32a0),
                  /*column7_row0*/ mload(0x2680),
                  PRIME),
                mulmod(
                  addmod(
                    1,
                    sub(
                      PRIME,
                      /*intermediate_value/state_transition/merkle_update/side_bit_extraction/bit_0*/ mload(0x32a0)),
                    PRIME),
                  /*column7_row256*/ mload(0x26c0),
                  PRIME),
                PRIME)
              mstore(0x3380, val)
              }

              {
              // Constraint expression for state_transition/merkle_update/same_siblings: state_transition__merkle_update__prev_authentication__sibling_0 - state_transition__merkle_update__new_authentication__sibling_0.
              let val := addmod(
                /*intermediate_value/state_transition/merkle_update/prev_authentication/sibling_0*/ mload(0x3360),
                sub(
                  PRIME,
                  /*intermediate_value/state_transition/merkle_update/new_authentication/sibling_0*/ mload(0x3380)),
                PRIME)

              // Numerator: point^(trace_length / 16384) - trace_generator^(31 * trace_length / 32)
              // val *= numerators[2]
              val := mulmod(val, mload(0x3e60), PRIME)
              // Denominator: point^(trace_length / 512) - 1
              // val *= denominator_invs[6]
              val := mulmod(val, mload(0x3860), PRIME)

              // res += val * (coefficients[86] + coefficients[87] * adjustments[5])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[86]*/ mload(0xe60),
                                       mulmod(/*coefficients[87]*/ mload(0xe80),
                                              /*adjustments[5]*/mload(0x4020),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // state_transition/merkle_update/prev_authentication/leaf_0 = (1 - state_transition__merkle_update__side_bit_extraction__bit_0) * column3_row0 + state_transition__merkle_update__side_bit_extraction__bit_0 * column3_row256
              let val := addmod(
                mulmod(
                  addmod(
                    1,
                    sub(
                      PRIME,
                      /*intermediate_value/state_transition/merkle_update/side_bit_extraction/bit_0*/ mload(0x32a0)),
                    PRIME),
                  /*column3_row0*/ mload(0x23a0),
                  PRIME),
                mulmod(
                  /*intermediate_value/state_transition/merkle_update/side_bit_extraction/bit_0*/ mload(0x32a0),
                  /*column3_row256*/ mload(0x23e0),
                  PRIME),
                PRIME)
              mstore(0x33a0, val)
              }

              {
              // Constraint expression for state_transition/merkle_set_prev_leaf: state_transition__merkle_update__prev_authentication__leaf_0 - column8_row4092.
              let val := addmod(
                /*intermediate_value/state_transition/merkle_update/prev_authentication/leaf_0*/ mload(0x33a0),
                sub(PRIME, /*column8_row4092*/ mload(0x2960)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 16384) - 1
              // val *= denominator_invs[12]
              val := mulmod(val, mload(0x3920), PRIME)

              // res += val * (coefficients[88] + coefficients[89] * adjustments[12])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[88]*/ mload(0xea0),
                                       mulmod(/*coefficients[89]*/ mload(0xec0),
                                              /*adjustments[12]*/mload(0x4100),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // state_transition/merkle_update/new_authentication/leaf_0 = (1 - state_transition__merkle_update__side_bit_extraction__bit_0) * column7_row0 + state_transition__merkle_update__side_bit_extraction__bit_0 * column7_row256
              let val := addmod(
                mulmod(
                  addmod(
                    1,
                    sub(
                      PRIME,
                      /*intermediate_value/state_transition/merkle_update/side_bit_extraction/bit_0*/ mload(0x32a0)),
                    PRIME),
                  /*column7_row0*/ mload(0x2680),
                  PRIME),
                mulmod(
                  /*intermediate_value/state_transition/merkle_update/side_bit_extraction/bit_0*/ mload(0x32a0),
                  /*column7_row256*/ mload(0x26c0),
                  PRIME),
                PRIME)
              mstore(0x33c0, val)
              }

              {
              // Constraint expression for state_transition/merkle_set_new_leaf: state_transition__merkle_update__new_authentication__leaf_0 - column8_row12284.
              let val := addmod(
                /*intermediate_value/state_transition/merkle_update/new_authentication/leaf_0*/ mload(0x33c0),
                sub(PRIME, /*column8_row12284*/ mload(0x2ac0)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 16384) - 1
              // val *= denominator_invs[12]
              val := mulmod(val, mload(0x3920), PRIME)

              // res += val * (coefficients[90] + coefficients[91] * adjustments[12])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[90]*/ mload(0xee0),
                                       mulmod(/*coefficients[91]*/ mload(0xf00),
                                              /*adjustments[12]*/mload(0x4100),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for modification_boundary_key: is_modification * (column9_row16376 * boundary_base - boundary_key).
              let val := mulmod(
                /*periodic_column/is_modification*/ mload(0xa0),
                addmod(
                  mulmod(
                    /*column9_row16376*/ mload(0x3000),
                    /*periodic_column/boundary_base*/ mload(0x80),
                    PRIME),
                  sub(PRIME, /*periodic_column/boundary_key*/ mload(0xe0)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[92] + coefficients[93] * adjustments[13])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[92]*/ mload(0xf20),
                                       mulmod(/*coefficients[93]*/ mload(0xf40),
                                              /*adjustments[13]*/mload(0x4120),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for modification_boundary_token: is_modification * (column9_row16360 * boundary_base - boundary_token).
              let val := mulmod(
                /*periodic_column/is_modification*/ mload(0xa0),
                addmod(
                  mulmod(
                    /*column9_row16360*/ mload(0x2fc0),
                    /*periodic_column/boundary_base*/ mload(0x80),
                    PRIME),
                  sub(PRIME, /*periodic_column/boundary_token*/ mload(0x100)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[94] + coefficients[95] * adjustments[13])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[94]*/ mload(0xf60),
                                       mulmod(/*coefficients[95]*/ mload(0xf80),
                                              /*adjustments[13]*/mload(0x4120),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for modification_boundary_amount0: is_modification * (column8_row3075 * boundary_base - boundary_amount0).
              let val := mulmod(
                /*periodic_column/is_modification*/ mload(0xa0),
                addmod(
                  mulmod(
                    /*column8_row3075*/ mload(0x2940),
                    /*periodic_column/boundary_base*/ mload(0x80),
                    PRIME),
                  sub(PRIME, /*periodic_column/boundary_amount0*/ mload(0x120)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[96] + coefficients[97] * adjustments[13])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[96]*/ mload(0xfa0),
                                       mulmod(/*coefficients[97]*/ mload(0xfc0),
                                              /*adjustments[13]*/mload(0x4120),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for modification_boundary_amount1: is_modification * (column8_row11267 * boundary_base - boundary_amount1).
              let val := mulmod(
                /*periodic_column/is_modification*/ mload(0xa0),
                addmod(
                  mulmod(
                    /*column8_row11267*/ mload(0x2aa0),
                    /*periodic_column/boundary_base*/ mload(0x80),
                    PRIME),
                  sub(PRIME, /*periodic_column/boundary_amount1*/ mload(0x140)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[98] + coefficients[99] * adjustments[13])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[98]*/ mload(0xfe0),
                                       mulmod(/*coefficients[99]*/ mload(0x1000),
                                              /*adjustments[13]*/mload(0x4120),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for modification_boundary_vault_id: is_modification * (column6_row255 * boundary_base - boundary_vault_id).
              let val := mulmod(
                /*periodic_column/is_modification*/ mload(0xa0),
                addmod(
                  mulmod(
                    /*column6_row255*/ mload(0x25c0),
                    /*periodic_column/boundary_base*/ mload(0x80),
                    PRIME),
                  sub(PRIME, /*periodic_column/boundary_vault_id*/ mload(0x160)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[100] + coefficients[101] * adjustments[13])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[100]*/ mload(0x1020),
                                       mulmod(/*coefficients[101]*/ mload(0x1040),
                                              /*adjustments[13]*/mload(0x4120),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // amounts_range_check/bit_0 = column9_row4 - (column9_row132 + column9_row132)
              let val := addmod(
                /*column9_row4*/ mload(0x2ca0),
                sub(
                  PRIME,
                  addmod(/*column9_row132*/ mload(0x2ee0), /*column9_row132*/ mload(0x2ee0), PRIME)),
                PRIME)
              mstore(0x33e0, val)
              }

              {
              // Constraint expression for amounts_range_check/bit: amounts_range_check__bit_0 * amounts_range_check__bit_0 - amounts_range_check__bit_0.
              let val := addmod(
                mulmod(
                  /*intermediate_value/amounts_range_check/bit_0*/ mload(0x33e0),
                  /*intermediate_value/amounts_range_check/bit_0*/ mload(0x33e0),
                  PRIME),
                sub(PRIME, /*intermediate_value/amounts_range_check/bit_0*/ mload(0x33e0)),
                PRIME)

              // Numerator: point^(trace_length / 8192) - trace_generator^(63 * trace_length / 64)
              // val *= numerators[6]
              val := mulmod(val, mload(0x3ee0), PRIME)
              // Denominator: point^(trace_length / 128) - 1
              // val *= denominator_invs[14]
              val := mulmod(val, mload(0x3960), PRIME)

              // res += val * (coefficients[102] + coefficients[103] * adjustments[14])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[102]*/ mload(0x1060),
                                       mulmod(/*coefficients[103]*/ mload(0x1080),
                                              /*adjustments[14]*/mload(0x4140),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for amounts_range_check/zero: column9_row4.
              let val := /*column9_row4*/ mload(0x2ca0)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 8192) - trace_generator^(63 * trace_length / 64)
              // val *= denominator_invs[15]
              val := mulmod(val, mload(0x3980), PRIME)

              // res += val * (coefficients[104] + coefficients[105] * adjustments[15])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[104]*/ mload(0x10a0),
                                       mulmod(/*coefficients[105]*/ mload(0x10c0),
                                              /*adjustments[15]*/mload(0x4160),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for total_token_a_not_changed: is_settlement * (column8_row3075 - column8_row11267 - (column8_row27651 - column8_row19459)).
              let val := mulmod(
                /*periodic_column/is_settlement*/ mload(0xc0),
                addmod(
                  addmod(
                    /*column8_row3075*/ mload(0x2940),
                    sub(PRIME, /*column8_row11267*/ mload(0x2aa0)),
                    PRIME),
                  sub(
                    PRIME,
                    addmod(
                      /*column8_row27651*/ mload(0x2b40),
                      sub(PRIME, /*column8_row19459*/ mload(0x2b00)),
                      PRIME)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[106] + coefficients[107] * adjustments[16])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[106]*/ mload(0x10e0),
                                       mulmod(/*coefficients[107]*/ mload(0x1100),
                                              /*adjustments[16]*/mload(0x4180),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for total_token_b_not_changed: is_settlement * (column8_row35843 - column8_row44035 - (column8_row60419 - column8_row52227)).
              let val := mulmod(
                /*periodic_column/is_settlement*/ mload(0xc0),
                addmod(
                  addmod(
                    /*column8_row35843*/ mload(0x2b80),
                    sub(PRIME, /*column8_row44035*/ mload(0x2c20)),
                    PRIME),
                  sub(
                    PRIME,
                    addmod(
                      /*column8_row60419*/ mload(0x2c60),
                      sub(PRIME, /*column8_row52227*/ mload(0x2c40)),
                      PRIME)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[108] + coefficients[109] * adjustments[16])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[108]*/ mload(0x1120),
                                       mulmod(/*coefficients[109]*/ mload(0x1140),
                                              /*adjustments[16]*/mload(0x4180),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for diff_a_range_check_input: (column9_row4 - (column8_row3075 - column8_row11267)) * is_settlement.
              let val := mulmod(
                addmod(
                  /*column9_row4*/ mload(0x2ca0),
                  sub(
                    PRIME,
                    addmod(
                      /*column8_row3075*/ mload(0x2940),
                      sub(PRIME, /*column8_row11267*/ mload(0x2aa0)),
                      PRIME)),
                  PRIME),
                /*periodic_column/is_settlement*/ mload(0xc0),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[110] + coefficients[111] * adjustments[16])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[110]*/ mload(0x1160),
                                       mulmod(/*coefficients[111]*/ mload(0x1180),
                                              /*adjustments[16]*/mload(0x4180),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for diff_b_range_check_input: (column9_row32772 - (column8_row35843 - column8_row44035)) * is_settlement.
              let val := mulmod(
                addmod(
                  /*column9_row32772*/ mload(0x3180),
                  sub(
                    PRIME,
                    addmod(
                      /*column8_row35843*/ mload(0x2b80),
                      sub(PRIME, /*column8_row44035*/ mload(0x2c20)),
                      PRIME)),
                  PRIME),
                /*periodic_column/is_settlement*/ mload(0xc0),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[112] + coefficients[113] * adjustments[16])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[112]*/ mload(0x11a0),
                                       mulmod(/*coefficients[113]*/ mload(0x11c0),
                                              /*adjustments[16]*/mload(0x4180),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for amounts_range_check_inputs: column9_row8196 - column8_row11267.
              let val := addmod(
                /*column9_row8196*/ mload(0x2f60),
                sub(PRIME, /*column8_row11267*/ mload(0x2aa0)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 16384) - 1
              // val *= denominator_invs[12]
              val := mulmod(val, mload(0x3920), PRIME)

              // res += val * (coefficients[114] + coefficients[115] * adjustments[6])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[114]*/ mload(0x11e0),
                                       mulmod(/*coefficients[115]*/ mload(0x1200),
                                              /*adjustments[6]*/mload(0x4040),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // settlement_id_range_check/bit_0 = column8_row1021 - (column8_row3069 + column8_row3069)
              let val := addmod(
                /*column8_row1021*/ mload(0x2820),
                sub(
                  PRIME,
                  addmod(/*column8_row3069*/ mload(0x2920), /*column8_row3069*/ mload(0x2920), PRIME)),
                PRIME)
              mstore(0x3400, val)
              }

              {
              // Constraint expression for settlement_id_range_check/bit: settlement_id_range_check__bit_0 * settlement_id_range_check__bit_0 - settlement_id_range_check__bit_0.
              let val := addmod(
                mulmod(
                  /*intermediate_value/settlement_id_range_check/bit_0*/ mload(0x3400),
                  /*intermediate_value/settlement_id_range_check/bit_0*/ mload(0x3400),
                  PRIME),
                sub(PRIME, /*intermediate_value/settlement_id_range_check/bit_0*/ mload(0x3400)),
                PRIME)

              // Numerator: point^(trace_length / 65536) - trace_generator^(31 * trace_length / 32)
              // val *= numerators[7]
              val := mulmod(val, mload(0x3f00), PRIME)
              // Denominator: point^(trace_length / 2048) - 1
              // val *= denominator_invs[4]
              val := mulmod(val, mload(0x3820), PRIME)

              // res += val * (coefficients[116] + coefficients[117] * adjustments[17])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[116]*/ mload(0x1220),
                                       mulmod(/*coefficients[117]*/ mload(0x1240),
                                              /*adjustments[17]*/mload(0x41a0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for settlement_id_range_check/zero: column8_row1021.
              let val := /*column8_row1021*/ mload(0x2820)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - trace_generator^(31 * trace_length / 32)
              // val *= denominator_invs[16]
              val := mulmod(val, mload(0x39a0), PRIME)

              // res += val * (coefficients[118] + coefficients[119] * adjustments[18])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[118]*/ mload(0x1260),
                                       mulmod(/*coefficients[119]*/ mload(0x1280),
                                              /*adjustments[18]*/mload(0x41c0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // sig_verify/doubling_key/x_squared = column9_row0 * column9_row0
              let val := mulmod(/*column9_row0*/ mload(0x2c80), /*column9_row0*/ mload(0x2c80), PRIME)
              mstore(0x3420, val)
              }

              {
              // Constraint expression for sig_verify/doubling_key/slope: sig_verify__doubling_key__x_squared + sig_verify__doubling_key__x_squared + sig_verify__doubling_key__x_squared + sig_config.alpha - (column9_row32 + column9_row32) * column9_row16.
              let val := addmod(
                addmod(
                  addmod(
                    addmod(
                      /*intermediate_value/sig_verify/doubling_key/x_squared*/ mload(0x3420),
                      /*intermediate_value/sig_verify/doubling_key/x_squared*/ mload(0x3420),
                      PRIME),
                    /*intermediate_value/sig_verify/doubling_key/x_squared*/ mload(0x3420),
                    PRIME),
                  /*sig_config.alpha*/ mload(0x240),
                  PRIME),
                sub(
                  PRIME,
                  mulmod(
                    addmod(/*column9_row32*/ mload(0x2d40), /*column9_row32*/ mload(0x2d40), PRIME),
                    /*column9_row16*/ mload(0x2ce0),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 16384) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[8]
              val := mulmod(val, mload(0x3f20), PRIME)
              // Denominator: point^(trace_length / 64) - 1
              // val *= denominator_invs[17]
              val := mulmod(val, mload(0x39c0), PRIME)

              // res += val * (coefficients[120] + coefficients[121] * adjustments[19])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[120]*/ mload(0x12a0),
                                       mulmod(/*coefficients[121]*/ mload(0x12c0),
                                              /*adjustments[19]*/mload(0x41e0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/doubling_key/x: column9_row16 * column9_row16 - (column9_row0 + column9_row0 + column9_row64).
              let val := addmod(
                mulmod(/*column9_row16*/ mload(0x2ce0), /*column9_row16*/ mload(0x2ce0), PRIME),
                sub(
                  PRIME,
                  addmod(
                    addmod(/*column9_row0*/ mload(0x2c80), /*column9_row0*/ mload(0x2c80), PRIME),
                    /*column9_row64*/ mload(0x2de0),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 16384) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[8]
              val := mulmod(val, mload(0x3f20), PRIME)
              // Denominator: point^(trace_length / 64) - 1
              // val *= denominator_invs[17]
              val := mulmod(val, mload(0x39c0), PRIME)

              // res += val * (coefficients[122] + coefficients[123] * adjustments[19])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[122]*/ mload(0x12e0),
                                       mulmod(/*coefficients[123]*/ mload(0x1300),
                                              /*adjustments[19]*/mload(0x41e0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/doubling_key/y: column9_row32 + column9_row96 - column9_row16 * (column9_row0 - column9_row64).
              let val := addmod(
                addmod(/*column9_row32*/ mload(0x2d40), /*column9_row96*/ mload(0x2e80), PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*column9_row16*/ mload(0x2ce0),
                    addmod(/*column9_row0*/ mload(0x2c80), sub(PRIME, /*column9_row64*/ mload(0x2de0)), PRIME),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 16384) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[8]
              val := mulmod(val, mload(0x3f20), PRIME)
              // Denominator: point^(trace_length / 64) - 1
              // val *= denominator_invs[17]
              val := mulmod(val, mload(0x39c0), PRIME)

              // res += val * (coefficients[124] + coefficients[125] * adjustments[19])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[124]*/ mload(0x1320),
                                       mulmod(/*coefficients[125]*/ mload(0x1340),
                                              /*adjustments[19]*/mload(0x41e0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // sig_verify/exponentiate_generator/bit = column9_row20 - (column9_row148 + column9_row148)
              let val := addmod(
                /*column9_row20*/ mload(0x2d00),
                sub(
                  PRIME,
                  addmod(/*column9_row148*/ mload(0x2f00), /*column9_row148*/ mload(0x2f00), PRIME)),
                PRIME)
              mstore(0x3440, val)
              }

              {
              // Constraint expression for sig_verify/exponentiate_generator/booleanity_test: sig_verify__exponentiate_generator__bit * (sig_verify__exponentiate_generator__bit - 1).
              let val := mulmod(
                /*intermediate_value/sig_verify/exponentiate_generator/bit*/ mload(0x3440),
                addmod(
                  /*intermediate_value/sig_verify/exponentiate_generator/bit*/ mload(0x3440),
                  sub(PRIME, 1),
                  PRIME),
                PRIME)

              // Numerator: point^(trace_length / 32768) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[9]
              val := mulmod(val, mload(0x3f40), PRIME)
              // Denominator: point^(trace_length / 128) - 1
              // val *= denominator_invs[14]
              val := mulmod(val, mload(0x3960), PRIME)

              // res += val * (coefficients[126] + coefficients[127] * adjustments[20])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[126]*/ mload(0x1360),
                                       mulmod(/*coefficients[127]*/ mload(0x1380),
                                              /*adjustments[20]*/mload(0x4200),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/exponentiate_generator/bit_extraction_end: column9_row20.
              let val := /*column9_row20*/ mload(0x2d00)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 32768) - trace_generator^(251 * trace_length / 256)
              // val *= denominator_invs[18]
              val := mulmod(val, mload(0x39e0), PRIME)

              // res += val * (coefficients[128] + coefficients[129] * adjustments[21])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[128]*/ mload(0x13a0),
                                       mulmod(/*coefficients[129]*/ mload(0x13c0),
                                              /*adjustments[21]*/mload(0x4220),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/exponentiate_generator/zeros_tail: column9_row20.
              let val := /*column9_row20*/ mload(0x2d00)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 32768) - trace_generator^(255 * trace_length / 256)
              // val *= denominator_invs[19]
              val := mulmod(val, mload(0x3a00), PRIME)

              // res += val * (coefficients[130] + coefficients[131] * adjustments[21])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[130]*/ mload(0x13e0),
                                       mulmod(/*coefficients[131]*/ mload(0x1400),
                                              /*adjustments[21]*/mload(0x4220),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/exponentiate_generator/add_points/slope: sig_verify__exponentiate_generator__bit * (column9_row36 - ecdsa_points__y) - column9_row100 * (column9_row68 - ecdsa_points__x).
              let val := addmod(
                mulmod(
                  /*intermediate_value/sig_verify/exponentiate_generator/bit*/ mload(0x3440),
                  addmod(
                    /*column9_row36*/ mload(0x2d60),
                    sub(PRIME, /*periodic_column/ecdsa_points/y*/ mload(0x1a0)),
                    PRIME),
                  PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*column9_row100*/ mload(0x2ea0),
                    addmod(
                      /*column9_row68*/ mload(0x2e00),
                      sub(PRIME, /*periodic_column/ecdsa_points/x*/ mload(0x180)),
                      PRIME),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 32768) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[9]
              val := mulmod(val, mload(0x3f40), PRIME)
              // Denominator: point^(trace_length / 128) - 1
              // val *= denominator_invs[14]
              val := mulmod(val, mload(0x3960), PRIME)

              // res += val * (coefficients[132] + coefficients[133] * adjustments[20])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[132]*/ mload(0x1420),
                                       mulmod(/*coefficients[133]*/ mload(0x1440),
                                              /*adjustments[20]*/mload(0x4200),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/exponentiate_generator/add_points/x: column9_row100 * column9_row100 - sig_verify__exponentiate_generator__bit * (column9_row68 + ecdsa_points__x + column9_row196).
              let val := addmod(
                mulmod(/*column9_row100*/ mload(0x2ea0), /*column9_row100*/ mload(0x2ea0), PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*intermediate_value/sig_verify/exponentiate_generator/bit*/ mload(0x3440),
                    addmod(
                      addmod(
                        /*column9_row68*/ mload(0x2e00),
                        /*periodic_column/ecdsa_points/x*/ mload(0x180),
                        PRIME),
                      /*column9_row196*/ mload(0x2f40),
                      PRIME),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 32768) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[9]
              val := mulmod(val, mload(0x3f40), PRIME)
              // Denominator: point^(trace_length / 128) - 1
              // val *= denominator_invs[14]
              val := mulmod(val, mload(0x3960), PRIME)

              // res += val * (coefficients[134] + coefficients[135] * adjustments[20])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[134]*/ mload(0x1460),
                                       mulmod(/*coefficients[135]*/ mload(0x1480),
                                              /*adjustments[20]*/mload(0x4200),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/exponentiate_generator/add_points/y: sig_verify__exponentiate_generator__bit * (column9_row36 + column9_row164) - column9_row100 * (column9_row68 - column9_row196).
              let val := addmod(
                mulmod(
                  /*intermediate_value/sig_verify/exponentiate_generator/bit*/ mload(0x3440),
                  addmod(/*column9_row36*/ mload(0x2d60), /*column9_row164*/ mload(0x2f20), PRIME),
                  PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*column9_row100*/ mload(0x2ea0),
                    addmod(
                      /*column9_row68*/ mload(0x2e00),
                      sub(PRIME, /*column9_row196*/ mload(0x2f40)),
                      PRIME),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 32768) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[9]
              val := mulmod(val, mload(0x3f40), PRIME)
              // Denominator: point^(trace_length / 128) - 1
              // val *= denominator_invs[14]
              val := mulmod(val, mload(0x3960), PRIME)

              // res += val * (coefficients[136] + coefficients[137] * adjustments[20])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[136]*/ mload(0x14a0),
                                       mulmod(/*coefficients[137]*/ mload(0x14c0),
                                              /*adjustments[20]*/mload(0x4200),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/exponentiate_generator/add_points/x_diff_inv: column9_row84 * (column9_row68 - ecdsa_points__x) - 1.
              let val := addmod(
                mulmod(
                  /*column9_row84*/ mload(0x2e40),
                  addmod(
                    /*column9_row68*/ mload(0x2e00),
                    sub(PRIME, /*periodic_column/ecdsa_points/x*/ mload(0x180)),
                    PRIME),
                  PRIME),
                sub(PRIME, 1),
                PRIME)

              // Numerator: point^(trace_length / 32768) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[9]
              val := mulmod(val, mload(0x3f40), PRIME)
              // Denominator: point^(trace_length / 128) - 1
              // val *= denominator_invs[14]
              val := mulmod(val, mload(0x3960), PRIME)

              // res += val * (coefficients[138] + coefficients[139] * adjustments[20])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[138]*/ mload(0x14e0),
                                       mulmod(/*coefficients[139]*/ mload(0x1500),
                                              /*adjustments[20]*/mload(0x4200),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // sig_verify/exponentiate_generator/bit_neg = 1 - sig_verify__exponentiate_generator__bit
              let val := addmod(
                1,
                sub(PRIME, /*intermediate_value/sig_verify/exponentiate_generator/bit*/ mload(0x3440)),
                PRIME)
              mstore(0x3460, val)
              }

              {
              // Constraint expression for sig_verify/exponentiate_generator/copy_point/x: sig_verify__exponentiate_generator__bit_neg * (column9_row196 - column9_row68).
              let val := mulmod(
                /*intermediate_value/sig_verify/exponentiate_generator/bit_neg*/ mload(0x3460),
                addmod(
                  /*column9_row196*/ mload(0x2f40),
                  sub(PRIME, /*column9_row68*/ mload(0x2e00)),
                  PRIME),
                PRIME)

              // Numerator: point^(trace_length / 32768) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[9]
              val := mulmod(val, mload(0x3f40), PRIME)
              // Denominator: point^(trace_length / 128) - 1
              // val *= denominator_invs[14]
              val := mulmod(val, mload(0x3960), PRIME)

              // res += val * (coefficients[140] + coefficients[141] * adjustments[20])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[140]*/ mload(0x1520),
                                       mulmod(/*coefficients[141]*/ mload(0x1540),
                                              /*adjustments[20]*/mload(0x4200),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/exponentiate_generator/copy_point/y: sig_verify__exponentiate_generator__bit_neg * (column9_row164 - column9_row36).
              let val := mulmod(
                /*intermediate_value/sig_verify/exponentiate_generator/bit_neg*/ mload(0x3460),
                addmod(
                  /*column9_row164*/ mload(0x2f20),
                  sub(PRIME, /*column9_row36*/ mload(0x2d60)),
                  PRIME),
                PRIME)

              // Numerator: point^(trace_length / 32768) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[9]
              val := mulmod(val, mload(0x3f40), PRIME)
              // Denominator: point^(trace_length / 128) - 1
              // val *= denominator_invs[14]
              val := mulmod(val, mload(0x3960), PRIME)

              // res += val * (coefficients[142] + coefficients[143] * adjustments[20])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[142]*/ mload(0x1560),
                                       mulmod(/*coefficients[143]*/ mload(0x1580),
                                              /*adjustments[20]*/mload(0x4200),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // sig_verify/exponentiate_key/bit = column9_row24 - (column9_row88 + column9_row88)
              let val := addmod(
                /*column9_row24*/ mload(0x2d20),
                sub(
                  PRIME,
                  addmod(/*column9_row88*/ mload(0x2e60), /*column9_row88*/ mload(0x2e60), PRIME)),
                PRIME)
              mstore(0x3480, val)
              }

              {
              // Constraint expression for sig_verify/exponentiate_key/booleanity_test: sig_verify__exponentiate_key__bit * (sig_verify__exponentiate_key__bit - 1).
              let val := mulmod(
                /*intermediate_value/sig_verify/exponentiate_key/bit*/ mload(0x3480),
                addmod(
                  /*intermediate_value/sig_verify/exponentiate_key/bit*/ mload(0x3480),
                  sub(PRIME, 1),
                  PRIME),
                PRIME)

              // Numerator: point^(trace_length / 16384) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[8]
              val := mulmod(val, mload(0x3f20), PRIME)
              // Denominator: point^(trace_length / 64) - 1
              // val *= denominator_invs[17]
              val := mulmod(val, mload(0x39c0), PRIME)

              // res += val * (coefficients[144] + coefficients[145] * adjustments[19])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[144]*/ mload(0x15a0),
                                       mulmod(/*coefficients[145]*/ mload(0x15c0),
                                              /*adjustments[19]*/mload(0x41e0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/exponentiate_key/bit_extraction_end: column9_row24.
              let val := /*column9_row24*/ mload(0x2d20)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 16384) - trace_generator^(251 * trace_length / 256)
              // val *= denominator_invs[20]
              val := mulmod(val, mload(0x3a20), PRIME)

              // res += val * (coefficients[146] + coefficients[147] * adjustments[6])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[146]*/ mload(0x15e0),
                                       mulmod(/*coefficients[147]*/ mload(0x1600),
                                              /*adjustments[6]*/mload(0x4040),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/exponentiate_key/zeros_tail: column9_row24.
              let val := /*column9_row24*/ mload(0x2d20)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 16384) - trace_generator^(255 * trace_length / 256)
              // val *= denominator_invs[21]
              val := mulmod(val, mload(0x3a40), PRIME)

              // res += val * (coefficients[148] + coefficients[149] * adjustments[6])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[148]*/ mload(0x1620),
                                       mulmod(/*coefficients[149]*/ mload(0x1640),
                                              /*adjustments[6]*/mload(0x4040),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/exponentiate_key/add_points/slope: sig_verify__exponentiate_key__bit * (column9_row8 - column9_row32) - column9_row40 * (column9_row48 - column9_row0).
              let val := addmod(
                mulmod(
                  /*intermediate_value/sig_verify/exponentiate_key/bit*/ mload(0x3480),
                  addmod(/*column9_row8*/ mload(0x2cc0), sub(PRIME, /*column9_row32*/ mload(0x2d40)), PRIME),
                  PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*column9_row40*/ mload(0x2d80),
                    addmod(/*column9_row48*/ mload(0x2da0), sub(PRIME, /*column9_row0*/ mload(0x2c80)), PRIME),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 16384) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[8]
              val := mulmod(val, mload(0x3f20), PRIME)
              // Denominator: point^(trace_length / 64) - 1
              // val *= denominator_invs[17]
              val := mulmod(val, mload(0x39c0), PRIME)

              // res += val * (coefficients[150] + coefficients[151] * adjustments[19])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[150]*/ mload(0x1660),
                                       mulmod(/*coefficients[151]*/ mload(0x1680),
                                              /*adjustments[19]*/mload(0x41e0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/exponentiate_key/add_points/x: column9_row40 * column9_row40 - sig_verify__exponentiate_key__bit * (column9_row48 + column9_row0 + column9_row112).
              let val := addmod(
                mulmod(/*column9_row40*/ mload(0x2d80), /*column9_row40*/ mload(0x2d80), PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*intermediate_value/sig_verify/exponentiate_key/bit*/ mload(0x3480),
                    addmod(
                      addmod(/*column9_row48*/ mload(0x2da0), /*column9_row0*/ mload(0x2c80), PRIME),
                      /*column9_row112*/ mload(0x2ec0),
                      PRIME),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 16384) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[8]
              val := mulmod(val, mload(0x3f20), PRIME)
              // Denominator: point^(trace_length / 64) - 1
              // val *= denominator_invs[17]
              val := mulmod(val, mload(0x39c0), PRIME)

              // res += val * (coefficients[152] + coefficients[153] * adjustments[19])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[152]*/ mload(0x16a0),
                                       mulmod(/*coefficients[153]*/ mload(0x16c0),
                                              /*adjustments[19]*/mload(0x41e0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/exponentiate_key/add_points/y: sig_verify__exponentiate_key__bit * (column9_row8 + column9_row72) - column9_row40 * (column9_row48 - column9_row112).
              let val := addmod(
                mulmod(
                  /*intermediate_value/sig_verify/exponentiate_key/bit*/ mload(0x3480),
                  addmod(/*column9_row8*/ mload(0x2cc0), /*column9_row72*/ mload(0x2e20), PRIME),
                  PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*column9_row40*/ mload(0x2d80),
                    addmod(
                      /*column9_row48*/ mload(0x2da0),
                      sub(PRIME, /*column9_row112*/ mload(0x2ec0)),
                      PRIME),
                    PRIME)),
                PRIME)

              // Numerator: point^(trace_length / 16384) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[8]
              val := mulmod(val, mload(0x3f20), PRIME)
              // Denominator: point^(trace_length / 64) - 1
              // val *= denominator_invs[17]
              val := mulmod(val, mload(0x39c0), PRIME)

              // res += val * (coefficients[154] + coefficients[155] * adjustments[19])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[154]*/ mload(0x16e0),
                                       mulmod(/*coefficients[155]*/ mload(0x1700),
                                              /*adjustments[19]*/mload(0x41e0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/exponentiate_key/add_points/x_diff_inv: column9_row56 * (column9_row48 - column9_row0) - 1.
              let val := addmod(
                mulmod(
                  /*column9_row56*/ mload(0x2dc0),
                  addmod(/*column9_row48*/ mload(0x2da0), sub(PRIME, /*column9_row0*/ mload(0x2c80)), PRIME),
                  PRIME),
                sub(PRIME, 1),
                PRIME)

              // Numerator: point^(trace_length / 16384) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[8]
              val := mulmod(val, mload(0x3f20), PRIME)
              // Denominator: point^(trace_length / 64) - 1
              // val *= denominator_invs[17]
              val := mulmod(val, mload(0x39c0), PRIME)

              // res += val * (coefficients[156] + coefficients[157] * adjustments[19])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[156]*/ mload(0x1720),
                                       mulmod(/*coefficients[157]*/ mload(0x1740),
                                              /*adjustments[19]*/mload(0x41e0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // sig_verify/exponentiate_key/bit_neg = 1 - sig_verify__exponentiate_key__bit
              let val := addmod(
                1,
                sub(PRIME, /*intermediate_value/sig_verify/exponentiate_key/bit*/ mload(0x3480)),
                PRIME)
              mstore(0x34a0, val)
              }

              {
              // Constraint expression for sig_verify/exponentiate_key/copy_point/x: sig_verify__exponentiate_key__bit_neg * (column9_row112 - column9_row48).
              let val := mulmod(
                /*intermediate_value/sig_verify/exponentiate_key/bit_neg*/ mload(0x34a0),
                addmod(
                  /*column9_row112*/ mload(0x2ec0),
                  sub(PRIME, /*column9_row48*/ mload(0x2da0)),
                  PRIME),
                PRIME)

              // Numerator: point^(trace_length / 16384) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[8]
              val := mulmod(val, mload(0x3f20), PRIME)
              // Denominator: point^(trace_length / 64) - 1
              // val *= denominator_invs[17]
              val := mulmod(val, mload(0x39c0), PRIME)

              // res += val * (coefficients[158] + coefficients[159] * adjustments[19])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[158]*/ mload(0x1760),
                                       mulmod(/*coefficients[159]*/ mload(0x1780),
                                              /*adjustments[19]*/mload(0x41e0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/exponentiate_key/copy_point/y: sig_verify__exponentiate_key__bit_neg * (column9_row72 - column9_row8).
              let val := mulmod(
                /*intermediate_value/sig_verify/exponentiate_key/bit_neg*/ mload(0x34a0),
                addmod(/*column9_row72*/ mload(0x2e20), sub(PRIME, /*column9_row8*/ mload(0x2cc0)), PRIME),
                PRIME)

              // Numerator: point^(trace_length / 16384) - trace_generator^(255 * trace_length / 256)
              // val *= numerators[8]
              val := mulmod(val, mload(0x3f20), PRIME)
              // Denominator: point^(trace_length / 64) - 1
              // val *= denominator_invs[17]
              val := mulmod(val, mload(0x39c0), PRIME)

              // res += val * (coefficients[160] + coefficients[161] * adjustments[19])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[160]*/ mload(0x17a0),
                                       mulmod(/*coefficients[161]*/ mload(0x17c0),
                                              /*adjustments[19]*/mload(0x41e0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/init_gen/x: column9_row68 - sig_config.shift_point.x.
              let val := addmod(/*column9_row68*/ mload(0x2e00), sub(PRIME, /*shift_point.x*/ mload(0x1e0)), PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 32768) - 1
              // val *= denominator_invs[22]
              val := mulmod(val, mload(0x3a60), PRIME)

              // res += val * (coefficients[162] + coefficients[163] * adjustments[21])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[162]*/ mload(0x17e0),
                                       mulmod(/*coefficients[163]*/ mload(0x1800),
                                              /*adjustments[21]*/mload(0x4220),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/init_gen/y: column9_row36 + sig_config.shift_point.y.
              let val := addmod(/*column9_row36*/ mload(0x2d60), /*shift_point.y*/ mload(0x200), PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 32768) - 1
              // val *= denominator_invs[22]
              val := mulmod(val, mload(0x3a60), PRIME)

              // res += val * (coefficients[164] + coefficients[165] * adjustments[21])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[164]*/ mload(0x1820),
                                       mulmod(/*coefficients[165]*/ mload(0x1840),
                                              /*adjustments[21]*/mload(0x4220),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/init_key/x: column9_row48 - sig_config.shift_point.x.
              let val := addmod(/*column9_row48*/ mload(0x2da0), sub(PRIME, /*shift_point.x*/ mload(0x1e0)), PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 16384) - 1
              // val *= denominator_invs[12]
              val := mulmod(val, mload(0x3920), PRIME)

              // res += val * (coefficients[166] + coefficients[167] * adjustments[6])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[166]*/ mload(0x1860),
                                       mulmod(/*coefficients[167]*/ mload(0x1880),
                                              /*adjustments[6]*/mload(0x4040),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/init_key/y: column9_row8 - sig_config.shift_point.y.
              let val := addmod(/*column9_row8*/ mload(0x2cc0), sub(PRIME, /*shift_point.y*/ mload(0x200)), PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 16384) - 1
              // val *= denominator_invs[12]
              val := mulmod(val, mload(0x3920), PRIME)

              // res += val * (coefficients[168] + coefficients[169] * adjustments[6])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[168]*/ mload(0x18a0),
                                       mulmod(/*coefficients[169]*/ mload(0x18c0),
                                              /*adjustments[6]*/mload(0x4040),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/add_results/slope: column9_row32676 - column9_row16328 - column9_row32724 * (column9_row32708 - column9_row16368).
              let val := addmod(
                addmod(
                  /*column9_row32676*/ mload(0x3060),
                  sub(PRIME, /*column9_row16328*/ mload(0x2f80)),
                  PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*column9_row32724*/ mload(0x30c0),
                    addmod(
                      /*column9_row32708*/ mload(0x3080),
                      sub(PRIME, /*column9_row16368*/ mload(0x2fe0)),
                      PRIME),
                    PRIME)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 32768) - 1
              // val *= denominator_invs[22]
              val := mulmod(val, mload(0x3a60), PRIME)

              // res += val * (coefficients[170] + coefficients[171] * adjustments[22])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[170]*/ mload(0x18e0),
                                       mulmod(/*coefficients[171]*/ mload(0x1900),
                                              /*adjustments[22]*/mload(0x4240),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/add_results/x: column9_row32724 * column9_row32724 - (column9_row32708 + column9_row16368 + column9_row16384).
              let val := addmod(
                mulmod(/*column9_row32724*/ mload(0x30c0), /*column9_row32724*/ mload(0x30c0), PRIME),
                sub(
                  PRIME,
                  addmod(
                    addmod(/*column9_row32708*/ mload(0x3080), /*column9_row16368*/ mload(0x2fe0), PRIME),
                    /*column9_row16384*/ mload(0x3020),
                    PRIME)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 32768) - 1
              // val *= denominator_invs[22]
              val := mulmod(val, mload(0x3a60), PRIME)

              // res += val * (coefficients[172] + coefficients[173] * adjustments[22])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[172]*/ mload(0x1920),
                                       mulmod(/*coefficients[173]*/ mload(0x1940),
                                              /*adjustments[22]*/mload(0x4240),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/add_results/y: column9_row32676 + column9_row16416 - column9_row32724 * (column9_row32708 - column9_row16384).
              let val := addmod(
                addmod(/*column9_row32676*/ mload(0x3060), /*column9_row16416*/ mload(0x3040), PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*column9_row32724*/ mload(0x30c0),
                    addmod(
                      /*column9_row32708*/ mload(0x3080),
                      sub(PRIME, /*column9_row16384*/ mload(0x3020)),
                      PRIME),
                    PRIME)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 32768) - 1
              // val *= denominator_invs[22]
              val := mulmod(val, mload(0x3a60), PRIME)

              // res += val * (coefficients[174] + coefficients[175] * adjustments[22])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[174]*/ mload(0x1960),
                                       mulmod(/*coefficients[175]*/ mload(0x1980),
                                              /*adjustments[22]*/mload(0x4240),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/add_results/x_diff_inv: column9_row32740 * (column9_row32708 - column9_row16368) - 1.
              let val := addmod(
                mulmod(
                  /*column9_row32740*/ mload(0x30e0),
                  addmod(
                    /*column9_row32708*/ mload(0x3080),
                    sub(PRIME, /*column9_row16368*/ mload(0x2fe0)),
                    PRIME),
                  PRIME),
                sub(PRIME, 1),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 32768) - 1
              // val *= denominator_invs[22]
              val := mulmod(val, mload(0x3a60), PRIME)

              // res += val * (coefficients[176] + coefficients[177] * adjustments[22])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[176]*/ mload(0x19a0),
                                       mulmod(/*coefficients[177]*/ mload(0x19c0),
                                              /*adjustments[22]*/mload(0x4240),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/extract_r/slope: column9_row32712 + sig_config.shift_point.y - column8_row4093 * (column9_row32752 - sig_config.shift_point.x).
              let val := addmod(
                addmod(/*column9_row32712*/ mload(0x30a0), /*shift_point.y*/ mload(0x200), PRIME),
                sub(
                  PRIME,
                  mulmod(
                    /*column8_row4093*/ mload(0x2980),
                    addmod(
                      /*column9_row32752*/ mload(0x3120),
                      sub(PRIME, /*shift_point.x*/ mload(0x1e0)),
                      PRIME),
                    PRIME)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 32768) - 1
              // val *= denominator_invs[22]
              val := mulmod(val, mload(0x3a60), PRIME)

              // res += val * (coefficients[178] + coefficients[179] * adjustments[22])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[178]*/ mload(0x19e0),
                                       mulmod(/*coefficients[179]*/ mload(0x1a00),
                                              /*adjustments[22]*/mload(0x4240),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/extract_r/x: column8_row4093 * column8_row4093 - (column9_row32752 + sig_config.shift_point.x + column9_row24).
              let val := addmod(
                mulmod(/*column8_row4093*/ mload(0x2980), /*column8_row4093*/ mload(0x2980), PRIME),
                sub(
                  PRIME,
                  addmod(
                    addmod(/*column9_row32752*/ mload(0x3120), /*shift_point.x*/ mload(0x1e0), PRIME),
                    /*column9_row24*/ mload(0x2d20),
                    PRIME)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 32768) - 1
              // val *= denominator_invs[22]
              val := mulmod(val, mload(0x3a60), PRIME)

              // res += val * (coefficients[180] + coefficients[181] * adjustments[22])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[180]*/ mload(0x1a20),
                                       mulmod(/*coefficients[181]*/ mload(0x1a40),
                                              /*adjustments[22]*/mload(0x4240),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/extract_r/x_diff_inv: column8_row20477 * (column9_row32752 - sig_config.shift_point.x) - 1.
              let val := addmod(
                mulmod(
                  /*column8_row20477*/ mload(0x2b20),
                  addmod(
                    /*column9_row32752*/ mload(0x3120),
                    sub(PRIME, /*shift_point.x*/ mload(0x1e0)),
                    PRIME),
                  PRIME),
                sub(PRIME, 1),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 32768) - 1
              // val *= denominator_invs[22]
              val := mulmod(val, mload(0x3a60), PRIME)

              // res += val * (coefficients[182] + coefficients[183] * adjustments[22])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[182]*/ mload(0x1a60),
                                       mulmod(/*coefficients[183]*/ mload(0x1a80),
                                              /*adjustments[22]*/mload(0x4240),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/z_nonzero: column9_row20 * column8_row12285 - 1.
              let val := addmod(
                mulmod(/*column9_row20*/ mload(0x2d00), /*column8_row12285*/ mload(0x2ae0), PRIME),
                sub(PRIME, 1),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 32768) - 1
              // val *= denominator_invs[22]
              val := mulmod(val, mload(0x3a60), PRIME)

              // res += val * (coefficients[184] + coefficients[185] * adjustments[22])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[184]*/ mload(0x1aa0),
                                       mulmod(/*coefficients[185]*/ mload(0x1ac0),
                                              /*adjustments[22]*/mload(0x4240),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/r_and_w_nonzero: column9_row24 * column9_row16336 - 1.
              let val := addmod(
                mulmod(/*column9_row24*/ mload(0x2d20), /*column9_row16336*/ mload(0x2fa0), PRIME),
                sub(PRIME, 1),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 16384) - 1
              // val *= denominator_invs[12]
              val := mulmod(val, mload(0x3920), PRIME)

              // res += val * (coefficients[186] + coefficients[187] * adjustments[12])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[186]*/ mload(0x1ae0),
                                       mulmod(/*coefficients[187]*/ mload(0x1b00),
                                              /*adjustments[12]*/mload(0x4100),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/q_on_curve/x_squared: column8_row28669 - column9_row0 * column9_row0.
              let val := addmod(
                /*column8_row28669*/ mload(0x2b60),
                sub(
                  PRIME,
                  mulmod(/*column9_row0*/ mload(0x2c80), /*column9_row0*/ mload(0x2c80), PRIME)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 32768) - 1
              // val *= denominator_invs[22]
              val := mulmod(val, mload(0x3a60), PRIME)

              // res += val * (coefficients[188] + coefficients[189] * adjustments[22])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[188]*/ mload(0x1b20),
                                       mulmod(/*coefficients[189]*/ mload(0x1b40),
                                              /*adjustments[22]*/mload(0x4240),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for sig_verify/q_on_curve/on_curve: column9_row32 * column9_row32 - (column9_row0 * column8_row28669 + sig_config.alpha * column9_row0 + sig_config.beta).
              let val := addmod(
                mulmod(/*column9_row32*/ mload(0x2d40), /*column9_row32*/ mload(0x2d40), PRIME),
                sub(
                  PRIME,
                  addmod(
                    addmod(
                      mulmod(/*column9_row0*/ mload(0x2c80), /*column8_row28669*/ mload(0x2b60), PRIME),
                      mulmod(/*sig_config.alpha*/ mload(0x240), /*column9_row0*/ mload(0x2c80), PRIME),
                      PRIME),
                    /*sig_config.beta*/ mload(0x260),
                    PRIME)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 32768) - 1
              // val *= denominator_invs[22]
              val := mulmod(val, mload(0x3a60), PRIME)

              // res += val * (coefficients[190] + coefficients[191] * adjustments[22])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[190]*/ mload(0x1b60),
                                       mulmod(/*coefficients[191]*/ mload(0x1b80),
                                              /*adjustments[22]*/mload(0x4240),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // packed_message = (((column6_row255 * vault_shift + column6_row49407) * amount_shift + column9_row4) * amount_shift + column9_row32772) * trade_shift + column8_row1021
              let val := addmod(
                mulmod(
                  addmod(
                    mulmod(
                      addmod(
                        mulmod(
                          addmod(
                            mulmod(/*column6_row255*/ mload(0x25c0), /*vault_shift*/ mload(0x300), PRIME),
                            /*column6_row49407*/ mload(0x2660),
                            PRIME),
                          /*amount_shift*/ mload(0x320),
                          PRIME),
                        /*column9_row4*/ mload(0x2ca0),
                        PRIME),
                      /*amount_shift*/ mload(0x320),
                      PRIME),
                    /*column9_row32772*/ mload(0x3180),
                    PRIME),
                  /*trade_shift*/ mload(0x340),
                  PRIME),
                /*column8_row1021*/ mload(0x2820),
                PRIME)
              mstore(0x34c0, val)
              }

              {
              // Constraint expression for maker_sig_input_packed: is_settlement * (column8_row7171 - packed_message).
              let val := mulmod(
                /*periodic_column/is_settlement*/ mload(0xc0),
                addmod(
                  /*column8_row7171*/ mload(0x2a00),
                  sub(PRIME, /*intermediate_value/packed_message*/ mload(0x34c0)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[192] + coefficients[193] * adjustments[16])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[192]*/ mload(0x1ba0),
                                       mulmod(/*coefficients[193]*/ mload(0x1bc0),
                                              /*adjustments[16]*/mload(0x4180),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for taker_sig_input_maker_hash: is_settlement * (column8_row36867 - column8_row8188).
              let val := mulmod(
                /*periodic_column/is_settlement*/ mload(0xc0),
                addmod(
                  /*column8_row36867*/ mload(0x2ba0),
                  sub(PRIME, /*column8_row8188*/ mload(0x2a20)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[194] + coefficients[195] * adjustments[16])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[194]*/ mload(0x1be0),
                                       mulmod(/*coefficients[195]*/ mload(0x1c00),
                                              /*adjustments[16]*/mload(0x4180),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for taker_sig_input_vault_a: is_settlement * (column8_row37891 - column6_row16639).
              let val := mulmod(
                /*periodic_column/is_settlement*/ mload(0xc0),
                addmod(
                  /*column8_row37891*/ mload(0x2bc0),
                  sub(PRIME, /*column6_row16639*/ mload(0x2620)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[196] + coefficients[197] * adjustments[16])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[196]*/ mload(0x1c20),
                                       mulmod(/*coefficients[197]*/ mload(0x1c40),
                                              /*adjustments[16]*/mload(0x4180),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for taker_sig_input_vault_b: is_settlement * (column8_row39939 - column6_row33023).
              let val := mulmod(
                /*periodic_column/is_settlement*/ mload(0xc0),
                addmod(
                  /*column8_row39939*/ mload(0x2be0),
                  sub(PRIME, /*column6_row33023*/ mload(0x2640)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[198] + coefficients[199] * adjustments[16])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[198]*/ mload(0x1c60),
                                       mulmod(/*coefficients[199]*/ mload(0x1c80),
                                              /*adjustments[16]*/mload(0x4180),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for copy_signature_input_maker: is_settlement * (column8_row8188 - column9_row20).
              let val := mulmod(
                /*periodic_column/is_settlement*/ mload(0xc0),
                addmod(
                  /*column8_row8188*/ mload(0x2a20),
                  sub(PRIME, /*column9_row20*/ mload(0x2d00)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[200] + coefficients[201] * adjustments[16])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[200]*/ mload(0x1ca0),
                                       mulmod(/*coefficients[201]*/ mload(0x1cc0),
                                              /*adjustments[16]*/mload(0x4180),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for copy_signature_input_taker: is_settlement * (column8_row40956 - column9_row32788).
              let val := mulmod(
                /*periodic_column/is_settlement*/ mload(0xc0),
                addmod(
                  /*column8_row40956*/ mload(0x2c00),
                  sub(PRIME, /*column9_row32788*/ mload(0x31a0)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[202] + coefficients[203] * adjustments[16])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[202]*/ mload(0x1ce0),
                                       mulmod(/*coefficients[203]*/ mload(0x1d00),
                                              /*adjustments[16]*/mload(0x4180),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for handle_empty_vault/consistency_key_change0: is_settlement * (column9_row0 - column9_row16376).
              let val := mulmod(
                /*periodic_column/is_settlement*/ mload(0xc0),
                addmod(
                  /*column9_row0*/ mload(0x2c80),
                  sub(PRIME, /*column9_row16376*/ mload(0x3000)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[204] + coefficients[205] * adjustments[23])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[204]*/ mload(0x1d20),
                                       mulmod(/*coefficients[205]*/ mload(0x1d40),
                                              /*adjustments[23]*/mload(0x4260),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for handle_empty_vault/consistency_token_change0: is_settlement * (column8_row4099 - column9_row16360).
              let val := mulmod(
                /*periodic_column/is_settlement*/ mload(0xc0),
                addmod(
                  /*column8_row4099*/ mload(0x29a0),
                  sub(PRIME, /*column9_row16360*/ mload(0x2fc0)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[206] + coefficients[207] * adjustments[23])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[206]*/ mload(0x1d60),
                                       mulmod(/*coefficients[207]*/ mload(0x1d80),
                                              /*adjustments[23]*/mload(0x4260),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for handle_empty_vault/consistency_key_change3: is_settlement * (column9_row0 - column9_row65528).
              let val := mulmod(
                /*periodic_column/is_settlement*/ mload(0xc0),
                addmod(
                  /*column9_row0*/ mload(0x2c80),
                  sub(PRIME, /*column9_row65528*/ mload(0x3220)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[208] + coefficients[209] * adjustments[23])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[208]*/ mload(0x1da0),
                                       mulmod(/*coefficients[209]*/ mload(0x1dc0),
                                              /*adjustments[23]*/mload(0x4260),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for handle_empty_vault/consistency_token_change3: is_settlement * (column8_row5123 - column9_row65512).
              let val := mulmod(
                /*periodic_column/is_settlement*/ mload(0xc0),
                addmod(
                  /*column8_row5123*/ mload(0x29c0),
                  sub(PRIME, /*column9_row65512*/ mload(0x3200)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[210] + coefficients[211] * adjustments[23])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[210]*/ mload(0x1de0),
                                       mulmod(/*coefficients[211]*/ mload(0x1e00),
                                              /*adjustments[23]*/mload(0x4260),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for handle_empty_vault/consistency_key_change1: is_settlement * (column9_row32768 - column9_row32760).
              let val := mulmod(
                /*periodic_column/is_settlement*/ mload(0xc0),
                addmod(
                  /*column9_row32768*/ mload(0x3160),
                  sub(PRIME, /*column9_row32760*/ mload(0x3140)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[212] + coefficients[213] * adjustments[23])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[212]*/ mload(0x1e20),
                                       mulmod(/*coefficients[213]*/ mload(0x1e40),
                                              /*adjustments[23]*/mload(0x4260),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for handle_empty_vault/consistency_token_change1: is_settlement * (column8_row4099 - column9_row32744).
              let val := mulmod(
                /*periodic_column/is_settlement*/ mload(0xc0),
                addmod(
                  /*column8_row4099*/ mload(0x29a0),
                  sub(PRIME, /*column9_row32744*/ mload(0x3100)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[214] + coefficients[215] * adjustments[23])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[214]*/ mload(0x1e60),
                                       mulmod(/*coefficients[215]*/ mload(0x1e80),
                                              /*adjustments[23]*/mload(0x4260),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for handle_empty_vault/consistency_key_change2: is_settlement * (column9_row32768 - column9_row49144).
              let val := mulmod(
                /*periodic_column/is_settlement*/ mload(0xc0),
                addmod(
                  /*column9_row32768*/ mload(0x3160),
                  sub(PRIME, /*column9_row49144*/ mload(0x31e0)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[216] + coefficients[217] * adjustments[23])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[216]*/ mload(0x1ea0),
                                       mulmod(/*coefficients[217]*/ mload(0x1ec0),
                                              /*adjustments[23]*/mload(0x4260),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for handle_empty_vault/consistency_token_change2: is_settlement * (column8_row5123 - column9_row49128).
              let val := mulmod(
                /*periodic_column/is_settlement*/ mload(0xc0),
                addmod(
                  /*column8_row5123*/ mload(0x29c0),
                  sub(PRIME, /*column9_row49128*/ mload(0x31c0)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[218] + coefficients[219] * adjustments[23])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[218]*/ mload(0x1ee0),
                                       mulmod(/*coefficients[219]*/ mload(0x1f00),
                                              /*adjustments[23]*/mload(0x4260),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for handle_empty_vault/vault_empty/empty_vault_booleanity: column8_row2045 * (1 - column8_row2045).
              let val := mulmod(
                /*column8_row2045*/ mload(0x28e0),
                addmod(1, sub(PRIME, /*column8_row2045*/ mload(0x28e0)), PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 8192) - 1
              // val *= denominator_invs[23]
              val := mulmod(val, mload(0x3a80), PRIME)

              // res += val * (coefficients[220] + coefficients[221] * adjustments[24])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[220]*/ mload(0x1f20),
                                       mulmod(/*coefficients[221]*/ mload(0x1f40),
                                              /*adjustments[24]*/mload(0x4280),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for handle_empty_vault/vault_empty/amount_zero_when_empty: column8_row2045 * column8_row3075.
              let val := mulmod(/*column8_row2045*/ mload(0x28e0), /*column8_row3075*/ mload(0x2940), PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 8192) - 1
              // val *= denominator_invs[23]
              val := mulmod(val, mload(0x3a80), PRIME)

              // res += val * (coefficients[222] + coefficients[223] * adjustments[24])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[222]*/ mload(0x1f60),
                                       mulmod(/*coefficients[223]*/ mload(0x1f80),
                                              /*adjustments[24]*/mload(0x4280),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for handle_empty_vault/vault_empty/amount_inv_zero_when_empty: column8_row2045 * column8_row6141.
              let val := mulmod(/*column8_row2045*/ mload(0x28e0), /*column8_row6141*/ mload(0x29e0), PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 8192) - 1
              // val *= denominator_invs[23]
              val := mulmod(val, mload(0x3a80), PRIME)

              // res += val * (coefficients[224] + coefficients[225] * adjustments[24])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[224]*/ mload(0x1fa0),
                                       mulmod(/*coefficients[225]*/ mload(0x1fc0),
                                              /*adjustments[24]*/mload(0x4280),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for handle_empty_vault/vault_empty/empty_when_amount_zero: column8_row3075 * column8_row6141 - (1 - column8_row2045).
              let val := addmod(
                mulmod(/*column8_row3075*/ mload(0x2940), /*column8_row6141*/ mload(0x29e0), PRIME),
                sub(PRIME, addmod(1, sub(PRIME, /*column8_row2045*/ mload(0x28e0)), PRIME)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 8192) - 1
              // val *= denominator_invs[23]
              val := mulmod(val, mload(0x3a80), PRIME)

              // res += val * (coefficients[226] + coefficients[227] * adjustments[24])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[226]*/ mload(0x1fe0),
                                       mulmod(/*coefficients[227]*/ mload(0x2000),
                                              /*adjustments[24]*/mload(0x4280),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for handle_empty_vault/consistency_key_stage0: (1 - column8_row2045) * column9_row16376 - column8_row3.
              let val := addmod(
                mulmod(
                  addmod(1, sub(PRIME, /*column8_row2045*/ mload(0x28e0)), PRIME),
                  /*column9_row16376*/ mload(0x3000),
                  PRIME),
                sub(PRIME, /*column8_row3*/ mload(0x2780)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 16384) - 1
              // val *= denominator_invs[12]
              val := mulmod(val, mload(0x3920), PRIME)

              // res += val * (coefficients[228] + coefficients[229] * adjustments[12])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[228]*/ mload(0x2020),
                                       mulmod(/*coefficients[229]*/ mload(0x2040),
                                              /*adjustments[12]*/mload(0x4100),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for handle_empty_vault/consistency_token_stage0: (1 - column8_row2045) * column9_row16360 - column8_row1027.
              let val := addmod(
                mulmod(
                  addmod(1, sub(PRIME, /*column8_row2045*/ mload(0x28e0)), PRIME),
                  /*column9_row16360*/ mload(0x2fc0),
                  PRIME),
                sub(PRIME, /*column8_row1027*/ mload(0x28a0)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 16384) - 1
              // val *= denominator_invs[12]
              val := mulmod(val, mload(0x3920), PRIME)

              // res += val * (coefficients[230] + coefficients[231] * adjustments[12])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[230]*/ mload(0x2060),
                                       mulmod(/*coefficients[231]*/ mload(0x2080),
                                              /*adjustments[12]*/mload(0x4100),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for handle_empty_vault/consistency_key_stage1: (1 - column8_row10237) * column9_row16376 - column8_row8195.
              let val := addmod(
                mulmod(
                  addmod(1, sub(PRIME, /*column8_row10237*/ mload(0x2a80)), PRIME),
                  /*column9_row16376*/ mload(0x3000),
                  PRIME),
                sub(PRIME, /*column8_row8195*/ mload(0x2a40)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 16384) - 1
              // val *= denominator_invs[12]
              val := mulmod(val, mload(0x3920), PRIME)

              // res += val * (coefficients[232] + coefficients[233] * adjustments[12])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[232]*/ mload(0x20a0),
                                       mulmod(/*coefficients[233]*/ mload(0x20c0),
                                              /*adjustments[12]*/mload(0x4100),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for handle_empty_vault/consistency_token_stage1: (1 - column8_row10237) * column9_row16360 - column8_row9219.
              let val := addmod(
                mulmod(
                  addmod(1, sub(PRIME, /*column8_row10237*/ mload(0x2a80)), PRIME),
                  /*column9_row16360*/ mload(0x2fc0),
                  PRIME),
                sub(PRIME, /*column8_row9219*/ mload(0x2a60)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 16384) - 1
              // val *= denominator_invs[12]
              val := mulmod(val, mload(0x3920), PRIME)

              // res += val * (coefficients[234] + coefficients[235] * adjustments[12])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[234]*/ mload(0x20e0),
                                       mulmod(/*coefficients[235]*/ mload(0x2100),
                                              /*adjustments[12]*/mload(0x4100),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for initial_vaults_root: column0_row_expr0 - initial_vaults_root.
              let val := addmod(
                /*column0_row_expr0*/ mload(0x22c0),
                sub(PRIME, /*initial_vaults_root*/ mload(0x2a0)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point - 1
              // val *= denominator_invs[24]
              val := mulmod(val, mload(0x3aa0), PRIME)

              // res += val * (coefficients[236] + coefficients[237] * adjustments[25])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[236]*/ mload(0x2120),
                                       mulmod(/*coefficients[237]*/ mload(0x2140),
                                              /*adjustments[25]*/mload(0x42a0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for final_vaults_root: column4_row_expr1 - final_vaults_root.
              let val := addmod(
                /*column4_row_expr1*/ mload(0x24e0),
                sub(PRIME, /*final_vaults_root*/ mload(0x2c0)),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point - trace_generator^(65536 * (trace_length / 65536 - 1))
              // val *= denominator_invs[25]
              val := mulmod(val, mload(0x3ac0), PRIME)

              // res += val * (coefficients[238] + coefficients[239] * adjustments[25])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[238]*/ mload(0x2160),
                                       mulmod(/*coefficients[239]*/ mload(0x2180),
                                              /*adjustments[25]*/mload(0x42a0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for copy_merkle_roots: column4_row_expr0 - column0_row_expr2.
              let val := addmod(
                /*column4_row_expr0*/ mload(0x2500),
                sub(PRIME, /*column0_row_expr2*/ mload(0x22e0)),
                PRIME)

              // Numerator: point - trace_generator^(65536 * (trace_length / 65536 - 1) + 49152)
              // val *= numerators[10]
              val := mulmod(val, mload(0x3f60), PRIME)
              // Denominator: point^(trace_length / 16384) - 1
              // val *= denominator_invs[12]
              val := mulmod(val, mload(0x3920), PRIME)

              // res += val * (coefficients[240] + coefficients[241] * adjustments[26])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[240]*/ mload(0x21a0),
                                       mulmod(/*coefficients[241]*/ mload(0x21c0),
                                              /*adjustments[26]*/mload(0x42c0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

              {
              // Constraint expression for copy_merkle_roots_modification: is_modification * (column4_row_expr0 - column4_row_expr1).
              let val := mulmod(
                /*periodic_column/is_modification*/ mload(0xa0),
                addmod(
                  /*column4_row_expr0*/ mload(0x2500),
                  sub(PRIME, /*column4_row_expr1*/ mload(0x24e0)),
                  PRIME),
                PRIME)

              // Numerator: 1
              // val *= 1
              // val := mulmod(val, 1, PRIME)
              // Denominator: point^(trace_length / 65536) - 1
              // val *= denominator_invs[13]
              val := mulmod(val, mload(0x3940), PRIME)

              // res += val * (coefficients[242] + coefficients[243] * adjustments[27])
              res := addmod(res,
                            mulmod(val,
                                   add(/*coefficients[242]*/ mload(0x21e0),
                                       mulmod(/*coefficients[243]*/ mload(0x2200),
                                              /*adjustments[27]*/mload(0x42e0),
                      PRIME)),
                      PRIME),
                      PRIME)
              }

            mstore(0, res)
            return(0, 0x20)
            }
        }
    }
}
