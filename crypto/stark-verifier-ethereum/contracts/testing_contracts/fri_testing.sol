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
        uint256 log_len = len.num_bits();
        uint256 generator = PrimeField.generator_power(uint8(log_len));
        uint256 start_of_coset = index - (index % coset.length);
        uint256 exp = len - index.bit_reverse2(log_len - 1);
        uint256 x_inv = generator.fpow(exp);
        uint256 result;

        for (uint256 i = 0; i < coset.length; i++) {
            console.log(coset[i]);
        }
        console.log(eval_point);
        console.log(x_inv);
        x_inv = 0x07b29494e473ce930b6238d02250fdbde4f31c35b05d1e7026e082c068ece7e7;

        (result, x_inv) = fold_coset(coset, eval_point, x_inv);

        // 3481714190203691579718548732381424157610379293143349507665901524421608007655
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
