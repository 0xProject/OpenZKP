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
import "PublicInputOffsets.sol";

contract PublicInputParser is PublicInputOffsets {
    uint256 constant internal K_MODULUS =
    0x800000000000011000000000000000000000000000000000000000000000001;

    IVerifierActions dexStateContract_;

    constructor(address dexStateContract)
        public {
        dexStateContract_ = IVerifierActions(dexStateContract);
    }

    function updateState(
        uint256[] memory publicInput,
        uint256[] memory applicationData,
        bytes memory availabilityProof
    )
        internal
    {
        require(
            publicInput.length >= OFFSET_MODIFICATION_DATA,
            "publicInput does not contain all required fields.");
        require(publicInput[OFFSET_VAULT_FINAL_ROOT] < K_MODULUS, "New vault root >= PRIME.");
        require(publicInput[OFFSET_TRADE_FINAL_ROOT] < K_MODULUS, "New trade root >= PRIME.");
        dexStateContract_.stateUpdate(
            publicInput[OFFSET_VAULT_INITIAL_ROOT],
            publicInput[OFFSET_VAULT_FINAL_ROOT],
            publicInput[OFFSET_TRADE_INITIAL_ROOT],
            publicInput[OFFSET_TRADE_FINAL_ROOT],
            publicInput[OFFSET_VAULT_TREE_HEIGHT],
            publicInput[OFFSET_TRADE_TREE_HEIGHT],
            availabilityProof
        );
        sendModifications(publicInput, applicationData);
    }

    function sendModifications(
        uint256[] memory publicInput,
        uint256[] memory applicationData
    ) private {
        require(
            applicationData.length >= APPLICATION_DATA_MODIFICATIONS_OFFSET,
            "applicationData does not contain all required fields.");
        uint256 nModifications = applicationData[APPLICATION_DATA_N_MODIFICATIONS_OFFSET];
        require(
            nModifications == (publicInput.length - OFFSET_MODIFICATION_DATA) / N_WORDS_PER_MODIFICATION,
            "Inconsistent number of modifications.");
        require(
            applicationData.length == nModifications + APPLICATION_DATA_MODIFICATIONS_OFFSET,
            "Inconsistent number of modifications in applicationData and publicInput.");

        for (uint256 i = 0; i < nModifications; i++) {
            uint256 modificationOffset = OFFSET_MODIFICATION_DATA + i * N_WORDS_PER_MODIFICATION;
            uint256 starkKey = publicInput[modificationOffset];
            uint256 requestingKey = applicationData[i + 1];
            uint256 tokenId = publicInput[modificationOffset + 1];

            require(starkKey < K_MODULUS, "Stark key >= PRIME");
            require(requestingKey < K_MODULUS, "Requesting key >= PRIME");
            require(tokenId < K_MODULUS, "Token id >= PRIME");

            uint256 actionParams = publicInput[modificationOffset + 2];
            uint256 amountBefore = (actionParams >> 192) & ((1 << 63) - 1);
            uint256 amountAfter = (actionParams >> 128) & ((1 << 63) - 1);
            uint256 vaultId = (actionParams >> 96) & ((1 << 31) - 1);

            if (requestingKey != 0) {
                // This is a false full withdrawal.
                require(
                    starkKey != requestingKey,
                    "False full withdrawal requesting_key should differ from the vault owner key.");
                require(amountBefore == amountAfter, "Amounts differ in false full withdrawal.");
                dexStateContract_.clearEscapeRequest(requestingKey, vaultId);
                continue;
            }

            // This is a deposit.
            if (amountAfter > amountBefore) {
                uint256 quantizedAmount = amountAfter - amountBefore;
                dexStateContract_.acceptDeposit(starkKey, vaultId, tokenId, quantizedAmount);
            } else {
                // This is a withdrawal, in case the final amount is zero,
                // handled as a full withdrawal.
                if (amountBefore > amountAfter) {
                    uint256 quantizedAmount = amountBefore - amountAfter;
                    dexStateContract_.acceptWithdrawal(starkKey, tokenId, quantizedAmount);
                }
                if (amountAfter == 0) {
                    dexStateContract_.clearEscapeRequest(starkKey, vaultId);
                }
            }
        }
    }
}
