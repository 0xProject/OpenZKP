pragma solidity 0.6.4;

import './public_coin.sol';


contract Fri {
    using PublicCoin for PublicCoin.Coin;

    // Reads from channel random and returns a list of random queries
    function get_queries(PublicCoin.Coin memory coin, uint8 max_bit_length, uint8 num_queries)
        internal
        pure
        returns (uint64[] memory)
    {
        uint64[] memory queries = new uint64[](num_queries);
        // This mask sets all digits to one below the bit length
        uint64 bit_mask = (uint64(2)**max_bit_length) - 1;

        // We derive four queries from each read
        for (uint256 i = 0; i <= num_queries / 4; i++) {
            bytes32 random = coin.read_bytes32();
            for (uint256 j = 0; j < 4; j++) {
                // For numbers of queries which are not diviable by four this prevents writing out of bounds.
                if (4 * i + j < num_queries) {
                    // Note - uint64(random) would take the last bytes in the random and this takes the first.
                    queries[4 * i + j] = uint64(bytes8(random)) & bit_mask;
                    // Shifts down so we can get the next set of random bytes
                    random <<= 64;
                }
            }
        }
        sort(queries);
        return queries;
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
}
