pragma solidity ^0.6.4;

import '../public_coin.sol';


contract PublicCoinTesting {
    using PublicCoin for PublicCoin.Coin;
    event log_bytes32(bytes32 data);

    // Reads a number of elements after initing to a starting value
    function init_and_read(bytes calldata starting_data, uint256 number) external {
        PublicCoin.Coin memory coin = PublicCoin.init_coin(starting_data);

        for (uint256 i = 0; i < number; i++) {
            bytes32 read = coin.read_bytes32();
            emit log_bytes32(read);
        }
    }

    // Creates a channel, writes to it and then returns the digest
    function init_and_write(bytes calldata starting_data, bytes32 data) external {
        PublicCoin.Coin memory coin = PublicCoin.init_coin(starting_data);
        coin.write_bytes32(data);
        emit log_bytes32(coin.digest);
    }
}
