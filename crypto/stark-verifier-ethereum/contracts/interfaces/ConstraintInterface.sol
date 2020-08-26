pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../proof_types.sol';
import '../public_coin.sol';


// solhint-disable-next-line
abstract contract ConstraintSystem {
    // The function should return a constraint paramters struct based on the public input.
    function initalize_system(bytes calldata public_input)
        external
        virtual
        view
        returns (ProofTypes.ProofParameters memory, PublicCoin.Coin memory);

    // This function should take all of the relevent function information and then return two things
    // (1) the evaulation of the constraints on the oods point and
    // (2) the calculation of the points on the polynomial which is commited too for fri
    function constraint_calculations(
        ProofTypes.OodsEvaluationData memory oods_eval_data,
        uint256[] memory queries,
        uint256 oods_point,
        uint256[] memory constraint_coeffiencts,
        uint256[] memory oods_coeffiencts
    ) public virtual returns (uint256[] memory, uint256);
}
