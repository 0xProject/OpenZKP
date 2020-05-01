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
    function expmod(uint256 base, uint256 exponent, uint256 modulus) internal returns (uint256 result) {
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

    // Lookup data at an index
    // These lookups cost around 530k of gas overhead in the small fib proof
    function lookup(EvalX memory eval_x, uint256 index) internal returns (uint256) {
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
}
