pragma solidity ^0.6.4;

import '../proof_of_work.sol';
import '../public_coin.sol';


contract ProofOfWorkTesting is ProofOfWork {
    function check_proof_of_work_external(bytes32 init_digest, bytes8 pow_nonce, uint8 pow_bits)
        external
        pure
        returns (bool)
    {
        PublicCoin.Coin memory coin = PublicCoin.Coin({digest: init_digest, counter: 0});
        return check_proof_of_work(coin, pow_nonce, pow_bits);
    }
}
