pragma solidity 0.6.4;
pragma experimental ABIEncoderV2;

import './public_coin.sol';
import './iterator.sol';
import './primefield.sol';
import './merkle.sol';
import './proof_types.sol';
import './utils.sol';
import './trace.sol';

import '@nomiclabs/buidler/console.sol';


contract Fri is Trace, MerkleVerifier {
    using PublicCoin for PublicCoin.Coin;
    using Iterators for Iterators.IteratorUint;
    using PrimeField for uint256;
    using PrimeField for PrimeField.EvalX;
    using PrimeField for uint256[];
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
        uint256[] last_layer_coefficients;
    }

    struct LayerContext {
        uint64 coset_size;
        uint64 step;
        uint64 len;
    }

    // First three bit reversered inverted eight order roots of unity
    // 1 / omega_8^2 = omega_8^6
    uint256 constant ROOT1 = 0x01dafdc6d65d66b5accedf99bcd607383ad971a9537cdf25d59e99d90becc81e;
    // 1 / omega_8^1 = omega_8^7
    uint256 constant ROOT2 = 0x0446ed3ce295dda2b5ea677394813e6eab8bfbc55397aacac8e6df6f4bc9ca34;
    // 1 / omega_8^3 = omega_8^5
    uint256 constant ROOT3 = 0x01cc9a01f2178b3736f524e1d06398916739deaa1bbed178c525a1e211901146;

    // Reads from channel random and returns a list of random queries
    function get_queries(PublicCoin.Coin memory coin, uint8 max_bit_length, uint8 num_queries)
        internal
        view
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
        uint256[] memory polynomial_at_queries
    ) internal {
        trace('fri_check', true);
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
                proof.last_layer_coefficients
            )
        );
        trace('fri_check', false);
    }

    // This function takes in fri values, decommitments, and layout and checks the folding and merkle proofs
    // Note the final layer folded values will be overwritten to the input data locations.
    function fold_and_check_fri_layers(FriContext memory fri_data) internal {
        trace('fold_and_check_fri_layers', true);
        PrimeField.EvalX memory eval = PrimeField.init_eval(fri_data.log_eval_domain_size);
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
            // Since these two arrays only truncate we can safely resize them
            if (fri_data.queries.length != merkle_indices.length) {
                uint256 num_queries = fri_data.queries.length;
                merkle_indices.truncate(num_queries);
                merkle_val.truncate(num_queries);
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
        uint256 interp_root = eval.lookup(eval.eval_domain_size / layer_context.len);

        // We now test that the commited last layer values interpolate the final fri folding values
        trace('last_layer', true);
        for (uint256 i = 0; i < fri_data.polynomial_at_queries.length; i++) {
            uint8 layer_num_bits = layer_context.len.num_bits();
            uint256 reversed_query = fri_data.queries[i].bit_reverse(layer_num_bits);
            uint256 x = interp_root.fpow(reversed_query);
            uint256 calculated = fri_data.last_layer_coefficients.horner_eval(x);
            require(calculated == fri_data.polynomial_at_queries[i], 'Last layer coeffients mismatch');
        }
        trace('last_layer', false);
        trace('fold_and_check_fri_layers', false);
    }

    // This function takes in a previous layer and fold and reads from it and writes new folded layers to the next layer.
    // It will overwrite any memory in that location.
    function fold_layer(
        uint256[] memory previous_layer,
        uint64[] memory previous_indicies,
        Iterators.IteratorUint memory extra_coset_data,
        PrimeField.EvalX memory eval_x,
        uint256 eval_point,
        LayerContext memory layer_context,
        bytes32[] memory coset_hash_output
    ) internal {
        trace('fold_layer', true);
        // Convert eval_point out of montgomery form
        eval_point = eval_point.from_montgomery();

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
            coset_hash_output[writes] = merkle_leaf_hash(next_coset);
            // Do the actual fold and write it to the next layer
            previous_layer[writes] = fold_coset(next_coset, eval_point, layer_context, min_coset_index / 2, eval_x);
            // Record the new index
            previous_indicies[writes] = uint64(min_coset_index / layer_context.coset_size);
            writes++;
        }
        previous_layer.truncate(writes);
        previous_indicies.truncate(writes);
        trace('fold_layer', false);
    }

    function fold_coset(
        uint256[] memory coset,
        uint256 eval_point,
        LayerContext memory layer_context,
        uint64 index,
        PrimeField.EvalX memory eval_x
    ) internal returns (uint256) {
        trace('fold_coset', true);

        uint256 domain_size = layer_context.len;
        uint8 log_domain_size = uint64(domain_size).num_bits();
        uint256 coset_size = coset.length;
        uint256 generator = eval_x.eval_domain_generator.fpow(layer_context.step);
        uint256 x0_inv = generator.fpow(domain_size - (index + 0).bit_reverse2(log_domain_size - 1));
        fold_coset_inner(coset, x0_inv, eval_point);

        // We return the fri folded point and the inverse for the base layer, which is our x_inv on the next level
        trace('fold_coset', false);
        return (coset[0]);
    }

    // Gas: 4888441
    // Gas: 4878902
    // Gas: 4726841
    // Gas: 4652249
    // Gas: 4577169
    function fold_coset_inner(uint256[] memory coset, uint256 x0_inv, uint256 eval_point) internal returns (uint256) {
        trace('fold_coset_inner', true);
        uint256 coset_size = coset.length;
        if (coset_size == 8) {
            coset[0] = fold(coset[0], coset[1], eval_point, x0_inv);
            coset[1] = fold(coset[2], coset[3], eval_point, x0_inv.fmul(ROOT1));
            coset[2] = fold(coset[4], coset[5], eval_point, x0_inv.fmul(ROOT2));
            coset[3] = fold(coset[6], coset[7], eval_point, x0_inv.fmul(ROOT3));
            coset_size = 4;
            x0_inv = x0_inv.fmul(x0_inv);
            eval_point = eval_point.fmul(eval_point);
        }
        if (coset_size == 4) {
            coset[0] = fold(coset[0], coset[1], eval_point, x0_inv);
            coset[1] = fold(coset[2], coset[3], eval_point, x0_inv.fmul(ROOT1));
            coset_size = 2;
            x0_inv = x0_inv.fmul(x0_inv);
            eval_point = eval_point.fmul(eval_point);
        }
        if (coset_size == 2) {
            coset[0] = fold(coset[0], coset[1], x0_inv, eval_point);
        }
        trace('fold_coset_inner', false);
        return coset[0];
    }

    // We now do the actual fri folding operation
    // f'(x) = (f(x) + f(-x)) + eval_point / x * (f(x) - f(-x))
    function fold(uint256 positive, uint256 negative, uint256 eval_point, uint256 x_inv) internal returns (uint256) {
        // even = f(x) + f(-x)  (without reduction)
        uint256 even = positive + negative;
        // odd = f(x) - f(-x)   (without reduction)
        uint256 odd = positive + PrimeField.MODULUS - negative;
        // factor = eval_point / x
        uint256 factor = eval_point.fmul(x_inv);
        // result = even + factor * odd
        return even.fadd(factor.fmul(odd));
    }
}
