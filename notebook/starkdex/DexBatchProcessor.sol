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

import "DexVerifier.sol";
import "PublicInputParser.sol";

contract DexBatchProcessor is DexVerifier, PublicInputParser {
    address operatorAddress_;

    /*
      Constructs a DexBatchProcessor contract.
      auxPolynomials should contain a list of contract addresses:
        * constraintPoly.
        * hashPointsX.
        * hashPointsY.
        * ecdsaPointsX.
        * ecdsaPointsY.
    */
    constructor(address[] memory auxPolynomials, address dexStateContract,
                address oodsContract, address operatorAddress)
        public
        DexVerifier(auxPolynomials, oodsContract)
        PublicInputParser(dexStateContract)
    {
        operatorAddress_ = operatorAddress;
    }

    modifier onlyOperator()
    {
        require(msg.sender == operatorAddress_, "ONLY_OPERATOR");
        _;
    }

    function verifyProofAndUpdateState(
        uint256[] calldata proofParams,
        uint256[] calldata proof,
        uint256[] calldata publicInput,
        uint256[] calldata applicationData,
        bytes calldata availabilityProof
    )
        external
        onlyOperator()
    {
        verifyProof(proofParams, proof, publicInput);
        updateState(publicInput, applicationData, availabilityProof);
    }
}
