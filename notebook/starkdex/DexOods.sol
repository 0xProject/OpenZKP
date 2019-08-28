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

import "MemoryMap.sol";
import "StarkParameters.sol";

contract DexOods is MemoryMap, StarkParameters {
    // For each query point we want to invert (2 + N_ROWS_IN_MASK) items:
    //  The query point itself (x).
    //  The denominator for the constraint polynomial (x-z^constraintDegree)
    //  [(x-(g^rowNumber)z) for rowNumber in mask].
    uint256 constant internal BATCH_INVERSE_CHUNK = (2 + N_ROWS_IN_MASK);
    uint256 constant internal BATCH_INVERSE_SIZE = MAX_N_QUERIES * BATCH_INVERSE_CHUNK;

    /*
      Builds and sums boundary constraints that check that the prover provided the proper
      Out of Domain evaluations for the trace and composition columns.

      The inputs to this function are:
        The verifier context.

      The boundary constraints for the trace enforce claims of the form f(g^k*z) = c by
      requiring the quotient (f(x) - c)/(x-g^k*z) to be a low degree polynomial.

      The boundary constraints for the composition enforce claims of the form h(z^d) = c by
      requiring the quotient (h(x) - c)/(x-z^d) to be a low degree polynomial.
      Where:
        f is a trace column.
        h is a composition column.
        z is the Out of Domain Sampling point.
        g is the trace generator
        k is the offset in the mask.
        d is the degree of the composition polynomial.
        c is the evaluation sent by the prover.
    */
    function() external {
        // This function assumes that the calldata contains the context as defined in MemoryMap.sol.
        // Note that ctx is a variable size array so the first uint256 cell contains its length.
        uint256[] memory ctx;
        assembly {
            let ctxSize := mul(add(calldataload(0), 1), 0x20)
            ctx := mload(0x40)
            mstore(0x40, add(ctx, ctxSize))
            calldatacopy(ctx, 0, ctxSize)
        }
        uint256[] memory batchInverseArray = new uint256[](2 * BATCH_INVERSE_SIZE);

        oodsPrepareInverses(ctx, batchInverseArray);

        uint256 kMontgomeryRInv_ = PrimeFieldElement0.K_MONTGOMERY_R_INV;

        assembly {
            let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001
            let kMontgomeryRInv := kMontgomeryRInv_
            let context := ctx
            let friValues := /*friValues*/ add(context, 0x720)
            let friValuesEnd := add(friValues,  mul(/*n_unique_queries*/ mload(add(context, 0x120)), 0x20))
            let friInvPoints := /*friInvPoints*/ add(context, 0x9e0)
            let traceQueryResponses := /*traceQueryQesponses*/ add(context, 0x5a80)

            let compositionQueryResponses := /*composition_query_responses*/ add(context, 0x7600)

            // Set denominatorsPtr to point to the batchInverseOut array.
            // The content of batchInverseOut is described in oodsPrepareInverses.
            let denominatorsPtr := add(batchInverseArray, 0x20)

            for {} lt(friValues, friValuesEnd) {friValues := add(friValues, 0x20)} {
                // res accumulates numbers modulo PRIME. Since 31*PRIME < 2**256, we may add up to
                // 31 numbers without fear of overflow, and use addmod modulo PRIME only every
                // 31 iterations, and once more at the very end.
                let res := 0

                // Trace constraints.

                // Mask items for column #0.
                {
                // Read the next element.
                let columnValue := mulmod(mload(traceQueryResponses), kMontgomeryRInv, PRIME)

                // res += c_0*(f_0(x) - f_0(z)) / (x - z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - z)^(-1)*/ mload(denominatorsPtr),
                                  /*oods_coefficients[0]*/ mload(add(context, 0x4a20)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[0]*/ mload(add(context, 0x3700)))),
                           PRIME))

                // res += c_1*(f_0(x) - f_0(g * z)) / (x - g * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g * z)^(-1)*/ mload(add(denominatorsPtr, 0x20)),
                                  /*oods_coefficients[1]*/ mload(add(context, 0x4a40)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[1]*/ mload(add(context, 0x3720)))),
                           PRIME))

                // res += c_2*(f_0(x) - f_0(g^255 * z)) / (x - g^255 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^255 * z)^(-1)*/ mload(add(denominatorsPtr, 0x380)),
                                  /*oods_coefficients[2]*/ mload(add(context, 0x4a60)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[2]*/ mload(add(context, 0x3740)))),
                           PRIME))

                // res += c_3*(f_0(x) - f_0(g^256 * z)) / (x - g^256 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^256 * z)^(-1)*/ mload(add(denominatorsPtr, 0x3a0)),
                                  /*oods_coefficients[3]*/ mload(add(context, 0x4a80)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[3]*/ mload(add(context, 0x3760)))),
                           PRIME))

                // res += c_4*(f_0(x) - f_0(g^511 * z)) / (x - g^511 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^511 * z)^(-1)*/ mload(add(denominatorsPtr, 0x3c0)),
                                  /*oods_coefficients[4]*/ mload(add(context, 0x4aa0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[4]*/ mload(add(context, 0x3780)))),
                           PRIME))

                // res += c_5*(f_0(x) - f_0(g^(512 * (vaults_path_length - 1) + 511) * z)) / (x - g^(512 * (vaults_path_length - 1) + 511) * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^(512 * (vaults_path_length - 1) + 511) * z)^(-1)*/ mload(add(denominatorsPtr, 0xc20)),
                                  /*oods_coefficients[5]*/ mload(add(context, 0x4ac0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[5]*/ mload(add(context, 0x37a0)))),
                           PRIME))

                // res += c_6*(f_0(x) - f_0(g^(16384 + 512 * (vaults_path_length - 1) + 511) * z)) / (x - g^(16384 + 512 * (vaults_path_length - 1) + 511) * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^(16384 + 512 * (vaults_path_length - 1) + 511) * z)^(-1)*/ mload(add(denominatorsPtr, 0xc40)),
                                  /*oods_coefficients[6]*/ mload(add(context, 0x4ae0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[6]*/ mload(add(context, 0x37c0)))),
                           PRIME))
                }

                // Mask items for column #1.
                {
                // Read the next element.
                let columnValue := mulmod(mload(add(traceQueryResponses, 0x20)), kMontgomeryRInv, PRIME)

                // res += c_7*(f_1(x) - f_1(z)) / (x - z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - z)^(-1)*/ mload(denominatorsPtr),
                                  /*oods_coefficients[7]*/ mload(add(context, 0x4b00)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[7]*/ mload(add(context, 0x37e0)))),
                           PRIME))

                // res += c_8*(f_1(x) - f_1(g * z)) / (x - g * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g * z)^(-1)*/ mload(add(denominatorsPtr, 0x20)),
                                  /*oods_coefficients[8]*/ mload(add(context, 0x4b20)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[8]*/ mload(add(context, 0x3800)))),
                           PRIME))

                // res += c_9*(f_1(x) - f_1(g^255 * z)) / (x - g^255 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^255 * z)^(-1)*/ mload(add(denominatorsPtr, 0x380)),
                                  /*oods_coefficients[9]*/ mload(add(context, 0x4b40)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[9]*/ mload(add(context, 0x3820)))),
                           PRIME))

                // res += c_10*(f_1(x) - f_1(g^256 * z)) / (x - g^256 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^256 * z)^(-1)*/ mload(add(denominatorsPtr, 0x3a0)),
                                  /*oods_coefficients[10]*/ mload(add(context, 0x4b60)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[10]*/ mload(add(context, 0x3840)))),
                           PRIME))
                }

                // Mask items for column #2.
                {
                // Read the next element.
                let columnValue := mulmod(mload(add(traceQueryResponses, 0x40)), kMontgomeryRInv, PRIME)

                // res += c_11*(f_2(x) - f_2(z)) / (x - z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - z)^(-1)*/ mload(denominatorsPtr),
                                  /*oods_coefficients[11]*/ mload(add(context, 0x4b80)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[11]*/ mload(add(context, 0x3860)))),
                           PRIME))
                }

                // Mask items for column #3.
                {
                // Read the next element.
                let columnValue := mulmod(mload(add(traceQueryResponses, 0x60)), kMontgomeryRInv, PRIME)

                // res += c_12*(f_3(x) - f_3(z)) / (x - z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - z)^(-1)*/ mload(denominatorsPtr),
                                  /*oods_coefficients[12]*/ mload(add(context, 0x4ba0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[12]*/ mload(add(context, 0x3880)))),
                           PRIME))

                // res += c_13*(f_3(x) - f_3(g * z)) / (x - g * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g * z)^(-1)*/ mload(add(denominatorsPtr, 0x20)),
                                  /*oods_coefficients[13]*/ mload(add(context, 0x4bc0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[13]*/ mload(add(context, 0x38a0)))),
                           PRIME))

                // res += c_14*(f_3(x) - f_3(g^256 * z)) / (x - g^256 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^256 * z)^(-1)*/ mload(add(denominatorsPtr, 0x3a0)),
                                  /*oods_coefficients[14]*/ mload(add(context, 0x4be0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[14]*/ mload(add(context, 0x38c0)))),
                           PRIME))

                // res += c_15*(f_3(x) - f_3(g^512 * z)) / (x - g^512 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^512 * z)^(-1)*/ mload(add(denominatorsPtr, 0x3e0)),
                                  /*oods_coefficients[15]*/ mload(add(context, 0x4c00)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[15]*/ mload(add(context, 0x38e0)))),
                           PRIME))

                // res += c_16*(f_3(x) - f_3(g^768 * z)) / (x - g^768 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^768 * z)^(-1)*/ mload(add(denominatorsPtr, 0x420)),
                                  /*oods_coefficients[16]*/ mload(add(context, 0x4c20)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[16]*/ mload(add(context, 0x3900)))),
                           PRIME))
                }

                // Mask items for column #4.
                {
                // Read the next element.
                let columnValue := mulmod(mload(add(traceQueryResponses, 0x80)), kMontgomeryRInv, PRIME)

                // res += c_17*(f_4(x) - f_4(z)) / (x - z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - z)^(-1)*/ mload(denominatorsPtr),
                                  /*oods_coefficients[17]*/ mload(add(context, 0x4c40)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[17]*/ mload(add(context, 0x3920)))),
                           PRIME))

                // res += c_18*(f_4(x) - f_4(g * z)) / (x - g * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g * z)^(-1)*/ mload(add(denominatorsPtr, 0x20)),
                                  /*oods_coefficients[18]*/ mload(add(context, 0x4c60)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[18]*/ mload(add(context, 0x3940)))),
                           PRIME))

                // res += c_19*(f_4(x) - f_4(g^255 * z)) / (x - g^255 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^255 * z)^(-1)*/ mload(add(denominatorsPtr, 0x380)),
                                  /*oods_coefficients[19]*/ mload(add(context, 0x4c80)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[19]*/ mload(add(context, 0x3960)))),
                           PRIME))

                // res += c_20*(f_4(x) - f_4(g^256 * z)) / (x - g^256 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^256 * z)^(-1)*/ mload(add(denominatorsPtr, 0x3a0)),
                                  /*oods_coefficients[20]*/ mload(add(context, 0x4ca0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[20]*/ mload(add(context, 0x3980)))),
                           PRIME))

                // res += c_21*(f_4(x) - f_4(g^511 * z)) / (x - g^511 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^511 * z)^(-1)*/ mload(add(denominatorsPtr, 0x3c0)),
                                  /*oods_coefficients[21]*/ mload(add(context, 0x4cc0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[21]*/ mload(add(context, 0x39a0)))),
                           PRIME))

                // res += c_22*(f_4(x) - f_4(g^(49152 + 512 * (vaults_path_length - 1) + 511) * z)) / (x - g^(49152 + 512 * (vaults_path_length - 1) + 511) * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^(49152 + 512 * (vaults_path_length - 1) + 511) * z)^(-1)*/ mload(add(denominatorsPtr, 0xc60)),
                                  /*oods_coefficients[22]*/ mload(add(context, 0x4ce0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[22]*/ mload(add(context, 0x39c0)))),
                           PRIME))

                // res += c_23*(f_4(x) - f_4(g^(512 * (vaults_path_length - 1) + 511) * z)) / (x - g^(512 * (vaults_path_length - 1) + 511) * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^(512 * (vaults_path_length - 1) + 511) * z)^(-1)*/ mload(add(denominatorsPtr, 0xc20)),
                                  /*oods_coefficients[23]*/ mload(add(context, 0x4d00)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[23]*/ mload(add(context, 0x39e0)))),
                           PRIME))
                }

                // Mask items for column #5.
                {
                // Read the next element.
                let columnValue := mulmod(mload(add(traceQueryResponses, 0xa0)), kMontgomeryRInv, PRIME)

                // res += c_24*(f_5(x) - f_5(z)) / (x - z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - z)^(-1)*/ mload(denominatorsPtr),
                                  /*oods_coefficients[24]*/ mload(add(context, 0x4d20)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[24]*/ mload(add(context, 0x3a00)))),
                           PRIME))

                // res += c_25*(f_5(x) - f_5(g * z)) / (x - g * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g * z)^(-1)*/ mload(add(denominatorsPtr, 0x20)),
                                  /*oods_coefficients[25]*/ mload(add(context, 0x4d40)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[25]*/ mload(add(context, 0x3a20)))),
                           PRIME))

                // res += c_26*(f_5(x) - f_5(g^255 * z)) / (x - g^255 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^255 * z)^(-1)*/ mload(add(denominatorsPtr, 0x380)),
                                  /*oods_coefficients[26]*/ mload(add(context, 0x4d60)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[26]*/ mload(add(context, 0x3a40)))),
                           PRIME))

                // res += c_27*(f_5(x) - f_5(g^256 * z)) / (x - g^256 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^256 * z)^(-1)*/ mload(add(denominatorsPtr, 0x3a0)),
                                  /*oods_coefficients[27]*/ mload(add(context, 0x4d80)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[27]*/ mload(add(context, 0x3a60)))),
                           PRIME))
                }

                // Mask items for column #6.
                {
                // Read the next element.
                let columnValue := mulmod(mload(add(traceQueryResponses, 0xc0)), kMontgomeryRInv, PRIME)

                // res += c_28*(f_6(x) - f_6(z)) / (x - z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - z)^(-1)*/ mload(denominatorsPtr),
                                  /*oods_coefficients[28]*/ mload(add(context, 0x4da0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[28]*/ mload(add(context, 0x3a80)))),
                           PRIME))

                // res += c_29*(f_6(x) - f_6(g^255 * z)) / (x - g^255 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^255 * z)^(-1)*/ mload(add(denominatorsPtr, 0x380)),
                                  /*oods_coefficients[29]*/ mload(add(context, 0x4dc0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[29]*/ mload(add(context, 0x3aa0)))),
                           PRIME))

                // res += c_30*(f_6(x) - f_6(g^767 * z)) / (x - g^767 * z)
                res := addmod(
                    res,
                    mulmod(mulmod(/*(x - g^767 * z)^(-1)*/ mload(add(denominatorsPtr, 0x400)),
                                  /*oods_coefficients[30]*/ mload(add(context, 0x4de0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[30]*/ mload(add(context, 0x3ac0)))),
                           PRIME),
                    PRIME)

                // res += c_31*(f_6(x) - f_6(g^1279 * z)) / (x - g^1279 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^1279 * z)^(-1)*/ mload(add(denominatorsPtr, 0x500)),
                                  /*oods_coefficients[31]*/ mload(add(context, 0x4e00)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[31]*/ mload(add(context, 0x3ae0)))),
                           PRIME))

                // res += c_32*(f_6(x) - f_6(g^16639 * z)) / (x - g^16639 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^16639 * z)^(-1)*/ mload(add(denominatorsPtr, 0x860)),
                                  /*oods_coefficients[32]*/ mload(add(context, 0x4e20)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[32]*/ mload(add(context, 0x3b00)))),
                           PRIME))

                // res += c_33*(f_6(x) - f_6(g^33023 * z)) / (x - g^33023 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^33023 * z)^(-1)*/ mload(add(denominatorsPtr, 0xa60)),
                                  /*oods_coefficients[33]*/ mload(add(context, 0x4e40)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[33]*/ mload(add(context, 0x3b20)))),
                           PRIME))

                // res += c_34*(f_6(x) - f_6(g^49407 * z)) / (x - g^49407 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^49407 * z)^(-1)*/ mload(add(denominatorsPtr, 0xb80)),
                                  /*oods_coefficients[34]*/ mload(add(context, 0x4e60)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[34]*/ mload(add(context, 0x3b40)))),
                           PRIME))
                }

                // Mask items for column #7.
                {
                // Read the next element.
                let columnValue := mulmod(mload(add(traceQueryResponses, 0xe0)), kMontgomeryRInv, PRIME)

                // res += c_35*(f_7(x) - f_7(z)) / (x - z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - z)^(-1)*/ mload(denominatorsPtr),
                                  /*oods_coefficients[35]*/ mload(add(context, 0x4e80)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[35]*/ mload(add(context, 0x3b60)))),
                           PRIME))

                // res += c_36*(f_7(x) - f_7(g * z)) / (x - g * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g * z)^(-1)*/ mload(add(denominatorsPtr, 0x20)),
                                  /*oods_coefficients[36]*/ mload(add(context, 0x4ea0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[36]*/ mload(add(context, 0x3b80)))),
                           PRIME))

                // res += c_37*(f_7(x) - f_7(g^256 * z)) / (x - g^256 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^256 * z)^(-1)*/ mload(add(denominatorsPtr, 0x3a0)),
                                  /*oods_coefficients[37]*/ mload(add(context, 0x4ec0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[37]*/ mload(add(context, 0x3ba0)))),
                           PRIME))

                // res += c_38*(f_7(x) - f_7(g^512 * z)) / (x - g^512 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^512 * z)^(-1)*/ mload(add(denominatorsPtr, 0x3e0)),
                                  /*oods_coefficients[38]*/ mload(add(context, 0x4ee0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[38]*/ mload(add(context, 0x3bc0)))),
                           PRIME))

                // res += c_39*(f_7(x) - f_7(g^768 * z)) / (x - g^768 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^768 * z)^(-1)*/ mload(add(denominatorsPtr, 0x420)),
                                  /*oods_coefficients[39]*/ mload(add(context, 0x4f00)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[39]*/ mload(add(context, 0x3be0)))),
                           PRIME))
                }

                // Mask items for column #8.
                {
                // Read the next element.
                let columnValue := mulmod(mload(add(traceQueryResponses, 0x100)), kMontgomeryRInv, PRIME)

                // res += c_40*(f_8(x) - f_8(z)) / (x - z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - z)^(-1)*/ mload(denominatorsPtr),
                                  /*oods_coefficients[40]*/ mload(add(context, 0x4f20)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[40]*/ mload(add(context, 0x3c00)))),
                           PRIME))

                // res += c_41*(f_8(x) - f_8(g * z)) / (x - g * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g * z)^(-1)*/ mload(add(denominatorsPtr, 0x20)),
                                  /*oods_coefficients[41]*/ mload(add(context, 0x4f40)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[41]*/ mload(add(context, 0x3c20)))),
                           PRIME))

                // res += c_42*(f_8(x) - f_8(g^2 * z)) / (x - g^2 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^2 * z)^(-1)*/ mload(add(denominatorsPtr, 0x40)),
                                  /*oods_coefficients[42]*/ mload(add(context, 0x4f60)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[42]*/ mload(add(context, 0x3c40)))),
                           PRIME))

                // res += c_43*(f_8(x) - f_8(g^3 * z)) / (x - g^3 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^3 * z)^(-1)*/ mload(add(denominatorsPtr, 0x60)),
                                  /*oods_coefficients[43]*/ mload(add(context, 0x4f80)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[43]*/ mload(add(context, 0x3c60)))),
                           PRIME))

                // res += c_44*(f_8(x) - f_8(g^4 * z)) / (x - g^4 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^4 * z)^(-1)*/ mload(add(denominatorsPtr, 0x80)),
                                  /*oods_coefficients[44]*/ mload(add(context, 0x4fa0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[44]*/ mload(add(context, 0x3c80)))),
                           PRIME))

                // res += c_45*(f_8(x) - f_8(g^6 * z)) / (x - g^6 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^6 * z)^(-1)*/ mload(add(denominatorsPtr, 0xa0)),
                                  /*oods_coefficients[45]*/ mload(add(context, 0x4fc0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[45]*/ mload(add(context, 0x3ca0)))),
                           PRIME))

                // res += c_46*(f_8(x) - f_8(g^7 * z)) / (x - g^7 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^7 * z)^(-1)*/ mload(add(denominatorsPtr, 0xc0)),
                                  /*oods_coefficients[46]*/ mload(add(context, 0x4fe0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[46]*/ mload(add(context, 0x3cc0)))),
                           PRIME))

                // res += c_47*(f_8(x) - f_8(g^1020 * z)) / (x - g^1020 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^1020 * z)^(-1)*/ mload(add(denominatorsPtr, 0x440)),
                                  /*oods_coefficients[47]*/ mload(add(context, 0x5000)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[47]*/ mload(add(context, 0x3ce0)))),
                           PRIME))

                // res += c_48*(f_8(x) - f_8(g^1021 * z)) / (x - g^1021 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^1021 * z)^(-1)*/ mload(add(denominatorsPtr, 0x460)),
                                  /*oods_coefficients[48]*/ mload(add(context, 0x5020)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[48]*/ mload(add(context, 0x3d00)))),
                           PRIME))

                // res += c_49*(f_8(x) - f_8(g^1022 * z)) / (x - g^1022 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^1022 * z)^(-1)*/ mload(add(denominatorsPtr, 0x480)),
                                  /*oods_coefficients[49]*/ mload(add(context, 0x5040)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[49]*/ mload(add(context, 0x3d20)))),
                           PRIME))

                // res += c_50*(f_8(x) - f_8(g^1024 * z)) / (x - g^1024 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^1024 * z)^(-1)*/ mload(add(denominatorsPtr, 0x4a0)),
                                  /*oods_coefficients[50]*/ mload(add(context, 0x5060)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[50]*/ mload(add(context, 0x3d40)))),
                           PRIME))

                // res += c_51*(f_8(x) - f_8(g^1026 * z)) / (x - g^1026 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^1026 * z)^(-1)*/ mload(add(denominatorsPtr, 0x4c0)),
                                  /*oods_coefficients[51]*/ mload(add(context, 0x5080)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[51]*/ mload(add(context, 0x3d60)))),
                           PRIME))

                // res += c_52*(f_8(x) - f_8(g^1027 * z)) / (x - g^1027 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^1027 * z)^(-1)*/ mload(add(denominatorsPtr, 0x4e0)),
                                  /*oods_coefficients[52]*/ mload(add(context, 0x50a0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[52]*/ mload(add(context, 0x3d80)))),
                           PRIME))

                // res += c_53*(f_8(x) - f_8(g^2044 * z)) / (x - g^2044 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^2044 * z)^(-1)*/ mload(add(denominatorsPtr, 0x520)),
                                  /*oods_coefficients[53]*/ mload(add(context, 0x50c0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[53]*/ mload(add(context, 0x3da0)))),
                           PRIME))

                // res += c_54*(f_8(x) - f_8(g^2045 * z)) / (x - g^2045 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^2045 * z)^(-1)*/ mload(add(denominatorsPtr, 0x540)),
                                  /*oods_coefficients[54]*/ mload(add(context, 0x50e0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[54]*/ mload(add(context, 0x3dc0)))),
                           PRIME))

                // res += c_55*(f_8(x) - f_8(g^2051 * z)) / (x - g^2051 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^2051 * z)^(-1)*/ mload(add(denominatorsPtr, 0x560)),
                                  /*oods_coefficients[55]*/ mload(add(context, 0x5100)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[55]*/ mload(add(context, 0x3de0)))),
                           PRIME))

                // res += c_56*(f_8(x) - f_8(g^3069 * z)) / (x - g^3069 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^3069 * z)^(-1)*/ mload(add(denominatorsPtr, 0x580)),
                                  /*oods_coefficients[56]*/ mload(add(context, 0x5120)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[56]*/ mload(add(context, 0x3e00)))),
                           PRIME))

                // res += c_57*(f_8(x) - f_8(g^3075 * z)) / (x - g^3075 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^3075 * z)^(-1)*/ mload(add(denominatorsPtr, 0x5a0)),
                                  /*oods_coefficients[57]*/ mload(add(context, 0x5140)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[57]*/ mload(add(context, 0x3e20)))),
                           PRIME))

                // res += c_58*(f_8(x) - f_8(g^4092 * z)) / (x - g^4092 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^4092 * z)^(-1)*/ mload(add(denominatorsPtr, 0x5c0)),
                                  /*oods_coefficients[58]*/ mload(add(context, 0x5160)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[58]*/ mload(add(context, 0x3e40)))),
                           PRIME))

                // res += c_59*(f_8(x) - f_8(g^4093 * z)) / (x - g^4093 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^4093 * z)^(-1)*/ mload(add(denominatorsPtr, 0x5e0)),
                                  /*oods_coefficients[59]*/ mload(add(context, 0x5180)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[59]*/ mload(add(context, 0x3e60)))),
                           PRIME))

                // res += c_60*(f_8(x) - f_8(g^4099 * z)) / (x - g^4099 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^4099 * z)^(-1)*/ mload(add(denominatorsPtr, 0x600)),
                                  /*oods_coefficients[60]*/ mload(add(context, 0x51a0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[60]*/ mload(add(context, 0x3e80)))),
                           PRIME))

                // res += c_61*(f_8(x) - f_8(g^5123 * z)) / (x - g^5123 * z)
                res := addmod(
                    res,
                    mulmod(mulmod(/*(x - g^5123 * z)^(-1)*/ mload(add(denominatorsPtr, 0x620)),
                                  /*oods_coefficients[61]*/ mload(add(context, 0x51c0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[61]*/ mload(add(context, 0x3ea0)))),
                           PRIME),
                    PRIME)

                // res += c_62*(f_8(x) - f_8(g^6141 * z)) / (x - g^6141 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^6141 * z)^(-1)*/ mload(add(denominatorsPtr, 0x640)),
                                  /*oods_coefficients[62]*/ mload(add(context, 0x51e0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[62]*/ mload(add(context, 0x3ec0)))),
                           PRIME))

                // res += c_63*(f_8(x) - f_8(g^7171 * z)) / (x - g^7171 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^7171 * z)^(-1)*/ mload(add(denominatorsPtr, 0x660)),
                                  /*oods_coefficients[63]*/ mload(add(context, 0x5200)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[63]*/ mload(add(context, 0x3ee0)))),
                           PRIME))

                // res += c_64*(f_8(x) - f_8(g^8188 * z)) / (x - g^8188 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^8188 * z)^(-1)*/ mload(add(denominatorsPtr, 0x680)),
                                  /*oods_coefficients[64]*/ mload(add(context, 0x5220)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[64]*/ mload(add(context, 0x3f00)))),
                           PRIME))

                // res += c_65*(f_8(x) - f_8(g^8195 * z)) / (x - g^8195 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^8195 * z)^(-1)*/ mload(add(denominatorsPtr, 0x6a0)),
                                  /*oods_coefficients[65]*/ mload(add(context, 0x5240)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[65]*/ mload(add(context, 0x3f20)))),
                           PRIME))

                // res += c_66*(f_8(x) - f_8(g^9219 * z)) / (x - g^9219 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^9219 * z)^(-1)*/ mload(add(denominatorsPtr, 0x6e0)),
                                  /*oods_coefficients[66]*/ mload(add(context, 0x5260)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[66]*/ mload(add(context, 0x3f40)))),
                           PRIME))

                // res += c_67*(f_8(x) - f_8(g^10237 * z)) / (x - g^10237 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^10237 * z)^(-1)*/ mload(add(denominatorsPtr, 0x700)),
                                  /*oods_coefficients[67]*/ mload(add(context, 0x5280)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[67]*/ mload(add(context, 0x3f60)))),
                           PRIME))

                // res += c_68*(f_8(x) - f_8(g^11267 * z)) / (x - g^11267 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^11267 * z)^(-1)*/ mload(add(denominatorsPtr, 0x720)),
                                  /*oods_coefficients[68]*/ mload(add(context, 0x52a0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[68]*/ mload(add(context, 0x3f80)))),
                           PRIME))

                // res += c_69*(f_8(x) - f_8(g^12284 * z)) / (x - g^12284 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^12284 * z)^(-1)*/ mload(add(denominatorsPtr, 0x740)),
                                  /*oods_coefficients[69]*/ mload(add(context, 0x52c0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[69]*/ mload(add(context, 0x3fa0)))),
                           PRIME))

                // res += c_70*(f_8(x) - f_8(g^12285 * z)) / (x - g^12285 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^12285 * z)^(-1)*/ mload(add(denominatorsPtr, 0x760)),
                                  /*oods_coefficients[70]*/ mload(add(context, 0x52e0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[70]*/ mload(add(context, 0x3fc0)))),
                           PRIME))

                // res += c_71*(f_8(x) - f_8(g^19459 * z)) / (x - g^19459 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^19459 * z)^(-1)*/ mload(add(denominatorsPtr, 0x880)),
                                  /*oods_coefficients[71]*/ mload(add(context, 0x5300)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[71]*/ mload(add(context, 0x3fe0)))),
                           PRIME))

                // res += c_72*(f_8(x) - f_8(g^20477 * z)) / (x - g^20477 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^20477 * z)^(-1)*/ mload(add(denominatorsPtr, 0x8a0)),
                                  /*oods_coefficients[72]*/ mload(add(context, 0x5320)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[72]*/ mload(add(context, 0x4000)))),
                           PRIME))

                // res += c_73*(f_8(x) - f_8(g^27651 * z)) / (x - g^27651 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^27651 * z)^(-1)*/ mload(add(denominatorsPtr, 0x8c0)),
                                  /*oods_coefficients[73]*/ mload(add(context, 0x5340)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[73]*/ mload(add(context, 0x4020)))),
                           PRIME))

                // res += c_74*(f_8(x) - f_8(g^28669 * z)) / (x - g^28669 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^28669 * z)^(-1)*/ mload(add(denominatorsPtr, 0x8e0)),
                                  /*oods_coefficients[74]*/ mload(add(context, 0x5360)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[74]*/ mload(add(context, 0x4040)))),
                           PRIME))

                // res += c_75*(f_8(x) - f_8(g^35843 * z)) / (x - g^35843 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^35843 * z)^(-1)*/ mload(add(denominatorsPtr, 0xa80)),
                                  /*oods_coefficients[75]*/ mload(add(context, 0x5380)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[75]*/ mload(add(context, 0x4060)))),
                           PRIME))

                // res += c_76*(f_8(x) - f_8(g^36867 * z)) / (x - g^36867 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^36867 * z)^(-1)*/ mload(add(denominatorsPtr, 0xaa0)),
                                  /*oods_coefficients[76]*/ mload(add(context, 0x53a0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[76]*/ mload(add(context, 0x4080)))),
                           PRIME))

                // res += c_77*(f_8(x) - f_8(g^37891 * z)) / (x - g^37891 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^37891 * z)^(-1)*/ mload(add(denominatorsPtr, 0xac0)),
                                  /*oods_coefficients[77]*/ mload(add(context, 0x53c0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[77]*/ mload(add(context, 0x40a0)))),
                           PRIME))

                // res += c_78*(f_8(x) - f_8(g^39939 * z)) / (x - g^39939 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^39939 * z)^(-1)*/ mload(add(denominatorsPtr, 0xae0)),
                                  /*oods_coefficients[78]*/ mload(add(context, 0x53e0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[78]*/ mload(add(context, 0x40c0)))),
                           PRIME))

                // res += c_79*(f_8(x) - f_8(g^40956 * z)) / (x - g^40956 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^40956 * z)^(-1)*/ mload(add(denominatorsPtr, 0xb00)),
                                  /*oods_coefficients[79]*/ mload(add(context, 0x5400)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[79]*/ mload(add(context, 0x40e0)))),
                           PRIME))

                // res += c_80*(f_8(x) - f_8(g^44035 * z)) / (x - g^44035 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^44035 * z)^(-1)*/ mload(add(denominatorsPtr, 0xb20)),
                                  /*oods_coefficients[80]*/ mload(add(context, 0x5420)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[80]*/ mload(add(context, 0x4100)))),
                           PRIME))

                // res += c_81*(f_8(x) - f_8(g^52227 * z)) / (x - g^52227 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^52227 * z)^(-1)*/ mload(add(denominatorsPtr, 0xba0)),
                                  /*oods_coefficients[81]*/ mload(add(context, 0x5440)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[81]*/ mload(add(context, 0x4120)))),
                           PRIME))

                // res += c_82*(f_8(x) - f_8(g^60419 * z)) / (x - g^60419 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^60419 * z)^(-1)*/ mload(add(denominatorsPtr, 0xbc0)),
                                  /*oods_coefficients[82]*/ mload(add(context, 0x5460)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[82]*/ mload(add(context, 0x4140)))),
                           PRIME))
                }

                // Mask items for column #9.
                {
                // Read the next element.
                let columnValue := mulmod(mload(add(traceQueryResponses, 0x120)), kMontgomeryRInv, PRIME)

                // res += c_83*(f_9(x) - f_9(z)) / (x - z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - z)^(-1)*/ mload(denominatorsPtr),
                                  /*oods_coefficients[83]*/ mload(add(context, 0x5480)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[83]*/ mload(add(context, 0x4160)))),
                           PRIME))

                // res += c_84*(f_9(x) - f_9(g^4 * z)) / (x - g^4 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^4 * z)^(-1)*/ mload(add(denominatorsPtr, 0x80)),
                                  /*oods_coefficients[84]*/ mload(add(context, 0x54a0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[84]*/ mload(add(context, 0x4180)))),
                           PRIME))

                // res += c_85*(f_9(x) - f_9(g^8 * z)) / (x - g^8 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^8 * z)^(-1)*/ mload(add(denominatorsPtr, 0xe0)),
                                  /*oods_coefficients[85]*/ mload(add(context, 0x54c0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[85]*/ mload(add(context, 0x41a0)))),
                           PRIME))

                // res += c_86*(f_9(x) - f_9(g^16 * z)) / (x - g^16 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^16 * z)^(-1)*/ mload(add(denominatorsPtr, 0x100)),
                                  /*oods_coefficients[86]*/ mload(add(context, 0x54e0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[86]*/ mload(add(context, 0x41c0)))),
                           PRIME))

                // res += c_87*(f_9(x) - f_9(g^20 * z)) / (x - g^20 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^20 * z)^(-1)*/ mload(add(denominatorsPtr, 0x120)),
                                  /*oods_coefficients[87]*/ mload(add(context, 0x5500)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[87]*/ mload(add(context, 0x41e0)))),
                           PRIME))

                // res += c_88*(f_9(x) - f_9(g^24 * z)) / (x - g^24 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^24 * z)^(-1)*/ mload(add(denominatorsPtr, 0x140)),
                                  /*oods_coefficients[88]*/ mload(add(context, 0x5520)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[88]*/ mload(add(context, 0x4200)))),
                           PRIME))

                // res += c_89*(f_9(x) - f_9(g^32 * z)) / (x - g^32 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^32 * z)^(-1)*/ mload(add(denominatorsPtr, 0x160)),
                                  /*oods_coefficients[89]*/ mload(add(context, 0x5540)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[89]*/ mload(add(context, 0x4220)))),
                           PRIME))

                // res += c_90*(f_9(x) - f_9(g^36 * z)) / (x - g^36 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^36 * z)^(-1)*/ mload(add(denominatorsPtr, 0x180)),
                                  /*oods_coefficients[90]*/ mload(add(context, 0x5560)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[90]*/ mload(add(context, 0x4240)))),
                           PRIME))

                // res += c_91*(f_9(x) - f_9(g^40 * z)) / (x - g^40 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^40 * z)^(-1)*/ mload(add(denominatorsPtr, 0x1a0)),
                                  /*oods_coefficients[91]*/ mload(add(context, 0x5580)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[91]*/ mload(add(context, 0x4260)))),
                           PRIME))

                // res += c_92*(f_9(x) - f_9(g^48 * z)) / (x - g^48 * z)
                res := addmod(
                    res,
                    mulmod(mulmod(/*(x - g^48 * z)^(-1)*/ mload(add(denominatorsPtr, 0x1c0)),
                                  /*oods_coefficients[92]*/ mload(add(context, 0x55a0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[92]*/ mload(add(context, 0x4280)))),
                           PRIME),
                    PRIME)

                // res += c_93*(f_9(x) - f_9(g^56 * z)) / (x - g^56 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^56 * z)^(-1)*/ mload(add(denominatorsPtr, 0x1e0)),
                                  /*oods_coefficients[93]*/ mload(add(context, 0x55c0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[93]*/ mload(add(context, 0x42a0)))),
                           PRIME))

                // res += c_94*(f_9(x) - f_9(g^64 * z)) / (x - g^64 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^64 * z)^(-1)*/ mload(add(denominatorsPtr, 0x200)),
                                  /*oods_coefficients[94]*/ mload(add(context, 0x55e0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[94]*/ mload(add(context, 0x42c0)))),
                           PRIME))

                // res += c_95*(f_9(x) - f_9(g^68 * z)) / (x - g^68 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^68 * z)^(-1)*/ mload(add(denominatorsPtr, 0x220)),
                                  /*oods_coefficients[95]*/ mload(add(context, 0x5600)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[95]*/ mload(add(context, 0x42e0)))),
                           PRIME))

                // res += c_96*(f_9(x) - f_9(g^72 * z)) / (x - g^72 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^72 * z)^(-1)*/ mload(add(denominatorsPtr, 0x240)),
                                  /*oods_coefficients[96]*/ mload(add(context, 0x5620)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[96]*/ mload(add(context, 0x4300)))),
                           PRIME))

                // res += c_97*(f_9(x) - f_9(g^84 * z)) / (x - g^84 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^84 * z)^(-1)*/ mload(add(denominatorsPtr, 0x260)),
                                  /*oods_coefficients[97]*/ mload(add(context, 0x5640)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[97]*/ mload(add(context, 0x4320)))),
                           PRIME))

                // res += c_98*(f_9(x) - f_9(g^88 * z)) / (x - g^88 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^88 * z)^(-1)*/ mload(add(denominatorsPtr, 0x280)),
                                  /*oods_coefficients[98]*/ mload(add(context, 0x5660)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[98]*/ mload(add(context, 0x4340)))),
                           PRIME))

                // res += c_99*(f_9(x) - f_9(g^96 * z)) / (x - g^96 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^96 * z)^(-1)*/ mload(add(denominatorsPtr, 0x2a0)),
                                  /*oods_coefficients[99]*/ mload(add(context, 0x5680)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[99]*/ mload(add(context, 0x4360)))),
                           PRIME))

                // res += c_100*(f_9(x) - f_9(g^100 * z)) / (x - g^100 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^100 * z)^(-1)*/ mload(add(denominatorsPtr, 0x2c0)),
                                  /*oods_coefficients[100]*/ mload(add(context, 0x56a0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[100]*/ mload(add(context, 0x4380)))),
                           PRIME))

                // res += c_101*(f_9(x) - f_9(g^112 * z)) / (x - g^112 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^112 * z)^(-1)*/ mload(add(denominatorsPtr, 0x2e0)),
                                  /*oods_coefficients[101]*/ mload(add(context, 0x56c0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[101]*/ mload(add(context, 0x43a0)))),
                           PRIME))

                // res += c_102*(f_9(x) - f_9(g^132 * z)) / (x - g^132 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^132 * z)^(-1)*/ mload(add(denominatorsPtr, 0x300)),
                                  /*oods_coefficients[102]*/ mload(add(context, 0x56e0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[102]*/ mload(add(context, 0x43c0)))),
                           PRIME))

                // res += c_103*(f_9(x) - f_9(g^148 * z)) / (x - g^148 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^148 * z)^(-1)*/ mload(add(denominatorsPtr, 0x320)),
                                  /*oods_coefficients[103]*/ mload(add(context, 0x5700)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[103]*/ mload(add(context, 0x43e0)))),
                           PRIME))

                // res += c_104*(f_9(x) - f_9(g^164 * z)) / (x - g^164 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^164 * z)^(-1)*/ mload(add(denominatorsPtr, 0x340)),
                                  /*oods_coefficients[104]*/ mload(add(context, 0x5720)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[104]*/ mload(add(context, 0x4400)))),
                           PRIME))

                // res += c_105*(f_9(x) - f_9(g^196 * z)) / (x - g^196 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^196 * z)^(-1)*/ mload(add(denominatorsPtr, 0x360)),
                                  /*oods_coefficients[105]*/ mload(add(context, 0x5740)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[105]*/ mload(add(context, 0x4420)))),
                           PRIME))

                // res += c_106*(f_9(x) - f_9(g^8196 * z)) / (x - g^8196 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^8196 * z)^(-1)*/ mload(add(denominatorsPtr, 0x6c0)),
                                  /*oods_coefficients[106]*/ mload(add(context, 0x5760)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[106]*/ mload(add(context, 0x4440)))),
                           PRIME))

                // res += c_107*(f_9(x) - f_9(g^16328 * z)) / (x - g^16328 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^16328 * z)^(-1)*/ mload(add(denominatorsPtr, 0x780)),
                                  /*oods_coefficients[107]*/ mload(add(context, 0x5780)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[107]*/ mload(add(context, 0x4460)))),
                           PRIME))

                // res += c_108*(f_9(x) - f_9(g^16336 * z)) / (x - g^16336 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^16336 * z)^(-1)*/ mload(add(denominatorsPtr, 0x7a0)),
                                  /*oods_coefficients[108]*/ mload(add(context, 0x57a0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[108]*/ mload(add(context, 0x4480)))),
                           PRIME))

                // res += c_109*(f_9(x) - f_9(g^16360 * z)) / (x - g^16360 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^16360 * z)^(-1)*/ mload(add(denominatorsPtr, 0x7c0)),
                                  /*oods_coefficients[109]*/ mload(add(context, 0x57c0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[109]*/ mload(add(context, 0x44a0)))),
                           PRIME))

                // res += c_110*(f_9(x) - f_9(g^16368 * z)) / (x - g^16368 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^16368 * z)^(-1)*/ mload(add(denominatorsPtr, 0x7e0)),
                                  /*oods_coefficients[110]*/ mload(add(context, 0x57e0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[110]*/ mload(add(context, 0x44c0)))),
                           PRIME))

                // res += c_111*(f_9(x) - f_9(g^16376 * z)) / (x - g^16376 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^16376 * z)^(-1)*/ mload(add(denominatorsPtr, 0x800)),
                                  /*oods_coefficients[111]*/ mload(add(context, 0x5800)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[111]*/ mload(add(context, 0x44e0)))),
                           PRIME))

                // res += c_112*(f_9(x) - f_9(g^16384 * z)) / (x - g^16384 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^16384 * z)^(-1)*/ mload(add(denominatorsPtr, 0x820)),
                                  /*oods_coefficients[112]*/ mload(add(context, 0x5820)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[112]*/ mload(add(context, 0x4500)))),
                           PRIME))

                // res += c_113*(f_9(x) - f_9(g^16416 * z)) / (x - g^16416 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^16416 * z)^(-1)*/ mload(add(denominatorsPtr, 0x840)),
                                  /*oods_coefficients[113]*/ mload(add(context, 0x5840)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[113]*/ mload(add(context, 0x4520)))),
                           PRIME))

                // res += c_114*(f_9(x) - f_9(g^32676 * z)) / (x - g^32676 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^32676 * z)^(-1)*/ mload(add(denominatorsPtr, 0x900)),
                                  /*oods_coefficients[114]*/ mload(add(context, 0x5860)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[114]*/ mload(add(context, 0x4540)))),
                           PRIME))

                // res += c_115*(f_9(x) - f_9(g^32708 * z)) / (x - g^32708 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^32708 * z)^(-1)*/ mload(add(denominatorsPtr, 0x920)),
                                  /*oods_coefficients[115]*/ mload(add(context, 0x5880)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[115]*/ mload(add(context, 0x4560)))),
                           PRIME))

                // res += c_116*(f_9(x) - f_9(g^32712 * z)) / (x - g^32712 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^32712 * z)^(-1)*/ mload(add(denominatorsPtr, 0x940)),
                                  /*oods_coefficients[116]*/ mload(add(context, 0x58a0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[116]*/ mload(add(context, 0x4580)))),
                           PRIME))

                // res += c_117*(f_9(x) - f_9(g^32724 * z)) / (x - g^32724 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^32724 * z)^(-1)*/ mload(add(denominatorsPtr, 0x960)),
                                  /*oods_coefficients[117]*/ mload(add(context, 0x58c0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[117]*/ mload(add(context, 0x45a0)))),
                           PRIME))

                // res += c_118*(f_9(x) - f_9(g^32740 * z)) / (x - g^32740 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^32740 * z)^(-1)*/ mload(add(denominatorsPtr, 0x980)),
                                  /*oods_coefficients[118]*/ mload(add(context, 0x58e0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[118]*/ mload(add(context, 0x45c0)))),
                           PRIME))

                // res += c_119*(f_9(x) - f_9(g^32744 * z)) / (x - g^32744 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^32744 * z)^(-1)*/ mload(add(denominatorsPtr, 0x9a0)),
                                  /*oods_coefficients[119]*/ mload(add(context, 0x5900)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[119]*/ mload(add(context, 0x45e0)))),
                           PRIME))

                // res += c_120*(f_9(x) - f_9(g^32752 * z)) / (x - g^32752 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^32752 * z)^(-1)*/ mload(add(denominatorsPtr, 0x9c0)),
                                  /*oods_coefficients[120]*/ mload(add(context, 0x5920)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[120]*/ mload(add(context, 0x4600)))),
                           PRIME))

                // res += c_121*(f_9(x) - f_9(g^32760 * z)) / (x - g^32760 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^32760 * z)^(-1)*/ mload(add(denominatorsPtr, 0x9e0)),
                                  /*oods_coefficients[121]*/ mload(add(context, 0x5940)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[121]*/ mload(add(context, 0x4620)))),
                           PRIME))

                // res += c_122*(f_9(x) - f_9(g^32768 * z)) / (x - g^32768 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^32768 * z)^(-1)*/ mload(add(denominatorsPtr, 0xa00)),
                                  /*oods_coefficients[122]*/ mload(add(context, 0x5960)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[122]*/ mload(add(context, 0x4640)))),
                           PRIME))

                // res += c_123*(f_9(x) - f_9(g^32772 * z)) / (x - g^32772 * z)
                res := addmod(
                    res,
                    mulmod(mulmod(/*(x - g^32772 * z)^(-1)*/ mload(add(denominatorsPtr, 0xa20)),
                                  /*oods_coefficients[123]*/ mload(add(context, 0x5980)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[123]*/ mload(add(context, 0x4660)))),
                           PRIME),
                    PRIME)

                // res += c_124*(f_9(x) - f_9(g^32788 * z)) / (x - g^32788 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^32788 * z)^(-1)*/ mload(add(denominatorsPtr, 0xa40)),
                                  /*oods_coefficients[124]*/ mload(add(context, 0x59a0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[124]*/ mload(add(context, 0x4680)))),
                           PRIME))

                // res += c_125*(f_9(x) - f_9(g^49128 * z)) / (x - g^49128 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^49128 * z)^(-1)*/ mload(add(denominatorsPtr, 0xb40)),
                                  /*oods_coefficients[125]*/ mload(add(context, 0x59c0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[125]*/ mload(add(context, 0x46a0)))),
                           PRIME))

                // res += c_126*(f_9(x) - f_9(g^49144 * z)) / (x - g^49144 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^49144 * z)^(-1)*/ mload(add(denominatorsPtr, 0xb60)),
                                  /*oods_coefficients[126]*/ mload(add(context, 0x59e0)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[126]*/ mload(add(context, 0x46c0)))),
                           PRIME))

                // res += c_127*(f_9(x) - f_9(g^65512 * z)) / (x - g^65512 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^65512 * z)^(-1)*/ mload(add(denominatorsPtr, 0xbe0)),
                                  /*oods_coefficients[127]*/ mload(add(context, 0x5a00)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[127]*/ mload(add(context, 0x46e0)))),
                           PRIME))

                // res += c_128*(f_9(x) - f_9(g^65528 * z)) / (x - g^65528 * z)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - g^65528 * z)^(-1)*/ mload(add(denominatorsPtr, 0xc00)),
                                  /*oods_coefficients[128]*/ mload(add(context, 0x5a20)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*oods_values[128]*/ mload(add(context, 0x4700)))),
                           PRIME))
                }

                // Advance traceQueryResponses by amount read (0x20 * nTraceColumns).
                traceQueryResponses := add(traceQueryResponses, 0x140)

                // Composition constraints.

                {
                // Read the next element.
                let columnValue := mulmod(mload(compositionQueryResponses), kMontgomeryRInv, PRIME)
                // res += c_129*(h_0(x) - C_0(z^2)) / (x - z^2)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - z^2)^(-1)*/ mload(add(denominatorsPtr, 0xc80)),
                                  /*oods_coefficients[129]*/ mload(add(context, 0x5a40)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*composition_oods_values[0]*/ mload(add(context, 0x4720)))),
                           PRIME))
                }

                {
                // Read the next element.
                let columnValue := mulmod(mload(add(compositionQueryResponses, 0x20)), kMontgomeryRInv, PRIME)
                // res += c_130*(h_1(x) - C_1(z^2)) / (x - z^2)
                res := add(
                    res,
                    mulmod(mulmod(/*(x - z^2)^(-1)*/ mload(add(denominatorsPtr, 0xc80)),
                                  /*oods_coefficients[130]*/ mload(add(context, 0x5a60)),
                                  PRIME),
                           add(columnValue, sub(PRIME, /*composition_oods_values[1]*/ mload(add(context, 0x4740)))),
                           PRIME))
                }

                // Advance compositionQueryResponses by amount read (0x20 * constraintDegree).
                compositionQueryResponses := add(compositionQueryResponses, 0x40)

                // Append the sum of the Out of Domain Sampling boundary constraints for the
                // trace and composition polynomials to the friValues array.
                mstore(friValues, mod(res, PRIME))

                // Append the friInvPoint of the current query to the friInvPoints array.
                mstore(friInvPoints, /*friInvPoint*/ mload(add(denominatorsPtr,0xca0)))
                friInvPoints := add(friInvPoints, 0x20)

                // Advance denominatorsPtr by chunk size (0x20 * (2+N_ROWS_IN_MASK)).
                denominatorsPtr := add(denominatorsPtr, 0xcc0)
            }
            return(/*friValues*/ add(context, 0x720), 0x580)
        }
    }

    /*
      Computes and performs batch inverse on all the denominators required for the Out of Domain
      Sampling boundary constraints.

      Since the friEvalPoints are calculated during the computation of the denominators
      this function also adds those to the batch inverse in prepartion for the FRI that follows.

      After this function returns, the batch_inverse_out array holds #queries
      chunks of size (2 + N_ROWS_IN_MASK) with the following structure:
      0..(N_ROWS_IN_MASK-1):   [(x - g^i * z)^(-1) for i in rowsInMask]
      N_ROWS_IN_MASK:          (x - z^constraintDegree)^-1
      N_ROWS_IN_MASK+1:        friEvalPointInv.
    */
    function oodsPrepareInverses(
        uint256[] memory context, uint256[] memory batchInverseArray)
        internal view {
        uint256 evalCosetOffset_ = PrimeFieldElement0.GENERATOR_VAL;
        // The array expmodsAndPoints stores subexpressions that are needed
        // for the denominators computation.
        // The array is segmented as follows:
        //    expmodsAndPoints[0:30] (.expmods) expmods used during calculations of the points below.
        //    expmodsAndPoints[30:130] (.points) points used during the denominators calculation.
        uint256[130] memory expmodsAndPoints;
        assembly {
            function expmod(base, exponent, modulus) -> res {
              let p := mload(0x40)
              mstore(p, 0x20)                 // Length of Base
              mstore(add(p, 0x20), 0x20)      // Length of Exponent
              mstore(add(p, 0x40), 0x20)      // Length of Modulus
              mstore(add(p, 0x60), base)      // Base
              mstore(add(p, 0x80), exponent)  // Exponent
              mstore(add(p, 0xa0), modulus)   // Modulus
              // call modexp precompile
              if iszero(staticcall(not(0), 0x05, p, 0xc0, p, 0x20)) {
                revert(0, 0)
              }
              res := mload(p)
            }

            let traceGenerator := /*trace_generator*/ mload(add(context, 0x1840))
            let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001

            // Prepare expmods for computations of trace generator powers.

            // expmodsAndPoints.expmods[0] = traceGenerator^2
            mstore(expmodsAndPoints,
                   mulmod(traceGenerator, // traceGenerator^1
                          traceGenerator, // traceGenerator^1
                          PRIME))

            // expmodsAndPoints.expmods[1] = traceGenerator^4
            mstore(add(expmodsAndPoints, 0x20),
                   mulmod(mload(expmodsAndPoints), // traceGenerator^2
                          mload(expmodsAndPoints), // traceGenerator^2
                          PRIME))

            // expmodsAndPoints.expmods[2] = traceGenerator^6
            mstore(add(expmodsAndPoints, 0x40),
                   mulmod(mload(add(expmodsAndPoints, 0x20)), // traceGenerator^4
                          mload(expmodsAndPoints), // traceGenerator^2
                          PRIME))

            // expmodsAndPoints.expmods[3] = traceGenerator^7
            mstore(add(expmodsAndPoints, 0x60),
                   mulmod(mload(add(expmodsAndPoints, 0x40)), // traceGenerator^6
                          traceGenerator, // traceGenerator^1
                          PRIME))

            // expmodsAndPoints.expmods[4] = traceGenerator^8
            mstore(add(expmodsAndPoints, 0x80),
                   mulmod(mload(add(expmodsAndPoints, 0x60)), // traceGenerator^7
                          traceGenerator, // traceGenerator^1
                          PRIME))

            // expmodsAndPoints.expmods[5] = traceGenerator^12
            mstore(add(expmodsAndPoints, 0xa0),
                   mulmod(mload(add(expmodsAndPoints, 0x80)), // traceGenerator^8
                          mload(add(expmodsAndPoints, 0x20)), // traceGenerator^4
                          PRIME))

            // expmodsAndPoints.expmods[6] = traceGenerator^16
            mstore(add(expmodsAndPoints, 0xc0),
                   mulmod(mload(add(expmodsAndPoints, 0xa0)), // traceGenerator^12
                          mload(add(expmodsAndPoints, 0x20)), // traceGenerator^4
                          PRIME))

            // expmodsAndPoints.expmods[7] = traceGenerator^20
            mstore(add(expmodsAndPoints, 0xe0),
                   mulmod(mload(add(expmodsAndPoints, 0xc0)), // traceGenerator^16
                          mload(add(expmodsAndPoints, 0x20)), // traceGenerator^4
                          PRIME))

            // expmodsAndPoints.expmods[8] = traceGenerator^24
            mstore(add(expmodsAndPoints, 0x100),
                   mulmod(mload(add(expmodsAndPoints, 0xe0)), // traceGenerator^20
                          mload(add(expmodsAndPoints, 0x20)), // traceGenerator^4
                          PRIME))

            // expmodsAndPoints.expmods[9] = traceGenerator^32
            mstore(add(expmodsAndPoints, 0x120),
                   mulmod(mload(add(expmodsAndPoints, 0x100)), // traceGenerator^24
                          mload(add(expmodsAndPoints, 0x80)), // traceGenerator^8
                          PRIME))

            // expmodsAndPoints.expmods[10] = traceGenerator^59
            mstore(add(expmodsAndPoints, 0x140),
                   mulmod(mload(add(expmodsAndPoints, 0x120)), // traceGenerator^32
                          mulmod(mload(add(expmodsAndPoints, 0xe0)), // traceGenerator^20
                                 mload(add(expmodsAndPoints, 0x60)), // traceGenerator^7
                                 PRIME),
                          PRIME))

            // expmodsAndPoints.expmods[11] = traceGenerator^223
            mstore(add(expmodsAndPoints, 0x160),
                   mulmod(mload(add(expmodsAndPoints, 0x140)), // traceGenerator^59
                          mulmod(mload(add(expmodsAndPoints, 0x140)), // traceGenerator^59
                                 mulmod(mload(add(expmodsAndPoints, 0x140)), // traceGenerator^59
                                        mulmod(mload(add(expmodsAndPoints, 0x120)), // traceGenerator^32
                                               mulmod(mload(add(expmodsAndPoints, 0xa0)), // traceGenerator^12
                                                      mload(expmodsAndPoints), // traceGenerator^2
                                                      PRIME),
                                               PRIME),
                                        PRIME),
                                 PRIME),
                          PRIME))

            // expmodsAndPoints.expmods[12] = traceGenerator^235
            mstore(add(expmodsAndPoints, 0x180),
                   mulmod(mload(add(expmodsAndPoints, 0x160)), // traceGenerator^223
                          mload(add(expmodsAndPoints, 0xa0)), // traceGenerator^12
                          PRIME))

            // expmodsAndPoints.expmods[13] = traceGenerator^252
            mstore(add(expmodsAndPoints, 0x1a0),
                   mulmod(mload(add(expmodsAndPoints, 0x180)), // traceGenerator^235
                          mulmod(mload(add(expmodsAndPoints, 0xc0)), // traceGenerator^16
                                 traceGenerator, // traceGenerator^1
                                 PRIME),
                          PRIME))

            // expmodsAndPoints.expmods[14] = traceGenerator^255
            mstore(add(expmodsAndPoints, 0x1c0),
                   mulmod(mload(add(expmodsAndPoints, 0x180)), // traceGenerator^235
                          mload(add(expmodsAndPoints, 0xe0)), // traceGenerator^20
                          PRIME))

            // expmodsAndPoints.expmods[15] = traceGenerator^263
            mstore(add(expmodsAndPoints, 0x1e0),
                   mulmod(mload(add(expmodsAndPoints, 0x1c0)), // traceGenerator^255
                          mload(add(expmodsAndPoints, 0x80)), // traceGenerator^8
                          PRIME))

            // expmodsAndPoints.expmods[16] = traceGenerator^765
            mstore(add(expmodsAndPoints, 0x200),
                   mulmod(mload(add(expmodsAndPoints, 0x1c0)), // traceGenerator^255
                          mulmod(mload(add(expmodsAndPoints, 0x1c0)), // traceGenerator^255
                                 mload(add(expmodsAndPoints, 0x1c0)), // traceGenerator^255
                                 PRIME),
                          PRIME))

            // expmodsAndPoints.expmods[17] = traceGenerator^1017
            mstore(add(expmodsAndPoints, 0x220),
                   mulmod(mload(add(expmodsAndPoints, 0x200)), // traceGenerator^765
                          mload(add(expmodsAndPoints, 0x1a0)), // traceGenerator^252
                          PRIME))

            // expmodsAndPoints.expmods[18] = traceGenerator^1018
            mstore(add(expmodsAndPoints, 0x240),
                   mulmod(mload(add(expmodsAndPoints, 0x220)), // traceGenerator^1017
                          traceGenerator, // traceGenerator^1
                          PRIME))

            // expmodsAndPoints.expmods[19] = traceGenerator^1023
            mstore(add(expmodsAndPoints, 0x260),
                   mulmod(mload(add(expmodsAndPoints, 0x220)), // traceGenerator^1017
                          mload(add(expmodsAndPoints, 0x40)), // traceGenerator^6
                          PRIME))

            // expmodsAndPoints.expmods[20] = traceGenerator^1024
            mstore(add(expmodsAndPoints, 0x280),
                   mulmod(mload(add(expmodsAndPoints, 0x260)), // traceGenerator^1023
                          traceGenerator, // traceGenerator^1
                          PRIME))

            // expmodsAndPoints.expmods[21] = traceGenerator^1030
            mstore(add(expmodsAndPoints, 0x2a0),
                   mulmod(mload(add(expmodsAndPoints, 0x280)), // traceGenerator^1024
                          mload(add(expmodsAndPoints, 0x40)), // traceGenerator^6
                          PRIME))

            // expmodsAndPoints.expmods[22] = traceGenerator^2048
            mstore(add(expmodsAndPoints, 0x2c0),
                   mulmod(mload(add(expmodsAndPoints, 0x2a0)), // traceGenerator^1030
                          mload(add(expmodsAndPoints, 0x240)), // traceGenerator^1018
                          PRIME))

            // expmodsAndPoints.expmods[23] = traceGenerator^2820
            mstore(add(expmodsAndPoints, 0x2e0),
                   mulmod(mload(add(expmodsAndPoints, 0x2c0)), // traceGenerator^2048
                          mulmod(mload(add(expmodsAndPoints, 0x200)), // traceGenerator^765
                                 mload(add(expmodsAndPoints, 0x60)), // traceGenerator^7
                                 PRIME),
                          PRIME))

            // expmodsAndPoints.expmods[24] = traceGenerator^3079
            mstore(add(expmodsAndPoints, 0x300),
                   mulmod(mload(add(expmodsAndPoints, 0x2c0)), // traceGenerator^2048
                          mulmod(mload(add(expmodsAndPoints, 0x2a0)), // traceGenerator^1030
                                 traceGenerator, // traceGenerator^1
                                 PRIME),
                          PRIME))

            // expmodsAndPoints.expmods[25] = traceGenerator^4007
            mstore(add(expmodsAndPoints, 0x320),
                   mulmod(mload(add(expmodsAndPoints, 0x300)), // traceGenerator^3079
                          mulmod(mload(add(expmodsAndPoints, 0x180)), // traceGenerator^235
                                 mulmod(mload(add(expmodsAndPoints, 0x180)), // traceGenerator^235
                                        mulmod(mload(add(expmodsAndPoints, 0x180)), // traceGenerator^235
                                               mload(add(expmodsAndPoints, 0x160)), // traceGenerator^223
                                               PRIME),
                                        PRIME),
                                 PRIME),
                          PRIME))

            // expmodsAndPoints.expmods[26] = traceGenerator^4043
            mstore(add(expmodsAndPoints, 0x340),
                   mulmod(mload(add(expmodsAndPoints, 0x320)), // traceGenerator^4007
                          mulmod(mload(add(expmodsAndPoints, 0x120)), // traceGenerator^32
                                 mload(add(expmodsAndPoints, 0x20)), // traceGenerator^4
                                 PRIME),
                          PRIME))

            // expmodsAndPoints.expmods[27] = traceGenerator^5093
            mstore(add(expmodsAndPoints, 0x360),
                   mulmod(mload(add(expmodsAndPoints, 0x340)), // traceGenerator^4043
                          mulmod(mload(add(expmodsAndPoints, 0x2a0)), // traceGenerator^1030
                                 mload(add(expmodsAndPoints, 0xe0)), // traceGenerator^20
                                 PRIME),
                          PRIME))

            // expmodsAndPoints.expmods[28] = traceGenerator^7174
            mstore(add(expmodsAndPoints, 0x380),
                   mulmod(mload(add(expmodsAndPoints, 0x360)), // traceGenerator^5093
                          mulmod(mload(add(expmodsAndPoints, 0x2c0)), // traceGenerator^2048
                                 mulmod(mload(add(expmodsAndPoints, 0x120)), // traceGenerator^32
                                        traceGenerator, // traceGenerator^1
                                        PRIME),
                                 PRIME),
                          PRIME))

            // expmodsAndPoints.expmods[29] = traceGenerator^8192
            mstore(add(expmodsAndPoints, 0x3a0),
                   mulmod(mload(add(expmodsAndPoints, 0x380)), // traceGenerator^7174
                          mload(add(expmodsAndPoints, 0x240)), // traceGenerator^1018
                          PRIME))

            let oodsPoint := /*oods_point*/ mload(add(context, 0x1860))
            {
              // point = -z
              let point := sub(PRIME, oodsPoint)
              // Compute denominators for rows with nonconst mask expression.
              // We compute those first because for the const rows we modify the point variable.

              // expmods_and_points.points[97] = -(g^(512 * (vaults_path_length - 1) + 511) * z)
              mstore(add(expmodsAndPoints, 0xfe0), mulmod(
                point,
                expmod(traceGenerator, add(mul(512, sub(/*vaults_path_length*/ mload(add(context, 0x1700)), 1)), 511), PRIME),
                PRIME))

              // expmods_and_points.points[98] = -(g^(16384 + 512 * (vaults_path_length - 1) + 511) * z)
              mstore(add(expmodsAndPoints, 0x1000), mulmod(
                point,
                expmod(traceGenerator, add(
                  16384,
                  add(mul(512, sub(/*vaults_path_length*/ mload(add(context, 0x1700)), 1)), 511)), PRIME),
                PRIME))

              // expmods_and_points.points[99] = -(g^(49152 + 512 * (vaults_path_length - 1) + 511) * z)
              mstore(add(expmodsAndPoints, 0x1020), mulmod(
                point,
                expmod(traceGenerator, add(
                  49152,
                  add(mul(512, sub(/*vaults_path_length*/ mload(add(context, 0x1700)), 1)), 511)), PRIME),
                PRIME))

              // Compute denominators for rows with const mask expression.

              // expmods_and_points.points[0] = -z
              mstore(add(expmodsAndPoints, 0x3c0), point)

              // point *= g
              point := mulmod(point, traceGenerator, PRIME)
              // expmods_and_points.points[1] = -(g * z)
              mstore(add(expmodsAndPoints, 0x3e0), point)

              // point *= g
              point := mulmod(point, traceGenerator, PRIME)
              // expmods_and_points.points[2] = -(g^2 * z)
              mstore(add(expmodsAndPoints, 0x400), point)

              // point *= g
              point := mulmod(point, traceGenerator, PRIME)
              // expmods_and_points.points[3] = -(g^3 * z)
              mstore(add(expmodsAndPoints, 0x420), point)

              // point *= g
              point := mulmod(point, traceGenerator, PRIME)
              // expmods_and_points.points[4] = -(g^4 * z)
              mstore(add(expmodsAndPoints, 0x440), point)

              // point *= g^2
              point := mulmod(point, /*traceGenerator^2*/ mload(expmodsAndPoints), PRIME)
              // expmods_and_points.points[5] = -(g^6 * z)
              mstore(add(expmodsAndPoints, 0x460), point)

              // point *= g
              point := mulmod(point, traceGenerator, PRIME)
              // expmods_and_points.points[6] = -(g^7 * z)
              mstore(add(expmodsAndPoints, 0x480), point)

              // point *= g
              point := mulmod(point, traceGenerator, PRIME)
              // expmods_and_points.points[7] = -(g^8 * z)
              mstore(add(expmodsAndPoints, 0x4a0), point)

              // point *= g^8
              point := mulmod(point, /*traceGenerator^8*/ mload(add(expmodsAndPoints, 0x80)), PRIME)
              // expmods_and_points.points[8] = -(g^16 * z)
              mstore(add(expmodsAndPoints, 0x4c0), point)

              // point *= g^4
              point := mulmod(point, /*traceGenerator^4*/ mload(add(expmodsAndPoints, 0x20)), PRIME)
              // expmods_and_points.points[9] = -(g^20 * z)
              mstore(add(expmodsAndPoints, 0x4e0), point)

              // point *= g^4
              point := mulmod(point, /*traceGenerator^4*/ mload(add(expmodsAndPoints, 0x20)), PRIME)
              // expmods_and_points.points[10] = -(g^24 * z)
              mstore(add(expmodsAndPoints, 0x500), point)

              // point *= g^8
              point := mulmod(point, /*traceGenerator^8*/ mload(add(expmodsAndPoints, 0x80)), PRIME)
              // expmods_and_points.points[11] = -(g^32 * z)
              mstore(add(expmodsAndPoints, 0x520), point)

              // point *= g^4
              point := mulmod(point, /*traceGenerator^4*/ mload(add(expmodsAndPoints, 0x20)), PRIME)
              // expmods_and_points.points[12] = -(g^36 * z)
              mstore(add(expmodsAndPoints, 0x540), point)

              // point *= g^4
              point := mulmod(point, /*traceGenerator^4*/ mload(add(expmodsAndPoints, 0x20)), PRIME)
              // expmods_and_points.points[13] = -(g^40 * z)
              mstore(add(expmodsAndPoints, 0x560), point)

              // point *= g^8
              point := mulmod(point, /*traceGenerator^8*/ mload(add(expmodsAndPoints, 0x80)), PRIME)
              // expmods_and_points.points[14] = -(g^48 * z)
              mstore(add(expmodsAndPoints, 0x580), point)

              // point *= g^8
              point := mulmod(point, /*traceGenerator^8*/ mload(add(expmodsAndPoints, 0x80)), PRIME)
              // expmods_and_points.points[15] = -(g^56 * z)
              mstore(add(expmodsAndPoints, 0x5a0), point)

              // point *= g^8
              point := mulmod(point, /*traceGenerator^8*/ mload(add(expmodsAndPoints, 0x80)), PRIME)
              // expmods_and_points.points[16] = -(g^64 * z)
              mstore(add(expmodsAndPoints, 0x5c0), point)

              // point *= g^4
              point := mulmod(point, /*traceGenerator^4*/ mload(add(expmodsAndPoints, 0x20)), PRIME)
              // expmods_and_points.points[17] = -(g^68 * z)
              mstore(add(expmodsAndPoints, 0x5e0), point)

              // point *= g^4
              point := mulmod(point, /*traceGenerator^4*/ mload(add(expmodsAndPoints, 0x20)), PRIME)
              // expmods_and_points.points[18] = -(g^72 * z)
              mstore(add(expmodsAndPoints, 0x600), point)

              // point *= g^12
              point := mulmod(point, /*traceGenerator^12*/ mload(add(expmodsAndPoints, 0xa0)), PRIME)
              // expmods_and_points.points[19] = -(g^84 * z)
              mstore(add(expmodsAndPoints, 0x620), point)

              // point *= g^4
              point := mulmod(point, /*traceGenerator^4*/ mload(add(expmodsAndPoints, 0x20)), PRIME)
              // expmods_and_points.points[20] = -(g^88 * z)
              mstore(add(expmodsAndPoints, 0x640), point)

              // point *= g^8
              point := mulmod(point, /*traceGenerator^8*/ mload(add(expmodsAndPoints, 0x80)), PRIME)
              // expmods_and_points.points[21] = -(g^96 * z)
              mstore(add(expmodsAndPoints, 0x660), point)

              // point *= g^4
              point := mulmod(point, /*traceGenerator^4*/ mload(add(expmodsAndPoints, 0x20)), PRIME)
              // expmods_and_points.points[22] = -(g^100 * z)
              mstore(add(expmodsAndPoints, 0x680), point)

              // point *= g^12
              point := mulmod(point, /*traceGenerator^12*/ mload(add(expmodsAndPoints, 0xa0)), PRIME)
              // expmods_and_points.points[23] = -(g^112 * z)
              mstore(add(expmodsAndPoints, 0x6a0), point)

              // point *= g^20
              point := mulmod(point, /*traceGenerator^20*/ mload(add(expmodsAndPoints, 0xe0)), PRIME)
              // expmods_and_points.points[24] = -(g^132 * z)
              mstore(add(expmodsAndPoints, 0x6c0), point)

              // point *= g^16
              point := mulmod(point, /*traceGenerator^16*/ mload(add(expmodsAndPoints, 0xc0)), PRIME)
              // expmods_and_points.points[25] = -(g^148 * z)
              mstore(add(expmodsAndPoints, 0x6e0), point)

              // point *= g^16
              point := mulmod(point, /*traceGenerator^16*/ mload(add(expmodsAndPoints, 0xc0)), PRIME)
              // expmods_and_points.points[26] = -(g^164 * z)
              mstore(add(expmodsAndPoints, 0x700), point)

              // point *= g^32
              point := mulmod(point, /*traceGenerator^32*/ mload(add(expmodsAndPoints, 0x120)), PRIME)
              // expmods_and_points.points[27] = -(g^196 * z)
              mstore(add(expmodsAndPoints, 0x720), point)

              // point *= g^59
              point := mulmod(point, /*traceGenerator^59*/ mload(add(expmodsAndPoints, 0x140)), PRIME)
              // expmods_and_points.points[28] = -(g^255 * z)
              mstore(add(expmodsAndPoints, 0x740), point)

              // point *= g
              point := mulmod(point, traceGenerator, PRIME)
              // expmods_and_points.points[29] = -(g^256 * z)
              mstore(add(expmodsAndPoints, 0x760), point)

              // point *= g^255
              point := mulmod(point, /*traceGenerator^255*/ mload(add(expmodsAndPoints, 0x1c0)), PRIME)
              // expmods_and_points.points[30] = -(g^511 * z)
              mstore(add(expmodsAndPoints, 0x780), point)

              // point *= g
              point := mulmod(point, traceGenerator, PRIME)
              // expmods_and_points.points[31] = -(g^512 * z)
              mstore(add(expmodsAndPoints, 0x7a0), point)

              // point *= g^255
              point := mulmod(point, /*traceGenerator^255*/ mload(add(expmodsAndPoints, 0x1c0)), PRIME)
              // expmods_and_points.points[32] = -(g^767 * z)
              mstore(add(expmodsAndPoints, 0x7c0), point)

              // point *= g
              point := mulmod(point, traceGenerator, PRIME)
              // expmods_and_points.points[33] = -(g^768 * z)
              mstore(add(expmodsAndPoints, 0x7e0), point)

              // point *= g^252
              point := mulmod(point, /*traceGenerator^252*/ mload(add(expmodsAndPoints, 0x1a0)), PRIME)
              // expmods_and_points.points[34] = -(g^1020 * z)
              mstore(add(expmodsAndPoints, 0x800), point)

              // point *= g
              point := mulmod(point, traceGenerator, PRIME)
              // expmods_and_points.points[35] = -(g^1021 * z)
              mstore(add(expmodsAndPoints, 0x820), point)

              // point *= g
              point := mulmod(point, traceGenerator, PRIME)
              // expmods_and_points.points[36] = -(g^1022 * z)
              mstore(add(expmodsAndPoints, 0x840), point)

              // point *= g^2
              point := mulmod(point, /*traceGenerator^2*/ mload(expmodsAndPoints), PRIME)
              // expmods_and_points.points[37] = -(g^1024 * z)
              mstore(add(expmodsAndPoints, 0x860), point)

              // point *= g^2
              point := mulmod(point, /*traceGenerator^2*/ mload(expmodsAndPoints), PRIME)
              // expmods_and_points.points[38] = -(g^1026 * z)
              mstore(add(expmodsAndPoints, 0x880), point)

              // point *= g
              point := mulmod(point, traceGenerator, PRIME)
              // expmods_and_points.points[39] = -(g^1027 * z)
              mstore(add(expmodsAndPoints, 0x8a0), point)

              // point *= g^252
              point := mulmod(point, /*traceGenerator^252*/ mload(add(expmodsAndPoints, 0x1a0)), PRIME)
              // expmods_and_points.points[40] = -(g^1279 * z)
              mstore(add(expmodsAndPoints, 0x8c0), point)

              // point *= g^765
              point := mulmod(point, /*traceGenerator^765*/ mload(add(expmodsAndPoints, 0x200)), PRIME)
              // expmods_and_points.points[41] = -(g^2044 * z)
              mstore(add(expmodsAndPoints, 0x8e0), point)

              // point *= g
              point := mulmod(point, traceGenerator, PRIME)
              // expmods_and_points.points[42] = -(g^2045 * z)
              mstore(add(expmodsAndPoints, 0x900), point)

              // point *= g^6
              point := mulmod(point, /*traceGenerator^6*/ mload(add(expmodsAndPoints, 0x40)), PRIME)
              // expmods_and_points.points[43] = -(g^2051 * z)
              mstore(add(expmodsAndPoints, 0x920), point)

              // point *= g^1018
              point := mulmod(point, /*traceGenerator^1018*/ mload(add(expmodsAndPoints, 0x240)), PRIME)
              // expmods_and_points.points[44] = -(g^3069 * z)
              mstore(add(expmodsAndPoints, 0x940), point)

              // point *= g^6
              point := mulmod(point, /*traceGenerator^6*/ mload(add(expmodsAndPoints, 0x40)), PRIME)
              // expmods_and_points.points[45] = -(g^3075 * z)
              mstore(add(expmodsAndPoints, 0x960), point)

              // point *= g^1017
              point := mulmod(point, /*traceGenerator^1017*/ mload(add(expmodsAndPoints, 0x220)), PRIME)
              // expmods_and_points.points[46] = -(g^4092 * z)
              mstore(add(expmodsAndPoints, 0x980), point)

              // point *= g
              point := mulmod(point, traceGenerator, PRIME)
              // expmods_and_points.points[47] = -(g^4093 * z)
              mstore(add(expmodsAndPoints, 0x9a0), point)

              // point *= g^6
              point := mulmod(point, /*traceGenerator^6*/ mload(add(expmodsAndPoints, 0x40)), PRIME)
              // expmods_and_points.points[48] = -(g^4099 * z)
              mstore(add(expmodsAndPoints, 0x9c0), point)

              // point *= g^1024
              point := mulmod(point, /*traceGenerator^1024*/ mload(add(expmodsAndPoints, 0x280)), PRIME)
              // expmods_and_points.points[49] = -(g^5123 * z)
              mstore(add(expmodsAndPoints, 0x9e0), point)

              // point *= g^1018
              point := mulmod(point, /*traceGenerator^1018*/ mload(add(expmodsAndPoints, 0x240)), PRIME)
              // expmods_and_points.points[50] = -(g^6141 * z)
              mstore(add(expmodsAndPoints, 0xa00), point)

              // point *= g^1030
              point := mulmod(point, /*traceGenerator^1030*/ mload(add(expmodsAndPoints, 0x2a0)), PRIME)
              // expmods_and_points.points[51] = -(g^7171 * z)
              mstore(add(expmodsAndPoints, 0xa20), point)

              // point *= g^1017
              point := mulmod(point, /*traceGenerator^1017*/ mload(add(expmodsAndPoints, 0x220)), PRIME)
              // expmods_and_points.points[52] = -(g^8188 * z)
              mstore(add(expmodsAndPoints, 0xa40), point)

              // point *= g^7
              point := mulmod(point, /*traceGenerator^7*/ mload(add(expmodsAndPoints, 0x60)), PRIME)
              // expmods_and_points.points[53] = -(g^8195 * z)
              mstore(add(expmodsAndPoints, 0xa60), point)

              // point *= g
              point := mulmod(point, traceGenerator, PRIME)
              // expmods_and_points.points[54] = -(g^8196 * z)
              mstore(add(expmodsAndPoints, 0xa80), point)

              // point *= g^1023
              point := mulmod(point, /*traceGenerator^1023*/ mload(add(expmodsAndPoints, 0x260)), PRIME)
              // expmods_and_points.points[55] = -(g^9219 * z)
              mstore(add(expmodsAndPoints, 0xaa0), point)

              // point *= g^1018
              point := mulmod(point, /*traceGenerator^1018*/ mload(add(expmodsAndPoints, 0x240)), PRIME)
              // expmods_and_points.points[56] = -(g^10237 * z)
              mstore(add(expmodsAndPoints, 0xac0), point)

              // point *= g^1030
              point := mulmod(point, /*traceGenerator^1030*/ mload(add(expmodsAndPoints, 0x2a0)), PRIME)
              // expmods_and_points.points[57] = -(g^11267 * z)
              mstore(add(expmodsAndPoints, 0xae0), point)

              // point *= g^1017
              point := mulmod(point, /*traceGenerator^1017*/ mload(add(expmodsAndPoints, 0x220)), PRIME)
              // expmods_and_points.points[58] = -(g^12284 * z)
              mstore(add(expmodsAndPoints, 0xb00), point)

              // point *= g
              point := mulmod(point, traceGenerator, PRIME)
              // expmods_and_points.points[59] = -(g^12285 * z)
              mstore(add(expmodsAndPoints, 0xb20), point)

              // point *= g^4043
              point := mulmod(point, /*traceGenerator^4043*/ mload(add(expmodsAndPoints, 0x340)), PRIME)
              // expmods_and_points.points[60] = -(g^16328 * z)
              mstore(add(expmodsAndPoints, 0xb40), point)

              // point *= g^8
              point := mulmod(point, /*traceGenerator^8*/ mload(add(expmodsAndPoints, 0x80)), PRIME)
              // expmods_and_points.points[61] = -(g^16336 * z)
              mstore(add(expmodsAndPoints, 0xb60), point)

              // point *= g^24
              point := mulmod(point, /*traceGenerator^24*/ mload(add(expmodsAndPoints, 0x100)), PRIME)
              // expmods_and_points.points[62] = -(g^16360 * z)
              mstore(add(expmodsAndPoints, 0xb80), point)

              // point *= g^8
              point := mulmod(point, /*traceGenerator^8*/ mload(add(expmodsAndPoints, 0x80)), PRIME)
              // expmods_and_points.points[63] = -(g^16368 * z)
              mstore(add(expmodsAndPoints, 0xba0), point)

              // point *= g^8
              point := mulmod(point, /*traceGenerator^8*/ mload(add(expmodsAndPoints, 0x80)), PRIME)
              // expmods_and_points.points[64] = -(g^16376 * z)
              mstore(add(expmodsAndPoints, 0xbc0), point)

              // point *= g^8
              point := mulmod(point, /*traceGenerator^8*/ mload(add(expmodsAndPoints, 0x80)), PRIME)
              // expmods_and_points.points[65] = -(g^16384 * z)
              mstore(add(expmodsAndPoints, 0xbe0), point)

              // point *= g^32
              point := mulmod(point, /*traceGenerator^32*/ mload(add(expmodsAndPoints, 0x120)), PRIME)
              // expmods_and_points.points[66] = -(g^16416 * z)
              mstore(add(expmodsAndPoints, 0xc00), point)

              // point *= g^223
              point := mulmod(point, /*traceGenerator^223*/ mload(add(expmodsAndPoints, 0x160)), PRIME)
              // expmods_and_points.points[67] = -(g^16639 * z)
              mstore(add(expmodsAndPoints, 0xc20), point)

              // point *= g^2820
              point := mulmod(point, /*traceGenerator^2820*/ mload(add(expmodsAndPoints, 0x2e0)), PRIME)
              // expmods_and_points.points[68] = -(g^19459 * z)
              mstore(add(expmodsAndPoints, 0xc40), point)

              // point *= g^1018
              point := mulmod(point, /*traceGenerator^1018*/ mload(add(expmodsAndPoints, 0x240)), PRIME)
              // expmods_and_points.points[69] = -(g^20477 * z)
              mstore(add(expmodsAndPoints, 0xc60), point)

              // point *= g^7174
              point := mulmod(point, /*traceGenerator^7174*/ mload(add(expmodsAndPoints, 0x380)), PRIME)
              // expmods_and_points.points[70] = -(g^27651 * z)
              mstore(add(expmodsAndPoints, 0xc80), point)

              // point *= g^1018
              point := mulmod(point, /*traceGenerator^1018*/ mload(add(expmodsAndPoints, 0x240)), PRIME)
              // expmods_and_points.points[71] = -(g^28669 * z)
              mstore(add(expmodsAndPoints, 0xca0), point)

              // point *= g^4007
              point := mulmod(point, /*traceGenerator^4007*/ mload(add(expmodsAndPoints, 0x320)), PRIME)
              // expmods_and_points.points[72] = -(g^32676 * z)
              mstore(add(expmodsAndPoints, 0xcc0), point)

              // point *= g^32
              point := mulmod(point, /*traceGenerator^32*/ mload(add(expmodsAndPoints, 0x120)), PRIME)
              // expmods_and_points.points[73] = -(g^32708 * z)
              mstore(add(expmodsAndPoints, 0xce0), point)

              // point *= g^4
              point := mulmod(point, /*traceGenerator^4*/ mload(add(expmodsAndPoints, 0x20)), PRIME)
              // expmods_and_points.points[74] = -(g^32712 * z)
              mstore(add(expmodsAndPoints, 0xd00), point)

              // point *= g^12
              point := mulmod(point, /*traceGenerator^12*/ mload(add(expmodsAndPoints, 0xa0)), PRIME)
              // expmods_and_points.points[75] = -(g^32724 * z)
              mstore(add(expmodsAndPoints, 0xd20), point)

              // point *= g^16
              point := mulmod(point, /*traceGenerator^16*/ mload(add(expmodsAndPoints, 0xc0)), PRIME)
              // expmods_and_points.points[76] = -(g^32740 * z)
              mstore(add(expmodsAndPoints, 0xd40), point)

              // point *= g^4
              point := mulmod(point, /*traceGenerator^4*/ mload(add(expmodsAndPoints, 0x20)), PRIME)
              // expmods_and_points.points[77] = -(g^32744 * z)
              mstore(add(expmodsAndPoints, 0xd60), point)

              // point *= g^8
              point := mulmod(point, /*traceGenerator^8*/ mload(add(expmodsAndPoints, 0x80)), PRIME)
              // expmods_and_points.points[78] = -(g^32752 * z)
              mstore(add(expmodsAndPoints, 0xd80), point)

              // point *= g^8
              point := mulmod(point, /*traceGenerator^8*/ mload(add(expmodsAndPoints, 0x80)), PRIME)
              // expmods_and_points.points[79] = -(g^32760 * z)
              mstore(add(expmodsAndPoints, 0xda0), point)

              // point *= g^8
              point := mulmod(point, /*traceGenerator^8*/ mload(add(expmodsAndPoints, 0x80)), PRIME)
              // expmods_and_points.points[80] = -(g^32768 * z)
              mstore(add(expmodsAndPoints, 0xdc0), point)

              // point *= g^4
              point := mulmod(point, /*traceGenerator^4*/ mload(add(expmodsAndPoints, 0x20)), PRIME)
              // expmods_and_points.points[81] = -(g^32772 * z)
              mstore(add(expmodsAndPoints, 0xde0), point)

              // point *= g^16
              point := mulmod(point, /*traceGenerator^16*/ mload(add(expmodsAndPoints, 0xc0)), PRIME)
              // expmods_and_points.points[82] = -(g^32788 * z)
              mstore(add(expmodsAndPoints, 0xe00), point)

              // point *= g^235
              point := mulmod(point, /*traceGenerator^235*/ mload(add(expmodsAndPoints, 0x180)), PRIME)
              // expmods_and_points.points[83] = -(g^33023 * z)
              mstore(add(expmodsAndPoints, 0xe20), point)

              // point *= g^2820
              point := mulmod(point, /*traceGenerator^2820*/ mload(add(expmodsAndPoints, 0x2e0)), PRIME)
              // expmods_and_points.points[84] = -(g^35843 * z)
              mstore(add(expmodsAndPoints, 0xe40), point)

              // point *= g^1024
              point := mulmod(point, /*traceGenerator^1024*/ mload(add(expmodsAndPoints, 0x280)), PRIME)
              // expmods_and_points.points[85] = -(g^36867 * z)
              mstore(add(expmodsAndPoints, 0xe60), point)

              // point *= g^1024
              point := mulmod(point, /*traceGenerator^1024*/ mload(add(expmodsAndPoints, 0x280)), PRIME)
              // expmods_and_points.points[86] = -(g^37891 * z)
              mstore(add(expmodsAndPoints, 0xe80), point)

              // point *= g^2048
              point := mulmod(point, /*traceGenerator^2048*/ mload(add(expmodsAndPoints, 0x2c0)), PRIME)
              // expmods_and_points.points[87] = -(g^39939 * z)
              mstore(add(expmodsAndPoints, 0xea0), point)

              // point *= g^1017
              point := mulmod(point, /*traceGenerator^1017*/ mload(add(expmodsAndPoints, 0x220)), PRIME)
              // expmods_and_points.points[88] = -(g^40956 * z)
              mstore(add(expmodsAndPoints, 0xec0), point)

              // point *= g^3079
              point := mulmod(point, /*traceGenerator^3079*/ mload(add(expmodsAndPoints, 0x300)), PRIME)
              // expmods_and_points.points[89] = -(g^44035 * z)
              mstore(add(expmodsAndPoints, 0xee0), point)

              // point *= g^5093
              point := mulmod(point, /*traceGenerator^5093*/ mload(add(expmodsAndPoints, 0x360)), PRIME)
              // expmods_and_points.points[90] = -(g^49128 * z)
              mstore(add(expmodsAndPoints, 0xf00), point)

              // point *= g^16
              point := mulmod(point, /*traceGenerator^16*/ mload(add(expmodsAndPoints, 0xc0)), PRIME)
              // expmods_and_points.points[91] = -(g^49144 * z)
              mstore(add(expmodsAndPoints, 0xf20), point)

              // point *= g^263
              point := mulmod(point, /*traceGenerator^263*/ mload(add(expmodsAndPoints, 0x1e0)), PRIME)
              // expmods_and_points.points[92] = -(g^49407 * z)
              mstore(add(expmodsAndPoints, 0xf40), point)

              // point *= g^2820
              point := mulmod(point, /*traceGenerator^2820*/ mload(add(expmodsAndPoints, 0x2e0)), PRIME)
              // expmods_and_points.points[93] = -(g^52227 * z)
              mstore(add(expmodsAndPoints, 0xf60), point)

              // point *= g^8192
              point := mulmod(point, /*traceGenerator^8192*/ mload(add(expmodsAndPoints, 0x3a0)), PRIME)
              // expmods_and_points.points[94] = -(g^60419 * z)
              mstore(add(expmodsAndPoints, 0xf80), point)

              // point *= g^5093
              point := mulmod(point, /*traceGenerator^5093*/ mload(add(expmodsAndPoints, 0x360)), PRIME)
              // expmods_and_points.points[95] = -(g^65512 * z)
              mstore(add(expmodsAndPoints, 0xfa0), point)

              // point *= g^16
              point := mulmod(point, /*traceGenerator^16*/ mload(add(expmodsAndPoints, 0xc0)), PRIME)
              // expmods_and_points.points[96] = -(g^65528 * z)
              mstore(add(expmodsAndPoints, 0xfc0), point)
            }

            let evalPointsPtr := /*oodsEvalPoints*/ add(context, 0x4760)
            let evalPointsEndPtr := add(evalPointsPtr,
                                           mul(/*n_unique_queries*/ mload(add(context, 0x120)), 0x20))
            let productsPtr := add(batchInverseArray, 0x20)
            let valuesPtr := add(add(batchInverseArray, 0x20), 0x11880)
            let partialProduct := 1
            let minusPointPow := sub(PRIME, mulmod(oodsPoint, oodsPoint, PRIME))
            for {} lt(evalPointsPtr, evalPointsEndPtr)
                     {evalPointsPtr := add(evalPointsPtr, 0x20)} {
                let evalPoint := mload(evalPointsPtr)

                // Shift evalPoint to evaluation domain coset.
                let shiftedEvalPoint := mulmod(evalPoint, evalCosetOffset_, PRIME)

                {
                // Calculate denominator for row 0: x - z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x3c0)))
                mstore(productsPtr, partialProduct)
                mstore(valuesPtr, denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 1: x - g * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x3e0)))
                mstore(add(productsPtr, 0x20), partialProduct)
                mstore(add(valuesPtr, 0x20), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 2: x - g^2 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x400)))
                mstore(add(productsPtr, 0x40), partialProduct)
                mstore(add(valuesPtr, 0x40), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 3: x - g^3 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x420)))
                mstore(add(productsPtr, 0x60), partialProduct)
                mstore(add(valuesPtr, 0x60), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 4: x - g^4 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x440)))
                mstore(add(productsPtr, 0x80), partialProduct)
                mstore(add(valuesPtr, 0x80), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 6: x - g^6 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x460)))
                mstore(add(productsPtr, 0xa0), partialProduct)
                mstore(add(valuesPtr, 0xa0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 7: x - g^7 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x480)))
                mstore(add(productsPtr, 0xc0), partialProduct)
                mstore(add(valuesPtr, 0xc0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 8: x - g^8 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x4a0)))
                mstore(add(productsPtr, 0xe0), partialProduct)
                mstore(add(valuesPtr, 0xe0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 16: x - g^16 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x4c0)))
                mstore(add(productsPtr, 0x100), partialProduct)
                mstore(add(valuesPtr, 0x100), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 20: x - g^20 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x4e0)))
                mstore(add(productsPtr, 0x120), partialProduct)
                mstore(add(valuesPtr, 0x120), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 24: x - g^24 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x500)))
                mstore(add(productsPtr, 0x140), partialProduct)
                mstore(add(valuesPtr, 0x140), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 32: x - g^32 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x520)))
                mstore(add(productsPtr, 0x160), partialProduct)
                mstore(add(valuesPtr, 0x160), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 36: x - g^36 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x540)))
                mstore(add(productsPtr, 0x180), partialProduct)
                mstore(add(valuesPtr, 0x180), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 40: x - g^40 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x560)))
                mstore(add(productsPtr, 0x1a0), partialProduct)
                mstore(add(valuesPtr, 0x1a0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 48: x - g^48 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x580)))
                mstore(add(productsPtr, 0x1c0), partialProduct)
                mstore(add(valuesPtr, 0x1c0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 56: x - g^56 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x5a0)))
                mstore(add(productsPtr, 0x1e0), partialProduct)
                mstore(add(valuesPtr, 0x1e0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 64: x - g^64 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x5c0)))
                mstore(add(productsPtr, 0x200), partialProduct)
                mstore(add(valuesPtr, 0x200), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 68: x - g^68 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x5e0)))
                mstore(add(productsPtr, 0x220), partialProduct)
                mstore(add(valuesPtr, 0x220), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 72: x - g^72 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x600)))
                mstore(add(productsPtr, 0x240), partialProduct)
                mstore(add(valuesPtr, 0x240), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 84: x - g^84 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x620)))
                mstore(add(productsPtr, 0x260), partialProduct)
                mstore(add(valuesPtr, 0x260), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 88: x - g^88 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x640)))
                mstore(add(productsPtr, 0x280), partialProduct)
                mstore(add(valuesPtr, 0x280), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 96: x - g^96 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x660)))
                mstore(add(productsPtr, 0x2a0), partialProduct)
                mstore(add(valuesPtr, 0x2a0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 100: x - g^100 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x680)))
                mstore(add(productsPtr, 0x2c0), partialProduct)
                mstore(add(valuesPtr, 0x2c0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 112: x - g^112 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x6a0)))
                mstore(add(productsPtr, 0x2e0), partialProduct)
                mstore(add(valuesPtr, 0x2e0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 132: x - g^132 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x6c0)))
                mstore(add(productsPtr, 0x300), partialProduct)
                mstore(add(valuesPtr, 0x300), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 148: x - g^148 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x6e0)))
                mstore(add(productsPtr, 0x320), partialProduct)
                mstore(add(valuesPtr, 0x320), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 164: x - g^164 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x700)))
                mstore(add(productsPtr, 0x340), partialProduct)
                mstore(add(valuesPtr, 0x340), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 196: x - g^196 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x720)))
                mstore(add(productsPtr, 0x360), partialProduct)
                mstore(add(valuesPtr, 0x360), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 255: x - g^255 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x740)))
                mstore(add(productsPtr, 0x380), partialProduct)
                mstore(add(valuesPtr, 0x380), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 256: x - g^256 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x760)))
                mstore(add(productsPtr, 0x3a0), partialProduct)
                mstore(add(valuesPtr, 0x3a0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 511: x - g^511 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x780)))
                mstore(add(productsPtr, 0x3c0), partialProduct)
                mstore(add(valuesPtr, 0x3c0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 512: x - g^512 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x7a0)))
                mstore(add(productsPtr, 0x3e0), partialProduct)
                mstore(add(valuesPtr, 0x3e0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 767: x - g^767 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x7c0)))
                mstore(add(productsPtr, 0x400), partialProduct)
                mstore(add(valuesPtr, 0x400), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 768: x - g^768 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x7e0)))
                mstore(add(productsPtr, 0x420), partialProduct)
                mstore(add(valuesPtr, 0x420), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 1020: x - g^1020 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x800)))
                mstore(add(productsPtr, 0x440), partialProduct)
                mstore(add(valuesPtr, 0x440), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 1021: x - g^1021 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x820)))
                mstore(add(productsPtr, 0x460), partialProduct)
                mstore(add(valuesPtr, 0x460), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 1022: x - g^1022 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x840)))
                mstore(add(productsPtr, 0x480), partialProduct)
                mstore(add(valuesPtr, 0x480), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 1024: x - g^1024 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x860)))
                mstore(add(productsPtr, 0x4a0), partialProduct)
                mstore(add(valuesPtr, 0x4a0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 1026: x - g^1026 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x880)))
                mstore(add(productsPtr, 0x4c0), partialProduct)
                mstore(add(valuesPtr, 0x4c0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 1027: x - g^1027 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x8a0)))
                mstore(add(productsPtr, 0x4e0), partialProduct)
                mstore(add(valuesPtr, 0x4e0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 1279: x - g^1279 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x8c0)))
                mstore(add(productsPtr, 0x500), partialProduct)
                mstore(add(valuesPtr, 0x500), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 2044: x - g^2044 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x8e0)))
                mstore(add(productsPtr, 0x520), partialProduct)
                mstore(add(valuesPtr, 0x520), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 2045: x - g^2045 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x900)))
                mstore(add(productsPtr, 0x540), partialProduct)
                mstore(add(valuesPtr, 0x540), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 2051: x - g^2051 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x920)))
                mstore(add(productsPtr, 0x560), partialProduct)
                mstore(add(valuesPtr, 0x560), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 3069: x - g^3069 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x940)))
                mstore(add(productsPtr, 0x580), partialProduct)
                mstore(add(valuesPtr, 0x580), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 3075: x - g^3075 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x960)))
                mstore(add(productsPtr, 0x5a0), partialProduct)
                mstore(add(valuesPtr, 0x5a0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 4092: x - g^4092 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x980)))
                mstore(add(productsPtr, 0x5c0), partialProduct)
                mstore(add(valuesPtr, 0x5c0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 4093: x - g^4093 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x9a0)))
                mstore(add(productsPtr, 0x5e0), partialProduct)
                mstore(add(valuesPtr, 0x5e0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 4099: x - g^4099 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x9c0)))
                mstore(add(productsPtr, 0x600), partialProduct)
                mstore(add(valuesPtr, 0x600), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 5123: x - g^5123 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x9e0)))
                mstore(add(productsPtr, 0x620), partialProduct)
                mstore(add(valuesPtr, 0x620), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 6141: x - g^6141 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xa00)))
                mstore(add(productsPtr, 0x640), partialProduct)
                mstore(add(valuesPtr, 0x640), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 7171: x - g^7171 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xa20)))
                mstore(add(productsPtr, 0x660), partialProduct)
                mstore(add(valuesPtr, 0x660), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 8188: x - g^8188 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xa40)))
                mstore(add(productsPtr, 0x680), partialProduct)
                mstore(add(valuesPtr, 0x680), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 8195: x - g^8195 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xa60)))
                mstore(add(productsPtr, 0x6a0), partialProduct)
                mstore(add(valuesPtr, 0x6a0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 8196: x - g^8196 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xa80)))
                mstore(add(productsPtr, 0x6c0), partialProduct)
                mstore(add(valuesPtr, 0x6c0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 9219: x - g^9219 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xaa0)))
                mstore(add(productsPtr, 0x6e0), partialProduct)
                mstore(add(valuesPtr, 0x6e0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 10237: x - g^10237 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xac0)))
                mstore(add(productsPtr, 0x700), partialProduct)
                mstore(add(valuesPtr, 0x700), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 11267: x - g^11267 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xae0)))
                mstore(add(productsPtr, 0x720), partialProduct)
                mstore(add(valuesPtr, 0x720), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 12284: x - g^12284 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xb00)))
                mstore(add(productsPtr, 0x740), partialProduct)
                mstore(add(valuesPtr, 0x740), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 12285: x - g^12285 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xb20)))
                mstore(add(productsPtr, 0x760), partialProduct)
                mstore(add(valuesPtr, 0x760), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 16328: x - g^16328 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xb40)))
                mstore(add(productsPtr, 0x780), partialProduct)
                mstore(add(valuesPtr, 0x780), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 16336: x - g^16336 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xb60)))
                mstore(add(productsPtr, 0x7a0), partialProduct)
                mstore(add(valuesPtr, 0x7a0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 16360: x - g^16360 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xb80)))
                mstore(add(productsPtr, 0x7c0), partialProduct)
                mstore(add(valuesPtr, 0x7c0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 16368: x - g^16368 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xba0)))
                mstore(add(productsPtr, 0x7e0), partialProduct)
                mstore(add(valuesPtr, 0x7e0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 16376: x - g^16376 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xbc0)))
                mstore(add(productsPtr, 0x800), partialProduct)
                mstore(add(valuesPtr, 0x800), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 16384: x - g^16384 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xbe0)))
                mstore(add(productsPtr, 0x820), partialProduct)
                mstore(add(valuesPtr, 0x820), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 16416: x - g^16416 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xc00)))
                mstore(add(productsPtr, 0x840), partialProduct)
                mstore(add(valuesPtr, 0x840), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 16639: x - g^16639 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xc20)))
                mstore(add(productsPtr, 0x860), partialProduct)
                mstore(add(valuesPtr, 0x860), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 19459: x - g^19459 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xc40)))
                mstore(add(productsPtr, 0x880), partialProduct)
                mstore(add(valuesPtr, 0x880), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 20477: x - g^20477 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xc60)))
                mstore(add(productsPtr, 0x8a0), partialProduct)
                mstore(add(valuesPtr, 0x8a0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 27651: x - g^27651 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xc80)))
                mstore(add(productsPtr, 0x8c0), partialProduct)
                mstore(add(valuesPtr, 0x8c0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 28669: x - g^28669 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xca0)))
                mstore(add(productsPtr, 0x8e0), partialProduct)
                mstore(add(valuesPtr, 0x8e0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 32676: x - g^32676 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xcc0)))
                mstore(add(productsPtr, 0x900), partialProduct)
                mstore(add(valuesPtr, 0x900), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 32708: x - g^32708 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xce0)))
                mstore(add(productsPtr, 0x920), partialProduct)
                mstore(add(valuesPtr, 0x920), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 32712: x - g^32712 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xd00)))
                mstore(add(productsPtr, 0x940), partialProduct)
                mstore(add(valuesPtr, 0x940), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 32724: x - g^32724 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xd20)))
                mstore(add(productsPtr, 0x960), partialProduct)
                mstore(add(valuesPtr, 0x960), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 32740: x - g^32740 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xd40)))
                mstore(add(productsPtr, 0x980), partialProduct)
                mstore(add(valuesPtr, 0x980), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 32744: x - g^32744 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xd60)))
                mstore(add(productsPtr, 0x9a0), partialProduct)
                mstore(add(valuesPtr, 0x9a0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 32752: x - g^32752 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xd80)))
                mstore(add(productsPtr, 0x9c0), partialProduct)
                mstore(add(valuesPtr, 0x9c0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 32760: x - g^32760 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xda0)))
                mstore(add(productsPtr, 0x9e0), partialProduct)
                mstore(add(valuesPtr, 0x9e0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 32768: x - g^32768 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xdc0)))
                mstore(add(productsPtr, 0xa00), partialProduct)
                mstore(add(valuesPtr, 0xa00), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 32772: x - g^32772 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xde0)))
                mstore(add(productsPtr, 0xa20), partialProduct)
                mstore(add(valuesPtr, 0xa20), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 32788: x - g^32788 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xe00)))
                mstore(add(productsPtr, 0xa40), partialProduct)
                mstore(add(valuesPtr, 0xa40), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 33023: x - g^33023 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xe20)))
                mstore(add(productsPtr, 0xa60), partialProduct)
                mstore(add(valuesPtr, 0xa60), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 35843: x - g^35843 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xe40)))
                mstore(add(productsPtr, 0xa80), partialProduct)
                mstore(add(valuesPtr, 0xa80), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 36867: x - g^36867 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xe60)))
                mstore(add(productsPtr, 0xaa0), partialProduct)
                mstore(add(valuesPtr, 0xaa0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 37891: x - g^37891 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xe80)))
                mstore(add(productsPtr, 0xac0), partialProduct)
                mstore(add(valuesPtr, 0xac0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 39939: x - g^39939 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xea0)))
                mstore(add(productsPtr, 0xae0), partialProduct)
                mstore(add(valuesPtr, 0xae0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 40956: x - g^40956 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xec0)))
                mstore(add(productsPtr, 0xb00), partialProduct)
                mstore(add(valuesPtr, 0xb00), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 44035: x - g^44035 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xee0)))
                mstore(add(productsPtr, 0xb20), partialProduct)
                mstore(add(valuesPtr, 0xb20), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 49128: x - g^49128 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xf00)))
                mstore(add(productsPtr, 0xb40), partialProduct)
                mstore(add(valuesPtr, 0xb40), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 49144: x - g^49144 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xf20)))
                mstore(add(productsPtr, 0xb60), partialProduct)
                mstore(add(valuesPtr, 0xb60), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 49407: x - g^49407 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xf40)))
                mstore(add(productsPtr, 0xb80), partialProduct)
                mstore(add(valuesPtr, 0xb80), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 52227: x - g^52227 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xf60)))
                mstore(add(productsPtr, 0xba0), partialProduct)
                mstore(add(valuesPtr, 0xba0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 60419: x - g^60419 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xf80)))
                mstore(add(productsPtr, 0xbc0), partialProduct)
                mstore(add(valuesPtr, 0xbc0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 65512: x - g^65512 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xfa0)))
                mstore(add(productsPtr, 0xbe0), partialProduct)
                mstore(add(valuesPtr, 0xbe0), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 65528: x - g^65528 * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xfc0)))
                mstore(add(productsPtr, 0xc00), partialProduct)
                mstore(add(valuesPtr, 0xc00), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 512 * (vaults_path_length - 1) + 511: x - g^(512 * (vaults_path_length - 1) + 511) * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0xfe0)))
                mstore(add(productsPtr, 0xc20), partialProduct)
                mstore(add(valuesPtr, 0xc20), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 16384 + 512 * (vaults_path_length - 1) + 511: x - g^(16384 + 512 * (vaults_path_length - 1) + 511) * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x1000)))
                mstore(add(productsPtr, 0xc40), partialProduct)
                mstore(add(valuesPtr, 0xc40), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate denominator for row 49152 + 512 * (vaults_path_length - 1) + 511: x - g^(49152 + 512 * (vaults_path_length - 1) + 511) * z.
                let denominator := add(shiftedEvalPoint, mload(add(expmodsAndPoints, 0x1020)))
                mstore(add(productsPtr, 0xc60), partialProduct)
                mstore(add(valuesPtr, 0xc60), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                {
                // Calculate the denominator for the composition polynomial columns: x - z^2
                let denominator := add(shiftedEvalPoint, minusPointPow)
                mstore(add(productsPtr, 0xc80), partialProduct)
                mstore(add(valuesPtr, 0xc80), denominator)
                partialProduct := mulmod(partialProduct, denominator, PRIME)
                }

                // Add evalPoint to batch inverse inputs.
                // inverse(evalPoint) is going to be used by FRI.
                mstore(add(productsPtr, 0xca0), partialProduct)
                mstore(add(valuesPtr, 0xca0), evalPoint)
                partialProduct := mulmod(partialProduct, evalPoint, PRIME)

                // Advance pointers.
                productsPtr := add(productsPtr, 0xcc0)
                valuesPtr := add(valuesPtr, 0xcc0)
            }

            let productsToValuesOffset := 0x11880
            let firstPartialProductPtr := add(batchInverseArray, 0x20)
            // Compute the inverse of the product.
            let prodInv := expmod(partialProduct, sub(PRIME, 2), PRIME)

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
            let currentPartialProductPtr := productsPtr
            // Loop in blocks of size 8 as much as possible: we can loop over a full block as long as
            // currentPartialProductPtr >= firstPartialProductPtr + 8*0x20, or equivalently,
            // currentPartialProductPtr > firstPartialProductPtr + 7*0x20.
            // We use the latter comparison since there is no >= evm opcode.
            let midPartialProductPtr := add(firstPartialProductPtr, 0xe0)
            for { } gt(currentPartialProductPtr, midPartialProductPtr) { } {
                currentPartialProductPtr := sub(currentPartialProductPtr, 0x20)
                // Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
                mstore(currentPartialProductPtr,
                       mulmod(mload(currentPartialProductPtr), prodInv, PRIME))
                // Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
                prodInv := mulmod(prodInv,
                                   mload(add(currentPartialProductPtr, productsToValuesOffset)),
                                   PRIME)

                currentPartialProductPtr := sub(currentPartialProductPtr, 0x20)
                // Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
                mstore(currentPartialProductPtr,
                       mulmod(mload(currentPartialProductPtr), prodInv, PRIME))
                // Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
                prodInv := mulmod(prodInv,
                                   mload(add(currentPartialProductPtr, productsToValuesOffset)),
                                   PRIME)

                currentPartialProductPtr := sub(currentPartialProductPtr, 0x20)
                // Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
                mstore(currentPartialProductPtr,
                       mulmod(mload(currentPartialProductPtr), prodInv, PRIME))
                // Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
                prodInv := mulmod(prodInv,
                                   mload(add(currentPartialProductPtr, productsToValuesOffset)),
                                   PRIME)

                currentPartialProductPtr := sub(currentPartialProductPtr, 0x20)
                // Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
                mstore(currentPartialProductPtr,
                       mulmod(mload(currentPartialProductPtr), prodInv, PRIME))
                // Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
                prodInv := mulmod(prodInv,
                                   mload(add(currentPartialProductPtr, productsToValuesOffset)),
                                   PRIME)

                currentPartialProductPtr := sub(currentPartialProductPtr, 0x20)
                // Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
                mstore(currentPartialProductPtr,
                       mulmod(mload(currentPartialProductPtr), prodInv, PRIME))
                // Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
                prodInv := mulmod(prodInv,
                                   mload(add(currentPartialProductPtr, productsToValuesOffset)),
                                   PRIME)

                currentPartialProductPtr := sub(currentPartialProductPtr, 0x20)
                // Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
                mstore(currentPartialProductPtr,
                       mulmod(mload(currentPartialProductPtr), prodInv, PRIME))
                // Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
                prodInv := mulmod(prodInv,
                                   mload(add(currentPartialProductPtr, productsToValuesOffset)),
                                   PRIME)

                currentPartialProductPtr := sub(currentPartialProductPtr, 0x20)
                // Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
                mstore(currentPartialProductPtr,
                       mulmod(mload(currentPartialProductPtr), prodInv, PRIME))
                // Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
                prodInv := mulmod(prodInv,
                                   mload(add(currentPartialProductPtr, productsToValuesOffset)),
                                   PRIME)

                currentPartialProductPtr := sub(currentPartialProductPtr, 0x20)
                // Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
                mstore(currentPartialProductPtr,
                       mulmod(mload(currentPartialProductPtr), prodInv, PRIME))
                // Update prodInv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
                prodInv := mulmod(prodInv,
                                   mload(add(currentPartialProductPtr, productsToValuesOffset)),
                                   PRIME)
            }

            // Loop over the remainder.
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
    }
}
