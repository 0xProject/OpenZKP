pragma solidity ^0.5.2;

contract OodsPoly {
    function() external {
          uint256 res;
          assembly {
            let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001
            // Copy input from calldata to memory.
            calldatacopy(0x0, 0x0, /*input_data_size*/ 2624)

            function expmod(base, exponent, modulus) -> res {
                let p := /*expmod_context*/ 3200
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
        mstore(2624,expmod(mload(0), 2097152, PRIME))
        mstore(2656,expmod(mload(0), 2, PRIME))
        mstore(2688,expmod(mload(0), 2097153, PRIME))
        mstore(2720,expmod(mload(0), 8192, PRIME))
        mstore(2752,expmod(mload(0), 2105344, PRIME))
        mstore(2784,expmod(mload(0), 2088961, PRIME))

        // Store the values which will be batch inverted
        mstore(3008, addmod(expmod(mload(0), 8192, PRIME), sub(PRIME , expmod(0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8, 2064384, PRIME)), PRIME))
        mstore(3040, addmod(expmod(mload(0), 8192, PRIME), sub(PRIME , 0x0000000000000000000000000000000000000000000000000000000000000001), PRIME))
        mstore(3072, addmod(mload(0), sub(PRIME , 0x0000000000000000000000000000000000000000000000000000000000000001), PRIME))
        mstore(3104, addmod(expmod(mload(0), 2097152, PRIME), sub(PRIME , 0x0000000000000000000000000000000000000000000000000000000000000001), PRIME))
        mstore(3136, addmod(mload(0), sub(PRIME , expmod(0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8, 2097151, PRIME)), PRIME))
        mstore(3168, addmod(expmod(mload(0), 8192, PRIME), sub(PRIME , expmod(0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8, 17179860992, PRIME)), PRIME))
      {
        // Compute the inverses of the denominators into denominator_invs using batch inverse.

        // Start by computing the cumulative product.
        // Let (d_0, d_1, d_2, ..., d_{n-1}) be the values in denominators. Then after this loop
        // denominator_invs will be (1, d_0, d_0 * d_1, ...) and prod will contain the value of
        // d_0 * ... * d_{n-1}.
        // Compute the offset between the partial_products array and the input values array.
        let products_to_values_offset := 192
        let prod := 1
        let partial_product_end_ptr := 3008
        for { let partial_product_ptr := 2816 }
            lt(partial_product_ptr, partial_product_end_ptr)
            { partial_product_ptr := add(partial_product_ptr, 0x20) } {
            mstore(partial_product_ptr, prod)
            // prod *= d_{i}.
            prod := mulmod(prod,
                           mload(add(partial_product_ptr, products_to_values_offset)),
                           PRIME)
        }

        let first_partial_product_ptr := 2816
        // Compute the inverse of the product.
        let prod_inv := expmod(prod, sub(PRIME, 2), PRIME)

        // Compute the inverses.
        // Loop over denominator_invs in reverse order.
        // current_partial_product_ptr is initialized to one past the end.
        for { let current_partial_product_ptr := 3008
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
        let val := mload(2112)
        res := addmod(res, mulmod(val, add(mload(256), mulmod(mload(288), mload(2624), PRIME)), PRIME),PRIME)
      }
      {
        let val := mload(2176)
        res := addmod(res, mulmod(val, add(mload(320), mulmod(mload(352), mload(2624), PRIME)), PRIME),PRIME)
      }
      {
        let val := mload(2240)
        res := addmod(res, mulmod(val, add(mload(384), mulmod(mload(416), mload(2624), PRIME)), PRIME),PRIME)
      }
      {
        let val := mload(2304)
        res := addmod(res, mulmod(val, add(mload(448), mulmod(mload(480), mload(2624), PRIME)), PRIME),PRIME)
      }
      {
        let val := mload(2368)
        res := addmod(res, mulmod(val, add(mload(512), mulmod(mload(544), mload(2624), PRIME)), PRIME),PRIME)
      }
      {
        let val := mload(2432)
        res := addmod(res, mulmod(val, add(mload(576), mulmod(mload(608), mload(2624), PRIME)), PRIME),PRIME)
      }
      {
        let val := mload(2496)
        res := addmod(res, mulmod(val, add(mload(640), mulmod(mload(672), mload(2624), PRIME)), PRIME),PRIME)
      }
      {
        let val := mload(2560)
        res := addmod(res, mulmod(val, add(mload(704), mulmod(mload(736), mload(2624), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mulmod(addmod(mload(96), sub(PRIME , mload(2112)), PRIME), addmod(mload(96), sub(PRIME , mload(2368)), PRIME), PRIME), mload(2880), PRIME)
        res := addmod(res, mulmod(val, add(mload(768), mulmod(mload(800), mload(2656), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(addmod(mload(64), sub(PRIME , mload(2496)), PRIME), mload(2944), PRIME)
        res := addmod(res, mulmod(val, add(mload(832), mulmod(mload(864), mload(2688), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mulmod(mulmod(addmod(mload(2496), sub(PRIME , mload(2144)), PRIME), addmod(mload(0), sub(PRIME , expmod(0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8, 2097151, PRIME)), PRIME), PRIME), mload(2976), PRIME), addmod(mload(2496), sub(PRIME , mload(2400)), PRIME), PRIME)
        res := addmod(res, mulmod(val, add(mload(896), mulmod(mload(928), mload(2720), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(addmod(mload(2496), sub(PRIME , 0x049ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804), PRIME), mload(2848), PRIME)
        res := addmod(res, mulmod(val, add(mload(960), mulmod(mload(992), mload(2752), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(addmod(mload(2560), sub(PRIME , 0x03ca0cfe4b3bc6ddf346d49d06ea0ed34e621062c0e056c1d0405d266e10268a), PRIME), mload(2848), PRIME)
        res := addmod(res, mulmod(val, add(mload(1024), mulmod(mload(1056), mload(2752), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mulmod(mulmod(addmod(mload(2112), sub(PRIME , mulmod(mload(2144), 0x0000000000000000000000000000000000000000000000000000000000000002, PRIME)), PRIME), addmod(addmod(mload(2112), sub(PRIME , mulmod(mload(2144), 0x0000000000000000000000000000000000000000000000000000000000000002, PRIME)), PRIME), sub(PRIME , 0x0000000000000000000000000000000000000000000000000000000000000001), PRIME), PRIME), addmod(expmod(mload(0), 8192, PRIME), sub(PRIME , expmod(0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8, 17179860992, PRIME)), PRIME), PRIME), mload(2912), PRIME)
        res := addmod(res, mulmod(val, add(mload(1088), mulmod(mload(1120), mload(2784), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mulmod(addmod(mulmod(addmod(mload(2112), sub(PRIME , mulmod(mload(2144), 0x0000000000000000000000000000000000000000000000000000000000000002, PRIME)), PRIME), addmod(mload(2560), sub(PRIME , mload(128)), PRIME), PRIME), sub(PRIME , mulmod(mload(2208), addmod(mload(2496), sub(PRIME , mload(192)), PRIME), PRIME)), PRIME), addmod(expmod(mload(0), 8192, PRIME), sub(PRIME , expmod(0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8, 17179860992, PRIME)), PRIME), PRIME), mload(2912), PRIME)
        res := addmod(res, mulmod(val, add(mload(1152), mulmod(mload(1184), mload(2784), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mulmod(addmod(mulmod(mload(2208), mload(2208), PRIME), sub(PRIME , mulmod(addmod(mload(2112), sub(PRIME , mulmod(mload(2144), 0x0000000000000000000000000000000000000000000000000000000000000002, PRIME)), PRIME), addmod(addmod(mload(2496), mload(192), PRIME), mload(2272), PRIME), PRIME)), PRIME), addmod(expmod(mload(0), 8192, PRIME), sub(PRIME , expmod(0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8, 17179860992, PRIME)), PRIME), PRIME), mload(2912), PRIME)
        res := addmod(res, mulmod(val, add(mload(1216), mulmod(mload(1248), mload(2784), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mulmod(addmod(mulmod(addmod(mload(2112), sub(PRIME , mulmod(mload(2144), 0x0000000000000000000000000000000000000000000000000000000000000002, PRIME)), PRIME), addmod(mload(2560), mload(2336), PRIME), PRIME), sub(PRIME , mulmod(mload(2208), addmod(mload(2496), sub(PRIME , mload(2272)), PRIME), PRIME)), PRIME), addmod(expmod(mload(0), 8192, PRIME), sub(PRIME , expmod(0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8, 17179860992, PRIME)), PRIME), PRIME), mload(2912), PRIME)
        res := addmod(res, mulmod(val, add(mload(1280), mulmod(mload(1312), mload(2784), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mulmod(mulmod(addmod(0x0000000000000000000000000000000000000000000000000000000000000001, sub(PRIME , addmod(mload(2112), sub(PRIME , mulmod(mload(2144), 0x0000000000000000000000000000000000000000000000000000000000000002, PRIME)), PRIME)), PRIME), addmod(mload(2496), sub(PRIME , mload(2272)), PRIME), PRIME), addmod(expmod(mload(0), 8192, PRIME), sub(PRIME , expmod(0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8, 17179860992, PRIME)), PRIME), PRIME), mload(2912), PRIME)
        res := addmod(res, mulmod(val, add(mload(1344), mulmod(mload(1376), mload(2784), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mulmod(mulmod(addmod(0x0000000000000000000000000000000000000000000000000000000000000001, sub(PRIME , addmod(mload(2112), sub(PRIME , mulmod(mload(2144), 0x0000000000000000000000000000000000000000000000000000000000000002, PRIME)), PRIME)), PRIME), addmod(mload(2560), sub(PRIME , mload(2336)), PRIME), PRIME), addmod(expmod(mload(0), 8192, PRIME), sub(PRIME , expmod(0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8, 17179860992, PRIME)), PRIME), PRIME), mload(2912), PRIME)
        res := addmod(res, mulmod(val, add(mload(1408), mulmod(mload(1440), mload(2784), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mload(2112), mload(2816), PRIME)
        res := addmod(res, mulmod(val, add(mload(1472), mulmod(mload(1504), mload(2752), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mload(2112), mload(2976), PRIME)
        res := addmod(res, mulmod(val, add(mload(1536), mulmod(mload(1568), mload(2752), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mulmod(mulmod(addmod(mload(2368), sub(PRIME , mulmod(mload(2400), 0x0000000000000000000000000000000000000000000000000000000000000002, PRIME)), PRIME), addmod(addmod(mload(2368), sub(PRIME , mulmod(mload(2400), 0x0000000000000000000000000000000000000000000000000000000000000002, PRIME)), PRIME), sub(PRIME , 0x0000000000000000000000000000000000000000000000000000000000000001), PRIME), PRIME), addmod(expmod(mload(0), 8192, PRIME), sub(PRIME , expmod(0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8, 17179860992, PRIME)), PRIME), PRIME), mload(2912), PRIME)
        res := addmod(res, mulmod(val, add(mload(1600), mulmod(mload(1632), mload(2784), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mulmod(addmod(mulmod(addmod(mload(2368), sub(PRIME , mulmod(mload(2400), 0x0000000000000000000000000000000000000000000000000000000000000002, PRIME)), PRIME), addmod(mload(2336), sub(PRIME , mload(160)), PRIME), PRIME), sub(PRIME , mulmod(mload(2464), addmod(mload(2272), sub(PRIME , mload(224)), PRIME), PRIME)), PRIME), addmod(expmod(mload(0), 8192, PRIME), sub(PRIME , expmod(0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8, 17179860992, PRIME)), PRIME), PRIME), mload(2912), PRIME)
        res := addmod(res, mulmod(val, add(mload(1664), mulmod(mload(1696), mload(2784), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mulmod(addmod(mulmod(mload(2464), mload(2464), PRIME), sub(PRIME , mulmod(addmod(mload(2368), sub(PRIME , mulmod(mload(2400), 0x0000000000000000000000000000000000000000000000000000000000000002, PRIME)), PRIME), addmod(addmod(mload(2272), mload(224), PRIME), mload(2528), PRIME), PRIME)), PRIME), addmod(expmod(mload(0), 8192, PRIME), sub(PRIME , expmod(0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8, 17179860992, PRIME)), PRIME), PRIME), mload(2912), PRIME)
        res := addmod(res, mulmod(val, add(mload(1728), mulmod(mload(1760), mload(2784), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mulmod(addmod(mulmod(addmod(mload(2368), sub(PRIME , mulmod(mload(2400), 0x0000000000000000000000000000000000000000000000000000000000000002, PRIME)), PRIME), addmod(mload(2336), mload(2592), PRIME), PRIME), sub(PRIME , mulmod(mload(2464), addmod(mload(2272), sub(PRIME , mload(2528)), PRIME), PRIME)), PRIME), addmod(expmod(mload(0), 8192, PRIME), sub(PRIME , expmod(0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8, 17179860992, PRIME)), PRIME), PRIME), mload(2912), PRIME)
        res := addmod(res, mulmod(val, add(mload(1792), mulmod(mload(1824), mload(2784), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mulmod(mulmod(addmod(0x0000000000000000000000000000000000000000000000000000000000000001, sub(PRIME , addmod(mload(2368), sub(PRIME , mulmod(mload(2400), 0x0000000000000000000000000000000000000000000000000000000000000002, PRIME)), PRIME)), PRIME), addmod(mload(2272), sub(PRIME , mload(2528)), PRIME), PRIME), addmod(expmod(mload(0), 8192, PRIME), sub(PRIME , expmod(0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8, 17179860992, PRIME)), PRIME), PRIME), mload(2912), PRIME)
        res := addmod(res, mulmod(val, add(mload(1856), mulmod(mload(1888), mload(2784), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mulmod(mulmod(addmod(0x0000000000000000000000000000000000000000000000000000000000000001, sub(PRIME , addmod(mload(2368), sub(PRIME , mulmod(mload(2400), 0x0000000000000000000000000000000000000000000000000000000000000002, PRIME)), PRIME)), PRIME), addmod(mload(2336), sub(PRIME , mload(2592)), PRIME), PRIME), addmod(expmod(mload(0), 8192, PRIME), sub(PRIME , expmod(0x01f4c2bfbeea04f127e46d76ecbf157b030530a6e7c1b2fd006547440e5ffcb8, 17179860992, PRIME)), PRIME), PRIME), mload(2912), PRIME)
        res := addmod(res, mulmod(val, add(mload(1920), mulmod(mload(1952), mload(2784), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mload(2368), mload(2816), PRIME)
        res := addmod(res, mulmod(val, add(mload(1984), mulmod(mload(2016), mload(2752), PRIME)), PRIME),PRIME)
      }
      {
        let val := mulmod(mload(2368), mload(2976), PRIME)
        res := addmod(res, mulmod(val, add(mload(2048), mulmod(mload(2080), mload(2752), PRIME)), PRIME),PRIME)
      }
      mstore(0, res)
      return(0, 0x20)
  }
  }
}
