pragma solidity ^0.6.4;

import '../primefield.sol';


contract PrimeFieldTester {
    using PrimeField for *;
    event log_bytes32(bytes32 data);

    function fpow_external(uint256 a, uint256 b) external {
        emit log_bytes32(bytes32(a.fpow(b)));
    }

    function inverse_external(uint256 a) external {
        emit log_bytes32(bytes32(a.inverse()));
    }

    function batch_invert_external(uint256[] calldata input) external {
        uint256[] memory out = new uint256[](input.length);
        input.batch_invert(out);
        for (uint256 i = 0; i < input.length; i++) {
            emit log_bytes32(bytes32(out[i]));
        }
    }

    function fmul_external(uint256 a, uint256 b) external pure returns (uint256) {
        return a.fmul(b);
    }

    function fadd_external(uint256 a, uint256 b) external pure returns (uint256) {
        return a.fadd(b);
    }
}
