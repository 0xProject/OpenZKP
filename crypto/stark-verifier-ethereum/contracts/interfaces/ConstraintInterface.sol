pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../stark_verifier.sol';
import '../proof_types.sol';


interface ConstraintSystem {
    // The function should return a constraint paramters struct based on the public input.
    function initalize_system(bytes calldata public_input)
        external
        view
        returns (ProofTypes.ProofParameters memory, PublicCoin.Coin memory);

    function calculate_commited_polynomial_points(ProofTypes.StarkProof calldata proof, ProofTypes.ProofParameters calldata params, uint64[] calldata queries, bytes32 oods_point, bytes32[] calldata constraint_coeffiencts) external view returns(bytes32[] memory, bytes32[] memory);
}
