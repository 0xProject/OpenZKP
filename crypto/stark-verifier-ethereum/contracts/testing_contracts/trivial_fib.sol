pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../interfaces/ConstraintInterface.sol';
import '../public_coin.sol';
import '../stark_verifier.sol';


// This trivial Fibonacci system returns constant values which are true only for one proof
// It should only be used for testing purposes
contract TrivialFib is ConstraintSystem {
    // These constants are derived from the small fib example in rust
    // TODO - The solidity prettier wants to delete all 'override' statements
    // We should remove this ignore statement when that changes.
    // prettier-ignore
    function initalize_system(bytes calldata public_input)
        external
        view
        override
        returns (StarkVerifier.ProofParameters memory, PublicCoin.Coin memory)
    {
        PublicCoin.Coin memory coin = PublicCoin.Coin({
            digest: 0xc891a11ddbc6c425fad523a7a4aeafa505d7aa1638cfffbd5b747100bc69e367,
            counter: 0
        });
        uint8[] memory fri_layout = new uint8[](3);
        fri_layout[0] = 3;
        fri_layout[1] = 3;
        fri_layout[2] = 2;

        StarkVerifier.ProofParameters memory params = StarkVerifier.ProofParameters({
            number_of_columns: 2,
            log_trace_length: 10,
            number_of_constraints: 4,
            log_blowup: 4,
            constraint_degree: 1,
            pow_bits: 10,
            number_of_queries: 20,
            fri_layout: fri_layout
        });

        return (params, coin);
    }
}
