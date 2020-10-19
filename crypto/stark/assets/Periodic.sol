pragma solidity ^0.6.6;

contract Periodic{name} \{
    function evaluate(uint256 x) external pure returns (uint256 y) \{
        assembly \{
            let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001
            {{ for coefficient in coefficients -}}
            {{ if @first -}}
            y := 0x{coefficient}
            {{ else -}} {{ if not @last -}}
            y := add(mulmod(x, y, PRIME), 0x{coefficient})
            {{ else -}}
            y := addmod(mulmod(x, y, PRIME), 0x{coefficient}, PRIME)
            {{ endif -}} {{ endif -}}
            {{ endfor }}
        }
    }
}
