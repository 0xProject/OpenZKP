pragma solidity ^0.6.4;

import './ring_buffer.sol';

contract MerkleVerifier {
    using RingBuffer for IndexRingBuffer;

    function verify_merkle_proof(
        bytes32 root,
        bytes32[] memory leaves,
        uint[] memory indices,
        bytes32[] memory decommitment)
    internal pure returns(bool) {
        IndexRingBuffer memory buffer = IndexRingBuffer({
            front: 0,
            back: leaves.length-1,
            indexes: indices,
            data: leaves,
            is_empty: false
        });
        uint decommitment_index = 0;

        while (true) {
           (uint index, bytes32 current_hash) = buffer.remove_front();

            if (index == 1) {
               return(root == current_hash);
            }

            bool is_left = index % 2 == 0;
            if (is_left) {
                // If it exists we peak at the next node
                if (buffer.has_next()) {
                    (uint next_index, bytes32 next_hash) = buffer.peak_front();

                    // This checks if the next index in the queue is the sibbling of this one
                    // If it is we use the data, otherwise we try the decommitment queue
                    if (next_index == index+1) {
                        // This force increments the front, may consider real method for this.
                        (next_index, next_hash) = buffer.remove_front();

                        // Because index is even it is the left hash so to get the next one we do:
                        bytes32 new_hash = merkleTreeHash(current_hash, next_hash);
                        buffer.add_to_rear(index/2, new_hash);
                    } else {
                        // Tries to read from decommitment and push new node, if it fails returns false
                        if (!read_decommitment_and_push(
                            is_left,
                            buffer,
                            decommitment,
                            decommitment_index,
                            current_hash,
                            index
                        )) {
                            return false;
                        }
                        decommitment_index ++;
                    }
                } else {
                    // Tries to read from decommitment and push new node, if it fails returns false
                    if (!read_decommitment_and_push(
                        is_left,
                        buffer,
                        decommitment,
                        decommitment_index,
                        current_hash,
                        index
                    )) {
                        return false;
                    }
                    decommitment_index ++;
                }
            } else {
                // Tries to read from decommitment and push new node, if it fails returns false
                if (!read_decommitment_and_push(
                    is_left,
                    buffer,
                    decommitment,
                    decommitment_index,
                    current_hash,
                    index
                )) {
                    return false;
                }
                decommitment_index ++;
            }
        }
    }

    // This function reads from decomitment and pushes the new node onto the buffer,
    // It returns true if decommitnet data exists and false if it doesn't.
    function read_decommitment_and_push(
        bool is_left,
        IndexRingBuffer memory buffer,
        bytes32[] memory decommitment,
        uint decommitment_index,
        bytes32 current_hash,
        uint index) internal pure returns(bool) {
            // We do not want a revert from reading too far into the decommitment.
            if (decommitment_index >= decommitment.length) {
                return false;
            }

            bytes32 new_hash;
            // Preform the hash
            if (is_left) {
                new_hash = merkleTreeHash(current_hash, decommitment[decommitment_index]);
            } else {
                new_hash = merkleTreeHash(decommitment[decommitment_index], current_hash);
            }
            // Add the new node to the buffer.
            // Note the buffer strictly shrinks in the algo so we can't overflow the size.
            buffer.add_to_rear(index/2, new_hash);

            return true;
    }

    bytes32 constant HASH_MASK = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF000000000000000000000000;

    function merkleTreeHash(bytes32 preimage_a, bytes32 preimage_b) internal pure returns(bytes32) {
        return keccak256(abi.encodePacked(preimage_a, preimage_b)) & HASH_MASK;
    }
}
