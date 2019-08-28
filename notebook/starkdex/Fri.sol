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

import "MemoryMap.sol";
import "MemoryAccessUtils.sol";
import "MerkleVerifierKeccak.sol";
import "VerifierChannel.sol";

/*
  The main component of FRI is the FRI step which takes
  the i-th layer evaluations on a coset c*<g> and produces a single evaluation in layer i+1.

  To this end we have a friCtx that holds the following data:
  evaluations:    holds the evaluations on the coset we are currently working on.
  group:          holds the group <g> in bit reversed order.
  halfInvGroup:   holds the group <g^-1>/<-1> in bit reversed order.
                  (We only need half of the inverse group)

  Note that due to the bit reversed order, a prefix of size 2^k of either group
  or halfInvGroup has the same structure (but for a smaller group).
*/
contract Fri is MemoryMap, MemoryAccessUtils, VerifierChannel, MerkleVerifierKeccak {
    uint256 constant internal MAX_COSET_SIZE = 2**MAX_SUPPORTED_MAX_FRI_STEP;
    uint256 constant internal FRI_GROUP_SIZE = 0x20 * MAX_COSET_SIZE;
    uint256 constant internal FRI_CTX_TO_COSET_EVALUATIONS_OFFSET = 0;
    uint256 constant internal FRI_CTX_TO_FRI_GROUP_OFFSET = FRI_GROUP_SIZE;
    uint256 constant internal FRI_CTX_TO_FRI_HALF_INV_GROUP_OFFSET =
    FRI_CTX_TO_FRI_GROUP_OFFSET + FRI_GROUP_SIZE;

    uint256 constant internal MM_FRI_QUEUE_INV_POINTS_OFFSET =
    0x20 * (MM_FRI_INV_POINTS - MM_FRI_VALUES);
    uint256 constant internal MM_FRI_QUEUE_QUERIES_OFFSET =
    0x20 * (MM_QUERIES - MM_FRI_VALUES);

    function nextLayerElementFromTwoPreviousLayerElements(
        uint256 fX, uint256 fMinusX, uint256 evalPoint, uint256 xInv)
        internal pure
        returns (uint256 res)
    {
        // Folding formula:
        // f(x)  = g(x^2) + xh(x^2)
        // f(-x) = g((-x)^2) - xh((-x)^2) = g(x^2) - xh(x^2)
        // =>
        // 2g(x^2) = f(x) + f(-x)
        // 2h(x^2) = (f(x) - f(-x))/x
        // => The 2*interpolation at evalPoint is:
        // 2*(g(x^2) + evalPoint*h(x^2)) = f(x) + f(-x) + evalPoint*(f(x) - f(-x))*xInv.
        //
        // Note that multiplying by 2 doesn't affect the degree,
        // so we can just agree to do that on both the prover and verifier.
        assembly {
            // PRIME is PrimeFieldElement0.K_MODULUS.
            let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001
            // Note that whenever we call add(), the result is always less than 2*PRIME,
            // so there are no overflows.
            res := addmod(add(fX, fMinusX),
                   mulmod(mulmod(evalPoint, xInv, PRIME),
                   add(fX, /*-fMinusX*/sub(PRIME, fMinusX)), PRIME), PRIME)
        }
    }

    /*
      Computes the evaluation of a polynomial f(x) = sum(a_i * x^i) on the given point.
      The coefficients of the polynomial are expected to be at:
        a_0 = ctx[mmLastFriLayer_], ..., a_{n-1} = ctx[mmLastFriLayer_ + n - 1]
      where n = friLastLayerDegBound.
      The function assumes that n is divisible by 8.
    */
    function hornerEval(uint256[] memory ctx, uint256 point, uint256 nCoefs)
        internal pure
        returns (uint256) {
        uint256 result = 0;
        uint256 prime = PrimeFieldElement0.K_MODULUS;

        uint256 coefsStart = ctx[MM_FRI_LAST_LAYER_PTR];

        require(nCoefs % 8 == 0, "N must be divisible by 8");
        assembly {
            let coefsPtr := add(coefsStart, mul(nCoefs, 0x20))
            for { } gt(coefsPtr, coefsStart) { } {
                // Reduce coefsPtr by 8 field elements.
                coefsPtr := sub(coefsPtr, 0x100)

                // Apply 4 Horner steps (result := result * point + coef).
                result :=
                    add(mload(add(coefsPtr, 0x80)), mulmod(
                    add(mload(add(coefsPtr, 0xa0)), mulmod(
                    add(mload(add(coefsPtr, 0xc0)), mulmod(
                    add(mload(add(coefsPtr, 0xe0)), mulmod(
                        result,
                    point, prime)),
                    point, prime)),
                    point, prime)),
                    point, prime))

                // Apply 4 additional Horner steps.
                result :=
                    add(mload(coefsPtr), mulmod(
                    add(mload(add(coefsPtr, 0x20)), mulmod(
                    add(mload(add(coefsPtr, 0x40)), mulmod(
                    add(mload(add(coefsPtr, 0x60)), mulmod(
                        result,
                    point, prime)),
                    point, prime)),
                    point, prime)),
                    point, prime))
            }
        }

        // Since the last operation was "add" (instead of "addmod"), we need to take result % prime.
        return result % prime;
    }

    /*
      Hashes the old digest with all last layer coefficients (at once), and stores a pointer
      to the coefficients.
      We keep the coefficients in Montgomery form, because our FRI implementation assumes that
      the polynomials in all the FRI layers are multiplied by MontgomeryR.
    */
    function readLastLayer(uint256[] memory ctx)
        internal pure
    {
        uint256 lmmChannel = MM_CHANNEL;
        uint256 friLastLayerDegBound = ctx[MM_FRI_LAST_LAYER_DEG_BOUND];
        uint256 lastLayerPtr;

        assembly {
            let channelPtr := add(add(ctx, 0x20), mul(lmmChannel, 0x20))
            lastLayerPtr := mload(channelPtr)

            // Copy the digest to the proof area
            // (store it before the coefficients - this is done because
            // keccak256 needs all data to be consecutive),
            // then hash and place back in digestPtr.
            let newDigestPtr := sub(lastLayerPtr, 0x20)
            let digestPtr := add(channelPtr, 0x20)
            // Overwriting the proof to minimize copying of data.
            mstore(newDigestPtr, mload(digestPtr))
            let length := mul(friLastLayerDegBound, 0x20)
            mstore(digestPtr, keccak256(newDigestPtr, add(length, 0x20)))
            // Note: proof pointer is not incremented until this point.

            mstore(channelPtr, add(lastLayerPtr, length))
        }

        ctx[MM_FRI_LAST_LAYER_PTR] = lastLayerPtr;
    }

    function verifyLastLayer(uint256[] memory ctx, uint256 nPoints)
        internal {
        uint256 friLastLayerDegBound = ctx[MM_FRI_LAST_LAYER_DEG_BOUND];
        uint256 groupOrderMinusOne = friLastLayerDegBound * ctx[MM_BLOW_UP_FACTOR] - 1;

        for (uint256 i = 0; i < nPoints; i++) {
            uint256 point = ctx[MM_FRI_INV_POINTS + i];
            // Invert point using inverse(point) == fpow(point, ord(point) - 1).

            point = fpow(point, groupOrderMinusOne);
            require(
                hornerEval(ctx, point, friLastLayerDegBound) == ctx[MM_FRI_VALUES + i],
                "Bad Last layer value.");
        }
    }

    /*
      Reads 4 elements, and applies 2 + 1 FRI transformations to obtain a single element.
      The basic FRI transformation is described in nextLayerElementFromTwoPreviousLayerElements().

      FRI layer n:                              f0 f1  f2 f3
      -----------------------------------------  \ / -- \ / -----------
      FRI layer n+1:                              f0    f2
      -------------------------------------------- \ ---/ -------------
      FRI layer n+2:                                 f0
    */
    function do2FriSteps(
        uint256 friHalfInvGroupPtr, uint256 evaluationsOnCosetPtr, uint256 cosetOffset_,
        uint256 friEvalPoint)
    internal pure returns (uint256 nextLayerValue, uint256 nextXInv) {
        assembly {
            let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001
            let xInv := cosetOffset_
            let f0 := mload(evaluationsOnCosetPtr)
            {
                let f1 := mload(add(evaluationsOnCosetPtr, 0x20))
                f0 := addmod(add(f0, f1),
                             mulmod(mulmod(friEvalPoint, xInv, PRIME),
                                    add(f0, /*-fMinusX*/sub(PRIME, f1)),
                                    PRIME),
                             PRIME)
            }
            let f2 := mload(add(evaluationsOnCosetPtr, 0x40))
            {
                let xInv1 := mulmod(mload(add(friHalfInvGroupPtr, 0x20)),
                                       xInv,
                                       PRIME)

                let f3 := mload(add(evaluationsOnCosetPtr, 0x60))
                f2 := addmod(add(f2, f3),
                             mulmod(add(f2, /*-fMinusX*/sub(PRIME, f3)),
                                    mulmod(friEvalPoint, xInv1, PRIME),
                                    PRIME),
                             PRIME)
            }

            let newXInv := mulmod(xInv, xInv, PRIME)
            nextXInv := mulmod(newXInv, newXInv, PRIME)
            nextLayerValue := addmod(add(f0, f2),
                          mulmod(mulmod(mulmod(friEvalPoint, friEvalPoint, PRIME),
                                        newXInv,
                                        PRIME),
                                 add(f0, /*-fMinusX*/sub(PRIME, f2)),
                                 PRIME),
                          PRIME)
        }
    }

    /*
      Reads 8 element, and applies 4 + 2 + 1 FRI transformation to obtain a single element.

      See do2FriSteps for more detailed explanation.
    */
    function do3FriSteps(
        uint256 friHalfInvGroupPtr, uint256 evaluationsOnCosetPtr, uint256 cosetOffset_,
        uint256 friEvalPoint)
    internal pure returns (uint256 nextLayerValue, uint256 nextXInv) {
        assembly {
            let f0 := mload(evaluationsOnCosetPtr)
            let xInv0 := cosetOffset_

            let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001
            let secondEvalPoint := mulmod(friEvalPoint, friEvalPoint, PRIME)
            {
                {
                let f1 := mload(add(evaluationsOnCosetPtr, 0x20))

                f0 := addmod(add(f0, f1),
                                  mulmod(mulmod(friEvalPoint, xInv0, PRIME),
                                         add(f0, /*-fMinusX*/sub(PRIME, f1)),
                                         PRIME),
                                  PRIME)
                }
                {
                    let f2 := mload(add(evaluationsOnCosetPtr, 0x40))
                    {
                    let f3 := mload(add(evaluationsOnCosetPtr, 0x60))

                    let xInv1 := mulmod(xInv0, mload(add(friHalfInvGroupPtr, 0x20)), PRIME)

                    f2 := addmod(add(f2, f3),
                                      mulmod(mulmod(friEvalPoint, xInv1, PRIME),
                                             add(f2, /*-fMinusX*/sub(PRIME, f3)),
                                             PRIME),
                                      PRIME)
                    }

                    xInv0 := mulmod(xInv0, xInv0, PRIME)
                    f0 := addmod(add(f0, f2),
                                 mulmod(mulmod(secondEvalPoint, xInv0, PRIME),
                                        add(f0, /*-fMinusX*/sub(PRIME, f2)),
                                        PRIME),
                                 PRIME)
                }
            }
            let f4 := mload(add(evaluationsOnCosetPtr, 0x80))
            {
                let xInv2 := mulmod(cosetOffset_, mload(add(friHalfInvGroupPtr, 0x40)), PRIME)
                {
                let f5 := mload(add(evaluationsOnCosetPtr, 0xa0))
                f4 := addmod(add(f4, f5),
                            mulmod(mulmod(friEvalPoint, xInv2, PRIME),
                                    add(f4, /*-fMinusX*/sub(PRIME, f5)),
                            PRIME),
                            PRIME)
                }
                {
                    let evalPointOverXInv3 := mulmod(friEvalPoint,
                                                         mulmod(cosetOffset_,
                                                                mload(add(friHalfInvGroupPtr, 0x60)),
                                                                PRIME),
                                                         PRIME)

                    let f6 := mload(add(evaluationsOnCosetPtr, 0xc0))
                    {
                        let f7 := mload(add(evaluationsOnCosetPtr, 0xe0))

                        f6 := addmod(add(f6, f7),
                                     mulmod(evalPointOverXInv3,
                                            add(f6, /*-fMinusX*/sub(PRIME, f7)),
                                            PRIME),
                                     PRIME)
                    }
                    f4 := addmod(add(f4, f6),
                                mulmod(mulmod(secondEvalPoint, mulmod(xInv2, xInv2, PRIME), PRIME),
                                        add(f4, /*-fMinusX*/sub(PRIME, f6)),
                                PRIME),
                                PRIME)
                }
            }

            let xInvCubed := mulmod(xInv0, xInv0, PRIME)
            nextXInv := mulmod(xInvCubed, xInvCubed, PRIME)
            nextLayerValue :=
                   addmod(add(f0, f4),
                          mulmod(mulmod(mulmod(secondEvalPoint, secondEvalPoint, PRIME),
                                        xInvCubed,
                                        PRIME),
                                 add(f0, /*-fMinusX*/sub(PRIME, f4)),
                                 PRIME),
                          PRIME)
        }
    }

    /*
      Gathers the "cosetSize" elements that belong to the same coset
      as the item at the top of the FRI queue and stores them in ctx[MM_FRI_STEP_VALUES:].

      Returns
        friQueueHead - friQueueHead_ + 0x20  * (# elements that were taken from the queue).
        cosetIdx - the start index of the coset that was gathered.
        cosetOffset_ - the xInv field element that corresponds to cosetIdx.
    */
    function gatherCosetInputs(
        uint256 channelPtr, uint256 friCtx, uint256 friQueueHead_, uint256 cosetSize)
        internal pure returns (uint256 friQueueHead, uint256 cosetIdx, uint256 cosetOffset_) {
        uint256 invPointsOffset_ = MM_FRI_QUEUE_INV_POINTS_OFFSET;
        uint256 queriesOffset_ = MM_FRI_QUEUE_QUERIES_OFFSET;

        uint256 evaluationsOnCosetPtr = friCtx + FRI_CTX_TO_COSET_EVALUATIONS_OFFSET;
        uint256 friGroupPtr = friCtx + FRI_CTX_TO_FRI_GROUP_OFFSET;

        friQueueHead = friQueueHead_;
        assembly {
            let queueItemIdx := mload(add(queriesOffset_, friQueueHead))
            // The coset index is represented by the most significant bits of the queue item index.
            cosetIdx := and(queueItemIdx, not(sub(cosetSize, 1)))
            let nextCosetIdx := add(cosetIdx, cosetSize)
            let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001

            // Get the algebraic coset offset:
            // I.e. given c*g^(-k) compute c, where
            //      g is the generator of the coset group.
            //      k is bitReverse(offsetWithinCoset, log2(cosetSize)).
            //
            // To do this we multiply the algebraic coset offset at the top of the queue (c*g^(-k))
            // by the group element that corresponds to the index inside the coset (g^k).
            cosetOffset_ := mulmod(
                /*(c*g^(-k)*/ mload(add(friQueueHead, invPointsOffset_)),
                /*(g^k)*/     mload(add(friGroupPtr,
                                        mul(/*offsetWithinCoset*/sub(queueItemIdx, cosetIdx),
                                            0x20))),
                PRIME)

            let proofPtr := mload(channelPtr)

            for { let index := cosetIdx } lt(index, nextCosetIdx) { index := add(index, 1) } {
                // Inline channel operation:
                // Assume we are going to read the next element from the proof.
                // If this is not the case add(proofPtr, 0x20) will be reverted.
                let fieldElementPtr := proofPtr
                proofPtr := add(proofPtr, 0x20)

                // Load the next index from the queue and check if it is our sibling.
                if eq(index, queueItemIdx) {
                    // Take element from the queue rather than from the proof
                    // and convert it back to Montgomery form for Merkle verification.
                    fieldElementPtr := friQueueHead

                    // Revert the read from proof.
                    proofPtr := sub(proofPtr, 0x20)

                    // Reading the next index here is safe due to the
                    // delimiter after the queries.
                    friQueueHead := add(friQueueHead, 0x20)
                    queueItemIdx := mload(add(friQueueHead, queriesOffset_))
                }

                mstore(evaluationsOnCosetPtr, mload(fieldElementPtr))
                evaluationsOnCosetPtr := add(evaluationsOnCosetPtr, 0x20)
            }

            mstore(channelPtr, proofPtr)
        }
    }

    /*
      Returns the bit reversal of num assuming it has the given number of bits.
      For example, if we have numberOfBits = 6 and num = (0b)1101 == (0b)001101,
      the function will return (0b)101100.
    */
    function bitReverse(uint256 num, uint256 numberOfBits)
    internal pure
        returns(uint256 numReversed)
    {
        assert((numberOfBits == 256) || (num < 2 ** numberOfBits));
        uint256 n = num;
        uint256 r = 0;
        for (uint256 k = 0; k < numberOfBits; k++) {
            r = (r * 2) | (n % 2);
            n = n / 2;
        }
        return r;
    }

    /*
      Initializes the FRI group and half inv group in the FRI context.
    */
    function initFriGroups(uint256[] memory ctx) internal {
        uint256 genTraceDomain = ctx[MM_TRACE_GENERATOR];
        uint256 mmFriGroup = MM_FRI_CTX + (FRI_CTX_TO_FRI_GROUP_OFFSET / 0x20);
        uint256 mmHalfFriInvGroup = MM_FRI_CTX + (FRI_CTX_TO_FRI_HALF_INV_GROUP_OFFSET / 0x20);

        // fpow(genTraceDomain, traceSize / MAX_COSET_SIZE) gives us the
        // coset generator.
        // Raising the result to the (MAX_COSET_SIZE - 1) power gives us the inverse.
        uint256 genFriGroup = fpow(genTraceDomain, ctx[MM_TRACE_LENGTH] / MAX_COSET_SIZE);

        uint256 genFriGroupInv = fpow(genFriGroup, (MAX_COSET_SIZE - 1));

        uint256 lastVal = ONE_VAL;
        uint256 lastValInv = ONE_VAL;
        ctx[mmHalfFriInvGroup + 0] = ONE_VAL;
        ctx[mmFriGroup + 0] = ONE_VAL;
        ctx[mmFriGroup + 1] = fsub(0, ONE_VAL);

        // To compute [1, -1 (== g^n/2), g^n/4, -g^n/4, ...]
        // we compute half the elements and derive the rest using negation.
        uint256 halfCosetSize = MAX_COSET_SIZE / 2;
        for (uint256 i = 1; i < halfCosetSize; i++) {
            lastVal = fmul(lastVal, genFriGroup);
            lastValInv = fmul(lastValInv, genFriGroupInv);
            uint256 idx = bitReverse(i, MAX_SUPPORTED_MAX_FRI_STEP-1);

            ctx[mmFriGroup + 2*idx] = lastVal;
            ctx[mmFriGroup + 2*idx + 1] = fsub(0, lastVal);
            ctx[mmHalfFriInvGroup + idx] = lastValInv;
        }
    }

    /*
      Operates on the coset of size friFoldedCosetSize that start at index.

      It produces 3 outputs:
        1. The field elements that result from doing FRI reductions on the coset.
        2. The pointInv elements for the location that corresponds to the first output.
        3. The root of a Merkle tree for the input layer.

      The input is read either form the queue or from the proof depending on data availability.
      Since the function reads from the queue it returns an updated head pointer.
    */
    function doFriSteps(
        uint256 friCtx, uint256 friQueueTail, uint256 cosetOffset_, uint256 friEvalPoint,
        uint256 friCosetSize, uint256 index, uint256 merkleQueuePtr)
        internal pure {
        uint256 friValue;

        uint256 evaluationsOnCosetPtr = friCtx + FRI_CTX_TO_COSET_EVALUATIONS_OFFSET;
        uint256 friHalfInvGroupPtr = friCtx + FRI_CTX_TO_FRI_HALF_INV_GROUP_OFFSET;

        if (friCosetSize == 8) {
            (friValue, cosetOffset_) = do3FriSteps(
                friHalfInvGroupPtr, evaluationsOnCosetPtr, cosetOffset_, friEvalPoint);
        } else if (friCosetSize == 4) {
            (friValue, cosetOffset_) = do2FriSteps(
                friHalfInvGroupPtr, evaluationsOnCosetPtr, cosetOffset_, friEvalPoint);
        } else {
            require(false, "Only step sizes of 2 or 3 are supported.");
        }

        uint256 invPointsOffset_ = MM_FRI_QUEUE_INV_POINTS_OFFSET;
        uint256 queriesOffset_ = MM_FRI_QUEUE_QUERIES_OFFSET;

        uint256 lhashMask = getHashMask();
        assembly {
            let indexInNextStep := div(index, friCosetSize)
            mstore(merkleQueuePtr, indexInNextStep)
            mstore(add(merkleQueuePtr, 0x20), and(lhashMask, keccak256(evaluationsOnCosetPtr,
                                                                          mul(0x20,friCosetSize))))

            mstore(friQueueTail, friValue)
            mstore(add(friQueueTail, invPointsOffset_), cosetOffset_)
            mstore(add(friQueueTail, queriesOffset_), indexInNextStep)
        }
    }

    /*
      Computes the FRI step with eta = log2(friCosetSize) for all the live queries.
      The input data is located in the array:
          ctx[mmFriValues:]
          ctx[mmQueries:]
          ctx[mmFriInvPoints:]

      The function returns the new head pointer and the number of live
      queries remaining after computing the FRI step.

      The number of live queries decreases whenever multiple query points in the same
      coset are reduced to a single query in the next FRI layer.

      As the function computes the next layer it also collects that data from
      the previous layer for Merkle verification.
    */
    function computeNextLayer(
        uint256 channelPtr, uint256 friQueuePptr, uint256 merkleQueuePtr, uint256 nQueries,
        uint256 friEvalPoint, uint256 friCosetSize, uint256 friCtx)
        internal pure returns (uint256 nLiveQueries) {
        uint256 merkleQueueTail = merkleQueuePtr;
        uint256 friQueueHead = friQueuePptr;
        uint256 friQueueTail = friQueuePptr;
        uint256 friQueueEnd = friQueueHead + (0x20 * nQueries);

        do {
            uint256 cosetOffset;
            uint256 index;
            (friQueueHead, index, cosetOffset) = gatherCosetInputs(
                channelPtr, friCtx, friQueueHead, friCosetSize);

            doFriSteps(
                friCtx, friQueueTail, cosetOffset, friEvalPoint, friCosetSize, index,
                merkleQueueTail);

            merkleQueueTail += 0x40;
            friQueueTail += 0x20;
        } while (friQueueHead < friQueueEnd);
        return (friQueueTail - friQueuePptr) / 0x20;
    }

    /*
      Verifies FRI layers.

      Upon entry and every time we pass through the "if (index < layerSize)" condition:
          ctx[mmFriValues:] holds the input for the next layer.
          ctx[mmQueries:] holds query indices.
          ctx[mmFriInvPoints:] holds the evaluation points:
            ctx[mmFriInvPoints + i] = inverse(
                fmul(layerGenerator,  bitReverse(ctx[mmQueries+i], logLayerSize)).
    */
    function friVerifyLayers(
        uint256[] memory ctx)
        internal
    {
        initFriGroups(ctx);
        uint256 channelPtr = getChannelPtr(ctx);
        uint256 merkleQueuePtr = getMerkleQueuePtr(ctx);

        uint256 friStep = 1;
        uint256 nLiveQueries = ctx[MM_N_UNIQUE_QUERIES];

        // Add 0 at the end of the queries array to avoid empty array check in readNextElment.
        ctx[MM_FRI_QUERIES_DELIMITER] = 0;

        // Rather than converting all the values from Montgomery to standard form,
        // we can just pretend that the values are in standard form but all
        // the committed polynomials are multiplied by MontgomeryR.
        //
        // The values in the proof are already multiplied by MontgomeryR,
        // but the inputs from the OODS oracle need to be fixed.
        for (uint256 i = 0; i < nLiveQueries; i++ ) {
            ctx[MM_FRI_VALUES + i] = fmul(ctx[MM_FRI_VALUES + i], K_MONTGOMERY_R);
        }

        uint256 friCtx = getPtr(ctx, MM_FRI_CTX);
        uint256 friQueue = getPtr(ctx, MM_FRI_VALUES);

        uint256[] memory friSteps = getFriSteps(ctx);
        uint256 nFriSteps = friSteps.length;
        while (friStep < nFriSteps) {
            uint256 friCosetSize = 2**friSteps[friStep];

            nLiveQueries = computeNextLayer(
                channelPtr, friQueue, merkleQueuePtr, nLiveQueries,
                ctx[MM_FRI_EVAL_POINTS + friStep], friCosetSize, friCtx);

            MerkleVerifier.verify(
                channelPtr, merkleQueuePtr, bytes32(ctx[MM_FRI_COMMITMENTS + friStep - 1]),
                nLiveQueries);
            friStep++;
        }

        verifyLastLayer(ctx, nLiveQueries);
    }
}
