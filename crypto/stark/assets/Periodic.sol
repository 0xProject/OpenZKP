pragma solidity ^0.6.6;

contract Periodic{name} \{
    function evaluate(uint256 x) external pure returns (uint256 y) \{
        assembly \{
            let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001
            y := 0x0
            {{ for coefficient in coefficients -}}
            {#- TODO: addmod -> add except for last row -#}
            y := addmod(mulmod(x, y, PRIME), 0x{coefficient}, PRIME)
            {{ endfor }}
        }
    }
}
