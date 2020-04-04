pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../stark_verifier.sol';


// These tests are intermeridary tests in the verification of the stark proof,
// if the functions change they will have to change too. Unfortunately, it is
// dificult to get access to data in any other way inside of a verify_proof call.
contract StarkDigestTesting is StarkVerifier {
    // Takes a proof and returns the coin digest after reading and writing
    function digest_read(StarkProof memory proof, ConstraintSystem constraints) public view returns (bytes32) {
        (ProofParameters memory constraint_parameters, PublicCoin.Coin memory coin) = constraints.initalize_system(
            proof.public_inputs
        );
        // Write data to the coin and read random data from it
        write_data_and_read_random(proof, constraint_parameters, coin);
        return coin.digest;
    }

    // Takes a proof and returns the queries after reading and writing
    function queries_read(StarkProof memory proof, ConstraintSystem constraints) public view returns (uint64[] memory) {
        (ProofParameters memory constraint_parameters, PublicCoin.Coin memory coin) = constraints.initalize_system(
            proof.public_inputs
        );
        // Write data to the coin and read random data from it
        write_data_and_read_random(proof, constraint_parameters, coin);
        // Preform the proof of work check
        require(check_proof_of_work(coin, proof.pow_nonce, constraint_parameters.pow_bits), 'POW Failed');
        // Read the query indecies from the coin
        uint8 eval_domain_log_size = constraint_parameters.log_trace_length + constraint_parameters.log_blowup;
        uint64[] memory queries = get_queries(coin, eval_domain_log_size, constraint_parameters.number_of_queries);
        return queries;
    }
}
