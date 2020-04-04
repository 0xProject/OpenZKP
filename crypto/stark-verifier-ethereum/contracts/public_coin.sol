pragma solidity ^0.6.4;


library PublicCoin {
    struct Coin {
        bytes32 digest;
        uint64 counter;
    }

    // Takes bytes to be written to the channel and writes them to the coin,
    // Note that because this is a memory refrence this updates the coin.
    function write_bytes32(Coin memory coin, bytes32 to_be_written) internal pure {
        bytes32 hashed = publicCoinHash(coin.digest, to_be_written);
        coin.counter = 0;
        coin.digest = hashed;
    }

    // Writes a list of bytes32 with each bytes32 written individually
    function write_many_bytes32(Coin memory coin, bytes32[] memory to_be_written) internal pure {
        for (uint256 i = 0; i < to_be_written.length; i++) {
            write_bytes32(coin, to_be_written[i]);
        }
    }

    // Flexible method to write a byte string
    function write_bytes(Coin memory coin, bytes memory to_be_written) internal pure {
        bytes32 new_hash = publicCoinHasher(abi.encodePacked(coin.digest, to_be_written));
        coin.digest = new_hash;
        coin.counter = 0;
    }

    // Uses the digest and counter of the coin to create a random number
    // Note that because this is a memory refrence this updates the coin.
    function read_bytes32(Coin memory coin) internal pure returns (bytes32) {
        bytes32 hashed = publicCoinHash(coin.digest, bytes32(uint256(coin.counter)));
        coin.counter++;
        return hashed;
    }

    // Bulk Read, reads 'how_many' times
    function read_many_bytes32(Coin memory coin, uint256 how_many) internal pure returns (bytes32[] memory) {
        bytes32[] memory result = new bytes32[](how_many);
        for (uint256 i = 0; i < how_many; i++) {
            result[i] = read_bytes32(coin);
        }
    }

    // Adds a new coin with the starting bytes of data hashed to form the digest.
    function init_coin(bytes memory starting_data) internal pure returns (Coin memory) {
        bytes32 hashed = publicCoinHasher(starting_data);
        Coin memory new_coin;
        new_coin.digest = hashed;
        return new_coin;
    }

    function publicCoinHash(bytes32 preimage_a, bytes32 preimage_b) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(preimage_a, preimage_b));
    }

    function publicCoinHasher(bytes memory data) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(data));
    }
}
