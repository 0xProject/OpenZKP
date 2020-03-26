pragma solidity ^0.6.4;


contract PublicCoin {
    struct Coin {
        bytes32 digest;
        uint64 counter;
    }

    // Takes bytes to be written to the channel and writes them to the coin,
    // Note that because this is a memory refrence this updates the coin.
    function write_bytes32(bytes32 to_be_written, Coin memory coin) internal pure {
        bytes32 hashed = publicCoinHash(coin.digest, to_be_written);
        coin.counter = 0;
        coin.digest = hashed;
    }

    // Uses the digest and counter of the coin to create a random number
    // Note that because this is a memory refrence this updates the coin.
    function read_bytes32(Coin memory coin) internal pure returns (bytes32) {
        bytes32 hashed = publicCoinHash(coin.digest, bytes32(uint256(coin.counter)));
        coin.counter++;
        return hashed;
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
