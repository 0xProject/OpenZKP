pragma solidity ^0.6.4;

import '../primefield.sol';


contract PrimeFieldTester is PrimeField {
    event log_bytes32(bytes32 data);

    function fpow_external(uint256 a, uint256 b) external {
        emit log_bytes32(bytes32(fpow(a, b)));
    }

    function inverse_external(uint256 a) external {
        emit log_bytes32(bytes32(inverse(a)));
    }

    function fmul_external(uint256 a, uint256 b) external pure returns (uint256) {
        return fmul(a, b);
    }

    function fadd_external(uint256 a, uint256 b) external pure returns (uint256) {
        return fadd(a, b);
    }
}
