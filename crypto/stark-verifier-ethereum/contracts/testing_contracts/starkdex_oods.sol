pragma solidity ^0.6.6;

contract StarkdexOodsPoly {
    fallback() external {
        assembly {
            let res := 0
            mstore(0, res)
            return(0, 0x20)
        }
    }
}
