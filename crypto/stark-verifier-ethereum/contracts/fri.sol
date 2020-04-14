pragma solidity 0.6.4;
pragma experimental ABIEncoderV2;

import './public_coin.sol';
import './iterator.sol';
import './primefield.sol';
import './merkle.sol';
import './proof_types.sol';

import '@nomiclabs/buidler/console.sol';

contract Fri is MerkleVerifier {
    using PublicCoin for PublicCoin.Coin;
    using Iterators for Iterators.IteratorBytes32;
    using PrimeField for *;

    struct FriContext {
        bytes32[][] fri_values;
        bytes32[] fri_commitments;
        bytes32[][] fri_decommitments;
        uint8[] fri_layout;
        bytes32[] x_inv_vals;
        bytes32[] eval_points;
        uint8 log_eval_domain_size;
        uint64[] queries;
        bytes32[] polynomial_at_queries;
    }

    struct LayerContext {
        uint64 coset_size;
        uint64 step;
        uint64 len;
    }

    // The Eval_X struct will lookup powers of x inside of the eval domain
    // It simplifies the interface, and can be made much more gas efficent
    // TODO - Move this into an x evaluator libary for style and interface
    struct Eval_X{
        uint256 eval_domain_generator;
        uint8 log_eval_domain_size;
        uint64 eval_domain_size;
    }
    // Lookup data at an index
    function lookup(Eval_X memory eval_x, uint256 index) internal returns(bytes32) {
        return (bytes32)(eval_x.eval_domain_generator.fpow(index));
    }

    // Returns a memory object which allows lookups
    function init_eval(uint8 log_eval_domain_size) internal returns(Eval_X memory) {
        return Eval_X(PrimeField.field_root(log_eval_domain_size), log_eval_domain_size, uint64(2)**(log_eval_domain_size));
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
        sort(queries);
        return queries;
    }

    // Unwraping endpoint because the main function has too deep of a stack otherwise
    function fri_layers(ProofTypes.StarkProof memory proof,
            uint8[] memory fri_layout,
            bytes32[] memory x_inv_vals,
            bytes32[] memory eval_points,
            uint8 log_eval_domain_size,
            uint64[] memory queries,
            bytes32[] memory polynomial_at_queries) internal {
                fold_and_check_fri_layers(FriContext(
                    proof.fri_values,
                    proof.fri_commitments,
                    proof.fri_decommitments,
                    fri_layout,
                    x_inv_vals,
                    eval_points,
                    log_eval_domain_size,
                    queries,
                    polynomial_at_queries
                ));
            }

    // This function takes in fri values, decommitments, and layout and checks the folding and merkle proofs
    // Note the final layer folded values will live in the
    function fold_and_check_fri_layers(
        FriContext memory fri_data
    ) public {
        Eval_X memory eval = init_eval(fri_data.log_eval_domain_size);
        LayerContext memory layer_context = LayerContext({
            len: uint64(2)**(fri_data.log_eval_domain_size),
            step: 1,
            coset_size: 0});
        uint256[] memory merkle_ind = new uint256[](fri_data.queries.length);
        bytes32[] memory merkle_val = new bytes32[](fri_data.queries.length);

        for(uint256 i = 0; i < fri_data.fri_layout.length; i ++) {
            layer_context.coset_size = uint64(2)**(fri_data.fri_layout[i]);
            // Overwrites and resizes the data array and the querry index array
            // They will contain the folded points and indexes
            // TODO - Doesn't change the x_invs, do they need to be raised to a power?
            fold_layer(
                fri_data.polynomial_at_queries,
                fri_data.queries,
                Iterators.init_iterator(fri_data.fri_values[i]),
                eval,
                fri_data.eval_points[i],
                layer_context,
                fri_data.x_inv_vals
            );
            // Merkle verification is in place but we need unchanged data in the next loop.
            deep_copy_and_convert(fri_data.queries, merkle_ind);
            deep_copy(fri_data.polynomial_at_queries, merkle_val);
            // Since these two arrays only shrink we can safely resize them
            if (fri_data.queries.length != merkle_ind.length) {
                uint256 num_queries = fri_data.queries.length;
                assembly {
                    mstore(merkle_ind, num_queries)
                    mstore(merkle_val, num_queries)
                }
            }
            // We now check that the folded indecies and values verify against thier decommitment
            require(verify_merkle_proof(
                fri_data.fri_commitments[i],
                merkle_val,
                merkle_ind,
                fri_data.fri_decommitments[i]
            ), "Fri merkle verification failed");
            layer_context.len /= uint64(layer_context.coset_size);
            layer_context.step *= uint64(layer_context.coset_size);
        }
    }

    // This function takes in a previous layer and fold and reads from it and writes new folded layers to the next layer.
    // It will overwrite any memory in that location.
    function fold_layer(
            bytes32[] memory previous_layer,
            uint64[] memory previous_indicies,
            Iterators.IteratorBytes32 memory extra_coset_data,
            Eval_X memory eval_x,
            bytes32 eval_point,
            LayerContext memory layer_context,
            bytes32[] memory x_inv_vals) internal {
        // Reads how many of the cosets we've read from
        uint256 writes = 0;
        uint64 current_index;
        bytes32[] memory next_coset = new bytes32[](layer_context.coset_size);
        uint256 i = 0;
        while (i < previous_layer.length) {
            current_index = previous_indicies[i];
            // Each coset length elements in the domain are one coset, so to find which one the current index is
            // we have to take it mod the length, to find the starting index we subtract the coset index from the
            // current one.
            uint64 min_coset_index = uint64((current_index) - (current_index%layer_context.coset_size));
            bytes32 x_inv_at_provided_index;
            uint256 x_inverse_coset_index;
            for(uint64 j = 0; j < layer_context.coset_size; j++) {
                // This check is if the current index is one which has data from the previous layer,
                // or if it's one with data provided in the proof
                if (current_index == j + min_coset_index) {
                    // Set this coset's data to the previous layer data at this index
                    next_coset[uint256(j)] = previous_layer[i];
                    x_inverse_coset_index = uint256(j);
                    x_inv_at_provided_index = x_inv_vals[i];
                    // Advance the index from the read
                    i++;
                    // Set the current index to the next one
                    current_index = previous_indicies[i];
                } else {
                    // This happens if the data isn't in the previous layer so we use our extra data.
                    next_coset[uint256(j)] = extra_coset_data.next();
                }
            }
            // Do the actual fold and write it to the next layer
            previous_layer[writes] = fold_coset(next_coset, eval_point, layer_context, current_index, eval_x);
            // Record the new index
            previous_indicies[writes] = uint64(min_coset_index/layer_context.coset_size);
            writes++;
        }
        // We need to manually resize the output arrays;
        assembly {
            mstore(previous_layer, writes)
            mstore(previous_indicies, writes)
        }
    }

    function fold_coset(
        bytes32[] memory coset,
        bytes32 eval_point,
        LayerContext memory layer_context,
        uint64 index,
        Eval_X memory eval_x
    ) internal returns(bytes32) {
        uint64 len = layer_context.len;
        uint64 step = layer_context.step;
        uint256 current_len = coset.length;
        while (current_len > 1) {
            for(uint i = 0; i < current_len; i += 2) {
                bytes32 x_inv = lookup(eval_x, (eval_x.eval_domain_size - bit_reverse(uint64(index + i/2), bits_in(len / 2)) * step)%eval_x.eval_domain_size);
                // f(x) + f(-x) + x_inv*eval_point*(f(x)-f(-x))
                coset[i/2] = coset[i].fadd(coset[i+1]).fadd(x_inv.fmul(eval_point).fmul(coset[i].fsub(coset[i+1])));
            }
            len /= 2;
            index /= 2;
            step *= 2;
            eval_point = eval_point.fmul(eval_point);
            current_len /= 2;
        }

        // We return the fri folded point and the inverse for the base layer, which is our x_inv on the next level
        return (coset[0]);
    }

    function bit_reverse(uint64 num, uint8 number_of_bits)
    internal view
        returns(uint256 num_reversed)
    {
        uint64 n = num;
        uint64 r = 0;
        for (uint8 k = 0; k < number_of_bits; k++) {
            r = (r * 2) | (n % 2);
            n = n / 2;
        }
        return r;
    }

    // TODO - redsign the functions to not need this and/or replace with effienct verions
    function bits_in(uint64 num) internal pure returns(uint8) {
        uint8 result = 0;
        while (num != 0) {
            result ++;
            num = num >> 1;
        }
        return result-1;
    }

    function deep_copy(bytes32[] memory a, bytes32[] memory b) internal pure {
        for (uint256 i = 0; i < a.length; i++) {
            b[i] = a[i];
        }
    }

    function deep_copy_and_convert(uint64[] memory a, uint256[] memory b) internal pure {
        for (uint256 i = 0; i < a.length; i++) {
            b[i] = a[i];
        }
    }

    // This function sorts the array
    // Note - We use insertion sort, the array is expected to be small so this shouldn't
    // cause problems.
    function sort(uint64[] memory data) internal pure {
        for (uint256 i = 0; i < data.length; i++) {
            uint256 j = i;
            while (j > 0 && data[j] < data[j - 1]) {
                (data[j], data[j - 1]) = (data[j - 1], data[j]);
                j--;
            }
        }
    }
}
