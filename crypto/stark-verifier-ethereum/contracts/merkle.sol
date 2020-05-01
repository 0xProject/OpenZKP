pragma solidity ^0.6.4;

import './trace.sol';


contract MerkleVerifier is Trace {
    bytes32 constant HASH_MASK = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF000000000000000000000000;

    // This function takes a set of data leaves and indices are 2^depth + leaf index and must be sorted in ascending order.
    // NOTE - An empty claim will revert
    // Total saved 696516
    // Total saved 697200
    // Total saved 723228
    function verify_merkle_proof(
        bytes32 root,
        bytes32[] memory leaves,
        uint256[] memory indices,
        bytes32[] memory decommitment
    ) internal returns (bool valid) {
        trace('verify_merkle_proof', true);
        require(leaves.length == indices.length, 'Invalid input');
        require(leaves.length > 0, 'No claimed data');
        assembly {
            // Read length and get rid of the length prefices
            let length := shl(5, mload(indices))
            indices := add(indices, 0x20)
            leaves := add(leaves, 0x20)
            decommitment := add(decommitment, 0x20)

            // Set up ring buffer
            let read_index := 0
            let write_index := 0

            for {} 1 {} {
                // Read the current index and store leaf hash in scratch space
                let index := mload(add(indices, read_index))
                mstore(shl(5, and(index, 1)), mload(add(leaves, read_index)))
                read_index := mod(add(read_index, 0x20), length)

                // Stop if we hit the root
                if eq(index, 1) {
                    // Root hash is stored right.
                    valid := eq(mload(0x20), root)
                    break
                }

                // Check if the next available index is the right sibbling.
                // `index | 1` turns index it to the right sibbling (no-op if it already is)
                switch eq(or(index, 1), mload(add(indices, read_index)))
                case 0 {
                    // No merge, read a decommitment
                    // It doesn't matter if we read decommitment beyond the end,
                    // we would read in garbage and not produce a valid root.
                    mstore(shl(5, xor(and(index, 1), 1)), mload(decommitment))
                    decommitment := add(decommitment, 0x20)
                }
                default {
                    // Merging with next item in ring buffer
                    mstore(0x20, mload(add(leaves, read_index)))
                    read_index := mod(add(read_index, 0x20), length)
                }
                mstore(add(indices, write_index), shr(1, index))
                mstore(add(leaves, write_index), and(keccak256(0x00, 0x40), HASH_MASK))
                write_index := mod(add(write_index, 0x20), length)
            }
        }
        trace('verify_merkle_proof', false);
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
