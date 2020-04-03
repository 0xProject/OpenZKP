pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import './interfaces/ConstraintInterface.sol';
import './public_coin.sol';
import './proof_of_work.sol';
import './fri.sol';


contract StarkVerifier is ProofOfWork, Fri {
    using PublicCoin for PublicCoin.Coin;

    // This struct contains all of the componets of the STARK proof.
    // Please note that any input which would be a 'FieldElement' in rust,
    // should be the montgomery bytes encoded field element
    struct StarkProof {
        // An array with the public inputs to the STARK
        bytes public_inputs;
        // An array with the flatened trace table decommitments
        // For a trace table with n coloums it will be length num_queries*n
        // and it will be laid out as:
        // [[query 1 col 1][query 1 col 2]...[query 1 col n]]...[[query q col 1]...[query q col n]]
        bytes32[] trace_values;
        // The commitment to those trace values
        bytes32 trace_commitment;
        // The trace table evaluated constraint values at the the query indices.
        bytes32[] constraint_values;
        // The commitment to thos constraint values
        bytes32 constraint_commitment;
        // The trace values used for the oods point constraint evaluation
        bytes32[] trace_oods_values;
        // The constraint values used for the oods point constraint evaluation
        bytes32[] constraint_oods_values;
        // The nonce used for the proof of work
        bytes8 pow_nonce;
        // The merkle decomitment for the trace values
        bytes32[] trace_decommitment;
        // The merkle decomitment for the constraint evaluated queries
        bytes32[] constraint_decommitment;
        // The values to complete each coset of fri at each layer.
        bytes32[][] fri_values;
        // The roots for each fri decommitment
        bytes32[] fri_commitments;
        // The merkle proof decommitment at each fri layer
        bytes32[][] fri_decommitments;
        // The coeffiencts of the last fri layer
        bytes32[] last_layer_coeffiencts;
    }

    // This struct contains the relevent information about the constraint system
    // It will be returned from a callout to the constraint system contract.
    struct ProofParameters {
        uint8 number_of_columns;
        uint8 log_trace_length;
        uint64 number_of_constraints;
        uint8 log_blowup;
        uint8 constraint_degree;
        uint8 pow_bits;
        uint8 number_of_queries;
        // TODO - Does the smaller size give us a real advantage
        uint8[] fri_layout;
    }

    // TODO - Figure out why making this external causes 'UnimplementedFeatureError' only when
    // it calls through to an internal function with proof as memory.
    function verify_proof(StarkProof memory proof, ConstraintSystem constraints) public view returns (bool) {
        // Initalize the coin and constraint system
        (ProofParameters memory constraint_parameters, PublicCoin.Coin memory coin) = constraints.initalize_system(
            proof.public_inputs
        );
        // Write data to the coin and read random data from it
        (
            bytes32[] memory constraint_coeffiencents,
            bytes32 oods_point,
            bytes32[] memory oods_coefficients,
            bytes32[] memory eval_points
        ) = write_data_and_read_random(proof, constraint_parameters, coin);
        // Preform the proof of work check
        require(check_proof_of_work(coin, proof.pow_nonce, constraint_parameters.pow_bits), 'POW Failed');
        // Read the query indices from the coin
        uint8 eval_domain_log_size = constraint_parameters.log_trace_length + constraint_parameters.log_blowup;
        uint64[] memory queries = get_queries(coin, eval_domain_log_size, constraint_parameters.number_of_queries);
    }

    // This function write to the channel and reads from the channel to get the randomized data
    function write_data_and_read_random(
        StarkProof memory proof,
        ProofParameters memory constraint_parameters,
        PublicCoin.Coin memory coin
    ) internal pure returns (bytes32[] memory, bytes32, bytes32[] memory, bytes32[] memory) {
        // Write the trace root to the coin
        coin.write_bytes32(proof.trace_commitment);
        // Read random constraint coefficentrs from the coin
        bytes32[] memory constraint_coeffiencents = coin.read_many_bytes32(
            2 * constraint_parameters.number_of_constraints
        );
        // Write the evaluated constraint root to the coin
        coin.write_bytes32(proof.constraint_commitment);
        // Read the oods point from the coin
        bytes32 oods_point = coin.read_bytes32();
        // Write the trace oods values to the coin
        coin.write_many_bytes32(proof.trace_oods_values);
        // Write the constraint oods values to the coin
        coin.write_many_bytes32(proof.constraint_oods_values);
        // Read the oods coeffients from the random coin
        bytes32[] memory oods_coefficients = coin.read_many_bytes32(
            proof.trace_oods_values.length + proof.constraint_oods_values.length
        );

        // Writes the fri merkle roots and reads eval points from the coin
        bytes32[] memory eval_points = new bytes32[](constraint_parameters.fri_layout.length);
        for (uint256 i; i < constraint_parameters.fri_layout.length; i++) {
            coin.write_bytes32(proof.fri_commitments[i]);
            eval_points[i] = coin.read_bytes32();
        }
        // Write the claimed last layer points a set of coeffient for the final layer fri check
        // NOTE - This is a fri layer so we have to write the whole thing at once
        coin.write_bytes(abi.encodePacked(proof.last_layer_coeffiencts));
        return (constraint_coeffiencents, oods_point, oods_coefficients, eval_points);
    }
}
