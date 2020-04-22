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

    function fmul_external(uint256 a, uint256 b) external pure returns (uint256) {
        return a.fmul(b);
    }

    function fadd_external(uint256 a, uint256 b) external pure returns (uint256) {
        return a.fadd(b);
    }
}
