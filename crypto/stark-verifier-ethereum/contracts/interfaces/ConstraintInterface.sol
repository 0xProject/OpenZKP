pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../stark_verifier.sol';
import '../public_coin.sol';


interface ConstraintSystem {
    // The function should return a constraint paramters struct based on the public input.
    function initalize_system(bytes calldata public_input)
        external
        view
        returns (StarkVerifier.ProofParameters memory, PublicCoin.Coin memory);
}
