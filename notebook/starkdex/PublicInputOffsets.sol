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

import "IVerifierActions.sol";

contract PublicInputOffsets {
    // The following constants are offsets of data expected in the public input.
    uint256 internal constant OFFSET_LOG_BATCH_SIZE = 0;
    uint256 internal constant OFFSET_N_TRANSACTIONS = 1;
    uint256 internal constant OFFSET_VAULT_INITIAL_ROOT = 2;
    uint256 internal constant OFFSET_VAULT_FINAL_ROOT = 3;
    uint256 internal constant OFFSET_TRADE_INITIAL_ROOT = 4;
    uint256 internal constant OFFSET_TRADE_FINAL_ROOT = 5;
    uint256 internal constant OFFSET_VAULT_TREE_HEIGHT = 6;
    uint256 internal constant OFFSET_TRADE_TREE_HEIGHT = 7;
    uint256 internal constant OFFSET_MODIFICATION_DATA = 8;
    uint256 internal constant APPLICATION_DATA_N_MODIFICATIONS_OFFSET = 0;
    uint256 internal constant APPLICATION_DATA_MODIFICATIONS_OFFSET = 1;

    uint256 internal constant N_WORDS_PER_MODIFICATION = 3;
}
