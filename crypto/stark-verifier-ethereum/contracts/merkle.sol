pragma solidity ^0.6.4;

import './trace.sol';


contract MerkleVerifier is Trace {
    bytes32 constant HASH_MASK = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF000000000000000000000000;

    // This function takes a set of data leaves and indices are 2^depth + leaf index and must be sorted in ascending order.
    // Note: `leaves` and `indices` will be overwritten in the process
    // NOTE - An empty claim will revert
    // TODO: Add high level algorithm documentation.
    function verify_merkle_proof(
        bytes32 root,
        bytes32[] memory leaves,
        uint256[] memory indices,
        bytes32[] memory decommitment
    ) internal returns (bool valid) {
        trace('verify_merkle_proof', true);
        require(leaves.length == indices.length, 'Invalid input');
        require(leaves.length > 0, 'No claimed data');
        // This algorihm does a lot of array indexing and is a major hot path.
        // It is implemented in assembly to avoid unecessary bounds checking.
        // We rely on 64 bytes of scratch space being available in 0x00..0x40
        // (this is where we will store left and right leave for hashing)
        // We also rely on arrays having a length prefixed memory layout
        // See <https://solidity.readthedocs.io/en/v0.6.6/assembly.html#conventions-in-solidity>
        // Finally we make heavy use of the fact that left indices have their lowest
        // bit zero, and right indices one.
        // For the original non-assembly implementation, see <https://github.com/0xProject/OpenZKP/blob/480b69b9f82ee8319884ce8212682b0be7fa3f39/crypto/stark-verifier-ethereum/contracts/merkle.sol#L11>
        assembly {
            // Read length and get rid of the length prefices
            let length := shl(5, mload(indices))
            indices := add(indices, 0x20)
            leaves := add(leaves, 0x20)
            decommitment := add(decommitment, 0x20)

            // Set up ring buffer
            // Every next layer will have equal or fewer values, so write_index
            // can never overrun read_index.
            let read_index := 0
            let write_index := 0

            for {} 1 {} {
                // Read the current index
                let index := mload(add(indices, read_index))
                // Store leaf hash in scratch space at 0x00 or 0x20 depending on
                // the lower bit of index (which indicates left or right node)
                mstore(shl(5, and(index, 1)), mload(add(leaves, read_index)))
                // Increment read pointer, wrappering around the end
                read_index := mod(add(read_index, 0x20), length)

                // Stop if we hit the root, which has index 1
                if eq(index, 1) {
                    // Root hash is stored right
                    valid := eq(mload(0x20), root)
                    break
                }

                // Check if the next index in the ring is the right sibbling.
                // `index | 1` turns index into the right sibbling (no-op if it already is)
                switch eq(or(index, 1), mload(add(indices, read_index)))
                case 0 {
                    // No merge with right sibbling, read a decommitment
                    // Decommitment goes in left or right, opposite of the index bit.
                    mstore(shl(5, and(not(index), 1)), mload(decommitment))
                    // It doesn't matter if we read decommitment beyond the end,
                    // we would read in garbage and not produce a valid root.
                    decommitment := add(decommitment, 0x20)
                }
                default {
                    // Merg with next item in ring buffer, which is the right sibbling.
                    // Current must be a left. Right sibbling hash goes into 0x20.
                    mstore(0x20, mload(add(leaves, read_index)))
                    read_index := mod(add(read_index, 0x20), length)
                }
                // New node index is half the current index
                mstore(add(indices, write_index), shr(1, index))
                // New node left and right leaf are stored in 0x00..0x40
                mstore(add(leaves, write_index), and(keccak256(0x00, 0x40), HASH_MASK))
                // Increment and wrap the write pointer
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
