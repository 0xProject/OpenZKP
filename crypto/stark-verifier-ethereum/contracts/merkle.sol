pragma solidity ^0.6.4;

import './ring_buffer.sol';
import './iterator.sol';
import './trace.sol';


contract MerkleVerifier is Trace {
    using RingBuffer for RingBuffer.IndexRingBuffer;
    using Iterators for Iterators.IteratorBytes32;

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

        // Setup our index buffer
        RingBuffer.IndexRingBuffer memory buffer = RingBuffer.IndexRingBuffer({
            front: 0,
            back: leaves.length - 1,
            indexes: indices,
            data: leaves,
            is_empty: false
        });

        // Setup our iterator
        Iterators.IteratorBytes32 memory decommitment_iter = Iterators.init_iterator(decommitment);

        while (true) {
            (uint256 index, bytes32 current_hash) = buffer.remove_front();

            // If the index is one this node is the root so we need to check if the proposed root matches
            if (index == 1) {
                bool valid = root == current_hash;
                trace('verify_merkle_proof', false);
                return valid;
            }
            bool is_left = index % 2 == 0;

            // If this is a left node then the node following it in the queue
            // may be a sibbling which we want to hash with it.
            if (is_left && buffer.has_next()) {
                (uint256 next_index, bytes32 next_hash) = buffer.peak_front();

                // This checks if the next index in the queue is the sibbling of this one
                // If it is we use the data, otherwise we try the decommitment queue
                if (next_index == index + 1) {
                    // This force increments the front, may consider real method for this.
                    (next_index, next_hash) = buffer.remove_front();

                    // Because index is even it is the left hash so to get the next one we do:
                    bytes32 new_hash = merkle_tree_hash(current_hash, next_hash);
                    buffer.add_to_rear(index / 2, new_hash);

                    // We indicate that a node was pushed, so that another won't be
                    continue;
                }
            }

            // Next we try to read from the decommitment and use that info to push a new hash into the queue
            // If we don't have more decommitment the proof fails
            if (!decommitment_iter.has_next()) {
                trace('verify_merkle_proof', false);
                return false;
            }

            // Reads from decommitment and pushes a new node
            trace('read_decommitment_and_push', true);
            bytes32 next_decommitment = decommitment_iter.next();
            bytes32 new_hash;
            // Preform the hash
            if (is_left) {
                new_hash = merkle_tree_hash(current_hash, next_decommitment);
            } else {
                new_hash = merkle_tree_hash(next_decommitment, current_hash);
            }
            // Add the new node to the buffer.
            // Note the buffer strictly shrinks in the algo so we can't overflow the size.
            buffer.add_to_rear(index / 2, new_hash);
            trace('read_decommitment_and_push', false);
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
