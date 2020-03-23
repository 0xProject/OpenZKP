pragma solidity ^0.6.4;

contract PrimeField {
    uint256 constant internal K_MODULUS = 0x800000000000011000000000000000000000000000000000000000000000001;
    uint256 constant internal K_MODULUS_MASK = 0x0fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
    uint256 constant internal K_MONTGOMERY_R = 0x7fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1;
    uint256 constant internal K_MONTGOMERY_R_INV = 0x40000000000001100000000000012100000000000000000000000000000000;
    uint256 constant internal GENERATOR = 3;
    uint256 constant internal ONE_VAL = 1;
    uint256 constant internal gen1024_val = 0x659d83946a03edd72406af6711825f5653d9e35dc125289a206c054ec89c4f1;

    function from_montgomery(uint256 val) internal pure returns (uint256 res) {
        assembly {
            res := mulmod(val, K_MONTGOMERY_R_INV,
                K_MODULUS)
        }
        return res;
    }

    function from_montgomery_bytes(bytes32 bs) internal pure returns (uint256) {
        /// Assuming bs is a 256bit bytes object, in montgomery form, it is read into a field
        /// element
        uint256 res = uint256(bs);
        return from_montgomery(res);
    }

    // This is an unchecked cast and should be used very carefully,
    // and only in cases when the data is already in the right form.
    function from_bytes_raw(bytes32 bs) internal pure returns(uint256 data) {
        assembly {
            data := bs
        }
    }

    // This is an unchecked cast and should be used very carefully,
    // and only in cases when the data is already in the right form.
    function from_bytes_array_raw(bytes32[] memory input) internal pure returns(uint256[] memory data) {
        assembly {
            data := input
        }
    }

    function to_montgomery_int(uint256 val) internal pure returns (uint256 res) {
        assembly {
            res := mulmod(val, K_MONTGOMERY_R,
                K_MODULUS)
        }
        return res;
    }

    function fmul(uint256 a, uint256 b) internal pure returns (uint256 res) {
        assembly {
            res := mulmod(a, b,
                K_MODULUS)
        }
        return res;
    }

    function fadd(uint256 a, uint256 b) internal pure returns (uint256 res) {
        assembly {
            res := addmod(a, b,
                K_MODULUS)
        }
        return res;
    }

    function fsub(uint256 a, uint256 b) internal pure returns (uint256 res) {
        assembly {
            res := addmod(a, sub(K_MODULUS, b),
                K_MODULUS)
        }
        return res;
    }

    function fdiv(uint256 a, uint256 b) internal returns (uint256) {
        // this function has not been optimized as we don't expect to use it due to batch inverse
        uint256 b_inv = inverse(b);
        uint256 res = fmul(a, b_inv);
        return res;
    }

    function fpow(uint256 val, uint256 exp) internal returns (uint256) {
        if (exp < 20) {
            return fpow2(val, exp);
        } else {
            return expmod(val, exp, K_MODULUS);
        }
    }

    function fpow2(uint256 val, uint256 exp) internal pure returns (uint256) {
        uint256 cur_pow = val;
        uint n = exp;
        uint256 res = 1;
        while (n > 0) {
            if ((n % 2) != 0) {
                res = fmul(res, cur_pow);
            }
            n = n / 2;
            cur_pow = fmul(cur_pow, cur_pow);
        }
        return res;
    }

    function expmod(uint256 base, uint256 exponent, uint256 modulus) internal returns (uint256 res) {
        assembly {
            let p := mload(0x40)
            mstore(p, 0x20)             // Length of Base
            mstore(add(p, 0x20), 0x20)  // Length of Exponent
            mstore(add(p, 0x40), 0x20)  // Length of Modulus
            mstore(add(p, 0x60), base)  // Base
            mstore(add(p, 0x80), exponent)   // Exponent
            mstore(add(p, 0xa0), modulus)   // Modulus
            // call modexp precompile
            if iszero(call(not(0), 0x05, 0, p, 0xc0, p, 0x20)) {
                revert(0, 0)
            }
            res := mload(p)
        }
    }

    function inverse(uint256 val) internal returns (uint256) {
        return expmod(val, K_MODULUS - 2, K_MODULUS);
    }
}
