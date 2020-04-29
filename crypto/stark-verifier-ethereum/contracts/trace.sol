pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;


// `name` is handles as a `bytes32` instead of string to lower the gast cost
// and avoid additional memory allocations.

contract Trace {
    event LogTrace(bytes32 name, bool enter, uint256 gasLeft, uint256 allocated);

    function trace(bytes32 name, bool enter) internal {
        uint256 gas = gasleft();
        uint256 allocated = 0;
        assembly {
            allocated := mload(0x40)
        }
        emit LogTrace(name, enter, gas, allocated);
    }
}
