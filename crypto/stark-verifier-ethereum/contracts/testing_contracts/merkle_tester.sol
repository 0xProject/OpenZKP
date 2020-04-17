pragma solidity ^0.6.4;

import '../merkle.sol';


contract MerkleVerifierTest is MerkleVerifier {
    function verify_merkle_proof_external(
        bytes32 root,
        bytes32[] calldata data_points,
        uint256[] calldata indices,
        bytes32[] calldata decommitment
    ) external view returns (bool) {
        return verify_merkle_proof(root, data_points, indices, decommitment);
    }
}
