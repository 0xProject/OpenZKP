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

    // We assume that the coeffients are in montgomery form, but that x is not
    function horner_eval(uint256[] memory coefficients, uint256 x) internal pure returns (uint256 result) {
        assembly {
            result := 0
            let len := mload(coefficients)
            if len {
                let start := add(coefficients, 0x20)
                let end := add(start, mul(len, 0x20))
                for {
                    let index := sub(end, 0x20)
                } gt(index, start) {
                    index := sub(index, 0x20)
                } {
                    result := mulmod(result, x, MODULUS)
                    result := add(result, mload(index))
                }
                result := mulmod(result, x, MODULUS)
                result := addmod(result, mload(start), MODULUS)
            }
        }

        /*
        for (uint256 i = coefficients.length - 1; i > 0; i--) {
            result = fadd(coefficients[i], fmul(b, x));
        }
        result = fadd(coefficients[0], fmul(b, x));
        */
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
