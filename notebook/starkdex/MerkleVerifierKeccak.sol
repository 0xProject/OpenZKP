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

import "MerkleVerifier.sol";

contract MerkleVerifierKeccak is MerkleVerifier {
    function hashNode(bytes32 left, bytes32 right)
        internal pure
        returns (bytes32 hash)
    {
        uint256 lhashMask = getHashMask();
        assembly {
            mstore(0x00, left)
            mstore(0x20, right)
            hash := and(lhashMask, keccak256(0x00, 0x40))
        }
        return hash;
    }
}
