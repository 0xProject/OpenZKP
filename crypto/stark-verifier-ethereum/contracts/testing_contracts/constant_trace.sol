pragma solidity ^0.6.6;
pragma experimental ABIEncoderV2;

import '../interfaces/ConstraintInterface.sol';
import '../default_cs.sol';


// The linter doesn't understand 'abstract' and thinks it's indentation

// solhint-disable-next-line indent
abstract contract ConstantTrace is DefaultConstraintSystem(1, 1, 1, 16) {
    // This lets us map rows -> inverse index,
    // In complex systems use a autogen binary search.
    function row_to_offset(uint256 row) internal override pure returns (uint256) {
        return row;
    }

    function layout_col_major() internal override pure returns (uint256[] memory) {
        uint256[] memory result = new uint256[](2);
        (result[0], result[1]) = (0, 0);
        return result;
    }

    function layout_rows() internal override pure returns (uint256[] memory) {
        uint256[] memory result = new uint256[](1);
        result[0] = 0;
        return result;
    }
}
