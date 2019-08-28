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

import "VerifierChannel.sol";

contract MerkleVerifier is VerifierChannel  {
    function hashNode(bytes32 left, bytes32 right) internal pure returns (bytes32);

    function fixEndianity(bytes32 value)
        internal pure
        returns (bytes32)
    {
        // Default implementation: No endianity swap.
        return value;
    }

    function getHashMask() internal pure returns(uint256) {
        // Default implementation.
        return 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF000000000000000000000000;
    }

    /*
      Verifies a Merkle tree decommitment for n leaves in a Merkle tree with N leaves.

      The inputs data sits in the queue at queuePtr.
      Each slot in the queue contains a 32 bytes leaf index and a 32 byte leaf value.
      The indices need to be in the range [N..2*N-1] and strictly incrementing.
      Decommitments are read from the channel in the ctx.

      The input data is destroyed during verification.
    */
    function verify(
        uint256 channelPtr,
        uint256 queuePtr,
        bytes32 root,
        uint256 n)
        internal pure
    {
        bytes32 hash;
        uint256 lhashMask = getHashMask();

        assembly {
            // queuePtr + i * 0x40 gives the i'th index in the queue.
            // hashesPtr + i * 0x40 gives the i'th hash in the queue.
            let hashesPtr := add(queuePtr, 0x20)
            let queueSize := mul(n, 0x40)
            let slotSize := 0x40

            // The items are in slots [0, n-1].
            let rdIdx := 0
            let wrIdx := 0 // = n % n

            // Iterate the queue until we hit the root.
            let index := mload(add(rdIdx, queuePtr))
            let proofPtr := mload(channelPtr)

            // while(index > 1)
            for { } gt(index, 1) { } {
                let siblingIndex := xor(index, 1)
                // sibblingOffset := 0x20 * lsb(siblingIndex)
                let sibblingOffset := mulmod(siblingIndex, 0x20, 0x40)

                // Store the hash corresponding to index in the correct slot.
                // 0 if index is even and 0x20 if index is odd.
                // The hash of the sibling will be written to the other slot.
                mstore(xor(0x20, sibblingOffset), mload(add(rdIdx, hashesPtr)))
                rdIdx := addmod(rdIdx, slotSize, queueSize)

                // Inline channel operation:
                // Assume we are going to read a new hash from the proof.
                // If this is not the case add(proofPtr, 0x20) will be reverted.
                let newHashPtr := proofPtr
                proofPtr := add(proofPtr, 0x20)

                // Push index/2 into the queue, before reading the next index.
                // The order is important, as otherwise we may try to read from an empty queue (in
                // the case where we are working on one item).
                // wrIdx will be updated after writing the relevant hash to the queue.
                mstore(add(wrIdx, queuePtr), div(index, 2))

                // Load the next index from the queue and check if it is our sibling.
                index := mload(add(rdIdx, queuePtr))
                if eq(index, siblingIndex) {
                    // Take sibling from queue rather than from proof.
                    newHashPtr := add(rdIdx, hashesPtr)
                    // Revert reading from proof.
                    proofPtr := sub(proofPtr, 0x20)
                    rdIdx := addmod(rdIdx, slotSize, queueSize)

                    // Index was consumed, read the next one.
                    // Note that the queue can't be empty at this point.
                    // The index of the parent of the current node was already pushed into the
                    // queue, and the parent is never the sibling.
                    index := mload(add(rdIdx, queuePtr))
                }

                mstore(sibblingOffset, mload(newHashPtr))

                // Push the new hash to the end of the queue.
                mstore(add(wrIdx, hashesPtr), and(lhashMask, keccak256(0x00, 0x40)))
                wrIdx := addmod(wrIdx, slotSize, queueSize)
            }
            hash := mload(add(rdIdx, hashesPtr))

            // Update the proof pointer in the context.
            mstore(channelPtr, proofPtr)
        }
        require(hash == root, "INVALID_MERKLE_PROOF");
    }
}
