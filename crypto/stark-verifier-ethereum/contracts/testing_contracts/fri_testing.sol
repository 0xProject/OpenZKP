pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../fri.sol';
import '../primefield.sol';


contract FriTesting is Fri {
    event log_bytes32(bytes32 data);

    function fold_coset_external(
        uint256[] calldata coset,
        uint256 eval_point,
        uint64 step,
        uint64 index,
        uint64 len,
        PrimeField.EvalX calldata eval_x
    ) external {
        LayerContext memory layer_context = LayerContext({
            coset_size: uint64(coset.length),
            step: step,
            len: len,
            generator: eval_x.eval_domain_generator.fpow(step),
            log_domain_size: uint64(len).num_bits()
        });

        (uint256 result, uint256 x_inv) = fold_coset(coset, eval_point, layer_context, index);
        emit log_bytes32(bytes32(result));
    }

    // TODO - Unused function path
    function fri_layering_external(
        ProofTypes.StarkProof memory proof,
        uint8[] memory fri_layout,
        uint256[] memory eval_points,
        uint8 log_eval_domain_size,
        uint64[] memory queries,
        uint256[] memory polynomial_at_queries
    ) public {
        fri_check(proof, fri_layout, eval_points, log_eval_domain_size, queries, polynomial_at_queries);
        // Because we use overwriting internal memory management this should now hold our outputs
        for (uint256 i = 0; i < polynomial_at_queries.length; i++) {
            emit log_bytes32((bytes32)(polynomial_at_queries[i]));
        }
    }
}
