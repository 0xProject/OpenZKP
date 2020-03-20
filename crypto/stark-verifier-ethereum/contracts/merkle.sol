pragma solidity ^0.6.4;

import './hashing.sol';
import './ring_buffer.sol';

contract MerkleVerifier is Hashing, RingBuffer {

    function verify_merkle_proof(
        bytes32 root,
        bytes32[] memory data_points,
        uint[] memory indices,
        bytes32[] memory decommitment)
    internal pure returns(bool) {
        IndexRingBuffer memory buffer = IndexRingBuffer({
            front: 0,
            back: data_points.length-1,
            indexes: indices,
            data: data_points,
            is_empty: false
        });
        uint decommitment_index = 0;

        while (true) {
           (uint index, bytes32 current_hash) = remove_front(buffer);

           uint parent = index/2;

           if (parent > 0) {
               // Checks if this is a left node
               if (index%2 == 0) {
                   // If it exists we peak at the next node
                   if (has_next(buffer)) {
                        (uint next_index, bytes32 next_hash) = peak_front(buffer);

                        // This checks if the next index in the queue is the sibbling of this one
                        // If it is we use the data, otherwise we try the decommitment queue
                        if (next_index == index+1) {
                            // This force increments the front, may consider real method for this.
                            (next_index, next_hash) = remove_front(buffer);

                            // Because index is even it is the left hash so to get the next one we do:
                            bytes32 new_hash = double_width_masked_hash(current_hash, next_hash);
                            add_to_rear(buffer, index/2, new_hash);
                        } else {
                            // Tries to read from decommitment and push new node, if it fails returns false
                            if (!read_decommitment_and_push(
                                true,
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
                            true,
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
                        false,
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
               // The only index with no parent is 1, which makes this the proposed root.
               return(root == current_hash);
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
            if (decommitment_index < decommitment.length) {
                bytes32 new_hash;

                // Preform the hash
                if (is_left) {
                    new_hash = double_width_masked_hash(current_hash, decommitment[decommitment_index]);
                } else {
                    new_hash = double_width_masked_hash(decommitment[decommitment_index], current_hash);
                }
                // Add the new node to the buffer.
                // Note the buffer strictly shrinks in the algo so we can't overflow the size.
                add_to_rear(buffer, index/2, new_hash);

                return true;
            } else {
                // Return that the decommitment data doesn't exist
                return false;
            }
    }
}
