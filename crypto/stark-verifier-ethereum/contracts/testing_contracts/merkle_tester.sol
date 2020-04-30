pragma solidity ^0.6.4;

import '../merkle.sol';


contract MerkleVerifierTest is MerkleVerifier {
    event log_bool(bool data);

    function verify_merkle_proof_external(
        bytes32 root,
        bytes32[] calldata data_points,
        uint256[] calldata indices,
        bytes32[] calldata decommitment
    ) external returns (bool) {
        bool result = verify_merkle_proof(root, data_points, indices, decommitment);
        emit log_bool(result);
    }
}
