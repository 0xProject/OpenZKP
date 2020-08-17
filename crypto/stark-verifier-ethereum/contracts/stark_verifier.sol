pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import './interfaces/ConstraintInterface.sol';
import './public_coin.sol';
import './proof_of_work.sol';
import './fri.sol';
import './proof_types.sol';
import './utils.sol';
import './trace.sol';

import '@nomiclabs/buidler/console.sol';


contract StarkVerifier is Trace, ProofOfWork, Fri, ProofTypes {
    using PublicCoin for PublicCoin.Coin;
    using Utils for *;

    // TODO - Figure out why making this external causes 'UnimplementedFeatureError' only when
    // it calls through to an internal function with proof as memory.
    function verify_proof(StarkProof memory proof, ConstraintSystem constraints) public returns (bool) {
        trace('verify_proof', true);

        // Initalize the coin and constraint system
        trace('initalize_system', true);
        (ProofParameters memory constraint_parameters, PublicCoin.Coin memory coin) = constraints.initalize_system(
            proof.public_inputs
        );
        trace('initalize_system', false);
        // Write data to the coin and read random data from it
        trace('write_data_and_read_random', true);
        (
            uint256[] memory constraint_coeffiencents,
            uint256 oods_point,
            uint256[] memory oods_coefficients,
            uint256[] memory eval_points
        ) = write_data_and_read_random(proof, constraint_parameters, coin);
        trace('write_data_and_read_random', false);
        // Preform the proof of work check
        require(check_proof_of_work(coin, proof.pow_nonce, constraint_parameters.pow_bits), 'POW Failed');
        // Read the query indices from the coin
        uint8 eval_domain_log_size = constraint_parameters.log_trace_length + constraint_parameters.log_blowup;
        trace('get_queries', true);
        uint256[] memory queries = get_queries(coin, eval_domain_log_size, constraint_parameters.number_of_queries);
        trace('get_queries', false);
        // Get the actual polynomial points which were commited too, and the inverses of the x_points where they were evaluated
        trace('constraint_calculations', true);
        OodsEvaluationData memory oods_data = OodsEvaluationData(
            proof.trace_values,
            proof.constraint_values,
            proof.trace_oods_values,
            proof.constraint_oods_values,
            constraint_parameters.log_trace_length,
            proof.public_inputs
        );
        (uint256[] memory fri_top_layer, uint256 constraint_evaluated_oods_point) = constraints.constraint_calculations(
            oods_data,
            queries,
            oods_point,
            constraint_coeffiencents,
            oods_coefficients
        );
        trace('constraint_calculations', false);

        uint8 log_eval_domain_size = constraint_parameters.log_trace_length + constraint_parameters.log_blowup;
        check_commitments(proof, constraint_parameters, queries, log_eval_domain_size);

        fri_check(proof, constraint_parameters.fri_layout, eval_points, log_eval_domain_size, queries, fri_top_layer);

        check_out_of_domain_sample_result(proof, oods_point, constraint_evaluated_oods_point);

        trace('verify_proof', false);
    }

    // This function write to the channel and reads from the channel to get the randomized data
    function write_data_and_read_random(
        StarkProof memory proof,
        ProofParameters memory constraint_parameters,
        PublicCoin.Coin memory coin
    )
        internal
        pure
        returns (
            uint256[] memory constraint_coeffiencents,
            uint256 oods_point,
            uint256[] memory oods_coefficients,
            uint256[] memory eval_points
        )
    {
        // Write the trace root to the coin
        coin.write_bytes32(proof.trace_commitment);
        // Read random constraint coefficentrs from the coin
        constraint_coeffiencents = coin.read_many_field_elements(2 * constraint_parameters.number_of_constraints);
        // Write the evaluated constraint root to the coin
        coin.write_bytes32(proof.constraint_commitment);
        // Read the oods point from the coin
        oods_point = coin.read_field_element();
        // Write the trace oods values to the coin
        coin.write_many_field_elements(proof.trace_oods_values);
        // Write the constraint oods values to the coin
        coin.write_many_field_elements(proof.constraint_oods_values);
        // Read the oods coeffients from the random coin
        oods_coefficients = coin.read_many_field_elements(
            proof.trace_oods_values.length + proof.constraint_oods_values.length
        );

        // Writes the fri merkle roots and reads eval points from the coin
        eval_points = new uint256[](constraint_parameters.fri_layout.length);
        for (uint256 i; i < constraint_parameters.fri_layout.length; i++) {
            coin.write_bytes32(proof.fri_commitments[i]);
            eval_points[i] = coin.read_field_element();
        }
        // Write the claimed last layer points a set of coeffient for the final layer fri check
        // NOTE - This is a fri layer so we have to write the whole thing at once
        // OPT: This creates several copies of the already large last layer coefficients.
        coin.write_layer(proof.last_layer_coefficients);

        return (constraint_coeffiencents, oods_point, oods_coefficients, eval_points);
    }

    // TODO - We can move the hashing abstraction into the merkle tree and avoid this extra allocation
    // Profiling - Apears to add around 900k gas! even ~600k with the optimizer on!
    function check_commitments(
        StarkProof memory proof,
        ProofParameters memory constraint_parameters,
        uint256[] memory queries,
        uint8 log_eval_domain_size
    ) internal {
        trace('check_commitments', true);

        bytes32[] memory merkle_hashes = new bytes32[](constraint_parameters.number_of_queries);
        uint256[] memory query_copy = new uint256[](queries.length);
        uint256 eval_domain_size = uint256(2)**(log_eval_domain_size);

        prepare_hashes_and_queries(
            proof.trace_values,
            uint256(constraint_parameters.number_of_columns),
            queries,
            eval_domain_size,
            merkle_hashes,
            query_copy
        );
        require(
            verify_merkle_proof(proof.trace_commitment, merkle_hashes, query_copy, proof.trace_decommitment),
            'Trace commitment proof failed'
        );

        prepare_hashes_and_queries(
            proof.constraint_values,
            uint256(constraint_parameters.constraint_degree),
            queries,
            eval_domain_size,
            merkle_hashes,
            query_copy
        );
        require(
            verify_merkle_proof(proof.constraint_commitment, merkle_hashes, query_copy, proof.constraint_decommitment),
            'Constraint commitment proof failed'
        );
        trace('check_commitments', false);
    }

    // Reads through the groups in the data and then hashes them and stores the hash in the output array
    // Also copies the queries into the output and adjusts them to merkle tree indexes.
    function prepare_hashes_and_queries(
        uint256[] memory data_groups,
        uint256 data_group_size,
        uint256[] memory queries,
        uint256 eval_domain_size,
        bytes32[] memory output_hashes,
        uint256[] memory output_queries
    ) internal {
        uint256[] memory group = new uint256[](data_group_size);
        for (uint256 i = 0; i < data_groups.length / data_group_size; i++) {
            for (uint256 j = 0; j < data_group_size; j++) {
                group[j] = data_groups[i * data_group_size + j];
            }
            output_hashes[i] = merkle_leaf_hash(group);
        }

        queries.deep_copy(output_queries);
        // TODO - Go to depth indexing in merkle to remove this
        for (uint256 i = 0; i < queries.length; i++) {
            output_queries[i] = output_queries[i] + eval_domain_size;
        }
        delete group;
    }

    function check_out_of_domain_sample_result(
        ProofTypes.StarkProof memory proof,
        uint256 oods_point,
        uint256 evaluated_oods_point
    ) internal {
        trace('check_out_of_domain_sample', true);
        // The final check is that the constraints evaluated at the out of domain sample are
        // equal to the values commited constraint values
        uint256 result = 0;
        uint256 power = uint256(1).to_montgomery();
        for (uint256 i = 0; i < proof.constraint_oods_values.length; i++) {
            uint256 oods_value_times_power = proof.constraint_oods_values[i].fmul_mont(power);
            result = result.fadd(oods_value_times_power);
            power = power.fmul_mont(oods_point);
        }
        require(result == evaluated_oods_point, 'Oods mismatch');
        trace('check_out_of_domain_sample', false);
    }
}
