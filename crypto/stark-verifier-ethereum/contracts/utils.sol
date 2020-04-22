pragma solidity ^0.6.4;


library Utils {
    function bit_reverse(uint64 num, uint8 number_of_bits) internal pure returns (uint256 num_reversed) {
        uint64 n = num;
        uint64 r = 0;
        for (uint8 k = 0; k < number_of_bits; k++) {
            r = (r * 2) | (n % 2);
            n = n / 2;
        }
        return r;
    }

    function num_bits(uint64 data) internal pure returns (uint8) {
        uint8 result = 0;
        if (data >= (1 << 32)) {
            result += 32;
            data >>= 32;
        }
        if (data >= (1 << 16)) {
            result += 16;
            data >>= 16;
        }
        if (data >= (1 << 8)) {
            result += 8;
            data >>= 8;
        }
        if (data >= (1 << 4)) {
            result += 4;
            data >>= 4;
        }
        if (data >= (1 << 2)) {
            result += 2;
            data >>= 2;
        }
        if (data >= 2) {
            result += 1;
            data >>= 1;
        }
        return result + uint8(data) - 1;
    }

    function deep_copy(bytes32[] memory a, bytes32[] memory b) internal pure {
        for (uint256 i = 0; i < a.length; i++) {
            b[i] = a[i];
        }
    }

    function deep_copy_and_convert(uint64[] memory a, uint256[] memory b) internal pure {
        for (uint256 i = 0; i < a.length; i++) {
            b[i] = a[i];
        }
    }

    // This function sorts the array
    // Note - We use insertion sort, the array is expected to be small so this shouldn't
    // cause problems.
    function sort(uint64[] memory data) internal pure {
        for (uint256 i = 0; i < data.length; i++) {
            uint256 j = i;
            while (j > 0 && data[j] < data[j - 1]) {
                (data[j], data[j - 1]) = (data[j - 1], data[j]);
                j--;
            }
        }
    }

    // The following functions resize a memory array by reseting the
    // first element of the array in memory, which as per this documentation
    // https://solidity.readthedocs.io/en/v0.6.4/assembly.html#conventions-in-solidity
    // is the place where the length is stored.
    // It will revert if the method is called in a way which would expand memory
    // because that would likely cause memory corruption.
    // ⚠️ WARNING ⚠️ - This method is not guaranteed to work and
    // any changes should be carefully considered ☢️ ☢️💥💥☢️ ☢️
    function truncate(bytes32[] memory data, uint256 to_len) internal pure {
        require(data.length >= to_len, 'Shrink Failed');
        assembly {
            mstore(data, to_len)
        }
    }

    // Type alias of the above function
    function truncate(uint256[] memory data, uint256 to_len) internal pure {
        require(data.length >= to_len, 'Shrink Failed');
        assembly {
            mstore(data, to_len)
        }
    }

    // Type alias of the above function
    function truncate(uint64[] memory data, uint256 to_len) internal pure {
        require(data.length >= to_len, 'Shrink Failed');
        assembly {
            mstore(data, to_len)
        }
    }
}
