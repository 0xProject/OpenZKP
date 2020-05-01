pragma solidity ^0.6.4;

import './ring_buffer.sol';
import './iterator.sol';
import './trace.sol';


contract MerkleVerifier is Trace {
    using RingBuffer for RingBuffer.IndexRingBuffer;
    using Iterators for Iterators.IteratorBytes32;

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
            bool needs_new_node = true;
            // If this is a left node then the node following it in the queue
            // may be a sibbling which we want to hash with it.
            if (is_left) {
                // If it exists we peak at the next node in the queue
                if (buffer.has_next()) {
                    (uint256 next_index, bytes32 next_hash) = buffer.peak_front();

                    // This checks if the next index in the queue is the sibbling of this one
                    // If it is we use the data, otherwise we try the decommitment queue
                    if (next_index == index + 1) {
                        // This force increments the front, may consider real method for this.
                        (next_index, next_hash) = buffer.remove_front();

                        // Because index is even it is the left hash so to get the next one we do:
                        bytes32 new_hash = merkleTreeHash(current_hash, next_hash);
                        buffer.add_to_rear(index / 2, new_hash);
                        // We indicate that a node was pushed, so that another won't be
                        needs_new_node = false;
                    }
                }
            }

            // Next we try to read from the decommitment and use that info to push a new hash into the queue
            if (needs_new_node) {
                // If we don't have more decommitment the proof fails
                if (!decommitment_iter.has_next()) {
                    trace('verify_merkle_proof', false);
                    return false;
                }
                // Reads from decommitment and pushes a new node
                read_decommitment_and_push(is_left, buffer, decommitment_iter, current_hash, index);
            }
        }
        trace('verify_merkle_proof', false);
    }

    // This function reads from decommitment and pushes the new node onto the buffer,
    // It returns true if decommitment data exists and false if it doesn't.
    function read_decommitment_and_push(
        bool is_left,
        RingBuffer.IndexRingBuffer memory buffer,
        Iterators.IteratorBytes32 memory decommitment,
        bytes32 current_hash,
        uint256 index
    ) internal {
        trace('read_decommitment_and_push', true);
        bytes32 next_decommitment = decommitment.next();
        bytes32 new_hash;
        // Preform the hash
        if (is_left) {
            new_hash = merkleTreeHash(current_hash, next_decommitment);
        } else {
            new_hash = merkleTreeHash(next_decommitment, current_hash);
        }
        // Add the new node to the buffer.
        // Note the buffer strictly shrinks in the algo so we can't overflow the size.
        buffer.add_to_rear(index / 2, new_hash);
        trace('read_decommitment_and_push', false);
    }

    bytes32 constant HASH_MASK = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF000000000000000000000000;

    function merkleTreeHash(bytes32 preimage_a, bytes32 preimage_b) internal returns (bytes32 hash) {
        // Equivalent to
        // hash = keccak256(abi.encodePacked(preimage_a, preimage_b)) & HASH_MASK
        // Using assembly for performance
        assembly {
            // The first 64 bytes of memory are scratch space
            mstore(0x00, preimage_a)
            mstore(0x20, preimage_b)
            hash := and(keccak256(0x00, 0x40), HASH_MASK)
        }
    }

    function merkleLeafHash(uint256[] memory leaf) internal pure returns (bytes32) {
        if (leaf.length == 1) {
            return (bytes32)(leaf[0]);
        } else {
            return keccak256(abi.encodePacked(leaf)) & HASH_MASK;
        }
    }
}
