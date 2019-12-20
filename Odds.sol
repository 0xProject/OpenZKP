  function oods_prepare_inverses(uint256[] memory context) internal pure {
        uint trace_generator = 0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8;
        uint oods_point = context[mm_oods_point];
        for (uint i = 0; i < context[mm_n_unique_queries]; i ++) {
            // Get the shifted eval point
            uint x = fmul(context[i + mm_oods_eval_points], PrimeFieldElement0.generator_val);
            // Preparing denominator for row 0
        context[batch_inverse_chunk*i + 0 + mm_batch_inverse_in] = fsub(x, fmul(oods_point, 0x0000000000000000000000000000000000000000000000000000000000000001));
            // Preparing denominator for row 1
        context[batch_inverse_chunk*i + 1 + mm_batch_inverse_in] = fsub(x, fmul(oods_point, 0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8));
        context[batch_inverse_chunk*i + 2 + mm_batch_inverse_in] = fsub(x, fpow2(oods_point, 2));
        context[batch_inverse_chunk*i + 3 + mm_batch_inverse_in] = context[i + mm_oods_eval_points];
}

    uint carried = 1;
    for (uint i = 0; i < context[mm_n_unique_queries]*4; i ++) {
        carried = fmul(carried, context[mm_batch_inverse_in+i]);
        context[mm_batch_inverse_out+i] = carried;
    }

    uint inv_prod = inverse2(carried);

    for (uint i = context[mm_n_unique_queries]*batch_inverse_chunk - 1; i > 0; i--) {
        context[mm_batch_inverse_out + i] = fmul(inv_prod, context[mm_batch_inverse_out + i - 1]);
        inv_prod = fmul(inv_prod, context[mm_batch_inverse_in + i]);
    }
    context[mm_batch_inverse_out] = inv_prod;
  }
  function oods_virtual_oracle(uint256[] memory ctx)
  internal {
  oods_prepare_inverses(ctx);

  uint k_montgomery_r_inv_ = PrimeFieldElement0.k_montgomery_r_inv;
  assembly {
      let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001
      let k_montgomery_r_inv := k_montgomery_r_inv_
      let context := ctx
      let fri_values := /*fri_values*/ add(context, 1824)
      let fri_values_end := add(fri_values,  mul(/*n_unique_queries*/ mload(add(context, 288)), 0x20))
      let fri_inv_points := /*fri_inv_points*/ add(context, 2528)
      let trace_query_responses := /*trace_query_responses*/ add(context, 14976)

      let composition_query_responses := /*composition_query_responses*/ add(context, 20608)

      // Set denominators_ptr to point to the batch_inverse_out array.
      // The content of batch_inverse_out is described in oods_prepare_inverses.
      let denominators_ptr := /*batch_inverse_out*/ add(context, 5344)
      for {} lt(fri_values, fri_values_end) {fri_values := add(fri_values, 0x20)} {
        let res := 0
              // Mask items for column #0.
                    {
                    // Read the next element.
                    let column_value := mulmod(mload(add(trace_query_responses, 0)), k_montgomery_r_inv, PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 0th row */ mload(add(denominators_ptr, 0)),
                                  /*oods_coefficients*/ mload(add(context, 14400)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 13088)))),
                           PRIME),
                    PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 1th row */ mload(add(denominators_ptr, 32)),
                                  /*oods_coefficients*/ mload(add(context, 14432)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 13120)))),
                           PRIME),
                    PRIME)
                }
              // Mask items for column #1.
                    {
                    // Read the next element.
                    let column_value := mulmod(mload(add(trace_query_responses, 32)), k_montgomery_r_inv, PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 0th row */ mload(add(denominators_ptr, 0)),
                                  /*oods_coefficients*/ mload(add(context, 14464)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 13152)))),
                           PRIME),
                    PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 1th row */ mload(add(denominators_ptr, 32)),
                                  /*oods_coefficients*/ mload(add(context, 14496)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 13184)))),
                           PRIME),
                    PRIME)
                }
              // Mask items for column #2.
                    {
                    // Read the next element.
                    let column_value := mulmod(mload(add(trace_query_responses, 64)), k_montgomery_r_inv, PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 0th row */ mload(add(denominators_ptr, 0)),
                                  /*oods_coefficients*/ mload(add(context, 14528)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 13216)))),
                           PRIME),
                    PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 1th row */ mload(add(denominators_ptr, 32)),
                                  /*oods_coefficients*/ mload(add(context, 14560)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 13248)))),
                           PRIME),
                    PRIME)
                }
              // Mask items for column #3.
                    {
                    // Read the next element.
                    let column_value := mulmod(mload(add(trace_query_responses, 96)), k_montgomery_r_inv, PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 0th row */ mload(add(denominators_ptr, 0)),
                                  /*oods_coefficients*/ mload(add(context, 14592)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 13280)))),
                           PRIME),
                    PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 1th row */ mload(add(denominators_ptr, 32)),
                                  /*oods_coefficients*/ mload(add(context, 14624)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 13312)))),
                           PRIME),
                    PRIME)
                }
              // Mask items for column #4.
                    {
                    // Read the next element.
                    let column_value := mulmod(mload(add(trace_query_responses, 128)), k_montgomery_r_inv, PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 0th row */ mload(add(denominators_ptr, 0)),
                                  /*oods_coefficients*/ mload(add(context, 14656)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 13344)))),
                           PRIME),
                    PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 1th row */ mload(add(denominators_ptr, 32)),
                                  /*oods_coefficients*/ mload(add(context, 14688)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 13376)))),
                           PRIME),
                    PRIME)
                }
              // Mask items for column #5.
                    {
                    // Read the next element.
                    let column_value := mulmod(mload(add(trace_query_responses, 160)), k_montgomery_r_inv, PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 0th row */ mload(add(denominators_ptr, 0)),
                                  /*oods_coefficients*/ mload(add(context, 14720)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 13408)))),
                           PRIME),
                    PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 1th row */ mload(add(denominators_ptr, 32)),
                                  /*oods_coefficients*/ mload(add(context, 14752)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 13440)))),
                           PRIME),
                    PRIME)
                }
              // Mask items for column #6.
                    {
                    // Read the next element.
                    let column_value := mulmod(mload(add(trace_query_responses, 192)), k_montgomery_r_inv, PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 0th row */ mload(add(denominators_ptr, 0)),
                                  /*oods_coefficients*/ mload(add(context, 14784)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 13472)))),
                           PRIME),
                    PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 1th row */ mload(add(denominators_ptr, 32)),
                                  /*oods_coefficients*/ mload(add(context, 14816)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 13504)))),
                           PRIME),
                    PRIME)
                }
              // Mask items for column #7.
                    {
                    // Read the next element.
                    let column_value := mulmod(mload(add(trace_query_responses, 224)), k_montgomery_r_inv, PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 0th row */ mload(add(denominators_ptr, 0)),
                                  /*oods_coefficients*/ mload(add(context, 14848)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 13536)))),
                           PRIME),
                    PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 1th row */ mload(add(denominators_ptr, 32)),
                                  /*oods_coefficients*/ mload(add(context, 14880)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 13568)))),
                           PRIME),
                    PRIME)
                }
              trace_query_responses := add(trace_query_responses, 256)
         // Composition constraints.
              {
            // Read the next element.
            let column_value := mulmod(mload(add(composition_query_responses, 0)), k_montgomery_r_inv, PRIME)
            res := addmod(
                res,
                mulmod(mulmod(mload(add(denominators_ptr, 64)),
                              mload(/*oods_coefficients*/ add(context, 14912)),
                              PRIME),
                       add(column_value,
                           sub(PRIME, /*composition_oods_values*/ mload(add(context, 13600)))),
                       PRIME),
                PRIME)
            }
              {
            // Read the next element.
            let column_value := mulmod(mload(add(composition_query_responses, 32)), k_montgomery_r_inv, PRIME)
            res := addmod(
                res,
                mulmod(mulmod(mload(add(denominators_ptr, 64)),
                              mload(/*oods_coefficients*/ add(context, 14944)),
                              PRIME),
                       add(column_value,
                           sub(PRIME, /*composition_oods_values*/ mload(add(context, 13632)))),
                       PRIME),
                PRIME)
            }
              // Advance the composition_query_responses by the the amount we've read (0x20 * constraint_degree).
    composition_query_responses := add(composition_query_responses, 64)

    // Append the sum of the trace boundary constraints to the fri_values array.
    // Note that we need to add the sum of the composition boundary constraints to those
    // values before running fri.
    mstore(fri_values, res)

    // Append the fri_inv_point of the current query to the fri_inv_points array.
    mstore(fri_inv_points, mload(add(denominators_ptr, 96)))
    fri_inv_points := add(fri_inv_points, 0x20)

    // Advance denominators_ptr by chunk size (0x20 * (2+n_rows_in_mask)).
    denominators_ptr := add(denominators_ptr, 128)
}
}
}
