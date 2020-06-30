pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../interfaces/ConstraintInterface.sol';
import '../default_cs.sol';


// The linter doesn't understand 'abstract' and thinks it's indentation

// solhint-disable-next-line indent
abstract contract RecurrenceTrace is DefaultConstraintSystem(2, 2, 2, 16) {
    function layout_col_major() internal override pure returns (uint256[] memory) {
        uint256[] memory result = new uint256[](8);
        (result[0], result[1]) = (0, 0);
        (result[2], result[3]) = (0, 1);
        (result[4], result[5]) = (1, 0);
        (result[6], result[7]) = (1, 1);
        return result;
    }

    function layout_rows() internal override pure returns (uint256[] memory) {
        uint256[] memory result = new uint256[](2);
        result[0] = 0;
        result[1] = 1;
        return result;
    }
}
