pragma solidity ^0.6.4;

import '../public_coin.sol';
import '@nomiclabs/buidler/console.sol';

contract PublicCoinTesting is PublicCoin {

    event log_bytes32(bytes32 data);

    // Reads a number of elements after initing to a starting value
    function init_and_read(bytes calldata starting_data, uint number) external {
        Coin memory coin = init_coin(starting_data);

        for (uint i = 0; i < number; i++) {
            bytes32 read = read_bytes32(coin);
            emit log_bytes32(read);
        }
    }

    // Creates a channel, writes to it and then returns the digest
    function init_and_write(bytes calldata starting_data, bytes32 data) external {
        Coin memory coin = init_coin(starting_data);
        write_bytes32(data, coin);
        emit log_bytes32(coin.digest);
    }
}