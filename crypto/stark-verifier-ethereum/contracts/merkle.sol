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
    ) internal returns (bool valid) {
        trace('verify_merkle_proof', true);
        require(leaves.length == indices.length, 'Invalid input');
        require(leaves.length > 0, 'No claimed data');
        assembly {
            // Read length and get rid of the length prefices
            let len := shl(5, mload(indices))
            indices := add(indices, 0x20)
            leaves := add(leaves, 0x20)
            decommitment := add(decommitment, 0x20)

            // Set up ring buffer
            let read_index := 0
            let write_index := 0

            for {} 1 {} {
                // Read the current index and store leaf hash in scratch space
                let index := shl(5, mload(add(indices, read_index)))
                mstore(and(index, 0x20), mload(add(leaves, read_index)))
                read_index := mod(add(read_index, 0x20), len)

                // Stop if we hit the root
                if eq(index, 0x20) {
                    valid := eq(mload(0x20), root)
                    break
                }

                // Check if the next available index is the right sibbling.
                // `index | 1` turns index it to the right sibbling (no-op if it already is)
                let merge := eq(or(index, 0x20), shl(5, mload(add(indices, read_index))))
                if merge {
                    mstore(0x20, mload(add(leaves, read_index)))
                    read_index := mod(add(read_index, 0x20), len)
                }
                if iszero(merge) {
                    // It doesn't matter if we read decommitment beyond the end,
                    // we would read in garbage and not produce a valid root.
                    mstore(xor(and(index, 0x20), 0x20), mload(decommitment))
                    decommitment := add(decommitment, 0x20)
                }
                mstore(add(indices, write_index), shr(6, index))
                mstore(add(leaves, write_index), and(keccak256(0x00, 0x40), HASH_MASK))
                write_index := mod(add(write_index, 0x20), len)
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
