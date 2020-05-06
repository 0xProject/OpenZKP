pragma solidity ^0.6.6;


contract ConstraintPolyLen256 {
    fallback() external {
        assembly {
            let res := 0
            let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001
            // NOTE - If compilation hits a stack depth error on variable PRIME,
            // then uncomment the following line and globally replace PRIME with mload(960)
            // mstore(960, 0x800000000000011000000000000000000000000000000000000000000000001)
            // Copy input from calldata to memory.
            calldatacopy(
                0x0,
                0x0,
                /*input_data_size*/
                480
            )

            function expmod(base, exponent, modulus) -> result {
                let
                    p /*expmod_context*/
                := 768
                mstore(p, 0x20) // Length of Base
                mstore(add(p, 0x20), 0x20) // Length of Exponent
                mstore(add(p, 0x40), 0x20) // Length of Modulus
                mstore(add(p, 0x60), base) // Base
                mstore(add(p, 0x80), exponent) // Exponent
                mstore(add(p, 0xa0), modulus) // Modulus
                // call modexp precompile
                if iszero(call(not(0), 0x05, 0, p, 0xc0, p, 0x20)) {
                    revert(0, 0)
                }
                result := mload(p)
            }

            function degree_adjustment(
                composition_polynomial_degree_bound,
                constraint_degree,
                numerator_degree,
                denominator_degree
            ) -> result {
                result := sub(
                    sub(composition_polynomial_degree_bound, 1),
                    sub(add(constraint_degree, numerator_degree), denominator_degree)
                )
            }

            function small_expmod(x, num, prime) -> result {
                result := 1
                for {
                    let ind := 0
                } lt(ind, num) {
                    ind := add(ind, 1)
                } {
                    result := mulmod(result, x, prime)
                }
            }
            // Store adjustment degrees
            mstore(480, expmod(mload(0x0), 256, PRIME))
            mstore(512, expmod(mload(0x0), 511, PRIME))
            mstore(544, expmod(mload(0x0), 257, PRIME))

            // Store the values which will be batch inverted
            mstore(
                672,
                addmod(
                    mload(0x0),
                    sub(PRIME, expmod(0x00c92ecbfd121b58bc7924d5dd51a717c4d78992d0e87cd59f1bc8d7ff0cb347, 150, PRIME)),
                    PRIME
                )
            )
            mstore(
                704,
                addmod(
                    mload(0x0),
                    sub(PRIME, expmod(0x00c92ecbfd121b58bc7924d5dd51a717c4d78992d0e87cd59f1bc8d7ff0cb347, 256, PRIME)),
                    PRIME
                )
            )
            mstore(
                736,
                addmod(
                    expmod(mload(0x0), 256, PRIME),
                    sub(PRIME, 0x0000000000000000000000000000000000000000000000000000000000000001),
                    PRIME
                )
            )
            {
                // Compute the inverses of the denominators into denominator_invs using batch inverse.

                // Start by computing the cumulative product.
                // Let (d_0, d_1, d_2, ..., d_{n-1}) be the values in denominators. Then after this loop
                // denominator_invs will be (1, d_0, d_0 * d_1, ...) and prod will contain the value of
                // d_0 * ... * d_{n-1}.
                // Compute the offset between the partial_products array and the input values array.
                let products_to_values := 96
                let prod := 1
                let partial_product_end_ptr := 672
                for {
                    let partial_product_ptr := 576
                } lt(partial_product_ptr, partial_product_end_ptr) {
                    partial_product_ptr := add(partial_product_ptr, 0x20)
                } {
                    mstore(partial_product_ptr, prod)
                    // prod *= d_{i}.
                    prod := mulmod(prod, mload(add(partial_product_ptr, products_to_values)), PRIME)
                }

                let first_partial_product_ptr := 576
                // Compute the inverse of the product.
                let prod_inv := expmod(prod, sub(PRIME, 2), PRIME)

                // Compute the inverses.
                // Loop over denominator_invs in reverse order.
                // current_partial_product_ptr is initialized to one past the end.
                for {
                    let current_partial_product_ptr := 672
                } gt(current_partial_product_ptr, first_partial_product_ptr) {

                } {
                    current_partial_product_ptr := sub(current_partial_product_ptr, 0x20)
                    // Store 1/d_{i} = (d_0 * ... * d_{i-1}) * 1/(d_0 * ... * d_{i}).
                    mstore(current_partial_product_ptr, mulmod(mload(current_partial_product_ptr), prod_inv, PRIME))
                    // Update prod_inv to be 1/(d_0 * ... * d_{i-1}) by multiplying by d_i.
                    prod_inv := mulmod(prod_inv, mload(add(current_partial_product_ptr, products_to_values)), PRIME)
                }
            }
            {
                let val := mulmod(
                    addmod(mload(384), sub(PRIME, small_expmod(mload(416), 2, PRIME)), PRIME),
                    mulmod(
                        addmod(
                            mload(0x0),
                            sub(
                                PRIME,
                                expmod(0x00c92ecbfd121b58bc7924d5dd51a717c4d78992d0e87cd59f1bc8d7ff0cb347, 255, PRIME)
                            ),
                            PRIME
                        ),
                        mload(640),
                        PRIME
                    ),
                    PRIME
                )
                res := addmod(res, mulmod(val, add(mload(96), mulmod(mload(128), mload(480), PRIME)), PRIME), PRIME)
            }
            {
                let val := mulmod(
                    addmod(addmod(mload(448), sub(PRIME, mload(352)), PRIME), sub(PRIME, mload(416)), PRIME),
                    mulmod(
                        addmod(
                            mload(0x0),
                            sub(
                                PRIME,
                                expmod(0x00c92ecbfd121b58bc7924d5dd51a717c4d78992d0e87cd59f1bc8d7ff0cb347, 255, PRIME)
                            ),
                            PRIME
                        ),
                        mload(640),
                        PRIME
                    ),
                    PRIME
                )
                res := addmod(res, mulmod(val, add(mload(160), mulmod(mload(192), mload(512), PRIME)), PRIME), PRIME)
            }
            {
                let val := mulmod(
                    addmod(
                        mload(352),
                        sub(PRIME, 0x0000000000000000000000000000000000000000000000000000000000000001),
                        PRIME
                    ),
                    mload(608),
                    PRIME
                )
                res := addmod(res, mulmod(val, add(mload(224), mulmod(mload(256), mload(544), PRIME)), PRIME), PRIME)
            }
            {
                let val := mulmod(addmod(mload(352), sub(PRIME, mload(64)), PRIME), mload(576), PRIME)
                res := addmod(res, mulmod(val, add(mload(288), mulmod(mload(320), mload(544), PRIME)), PRIME), PRIME)
            }
            mstore(0, res)
            return(0, 0x20)
        }
    }
}
