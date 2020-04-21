pragma solidity 0.6.4;
pragma experimental ABIEncoderV2;

import './public_coin.sol';
import './iterator.sol';
import './primefield.sol';
import './merkle.sol';
import './proof_types.sol';
import './utils.sol';

import '@nomiclabs/buidler/console.sol';


contract Fri is MerkleVerifier {
    using PublicCoin for PublicCoin.Coin;
    using Iterators for Iterators.IteratorUint;
    using PrimeField for uint256;
    using Utils for *;

    struct FriContext {
        uint256[][] fri_values;
        bytes32[] fri_commitments;
        bytes32[][] fri_decommitments;
        uint8[] fri_layout;
        uint256[] eval_points;
        uint8 log_eval_domain_size;
        uint64[] queries;
        uint256[] polynomial_at_queries;
        uint256[] last_layer_coeffiencts;
    }

    struct LayerContext {
        uint64 coset_size;
        uint64 step;
        uint64 len;
    }

    // The EvalX struct will lookup powers of x inside of the eval domain
    // It simplifies the interface, and can be made much more gas efficent
    // TODO - Move this into an x evaluator libary for style and interface
    struct EvalX {
        uint256 eval_domain_generator;
        uint8 log_eval_domain_size;
        uint64 eval_domain_size;
    }

    // Lookup data at an index
    // These lookups cost around 530k of gas overhead in the small fib proof
    function lookup(EvalX memory eval_x, uint256 index) internal returns (uint256) {
        return eval_x.eval_domain_generator.fpow(index);
    }

    // Returns a memory object which allows lookups
    function init_eval(uint8 log_eval_domain_size) internal returns (EvalX memory) {
        return
            EvalX(
                PrimeField.generator_power(log_eval_domain_size),
                log_eval_domain_size,
                uint64(2)**(log_eval_domain_size)
            );
    }

    // Reads from channel random and returns a list of random queries
    function get_queries(PublicCoin.Coin memory coin, uint8 max_bit_length, uint8 num_queries)
        internal
        pure
        returns (uint64[] memory)
    {
        uint64[] memory queries = new uint64[](num_queries);
        // This mask sets all digits to one below the bit length
        uint64 bit_mask = (uint64(2)**max_bit_length) - 1;

        // We derive four queries from each read
        for (uint256 i = 0; i <= num_queries / 4; i++) {
            bytes32 random = coin.read_bytes32();
            for (uint256 j = 0; j < 4; j++) {
                // For numbers of queries which are not diviable by four this prevents writing out of bounds.
                if (4 * i + j < num_queries) {
                    // Note - uint64(random) would take the last bytes in the random and this takes the first.
                    queries[4 * i + j] = uint64(bytes8(random)) & bit_mask;
                    // Shifts down so we can get the next set of random bytes
                    random <<= 64;
                }
            }
        }
        queries.sort();
        return queries;
    }

    // Unwraping endpoint because the main function has too deep of a stack otherwise
    function fri_check(
        ProofTypes.StarkProof memory proof,
        uint8[] memory fri_layout,
        uint256[] memory eval_points,
        uint8 log_eval_domain_size,
        uint64[] memory queries,
        uint256[] memory polynomial_at_queries,
        uint256 oods_point,
        uint256 evaluated_oods_point
    ) internal {
        fold_and_check_fri_layers(
            FriContext(
                proof.fri_values,
                proof.fri_commitments,
                proof.fri_decommitments,
                fri_layout,
                eval_points,
                log_eval_domain_size,
                queries,
                polynomial_at_queries,
                proof.last_layer_coeffiencts
            )
        );

        // The final check is that the constraints evaluated at the out of domain sample are
        // equal to the values commited constraint values
        uint256 result = 0;
        uint256 power = uint256(1).to_montgomery();
        for (uint256 i = 0; i < proof.constraint_oods_values.length; i++) {
            result = result.fadd(proof.constraint_oods_values[i].fmul_mont(power));
            power = power.fmul_mont(oods_point);
        }
        require(result == evaluated_oods_point, 'Oods mismatch');
    }

    // This function takes in fri values, decommitments, and layout and checks the folding and merkle proofs
    // Note the final layer folded values will be overwritten to the input data locations.
    function fold_and_check_fri_layers(FriContext memory fri_data) internal {
        EvalX memory eval = init_eval(fri_data.log_eval_domain_size);
        LayerContext memory layer_context = LayerContext({
            len: uint64(2)**(fri_data.log_eval_domain_size),
            step: 1,
            coset_size: 0
        });
        uint256[] memory merkle_indices = new uint256[](fri_data.queries.length);
        bytes32[] memory merkle_val = new bytes32[](fri_data.queries.length);

        for (uint256 i = 0; i < fri_data.fri_layout.length; i++) {
            layer_context.coset_size = uint64(2)**(fri_data.fri_layout[i]);
            // Overwrites and resizes the data array and the querry index array
            // They will contain the folded points and indexes
            fold_layer(
                fri_data.polynomial_at_queries,
                fri_data.queries,
                Iterators.init_iterator(fri_data.fri_values[i]),
                eval,
                fri_data.eval_points[i],
                layer_context,
                merkle_val
            );
            // Merkle verification is in place but we need unchanged data in the next loop.
            fri_data.queries.deep_copy_and_convert(merkle_indices);
            // Since these two arrays only shrink we can safely resize them
            if (fri_data.queries.length != merkle_indices.length) {
                uint256 num_queries = fri_data.queries.length;
                merkle_indices.shrink(num_queries);
                merkle_val.shrink(num_queries);
            }
            // TODO - Consider abstracting it up to a (depth, index) format like in the rust code.
            for (uint256 j = 0; j < merkle_indices.length; j++) {
                merkle_indices[j] += (layer_context.len / uint64(layer_context.coset_size));
            }
            // We now check that the folded indices and values verify against their decommitment
            require(
                verify_merkle_proof(
                    fri_data.fri_commitments[i],
                    merkle_val,
                    merkle_indices,
                    fri_data.fri_decommitments[i]
                ),
                'Fri merkle verification failed'
            );
            layer_context.len /= uint64(layer_context.coset_size);
            layer_context.step *= uint64(layer_context.coset_size);
        }

        // Looks up a root of unity in the final domain
        uint256 interp_root = lookup(eval, eval.eval_domain_size / layer_context.len);

        // We now test that the commited last layer values interpolate the final fri folding values
        for (uint256 i = 0; i < fri_data.polynomial_at_queries.length; i++) {
            uint256 x = interp_root.fpow(fri_data.queries[i].bit_reverse(layer_context.len.num_bits()));
            uint256 calculated = PrimeField.horner_eval(fri_data.last_layer_coeffiencts, x);
            require(calculated == fri_data.polynomial_at_queries[i], 'Last layer coeffients mismatch');
        }
    }

    // This function takes in a previous layer and fold and reads from it and writes new folded layers to the next layer.
    // It will overwrite any memory in that location.
    function fold_layer(
        uint256[] memory previous_layer,
        uint64[] memory previous_indicies,
        Iterators.IteratorUint memory extra_coset_data,
        EvalX memory eval_x,
        uint256 eval_point,
        LayerContext memory layer_context,
        bytes32[] memory coset_hash_output
    ) internal {
        // Reads how many of the cosets we've read from
        uint256 writes = 0;
        uint64 current_index;
        uint256[] memory next_coset = new uint256[](layer_context.coset_size);
        uint256 i = 0;
        while (i < previous_layer.length) {
            current_index = previous_indicies[i];
            // Each coset length elements in the domain are one coset, so to find which one the current index is
            // we have to take it mod the length, to find the starting index we subtract the coset index from the
            // current one.
            uint64 min_coset_index = uint64((current_index) - (current_index % layer_context.coset_size));
            for (uint64 j = 0; j < layer_context.coset_size; j++) {
                // This check is if the current index is one which has data from the previous layer,
                // or if it's one with data provided in the proof
                if (current_index == j + min_coset_index) {
                    // Set this coset's data to the previous layer data at this index
                    next_coset[uint256(j)] = previous_layer[i];
                    // Advance the index from the read
                    i++;
                    if (i < previous_indicies.length) {
                        // Set the current index to the next one
                        current_index = previous_indicies[i];
                    }
                } else {
                    // This happens if the data isn't in the previous layer so we use our extra data.
                    next_coset[uint256(j)] = extra_coset_data.next();
                }
            }
            // Hash the coset and store it so we can do a merkle proof against it
            coset_hash_output[writes] = merkleLeafHash(next_coset);
            // Do the actual fold and write it to the next layer
            previous_layer[writes] = fold_coset(next_coset, eval_point, layer_context, min_coset_index / 2, eval_x);
            // Record the new index
            previous_indicies[writes] = uint64(min_coset_index / layer_context.coset_size);
            writes++;
        }
        if (previous_layer.length > writes) {
            previous_layer.shrink(writes);
            previous_indicies.shrink(writes);
        }
    }

    function fold_coset(
        uint256[] memory coset,
        uint256 eval_point,
        LayerContext memory layer_context,
        uint64 index,
        EvalX memory eval_x
    ) internal returns (uint256) {
        // TODO - This could likely be one variable and the eval domain size in the layer context
        uint64 len = layer_context.len;
        uint64 step = layer_context.step;
        uint256 current_len = coset.length;
        while (current_len > 1) {
            for (uint256 i = 0; i < current_len; i += 2) {
                uint256 x_inv = lookup(
                    eval_x,
                    (eval_x.eval_domain_size - uint64(index + i / 2).bit_reverse((len / 2).num_bits()) * step) %
                        eval_x.eval_domain_size
                );
                coset[i / 2] = coset[i].fadd(coset[i + 1]).fadd(
                    x_inv.fmul(eval_point).fmul_mont(coset[i].fsub(coset[i + 1]))
                );
            }
            len /= 2;
            index /= 2;
            step *= 2;
            eval_point = eval_point.fmul_mont(eval_point);
            current_len /= 2;
        }

        // We return the fri folded point and the inverse for the base layer, which is our x_inv on the next level
        return (coset[0]);
    }
}
