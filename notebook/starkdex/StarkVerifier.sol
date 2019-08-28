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

import "Fri.sol";
import "MemoryMap.sol";
import "MemoryAccessUtils.sol";
import "MerkleVerifierKeccak.sol";
import "VerifierChannel.sol";

contract StarkVerifier is MemoryMap, MemoryAccessUtils, VerifierChannel, MerkleVerifierKeccak, Fri {
    address oodsContractAddress;

    function airSpecificInit(uint256[] memory publicInput)
        internal returns (uint256[] memory ctx, uint256 logTraceLength);

    uint256 constant internal PROOF_PARAMS_LOG_BLOWUP_FACTOR_OFFSET = 0;
    uint256 constant internal PROOF_PARAMS_PROOF_OF_WORK_BITS_OFFSET = 1;
    uint256 constant internal PROOF_PARAMS_FRI_LAST_LAYER_DEG_BOUND_OFFSET = 2;
    uint256 constant internal PROOF_PARAMS_N_FRI_STEPS_OFFSET = 3;
    uint256 constant internal PROOF_PARAMS_FRI_STEPS_OFFSET = 4;

    function validateFriParams(
        uint256[] memory friSteps, uint256 logTraceLength, uint256 logFriLastLayerDegBound)
        internal pure {
        require (friSteps[0] == 0, "Only eta0 == 0 is currently supported");

        uint256 expectedLogDegBound = logFriLastLayerDegBound;
        uint256 nFriSteps = friSteps.length;
        for (uint256 i = 1; i < nFriSteps; i++) {
            uint256 friStep = friSteps[i];
            require(friStep > 0, "Only the first fri step can be 0");
            require(friStep <= 3, "Max supported fri step is 3.");
            expectedLogDegBound += friStep;
        }

        // FRI starts with a polynomial of degree 'traceLength'.
        // After applying all the FRI steps we expect to get a polynomial of degree less
        // than friLastLayerDegBound.
        require (
            expectedLogDegBound == logTraceLength, "Fri params do not match trace length");
    }

    uint256 constant internal SECURITY_BITS = 80;

    function initVerifierParams(uint256[] memory publicInput, uint256[] memory proofParams)
        internal returns (uint256[] memory ctx) {
        require (proofParams.length > PROOF_PARAMS_FRI_STEPS_OFFSET, "Invalid proofParams.");
        require (
            proofParams.length == (
                PROOF_PARAMS_FRI_STEPS_OFFSET + proofParams[PROOF_PARAMS_N_FRI_STEPS_OFFSET]),
            "Invalid proofParams.");
        uint256 logBlowupFactor = proofParams[PROOF_PARAMS_LOG_BLOWUP_FACTOR_OFFSET];
        require (logBlowupFactor <= 16, "logBlowupFactor must be at most 16");
        require (logBlowupFactor >= 1, "logBlowupFactor must be at least 1");

        uint256 proofOfWorkBits = proofParams[PROOF_PARAMS_PROOF_OF_WORK_BITS_OFFSET];
        require (proofOfWorkBits <= 50, "proofOfWorkBits must be at most 50");

        uint256 logFriLastLayerDegBound = (
            proofParams[PROOF_PARAMS_FRI_LAST_LAYER_DEG_BOUND_OFFSET]
        );
        require (
            logFriLastLayerDegBound <= 10, "logFriLastLayerDegBound must be at most 10.");

        uint256 nFriSteps = proofParams[PROOF_PARAMS_N_FRI_STEPS_OFFSET];
        require (nFriSteps <= 10, "Too many fri steps.");
        require (nFriSteps > 1, "Not enough fri steps.");

        uint256[] memory friSteps = new uint256[](nFriSteps);
        for (uint256 i = 0; i < nFriSteps; i++) {
            friSteps[i] = proofParams[PROOF_PARAMS_FRI_STEPS_OFFSET + i];
        }

        uint256 logTraceLength;
        (ctx, logTraceLength) = airSpecificInit(publicInput);

        validateFriParams(friSteps, logTraceLength, logFriLastLayerDegBound);

        uint256 friStepsPtr = getPtr(ctx, MM_FRI_STEPS_PTR);
        assembly {
            mstore(friStepsPtr, friSteps)
        }
        ctx[MM_FRI_LAST_LAYER_DEG_BOUND] = 2**logFriLastLayerDegBound;
        ctx[MM_TRACE_LENGTH] = 2 ** logTraceLength;

        ctx[MM_BLOW_UP_FACTOR] = 2**logBlowupFactor;
        ctx[MM_PROOF_OF_WORK_BITS] = proofOfWorkBits;

        // nQueries = roundup((SECURITY_BITS - proofOfWorkBits) / logBlowupFactor);
        uint256 nQueries = (
            (SECURITY_BITS - proofOfWorkBits + logBlowupFactor - 1) / logBlowupFactor
        );

        require (nQueries > 0, "Number of queries must be at least one");
        require (MAX_N_QUERIES >= nQueries, "Too many queries.");
        ctx[MM_N_UNIQUE_QUERIES] = nQueries;

        // We start with log_evalDomainSize = logTraceSize and update it here.
        ctx[MM_LOG_EVAL_DOMAIN_SIZE] = logTraceLength + logBlowupFactor;
        ctx[MM_EVAL_DOMAIN_SIZE] = 2**ctx[MM_LOG_EVAL_DOMAIN_SIZE];

        uint256 gen_evalDomain = fpow(GENERATOR_VAL, (K_MODULUS - 1) / ctx[MM_EVAL_DOMAIN_SIZE]);
        ctx[MM_EVAL_DOMAIN_GENERATOR] = gen_evalDomain;
        uint256 genTraceDomain = fpow(gen_evalDomain, ctx[MM_BLOW_UP_FACTOR]);
        ctx[MM_TRACE_GENERATOR] = genTraceDomain;
    }

    function getPublicInputHash(uint256[] memory publicInput) internal pure returns (bytes32);

    function oodsConsistencyCheck(uint256[] memory ctx) internal;

    function getNColumnsInTrace() internal pure returns(uint256);

    function getNColumnsInComposition() internal pure returns(uint256);

    function getMmCoefficients() internal pure returns(uint256);

    function getMmOodsValues() internal pure returns(uint256);

    function getMmOodsCoefficients() internal pure returns(uint256);

    function getNCoefficients() internal pure returns(uint256);

    function getNOodsValues() internal pure returns(uint256);

    function getNOodsCoefficients() internal pure returns(uint256);

    function hashRow(uint256[] memory ctx, uint256 offset, uint256 length)
    internal pure returns (uint256 res) {
        assembly {
            res := keccak256(add(add(ctx, 0x20), offset), length)
        }
        res &= getHashMask();
    }

    /*
      Adjusts the query indices and generates evaluation points for each query index.
      The operations above are independent but we can save gas by combining them as both
      operations require us to iterate the queries array.

      Indices adjustment:
          The query indices adjustment is needed because both the Merkle verification and FRI
          expect queries "full binary tree in array" indices.
          The adjustment is simply adding evalDomainSize to each query.
          Note that evalDomainSize == 2^(#FRI layers) == 2^(Merkle tree hight).

      evalPoints generation:
          for each query index "idx" we compute the corresponding evaluation point:
              g^(bitReverse(idx, log_evalDomainSize).
    */
    function adjustQueryIndicesAndPrepareEvalPoints(uint256[] memory ctx) internal {
        uint256 nUniqueQueries = ctx[MM_N_UNIQUE_QUERIES];
        uint256 queryPtr = getPtr(ctx, MM_QUERIES);
        uint256 queries_end = queryPtr + nUniqueQueries * 0x20;
        uint256 evalPointsPtr = getPtr(ctx, MM_OODS_EVAL_POINTS);
        uint256 log_evalDomainSize = ctx[MM_LOG_EVAL_DOMAIN_SIZE];
        uint256 evalDomainSize = ctx[MM_EVAL_DOMAIN_SIZE];
        uint256 evalDomainGenerator = ctx[MM_EVAL_DOMAIN_GENERATOR];

        assembly {
            /*
              Returns the bit reversal of value assuming it has the given number of bits.
              numberOfBits must be <= 64.
            */
            function bitReverse(value, numberOfBits) -> res {
                // Bit reverse value by swapping 1 bit chunks then 2 bit chunks and so forth.
                // Each swap is done by masking out and shifting one of the chunks by twice its size.
                // Finally, we use div to align the result to the right.
                res := value
                // Swap 1 bit chunks.
                res := or(mul(and(res, 0x5555555555555555), 0x4),
                        and(res, 0xaaaaaaaaaaaaaaaa))
                // Swap 2 bit chunks.
                res := or(mul(and(res, 0x6666666666666666), 0x10),
                        and(res, 0x19999999999999998))
                // Swap 4 bit chunks.
                res := or(mul(and(res, 0x7878787878787878), 0x100),
                        and(res, 0x78787878787878780))
                // Swap 8 bit chunks.
                res := or(mul(and(res, 0x7f807f807f807f80), 0x10000),
                        and(res, 0x7f807f807f807f8000))
                // Swap 16 bit chunks.
                res := or(mul(and(res, 0x7fff80007fff8000), 0x100000000),
                        and(res, 0x7fff80007fff80000000))
                // Swap 32 bit chunks.
                res := or(mul(and(res, 0x7fffffff80000000), 0x10000000000000000),
                        and(res, 0x7fffffff8000000000000000))
                // Right align the result.
                res := div(res, exp(2, sub(127, numberOfBits)))
            }

            function expmod(base, exponent, modulus) -> res {
                let p := mload(0x40)
                mstore(p, 0x20)                 // Length of Base.
                mstore(add(p, 0x20), 0x20)      // Length of Exponent.
                mstore(add(p, 0x40), 0x20)      // Length of Modulus.
                mstore(add(p, 0x60), base)      // Base.
                mstore(add(p, 0x80), exponent)  // Exponent.
                mstore(add(p, 0xa0), modulus)   // Modulus.
                // Call modexp precompile.
                if iszero(call(not(0), 0x05, 0, p, 0xc0, p, 0x20)) {
                    revert(0, 0)
                }
                res := mload(p)
            }

            let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001

            for {} lt(queryPtr, queries_end) {queryPtr := add(queryPtr, 0x20)} {
                let queryIdx := mload(queryPtr)
                // Adjust queryIdx, see comment in function description.
                let adjustedQueryIdx := add(queryIdx, evalDomainSize)
                mstore(queryPtr, adjustedQueryIdx)

                // Compute the evaluation point corresponding to the current queryIdx.
                mstore(evalPointsPtr, expmod(evalDomainGenerator,
                                             bitReverse(queryIdx, log_evalDomainSize),
                                             PRIME))
                evalPointsPtr := add(evalPointsPtr, 0x20)
            }
        }
    }

    function readQuriesResponsesAndDecommit(
        uint256[] memory ctx, uint256 nColumns, uint256 proofDataPtr, bytes32 merkleRoot)
         internal pure {
        uint256 nUniqueQueries = ctx[MM_N_UNIQUE_QUERIES];
        uint256 channelPtr = getPtr(ctx, MM_CHANNEL);
        uint256 queryPtr = getPtr(ctx, MM_QUERIES);
        uint256 queries_end = queryPtr + nUniqueQueries * 0x20;
        uint256 merkleQueuePtr = getPtr(ctx, MM_MERKLE_QUEUE);
        uint256 rowSize = 0x20 * nColumns;
        uint256 lhashMask = getHashMask();

        assembly {
            let proofPtr := mload(channelPtr)
            let merklePtr := merkleQueuePtr

            for {} lt(queryPtr, queries_end) {queryPtr := add(queryPtr, 0x20)} {
                let merkleLeaf := and(keccak256(proofPtr, rowSize), lhashMask)
                if eq(rowSize, 0x20) {
                    // If a leaf contains only 1 field element we don't hash it.
                    merkleLeaf := mload(proofPtr)
                }

                // push(queryIdx, hash(row)) to merkleQueue.
                mstore(merklePtr, mload(queryPtr))
                mstore(add(merklePtr, 0x20), merkleLeaf)
                merklePtr := add(merklePtr, 0x40)

                // Copy query responses to proofData array.
                // This array will be sent to the OODS contract.
                for {let proofDataChunk_end := add(proofPtr, rowSize)}
                        lt(proofPtr, proofDataChunk_end)
                        {proofPtr := add(proofPtr, 0x20)} {
                    mstore(proofDataPtr, mload(proofPtr))
                    proofDataPtr := add(proofDataPtr, 0x20)
                }
            }

            mstore(channelPtr, proofPtr)
        }

        MerkleVerifier.verify(channelPtr, merkleQueuePtr, merkleRoot, nUniqueQueries);
    }

    /*
      Computes the first FRI layer by reading the query responses and calling
      the OODS contract.

      The OODS contract will build and sum boundary constraints that check that
      the prover provided the proper evaluations for the Out of Domain Sampling.

      I.e. if the prover said that f(z) = c, the first FRI layer will include
      the term (f(x) - c)/(x-z).
    */
    function computeFirstFriLayer(uint256[] memory ctx) internal {
        adjustQueryIndicesAndPrepareEvalPoints(ctx);
        readQuriesResponsesAndDecommit(
            ctx, getNColumnsInTrace(), getPtr(ctx, MM_TRACE_QUERY_RESPONSES),
            bytes32(ctx[MM_TRACE_COMMITMENT]));

        readQuriesResponsesAndDecommit(
            ctx, getNColumnsInComposition(), getPtr(ctx, MM_COMPOSITION_QUERY_RESPONSES),
            bytes32(ctx[MM_OODS_COMMITMENT]));

        address oodsAddress = oodsContractAddress;
        uint256 friQueue = getPtr(ctx, MM_FRI_VALUES);
        uint256 returnDataSize = MAX_N_QUERIES * 0x40;
        assembly {
            // Call the OODS contract.
            if iszero(staticcall(not(0), oodsAddress, ctx,
                                 /*sizeof(ctx)*/ mul(add(mload(ctx), 1), 0x20),
                                 friQueue, returnDataSize)) {
              returndatacopy(0, 0, returndatasize)
              revert(0, returndatasize)
            }
        }
    }

    function verifyProof(
        uint256[] memory proofParams, uint256[] memory proof, uint256[] memory publicInput)
        internal {
        uint256[] memory ctx = initVerifierParams(publicInput, proofParams);
        uint256 channelPtr = getChannelPtr(ctx);

        initChannel(channelPtr,  getProofPtr(proof), getPublicInputHash(publicInput));

        // Read trace commitment.
        ctx[MM_TRACE_COMMITMENT] = uint256(readHash(channelPtr, true));
        VerifierChannel.sendFieldElements(
            channelPtr, getNCoefficients(), getPtr(ctx, getMmCoefficients()));

        ctx[MM_OODS_COMMITMENT] = uint256(readHash(channelPtr, true));

        // Send Out of Domain Sampling point.
        VerifierChannel.sendFieldElements(channelPtr, 1, getPtr(ctx, MM_OODS_POINT));

        // Read the answers to the Out of Domain Sampling.
        uint256 lmmOodsValues = getMmOodsValues();
        for (uint256 i = lmmOodsValues; i < lmmOodsValues+getNOodsValues(); i++) {
            ctx[i] = VerifierChannel.readFieldElement(channelPtr, true);
        }
        oodsConsistencyCheck(ctx);
        VerifierChannel.sendFieldElements(
            channelPtr, getNOodsCoefficients(), getPtr(ctx, getMmOodsCoefficients()));
        ctx[MM_FRI_COMMITMENTS] = uint256(VerifierChannel.readHash(channelPtr, true));

        uint256 nFriSteps = getFriSteps(ctx).length;
        uint256 fri_evalPointPtr = getPtr(ctx, MM_FRI_EVAL_POINTS);
        for (uint256 i = 1; i < nFriSteps - 1; i++) {
            VerifierChannel.sendFieldElements(channelPtr, 1, fri_evalPointPtr + i * 0x20);
            ctx[MM_FRI_COMMITMENTS + i] = uint256(VerifierChannel.readHash(channelPtr, true));
        }

        // Send last random FRI evaluation point.
        VerifierChannel.sendFieldElements(
            channelPtr, 1, getPtr(ctx, MM_FRI_EVAL_POINTS + nFriSteps - 1));

        // Read FRI last layer commitment.
        Fri.readLastLayer(ctx);

        // Generate queries.
        VerifierChannel.verifyProofOfWork(channelPtr, ctx[MM_PROOF_OF_WORK_BITS]);
        ctx[MM_N_UNIQUE_QUERIES] = VerifierChannel.sendRandomQueries(
            channelPtr, ctx[MM_N_UNIQUE_QUERIES], ctx[MM_EVAL_DOMAIN_SIZE] - 1,
            getPtr(ctx, MM_QUERIES));

        computeFirstFriLayer(ctx);

        friVerifyLayers(ctx);
    }
}
