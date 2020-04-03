pragma solidity 0.6.4;

import './public_coin.sol';

contract ProofOfWork {
    using PublicCoin for PublicCoin.Coin;

    // Given a coin and a nonce hashes the random form the coin and checks that the proof of works passes
    // NOTE - This function also advances the coin by writing the pow_nonce to it
    function check_proof_of_work(PublicCoin.Coin memory coin, bytes8 pow_nonce, uint8 pow_bits)
        internal
        pure
        returns (bool)
    {
        bytes32 seed = keccak256(abi.encodePacked(hex'0123456789abcded', coin.digest, pow_bits));
        bytes32 response = keccak256(abi.encodePacked(seed, pow_nonce));
        coin.write_bytes(abi.encodePacked(pow_nonce));
        return (response & ~(bytes32)((uint256(1)<<(255-pow_bits)) - 1)) == 0x0;
    }
}
