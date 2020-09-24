
pragma solidity ^0.6.6;
pragma experimental ABIEncoderV2;

import '../interfaces/ConstraintInterface.sol';
import '../default_cs.sol';

abstract contract {name}Trace is DefaultConstraintSystem({constraint_degree}, {num_rows}, {num_cols}, {blowup}) \{
    // This lets us map rows -> inverse index,
    // In complex systems use a autogen binary search.
    function row_to_offset(uint256 row) internal pure override returns(uint256) \{
        {{ if row_offsets -}}
        {{ for ro in row_offsets -}}
        {{ if @first -}}
        if (row == {ro.row}) \{ return {ro.index}; }
        {{ else -}}
        else if (row == {ro.row}) \{ return {ro.index}; }
        {{ endif -}}
        {{ endfor -}} 
        {{ else -}}
        return row;
        {{ endif }}
    }

    function layout_col_major() internal pure override returns(uint256[] memory) \{
        uint256[] memory result = new uint256[]({column_layout_size});
        {{ for column in column_layout -}}
        result[{@index}] = {column};
        {{ endfor }}
        return result;
    }

    function layout_rows() internal pure override returns(uint256[] memory) \{
        uint256[] memory result = new uint256[]({num_rows});
        {{ for row in row_layout -}}
        result[{@index}] = {row};
        {{ endfor }}
        return result;
    }
}
