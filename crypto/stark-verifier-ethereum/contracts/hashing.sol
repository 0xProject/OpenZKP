pragma solidity ^0.6.4;

contract Hashing {

    // This hash on bytes memory will determine the hash used throughout the proof system
    function hasher(bytes memory preimage) internal pure returns(bytes32) {
        return keccak256(preimage);
    }

    function double_width_hash(bytes32 preimage_a, bytes32 preimage_b) internal pure returns(bytes32) {
        return hasher(abi.encodePacked(preimage_a, preimage_b));
    }
}
