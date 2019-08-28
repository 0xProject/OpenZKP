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

pragma solidity ^0.5.2;

/*
  We store the state of the verifier in a continuous chunk of memory.
  The offsets of the different fields are listed below.
  E.g. The offset of the i'th FRI commitment is [MM_FRI_COMMITMENTS + i].
*/
contract MemoryMap {
    uint256 constant internal CHANNEL_STATE_SIZE = 3;
    uint256 constant internal MAX_N_QUERIES =  22;
    uint256 constant internal FRI_QUEUE_SIZE = MAX_N_QUERIES;

    uint256 constant internal MAX_SUPPORTED_MAX_FRI_STEP = 3;

    uint256 constant internal MM_EVAL_DOMAIN_SIZE =                          0x0;
    uint256 constant internal MM_BLOW_UP_FACTOR =                            0x1;
    uint256 constant internal MM_LOG_EVAL_DOMAIN_SIZE =                      0x2;
    uint256 constant internal MM_PROOF_OF_WORK_BITS =                        0x3;
    uint256 constant internal MM_EVAL_DOMAIN_GENERATOR =                     0x4;
    uint256 constant internal MM_PUBLIC_INPUT_PTR =                          0x5;
    uint256 constant internal MM_TRACE_COMMITMENT =                          0x6;
    uint256 constant internal MM_OODS_COMMITMENT =                           0x7;
    uint256 constant internal MM_N_UNIQUE_QUERIES =                          0x8;
    uint256 constant internal MM_CHANNEL =                                   0x9; // uint256[3]
    uint256 constant internal MM_MERKLE_QUEUE =                              0xc; // uint256[44]
    uint256 constant internal MM_FRI_VALUES =                               0x38; // uint256[22]
    uint256 constant internal MM_FRI_INV_POINTS =                           0x4e; // uint256[22]
    uint256 constant internal MM_QUERIES =                                  0x64; // uint256[22]
    uint256 constant internal MM_FRI_QUERIES_DELIMITER =                    0x7a;
    uint256 constant internal MM_FRI_CTX =                                  0x7b; // uint256[20]
    uint256 constant internal MM_FRI_STEPS_PTR =                            0x8f;
    uint256 constant internal MM_FRI_EVAL_POINTS =                          0x90; // uint256[10]
    uint256 constant internal MM_FRI_COMMITMENTS =                          0x9a; // uint256[10]
    uint256 constant internal MM_FRI_LAST_LAYER_DEG_BOUND =                 0xa4;
    uint256 constant internal MM_FRI_LAST_LAYER_PTR =                       0xa5;
    uint256 constant internal MM_CONSTRAINT_POLY_ARGS_START =               0xa6;
    uint256 constant internal MM_PERIODIC_COLUMN__HASH_POOL_POINTS__X =     0xa6;
    uint256 constant internal MM_PERIODIC_COLUMN__HASH_POOL_POINTS__Y =     0xa7;
    uint256 constant internal MM_PERIODIC_COLUMN__MERKLE_HASH_POINTS__X =   0xa8;
    uint256 constant internal MM_PERIODIC_COLUMN__MERKLE_HASH_POINTS__Y =   0xa9;
    uint256 constant internal MM_PERIODIC_COLUMN__BOUNDARY_BASE =           0xaa;
    uint256 constant internal MM_PERIODIC_COLUMN__IS_MODIFICATION =         0xab;
    uint256 constant internal MM_PERIODIC_COLUMN__IS_SETTLEMENT =           0xac;
    uint256 constant internal MM_PERIODIC_COLUMN__BOUNDARY_KEY =            0xad;
    uint256 constant internal MM_PERIODIC_COLUMN__BOUNDARY_TOKEN =          0xae;
    uint256 constant internal MM_PERIODIC_COLUMN__BOUNDARY_AMOUNT0 =        0xaf;
    uint256 constant internal MM_PERIODIC_COLUMN__BOUNDARY_AMOUNT1 =        0xb0;
    uint256 constant internal MM_PERIODIC_COLUMN__BOUNDARY_VAULT_ID =       0xb1;
    uint256 constant internal MM_PERIODIC_COLUMN__ECDSA_POINTS__X =         0xb2;
    uint256 constant internal MM_PERIODIC_COLUMN__ECDSA_POINTS__Y =         0xb3;
    uint256 constant internal MM_TRACE_LENGTH =                             0xb4;
    uint256 constant internal MM_SHIFT_POINT_X =                            0xb5;
    uint256 constant internal MM_SHIFT_POINT_Y =                            0xb6;
    uint256 constant internal MM_VAULTS_PATH_LENGTH =                       0xb7;
    uint256 constant internal MM_SIG_CONFIG_ALPHA =                         0xb8;
    uint256 constant internal MM_SIG_CONFIG_BETA =                          0xb9;
    uint256 constant internal MM_N_MODIFICATIONS =                          0xba;
    uint256 constant internal MM_INITIAL_VAULTS_ROOT =                      0xbb;
    uint256 constant internal MM_FINAL_VAULTS_ROOT =                        0xbc;
    uint256 constant internal MM_N_SETTLEMENTS =                            0xbd;
    uint256 constant internal MM_VAULT_SHIFT =                              0xbe;
    uint256 constant internal MM_AMOUNT_SHIFT =                             0xbf;
    uint256 constant internal MM_TRADE_SHIFT =                              0xc0;
    uint256 constant internal MM_TRACE_GENERATOR =                          0xc1;
    uint256 constant internal MM_OODS_POINT =                               0xc2;
    uint256 constant internal MM_COEFFICIENTS =                             0xc3; // uint256[244]
    uint256 constant internal MM_OODS_VALUES =                             0x1b7; // uint256[129]
    uint256 constant internal MM_CONSTRAINT_POLY_ARGS_END =                0x238;
    uint256 constant internal MM_COMPOSITION_OODS_VALUES =                 0x238; // uint256[2]
    uint256 constant internal MM_OODS_EVAL_POINTS =                        0x23a; // uint256[22]
    uint256 constant internal MM_OODS_COEFFICIENTS =                       0x250; // uint256[131]
    uint256 constant internal MM_TRACE_QUERY_RESPONSES =                   0x2d3; // uint256[220]
    uint256 constant internal MM_COMPOSITION_QUERY_RESPONSES =             0x3af; // uint256[44]
    uint256 constant internal MM_CONTEXT_SIZE =                            0x3db;
}
