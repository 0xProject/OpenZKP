pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../fri.sol';

contract FriTesting is Fri {
    event log_bytes32(bytes32 data);
    function fold_coset_external(
        bytes32[] calldata coset,
        bytes32 eval_point,
        uint64 step,
        uint64 index,
        uint64 len,
        Eval_X calldata eval_x
    ) external {
        emit log_bytes32(fold_coset(coset, eval_point, LayerContext(0, step, len), index, eval_x));
    }
}
