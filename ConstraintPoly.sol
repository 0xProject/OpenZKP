pragma solidity ^0.5.2;

contract OodsPoly {
    function() external {
          uint256 res;
          assembly {
            let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001
            // Copy input from calldata to memory.
            calldatacopy(0x0, 0x0, /*input_data_size*/ 672)

            function expmod(base, exponent, modulus) -> res {
                let p := /*expmod_context*/ 1024
                mstore(p, 0x20)                 // Length of Base
                mstore(add(p, 0x20), 0x20)      // Length of Exponent
                mstore(add(p, 0x40), 0x20)      // Length of Modulus
                mstore(add(p, 0x60), base)      // Base
                mstore(add(p, 0x80), exponent)  // Exponent
                mstore(add(p, 0xa0), modulus)   // Modulus
                // call modexp precompile
                if iszero(call(not(0), 0x05, 0, p, 0xc0, p, 0x20)) {
                    revert(0, 0)
                }
                res := mload(p)
            }
    
            function degree_adjustment(composition_polynomial_degree_bound, constraint_degree, numerator_degree,
                denominator_degree) -> res {
                                        res := sub(sub(composition_polynomial_degree_bound, 1),
                       sub(add(constraint_degree, numerator_degree), denominator_degree))
                    }
    
            function small_expmod(x, num, prime) -> res {
                res := 1
                for { let ind := 0 } lt(ind, num) { ind := add(ind, 1) } {
                       res := mulmod(res, x, prime)
                }
            }
        // Store adjustment degrees
        mstore(672,expmod(mload(0), 256, PRIME))
        mstore(704,expmod(mload(0), 257, PRIME))
        mstore(736,expmod(mload(0), 513, PRIME))

        // Store the values which will be batch inverted
        mstore(896, addmod(expmod(mload(0), 256, PRIME), sub(PRIME , 0x0000000000000000000000000000000000000000000000000000000000000001), PRIME))
        mstore(928, addmod(mload(0), sub(PRIME , expmod(0x00c92ecbfd121b58bc7924d5dd51a717c4d78992d0e87cd59f1bc8d7ff0cb347, 255, PRIME)), PRIME))
        mstore(960, addmod(mload(0), sub(PRIME , 0x01), PRIME))
        mstore(992, addmod(mload(0), sub(PRIME , expmod(0x00c92ecbfd121b58bc7924d5dd51a717c4d78992d0e87cd59f1bc8d7ff0cb347, 128, PRIME)), PRIME))
      {
        // Compute the inverses of the denominators into denominator_invs using batch inverse.

        // Start by computing the cumulative product.
        // Let (d_0, d_1, d_2, ..., d_{n-1}) be the values in denominators. Then after this loop
        // denominator_invs will be (1, d_0, d_0 * d_1, ...) and prod will contain the value of
        // d_0 * ... * d_{n-1}.
        // Compute the offset between the partial_products array and the input values array.
        let products_to_values_offset := 128
        let prod := 1
        let partial_product_end_ptr := 896
        for { let partial_product_ptr := 768 }
            lt(partial_product_ptr, partial_product_end_ptr)
            { partial_product_ptr := add(partial_product_ptr, 0x20) } {
            mstore(partial_product_ptr, prod)
            // prod *= d_{i}.
            prod := mulmod(prod,
                           mload(add(partial_product_ptr, products_to_values_offset)),
                           PRIME)
        }

        let first_partial_product_ptr := 768
        // Compute the inverse of the product.
        let prod_inv := expmod(prod, sub(PRIME, 2), PRIME)

        // Compute the inverses.
        // Loop over denominator_invs in reverse order.
        // current_partial_product_ptr is initialized to one past the end.
        for { let current_partial_product_ptr := 896
            } gt(current_partial_product_ptr, first_partial_product_ptr) { } {
            current_partial_product_ptr := sub(current_partial_product_ptr, 0x20)
            // Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
            mstore(current_partial_product_ptr,
                   mulmod(mload(current_partial_product_ptr), prod_inv, PRIME))
            // Update prod_inv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
            prod_inv := mulmod(prod_inv,
                               mload(add(current_partial_product_ptr, products_to_values_offset)),
                               PRIME)
        }
      }
      {
        let val := mulmod(addmod(addmod(addmod(small_expmod(mload(544), 3, PRIME), mulmod(mulmod(mulmod(0x0000000000000000000000000000000000000000000000000000000000000003, 0x000000000000000000000000000000000000000000000000000000000000000b, PRIME), mload(544), PRIME), small_expmod(mload(608), 2, PRIME), PRIME), PRIME), mload(128), PRIME), sub(PRIME , mload(576)), PRIME), mulmod(addmod(small_expmod(mload(0), 2, PRIME), sub(PRIME , 0x05c0a717bca84bf725164a76ab0137402eedac496081f8c38c71bd5031055652), PRIME), mload(768), PRIME), PRIME)
        res := addmod(res, mulmod(val, add(mload(160), mulmod(mload(192), mload(672), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(addmod(addmod(mulmod(0x0000000000000000000000000000000000000000000000000000000000000003, small_expmod(mload(544), 2, PRIME), PRIME), mulmod(0x000000000000000000000000000000000000000000000000000000000000000b, small_expmod(mload(608), 3, PRIME), PRIME), PRIME), sub(PRIME , mload(640)), PRIME), mulmod(addmod(mload(0), sub(PRIME , expmod(0x00c92ecbfd121b58bc7924d5dd51a717c4d78992d0e87cd59f1bc8d7ff0cb347, 255, PRIME)), PRIME), mload(768), PRIME), PRIME)
        res := addmod(res, mulmod(val, add(mload(224), mulmod(mload(256), mload(704), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(addmod(mload(544), sub(PRIME , mload(32)), PRIME), mload(832), PRIME)
        res := addmod(res, mulmod(val, add(mload(288), mulmod(mload(320), mload(736), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mload(608), mload(832), PRIME)
        res := addmod(res, mulmod(val, add(mload(352), mulmod(mload(384), mload(736), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(addmod(mload(544), sub(PRIME , mload(64)), PRIME), mload(864), PRIME)
        res := addmod(res, mulmod(val, add(mload(416), mulmod(mload(448), mload(736), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(addmod(mload(544), sub(PRIME , mload(96)), PRIME), mload(800), PRIME)
        res := addmod(res, mulmod(val, add(mload(480), mulmod(mload(512), mload(736), PRIME)), PRIME),PRIME)
      }
      mstore(0, res)
      return(0, 0x20)
  }
  }
}
