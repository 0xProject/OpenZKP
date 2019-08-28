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

import "Prng.sol";

contract VerifierChannel is Prng {
    /*
      We store the state of the channel in uint256[3] as follows:
        [0] proof pointer.
        [1] prng digest.
        [2] prng counter.
    */
    uint256 constant internal CHANNEL_STATE_SIZE = 3;

    function getPrngPtr(uint256 channelPtr)
        internal pure
        returns (uint256)
    {
        return channelPtr + 0x20;
    }

    function initChannel(uint256 channelPtr, uint256 proofPtr, bytes32 publicInputHash)
        internal pure
    {
        assembly {
            // Skip 0x20 bytes length at the beginning of the proof.
            mstore(channelPtr, add(proofPtr, 0x20))
        }

        initPrng(getPrngPtr(channelPtr), publicInputHash);
    }

    function sendFieldElements(uint256 channelPtr, uint256 nElements, uint256 targetPtr)
        internal pure
    {
        assembly {
            let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001
            let PRIME_MON_R_INV := 0x40000000000001100000000000012100000000000000000000000000000000
            let PRIME_MASK := 0x0fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
            let digestPtr := add(channelPtr, 0x20)
            let counterPtr := add(channelPtr, 0x40)

            let endPtr := add(targetPtr, mul(nElements, 0x20))
            for { } lt(targetPtr, endPtr) { targetPtr := add(targetPtr, 0x20) } {
                let fieldElement := PRIME
                // while (fieldElement >= PRIME)
                for { } iszero(lt(fieldElement, PRIME)) { } {
                    fieldElement := and(keccak256(digestPtr, 0x40), PRIME_MASK)
                    // counter += 1;
                    mstore(counterPtr, add(mload(counterPtr), 1))
                }
                mstore(targetPtr, mulmod(fieldElement, PRIME_MON_R_INV, PRIME))
            }
        }
    }

    /*
      Sends random queries and returns an array of queries sorted in ascending order.
      Generates count queries in the range [0, mask] and returns the number of unique queries.
      Note that mask is of the form 2^k-1 (for some k).
    */
    function sendRandomQueries(
        uint256 channelPtr, uint256 count, uint256 mask, uint256 queriesOutPtr)
        internal pure returns (uint256)
    {
        uint256 val;
        uint256 shift = 0;
        uint256 endPtr = queriesOutPtr;
        for (uint256 i = 0; i < count; i++) {
            if (shift == 0) {
                val = uint256(getRandomBytes(getPrngPtr(channelPtr)));
                shift = 0x100;
            }
            shift -= 0x40;
            uint256 queryIdx = (val >> shift) & mask;

            uint256 ptr = endPtr;
            uint256 curr;
            // Insert new queryIdx in the correct place like insertion sort.

            while (ptr > queriesOutPtr) {
                assembly {
                    curr := mload(sub(ptr, 0x20))
                }

                if (queryIdx >= curr) {
                    break;
                }

                assembly {
                    mstore(ptr, curr)
                }
                ptr -= 0x20;
            }

            if (queryIdx != curr) {
                assembly {
                    mstore(ptr, queryIdx)
                }
                endPtr += 0x20;
            } else {
                // Revert right shuffling.
                while (ptr < endPtr) {
                    assembly {
                        mstore(ptr, mload(add(ptr, 0x20)))
                        ptr := add(ptr, 0x20)
                    }
                }
            }
        }

        return (endPtr - queriesOutPtr) / 0x20;
    }

    function readBytes(uint256 channelPtr, bool mix)
        internal pure
        returns (bytes32)
    {
        uint256 proofPtr;
        bytes32 val;

        assembly {
            proofPtr := mload(channelPtr)
            val := mload(proofPtr)
            mstore(channelPtr, add(proofPtr, 0x20))
        }
        if (mix) {
            assembly {
                let digestPtr := add(channelPtr, 0x20)
                let counterPtr := add(digestPtr, 0x20)
                mstore(counterPtr, val)
                // prng.digest := keccak256(digest||val), nonce was written earlier.
                mstore(digestPtr, keccak256(digestPtr, 0x40))
                // prng.counter := 0.
                mstore(counterPtr, 0)
            }
        }

        return val;
    }

    function readHash(uint256 channelPtr, bool mix)
        internal pure
        returns (bytes32)
    {
        bytes32 val = readBytes(channelPtr, mix);

        return val;
    }

    function readFieldElement(uint256 channelPtr, bool mix)
        internal pure returns (uint256) {
        uint256 val = fromMontgomery(uint256(readBytes(channelPtr, mix)));

        return val;
    }

    function verifyProofOfWork(uint256 channelPtr, uint256 proofOfWorkBits) internal pure {
        if (proofOfWorkBits == 0) {
            return;
        }

        uint256 proofOfWorkDigest;
        assembly {
            // [0:29] := 0123456789abcded || digest || workBits.
            mstore(0, 0x0123456789abcded000000000000000000000000000000000000000000000000)
            let digest := mload(add(channelPtr, 0x20))
            mstore(0x8, digest)
            mstore8(0x28, proofOfWorkBits)
            mstore(0, keccak256(0, 0x29))

            let proofPtr := mload(channelPtr)
            mstore(0x20, mload(proofPtr))
            // proofOfWorkDigest:= keccak256(keccak256(0123456789abcded || digest || workBits) || nonce).
            proofOfWorkDigest := keccak256(0, 0x28)

            mstore(0, digest)
            //prng.digest := keccak256(digest||nonce), nonce was written earlier.
            mstore(add(channelPtr, 0x20), keccak256(0, 0x28))
            //prng.counter := 0
            mstore(add(channelPtr, 0x40), 0)

            mstore(channelPtr, add(proofPtr, 0x8))
        }

        uint256 proofOfWorkThreshold = uint256(1) << (256 - proofOfWorkBits);
        require(proofOfWorkDigest < proofOfWorkThreshold, "Proof of work check failed.");
    }
}
