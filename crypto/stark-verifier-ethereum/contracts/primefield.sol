pragma solidity ^0.6.4;


contract PrimeField {
    uint256 internal constant MODULUS = 0x800000000000011000000000000000000000000000000000000000000000001;
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

    function to_montgomery_int(uint256 value) internal pure returns (uint256) {
        return mulmod(value, MONTGOMERY_R, MODULUS);
    }

    function fmul(uint256 a, uint256 b) internal pure returns (uint256) {
        return mulmod(a, b, MODULUS);
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
        return expmod(value, MODULUS - 2, MODULUS);
    }
}
