pragma solidity ^0.6.4;

library PrimeField {
    uint256 internal constant MODULUS = 0x0800000000000011000000000000000000000000000000000000000000000001;
    uint256 internal constant MODULUS_MASK = 0x0fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
    uint256 internal constant MONTGOMERY_R = 0x7fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1;
    uint256 internal constant MONTGOMERY_R_INV = 0x40000000000001100000000000012100000000000000000000000000000000;
    uint256 internal constant GENERATOR = 3;
    uint256 internal constant ONE = 1;

    function from_montgomery(uint256 value) internal pure returns (uint256) {
        return mulmod(value, MONTGOMERY_R_INV, MODULUS);
    }

    function from_montgomery_bytes(bytes32 bs) internal pure returns (uint256) {
        return from_montgomery(uint256(bs));
    }

    // This is an unchecked cast and should be used very carefully,
    // and only in cases when the data is already in the right form.
    function from_bytes_array_raw(bytes32[] memory input) internal pure returns (uint256[] memory data) {
        assembly {
            data := input
        }
    }

    function to_montgomery(uint256 value) internal pure returns (uint256) {
        return mulmod(value, MONTGOMERY_R, MODULUS);
    }

    function fmul(uint256 a, uint256 b) internal pure returns (uint256) {
        return mulmod(a, b, MODULUS);
    }

    function fmul_mont(uint256 a, uint256 b) internal pure returns (uint256) {
        return fmul(fmul(a, b), MONTGOMERY_R_INV);
    }

    function fadd(uint256 a, uint256 b) internal pure returns (uint256) {
        return addmod(a, b, MODULUS);
    }

    function fsub(uint256 a, uint256 b) internal pure returns (uint256) {
        return addmod(a, MODULUS - b, MODULUS);
    }

    function fpow(uint256 value, uint256 exp) internal returns (uint256) {
        return expmod(value, exp, MODULUS);
    }

    // There's still no native call to the exp mod precompile in solidity
    function expmod(
        uint256 base,
        uint256 exponent,
        uint256 modulus
    ) internal returns (uint256 result) {
        // TODO - Check if gas is based on absolute input length or on indicated length
        // that will have massive gas implications [13k for a square vs 50]
        assembly {
            let p := mload(0x40)
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
    }

    function inverse(uint256 value) internal returns (uint256) {
        // The expmod version here costs 13758 gas
        return expmod(value, MODULUS - 2, MODULUS);
    }

    // Reverts if unavailable
    function generator_power(uint8 log_order) internal returns (uint256) {
        uint256 maybe_exact = (MODULUS - 1) / (uint256(2)**log_order);
        require(maybe_exact * (uint256(2)**log_order) == (MODULUS - 1), 'Root unavailable');
        return expmod(GENERATOR, maybe_exact, MODULUS);
    }

    // Returns the primitive root of unity of a given order. Reverts if unavailable
    function root(uint256 order) internal returns (uint256) {
        require((MODULUS - 1) % order == 0, 'Root unavailable');
        return expmod(GENERATOR, (MODULUS - 1) / order, MODULUS);
    }

    // Evaluates the polynomial given by `coefficients` in `x`.
    // `coefficients` in low-to-high order.
    function horner_eval(uint256[] memory coefficients, uint256 x) internal pure returns (uint256 result) {
        // Assembly implementation of Horner evaluation for performance reasons.
        // This is a function in the hot-path and we want to avoid bounds checks
        // on the coefficients array.
        // prettier-ignore
        // We assume coefficients is stored in length-prefixed form.
        // See <https://solidity.readthedocs.io/en/v0.6.6/assembly.html#conventions-in-solidity>
        assembly {
            result := 0
            let modulus := MODULUS
            let length := mload(coefficients)
            if length {
                // Compute start and end of the coefficient array
                let start := add(coefficients, 0x20)
                let end := add(start, shl(5, length))
                // Index pointer start at the last value.
                let index := sub(end, 0x20)
                // Eight times unrolled loop
                for {} gt(length, 8) {} {
                    result := mulmod(result, x, MODULUS)
                    result := add(result, mload(index))
                    index := sub(index, 0x20)
                    result := mulmod(result, x, MODULUS)
                    result := add(result, mload(index))
                    index := sub(index, 0x20)
                    result := mulmod(result, x, MODULUS)
                    result := add(result, mload(index))
                    index := sub(index, 0x20)
                    result := mulmod(result, x, MODULUS)
                    result := add(result, mload(index))
                    index := sub(index, 0x20)
                    result := mulmod(result, x, MODULUS)
                    result := add(result, mload(index))
                    index := sub(index, 0x20)
                    result := mulmod(result, x, MODULUS)
                    result := add(result, mload(index))
                    index := sub(index, 0x20)
                    result := mulmod(result, x, MODULUS)
                    result := add(result, mload(index))
                    index := sub(index, 0x20)
                    result := mulmod(result, x, MODULUS)
                    result := add(result, mload(index))
                    index := sub(index, 0x20)
                    length := sub(length, 8)
                }
                // Base loop
                // The `add` can not overflow because modulus is less than 2^255.
                // The next `mulmod` will handle the reduction.
                for {} gt(index, start) {} {
                    result := mulmod(result, x, MODULUS)
                    result := add(result, mload(index))
                    index := sub(index, 0x20)
                }
                // Last value, need to use `addmod` here so final result is
                // reduced.
                result := mulmod(result, x, MODULUS)
                result := addmod(result, mload(start), modulus)
            }
        }
    }

    // The EvalX struct will lookup powers of x inside of the eval domain
    // It simplifies the interface, and can be made much more gas efficent
    struct EvalX {
        uint256 eval_domain_generator;
        uint8 log_eval_domain_size;
        uint64 eval_domain_size;
    }

    // TODO - Remove this
    // Solidity won't let libraries inherit, and we depend on libary syntax
    // but also on trace not bieng a libary so it's not possible to make
    // the primefield trace compatible without refactors, so we repeat code
    event LogTrace(bytes32 name, bool enter, uint256 gasLeft, uint256 allocated);
    modifier trace_mod(bytes32 name) {
        trace(name, true);
        _;
        trace(name, false);
    }

    function trace(bytes32 name, bool enter) internal {
        uint256 gas_left = gasleft();
        uint256 allocated = 0;
        assembly {
            allocated := mload(0x40)
        }
        emit LogTrace(name, enter, gas_left, allocated);
    }

    // Lookup data at an index
    function lookup(EvalX memory eval_x, uint256 index) internal trace_mod('eval_x_lookup') returns (uint256) {
        return fpow(eval_x.eval_domain_generator, index);
    }

    // Returns a memory object which allows lookups
    function init_eval(uint8 log_eval_domain_size) internal returns (EvalX memory) {
        return
            EvalX(
                PrimeField.generator_power(log_eval_domain_size),
                log_eval_domain_size,
                uint64(2)**(log_eval_domain_size)
            );
    }

    //     uint256[] memory batch_out = new uint256[](batch_in.length);
    // uint256 carried = 1;
    // uint256 pre_stored_len = batch_in.length;
    // for (uint256 i = 0; i < pre_stored_len; ) {
    //     carried = mulmod(carried, batch_in[i], PrimeField.MODULUS);
    //     batch_out[i] = carried;
    //     assembly {
    //         i := add(i, 1)
    //     }
    // }

    // uint256 inv_prod = carried.inverse();

    // for (uint256 i = batch_out.length - 1; i > 0; ) {
    //     batch_out[i] = mulmod(inv_prod, batch_out[i - 1], PrimeField.MODULUS);
    //     inv_prod = inv_prod.fmul(batch_in[i]);
    //     assembly {
    //         i := sub(i, 1)
    //     }
    // }
    // batch_out[0] = inv_prod;

    uint256 constant MODULUS_SUB_2 = 0x0800000000000010ffffffffffffffffffffffffffffffffffffffffffffffff;

    // This is a pure assembly optiomized version of a batch inversion
    // If the batch inversion input data array contains a zero, the batch
    // inversion will fail.
    // TODO - Inplace version/ version without output array?
    function batch_invert(uint256[] memory input_data, uint256[] memory output_data) internal returns(uint256 result) {
        require(input_data.length == output_data.length);

        assembly {
            // Uses the fact that data^p = data => data^(p-2) * data = 1
            // to calculate the multiplicative inverse in the field
            function invert(data) -> invert_result {
                let p := mload(0x40)
                mstore(p, 0x20) // Length of Base
                mstore(add(p, 0x20), 0x20) // Length of Exponent
                mstore(add(p, 0x40), 0x20) // Length of Modulus
                mstore(add(p, 0x60), data) // Base
                mstore(add(p, 0x80), MODULUS_SUB_2) // Exponent
                mstore(add(p, 0xa0), MODULUS) // Modulus
                // call modexp precompile
                if iszero(call(not(0), 0x05, 0, p, 0xc0, p, 0x20)) {
                    revert(0, 0)
                }
                invert_result := mload(p)
            }

            let carried := 1

            // This local copy of pointers to data
            // will be manipulated instead of the real thing
            let in_pointer := add(input_data, 32)
            // Note - we don't keep a copy of the output pointer
            // intead we keep the diffrence between the memory
            // arrays and use that to adjust the local pointer.
            // This works no matter memory layout because of the
            // modularity of evm additon
            // TODO - does this dif method actually save anything?
            let out_dif := sub(output_data, input_data)

            // The end bound of the following loop is when it's
            // 32*len past the data pointer
            let final_pointer := add(in_pointer, mul(mload(input_data), 32))

            // We interate on the pointer by moving forward
            // a word at a time and then checking we aren't
            // beyond the final pointer.
            for {} lt(in_pointer, final_pointer) {in_pointer := add(in_pointer, 32)} {
                // We want to get the product of all of the previous
                // elements into each slot of output data
                carried := mulmod(carried, mload(in_pointer), MODULUS)
                // Using the outdif we store into the output array
                mstore(add(out_dif, in_pointer), carried)
            }

            // Invert the product of all of the numbers
            carried := invert(carried)
            // At this point the in_pointer is beyond the data
            // So we move it back by one word.
            in_pointer := sub(in_pointer, 32)
            // We want to break when our in pointer points to
            // the very first data slot
            final_pointer := add(input_data, 32)
            // We now move backwards through the input data array
            for {} gt(in_pointer, final_pointer) {in_pointer := sub(in_pointer, 32)} {
                // Get out output pointer from the in pointer
                let out_pointer := add(in_pointer, out_dif)
                // Load a data slot before out pointer
                let out_data_i_minus_1 := mload(sub(out_pointer, 32))
                // Mul the cumulative inverse with the cummulative product
                // from a step before to get the ith inverse
                let ith_inverse := mulmod(carried, out_data_i_minus_1, MODULUS)
                // Store that ith inverse
                mstore(out_pointer, ith_inverse)
                // Update the cumulative product
                carried := mulmod(carried, mload(in_pointer), MODULUS)
            }
            // We increment down to but don't set out[0]
            // in the loop, so we set that here.
            mstore(add(output_data, 32), carried)
        }
    }
}
