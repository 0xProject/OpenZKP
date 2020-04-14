pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import './interfaces/ConstraintInterface.sol';
import './public_coin.sol';
import './proof_of_work.sol';
import './fri.sol';
import './proof_types.sol';


contract StarkVerifier is ProofOfWork, Fri, ProofTypes {
    using PublicCoin for PublicCoin.Coin;

    // TODO - Figure out why making this external causes 'UnimplementedFeatureError' only when
    // it calls through to an internal function with proof as memory.
    function verify_proof(StarkProof memory proof, ConstraintSystem constraints) public returns (bool) {
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
        // Get the actual polynomial points which were commited too, and the inverses of the x_points where they were evaluated
        (bytes32[] memory fri_top_layer, bytes32[] memory x_inv_vals) = constraints.calculate_commited_polynomial_points(proof, constraint_parameters, queries, oods_point, constraint_coeffiencents);

        uint8 log_eval_domain_size = constraint_parameters.log_trace_length + constraint_parameters.log_blowup;

        fri_layers(
            proof,
            constraint_parameters.fri_layout,
            x_inv_vals,
            eval_points,
            log_eval_domain_size,
            queries,
            fri_top_layer
        );
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
