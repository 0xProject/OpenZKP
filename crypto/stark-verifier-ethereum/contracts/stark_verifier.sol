pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import './interfaces/ConstraintInterface.sol';
import './public_coin.sol';

contract StarkVerifier {
    using PublicCoin for PublicCoin.Coin;
    event log_bytes32(bytes32 data);

    // This struct contains all of the componets of the STARK proof.
    // Please note that any input which would be a 'FieldElement' in rust,
    // should be the montgomery bytes encoded field element
    struct StarkProof {
        // An array with the public inputs to the STARK
        bytes32[] public_inputs;
        // An array with the flatened trace table decommitments
        // For a trace table with n coloums it will be length num_queries*n
        // and it will be laid out as:
        // [[query 1 col 1][query 1 col 2]...[query 1 col n]]...[[query q col 1]...[query q col n]]
        bytes32[] trace_values;
        // The commitment to those trace values
        bytes32 trace_root;
        // The trace table evaluated constraint values at the the query indeicies.
        bytes32[] constraint_values;
        // The commitmnet to thos constraint values
        bytes32 constraint_root;
        // The trace values used for the oods point constraint evaluation
        bytes32[] trace_oods_values;
        // The constraint values used for the oods point constraint evaluation
        bytes32[] constraint_oods_values;
        // The nonce used for the proof of work
        bytes8 pow_nonce;
        // The merkle decomitment for the trace values
        bytes32[] trace_decommitment;
        // The merkle decomitment for the constraint evaluted queries
        bytes32[] constraint_decommitment;
        // The extra values needed at each fri layer
        bytes32[][] fri_values;
        // The roots for each fri decommitment
        bytes32[] fri_roots;
        // The merkle proof decommitment at each fri layer
        bytes32[][] fri_decommitments;
        // The coeffiencts of the last fri layer
        bytes32[] last_layer_coeffiencts;
    }

    // This struct contains the relevent information about the constraint system
    // It will be returned from a callout to the constraint system contract.
    struct ConstraintParameters {
        uint8  number_of_columns;
        uint8  log_trace_length;
        uint64 number_of_constraints;
        uint8  log_blowup;
        uint8  constraint_degree;
        uint8  pow_bits;
        uint8  number_of_queries;
        // TODO - Does the smaller size give us a real advantage
        uint8[] fri_layout;
    }

    // TODO - Figure out why making this external causes 'UnimplementedFeatureError' only when
    // it calls through to an internal function with proof as memory.
    function verify_proof(StarkProof memory proof, ConstraintSystem constraints) public returns(bool) {
        // Initalize the coin and constraint system
        (ConstraintParameters memory constraint_parameters, PublicCoin.Coin memory coin) = constraints.initalize_system(proof.public_inputs);
        // Write data to the coin and read random data from it
        (bytes32[] memory constraint_coeffiencents, bytes32 oods_point, bytes32[] memory oods_coefficients, bytes32[] memory eval_points) = write_data_and_read_random(proof, constraint_parameters, coin);
        // Preform the proof of work check
        require(check_proof_of_work(coin, proof.pow_nonce, constraint_parameters.pow_bits), "POW Failed");
        // Read the query indecies from the coin
        uint8 eval_domain_log_size = constraint_parameters.log_trace_length + constraint_parameters.log_blowup;
        uint64[] memory queries = get_queries(coin, eval_domain_log_size-1, constraint_parameters.number_of_queries);
    }

    // This function write to the channel and reads from the channel to get the randomized data
    function write_data_and_read_random(StarkProof memory proof, ConstraintParameters memory constraint_parameters, PublicCoin.Coin memory coin) internal pure returns (bytes32[] memory, bytes32, bytes32[] memory, bytes32[] memory) {
        // Write the trace root to the coin
        coin.write_bytes32(proof.trace_root);
        // Read random constraint coefficentrs from the coin
        bytes32[] memory constraint_coeffiencents = coin.read_many_bytes32(2*constraint_parameters.number_of_constraints);
        // Write the evaluated constraint root to the coin
        coin.write_bytes32(proof.constraint_root);
        // Read the oods point from the coin
        bytes32 oods_point = coin.read_bytes32();
        // Write the trace oods values to the coin
        coin.write_many_bytes32(proof.trace_oods_values);
        // Write the constraint oods values to the coin
        coin.write_many_bytes32(proof.constraint_oods_values);
        // Read the oods coeffients from the random coin
        bytes32[] memory oods_coefficients = coin.read_many_bytes32(proof.trace_oods_values.length + proof.constraint_oods_values.length);

        // Writes the fri merkle roots and reads eval points from the coin
        bytes32[] memory eval_points = new bytes32[](constraint_parameters.fri_layout.length);
        for (uint256 i; i < constraint_parameters.fri_layout.length; i++) {
            coin.write_bytes32(proof.fri_roots[i]);
            eval_points[i] = coin.read_bytes32();
        }
        // Write the claimed last layer points a set of coeffient for the final layer fri check
        // NOTE - This is a fri layer so we have to write the whole thing at once
        coin.write_bytes(abi.encodePacked(proof.last_layer_coeffiencts));
        return (constraint_coeffiencents, oods_point, oods_coefficients, eval_points);
    }

    // Given a coin and a nonce hashes the random form the coin and checks that the proof of works passes
    // NOTE - This function also advances the coin by writing the pow_nonce to it
    function check_proof_of_work(PublicCoin.Coin memory coin, bytes8 pow_nonce, uint8 pow_bits) internal pure returns(bool) {
        bytes32 rand_from_channel = coin.read_bytes32();
        bytes32 seed = keccak256(abi.encodePacked(hex"0123456789abcded", rand_from_channel, pow_bits));
        bytes32 response = keccak256(abi.encodePacked(seed, pow_nonce));
        uint8 leading_zeros = leading_zeros(response);
        coin.write_bytes(abi.encodePacked(pow_nonce));
        return leading_zeros >= pow_bits;
    }

    // Returns the number of leading zeros in a bytes32 data
    function leading_zeros(bytes32 data) internal pure returns(uint8) {
        // One set in the leading bit position
        bytes32 leading_one = 0x8000000000000000000000000000000000000000000000000000000000000000;
        uint8 result = 0;
        while (leading_one & data == 0) {
            leading_one >> 1;
            result++;
            // We don't want a revert if the zero is passed in
            if (result == 255) {
                return result;
            }
        }
        return result;
    }

    // Reads from channel random and returns a list of random queries
    function get_queries(PublicCoin.Coin memory coin, uint8 max_bit_length, uint8 num_queries) internal view returns(uint64[] memory) {
        uint64[] memory queries = new uint64[](num_queries);
        // This mask sets all digits to one below the bit length
        uint64 bit_mask = (uint64(2)**max_bit_length) - 1;

        // We derive four queries from each read
        for (uint256 i = 0; i <= num_queries/4; i ++) {
            bytes32 random = coin.read_bytes32();
            for (uint256 j = 0; j < 4; j ++) {
                // For numbers of queries which are not diviable by four this prevents writing out of bounds.
                if (4*i + j < num_queries) {
                    // Note - uint64(random) would take the last bytes in the random and this takes the first.
                    queries[4*i + j] = uint64(bytes8(random)) & bit_mask;
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
            while (j > 0 && data[j] < data[j-1]) {
                uint64 held = data[j];
                data[j] = data[j-1];
                data[j-1] = held;
                j--;
            }
        }
    }
}
