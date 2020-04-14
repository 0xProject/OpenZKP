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
        uint8[] memory fri_layout = constraint_parameters.fri_layout;

        // This hack is horific and shoulnd't exist
        // uint256[] memory ptrs = new uint256[](7);
        {
            stack_hack_encode(proof,
                              fri_layout,
                              x_inv_vals,
                              eval_points,
                              queries);
        }

    }

    function stack_depth_fri_hack(uint256[7] memory pointers, uint8 log_eval_domain_size, bytes32[] memory fri_top_layer) internal {
        bytes32[][] memory fri_values;
        uint256 ptr = pointers[0];
        assembly {
            fri_values := ptr
        }
        bytes32[] memory fri_commitments;
        ptr = pointers[1];
        assembly {
            fri_commitments := ptr
        }
        bytes32[][] memory fri_decommitments;
        ptr = pointers[2];
        assembly {
            fri_decommitments := ptr
        }
        uint8[] memory fri_layout;
        ptr = pointers[3];
        assembly {
            fri_layout := ptr
        }
        bytes32[] memory x_inv_vals;
        ptr = pointers[4];
        assembly {
            x_inv_vals := ptr
        }
        bytes32[] memory eval_points;
        ptr = pointers[5];
        assembly {
            eval_points := ptr
        }
        uint64[] memory queries;
        ptr = pointers[6];
        assembly {
            queries := ptr
        }

        fold_and_check_fri_layers(
            fri_values,
            fri_commitments,
            fri_decommitments,
            fri_layout,
            x_inv_vals,
            eval_points,
            log_eval_domain_size,
            queries,
            fri_top_layer
        );
    }

    // This function forces the solidity complier to write the pointers to memory
    // It then returns their memory location
    function stack_hack_encode(ProofTypes.StarkProof memory proof,
        uint8[] memory fri_layout,
        bytes32[] memory x_inv_vals,
        bytes32[] memory eval_points,
        uint64[] memory queries) internal returns(uint256[] memory result) {

        uint256 ptr;
        bytes32[][] memory unwraped_reference = proof.fri_values;
        assembly {
            ptr := unwraped_reference
        }
        result[0] = ptr;

        bytes32[] memory unwraped_reference_2 = proof.fri_commitments;
        assembly {
            ptr := unwraped_reference_2
        }
        result[1] = ptr;

        unwraped_reference = proof.fri_decommitments;
        assembly {
            ptr := unwraped_reference
        }
        result[2] = ptr;

        assembly {
            ptr := fri_layout
        }
        result[3] = ptr;

        assembly {
            ptr := x_inv_vals
        }
        result[4] = ptr;

        assembly {
            ptr := eval_points
        }
        result[5] = ptr;

        assembly {
            ptr := queries
        }
        result[6] = ptr;
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
