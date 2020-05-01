pragma solidity ^0.6.4;

import './trace.sol';


contract MerkleVerifier is Trace {
    bytes32 constant HASH_MASK = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF000000000000000000000000;

    // This function takes a set of data leaves and indices are 2^depth + leaf index and must be sorted in ascending order.
    // NOTE - An empty claim will revert
    function verify_merkle_proof(
        bytes32 root,
        bytes32[] memory leaves,
        uint256[] memory indices,
        bytes32[] memory decommitment
    ) internal returns (bool) {
        trace('verify_merkle_proof', true);
        require(leaves.length > 0, 'No claimed data');

        // Setup our ring buffer
        // Each next layer will be equally sized or smaller, so `write_index`
        // will never overrun `read_index`.
        uint256 read_index = 0;
        uint256 write_index = 0;

        // Setup our decommitment iterator
        uint256 decommitment_index = 0;

        while (true) {
            uint256 index = indices[read_index];
            bytes32 current_hash = leaves[read_index];
            read_index += 1;
            read_index %= leaves.length;

            // If the index is one this node is the root so we need to check if the proposed root matches
            if (index == 1) {
                bool valid = root == current_hash;
                trace('verify_merkle_proof', false);
                return valid;
            }

            // Check if the next available index is the right sibbling.
            // `index | 1` turns index it to the right sibbling (no-op if it already is)
            if (indices[read_index] == index | 1) {
                // We found the right neighbour, merge nodes
                indices[write_index] = index / 2;
                leaves[write_index] = merkle_tree_hash(current_hash, leaves[read_index]);
                read_index += 1;
                read_index %= leaves.length;
                write_index += 1;
                write_index %= leaves.length;
                continue;
            }

            // Next we try to read from the decommitment and use that info to push a new hash into the queue
            // If we don't have more decommitment the proof fails
            if (decommitment_index >= decommitment.length) {
                trace('verify_merkle_proof', false);
                return false;
            }

            // Reads from decommitment and pushes a new node
            bytes32 next_decommitment = decommitment[decommitment_index];
            if (index & 1 == 0) {
                // index is left
                leaves[write_index] = merkle_tree_hash(current_hash, next_decommitment);
            } else {
                // index is right
                leaves[write_index] = merkle_tree_hash(next_decommitment, current_hash);
            }
            indices[write_index] = index / 2;
            decommitment_index += 1;
            write_index += 1;
            write_index %= leaves.length;
        }
        assert(false); // Unreachable
    }

    function merkle_tree_hash(bytes32 preimage_a, bytes32 preimage_b) internal returns (bytes32 hash) {
        // Equivalent to
        // hash = keccak256(abi.encodePacked(preimage_a, preimage_b)) & HASH_MASK
        // Using assembly for performance
        assembly {
            // The first 64 bytes of memory are scratch space
            // See <https://solidity.readthedocs.io/en/v0.6.6/assembly.html#conventions-in-solidity>
            mstore(0x00, preimage_a)
            mstore(0x20, preimage_b)
            hash := and(keccak256(0x00, 0x40), HASH_MASK)
        }
    }

    function merkle_leaf_hash(uint256[] memory leaf) internal pure returns (bytes32 hash) {
        if (leaf.length == 1) {
            hash = (bytes32)(leaf[0]);
        } else {
            // Equivalent to
            // hash = keccak256(abi.encodePacked(leaf)) & HASH_MASK;
            // Using assembly for performance
            assembly {
                // Arrays are stored length-prefixed.
                // See <https://solidity.readthedocs.io/en/v0.6.6/assembly.html#conventions-in-solidity>
                let len := mload(leaf)
                hash := and(keccak256(add(leaf, 0x20), mul(len, 0x20)), HASH_MASK)
            }
        }
    }
}
