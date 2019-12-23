  function oods_prepare_inverses(uint256[] memory context) internal pure {
        uint trace_generator = 0x00c92ecbfd121b58bc7924d5dd51a717c4d78992d0e87cd59f1bc8d7ff0cb347;
        uint oods_point = context[mm_oods_point];
        for (uint i = 0; i < context[mm_n_unique_queries]; i ++) {
            // Get the shifted eval point
            uint x = fmul(context[i + mm_oods_eval_points], PrimeFieldElement0.generator_val);
            // Preparing denominator for row 0
        context[batch_inverse_chunk*i + 0 + mm_batch_inverse_in] = fsub(x, fmul(oods_point, 0x0000000000000000000000000000000000000000000000000000000000000001));
            // Preparing denominator for row 1
        context[batch_inverse_chunk*i + 1 + mm_batch_inverse_in] = fsub(x, fmul(oods_point, 0x00c92ecbfd121b58bc7924d5dd51a717c4d78992d0e87cd59f1bc8d7ff0cb347));
        context[batch_inverse_chunk*i + 2 + mm_batch_inverse_in] = fsub(x, fpow2(oods_point, 3));
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
      let trace_query_responses := /*trace_query_responses*/ add(context, 12704)

      let composition_query_responses := /*composition_query_responses*/ add(context, 14112)

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
                                  /*oods_coefficients*/ mload(add(context, 12480)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 11520)))),
                           PRIME),
                    PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 1th row */ mload(add(denominators_ptr, 32)),
                                  /*oods_coefficients*/ mload(add(context, 12512)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 11552)))),
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
                                  /*oods_coefficients*/ mload(add(context, 12544)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 11584)))),
                           PRIME),
                    PRIME)
              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for 1th row */ mload(add(denominators_ptr, 32)),
                                  /*oods_coefficients*/ mload(add(context, 12576)),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, 11616)))),
                           PRIME),
                    PRIME)
                }
              trace_query_responses := add(trace_query_responses, 64)
         // Composition constraints.
              {
            // Read the next element.
            let column_value := mulmod(mload(add(composition_query_responses, 0)), k_montgomery_r_inv, PRIME)
            res := addmod(
                res,
                mulmod(mulmod(mload(add(denominators_ptr, 64)),
                              mload(/*oods_coefficients*/ add(context, 12608)),
                              PRIME),
                       add(column_value,
                           sub(PRIME, /*composition_oods_values*/ mload(add(context, 11648)))),
                       PRIME),
                PRIME)
            }
              {
            // Read the next element.
            let column_value := mulmod(mload(add(composition_query_responses, 32)), k_montgomery_r_inv, PRIME)
            res := addmod(
                res,
                mulmod(mulmod(mload(add(denominators_ptr, 64)),
                              mload(/*oods_coefficients*/ add(context, 12640)),
                              PRIME),
                       add(column_value,
                           sub(PRIME, /*composition_oods_values*/ mload(add(context, 11680)))),
                       PRIME),
                PRIME)
            }
              {
            // Read the next element.
            let column_value := mulmod(mload(add(composition_query_responses, 64)), k_montgomery_r_inv, PRIME)
            res := addmod(
                res,
                mulmod(mulmod(mload(add(denominators_ptr, 64)),
                              mload(/*oods_coefficients*/ add(context, 12672)),
                              PRIME),
                       add(column_value,
                           sub(PRIME, /*composition_oods_values*/ mload(add(context, 11712)))),
                       PRIME),
                PRIME)
            }
              // Advance the composition_query_responses by the the amount we've read (0x20 * constraint_degree).
    composition_query_responses := add(composition_query_responses, 96)

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
