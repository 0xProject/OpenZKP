pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;


// `name` is handles as a `bytes32` instead of string to lower the gast cost
// and avoid additional memory allocations.

contract Trace {
    // TODO: Could log `this` (current contract address) to track call stack
    // This won't work on self-calls though.
    event LogTrace(bytes32 name, bool enter, uint256 gasLeft, uint256 allocated);

    modifier trace_mod(bytes32 name) {
        trace(name, true);
        _;
        trace(name, false);
    }

    function trace(bytes32 name, bool enter) internal {
        uint256 gas_left = gasleft();
        uint256 allocated = 0;
        assembly {
            allocated := mload(0x40)
        }
        emit LogTrace(name, enter, gas_left, allocated);
    }

    function trace_call(bytes32 name) internal returns (uint256) {
        uint256 gas_left = gasleft();
        uint256 allocated = 0;
        assembly {
            allocated := mload(0x40)
        }
        emit LogTrace(name, true, gas_left, allocated);
        gas_left = gasleft();
        return gas_left - (gas_left % 10000000);
    }
}
