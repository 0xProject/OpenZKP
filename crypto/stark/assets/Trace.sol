
pragma solidity ^0.6.6;
pragma experimental ABIEncoderV2;

import '../interfaces/ConstraintInterface.sol';
import '../default_cs.sol';

abstract contract {name}Trace is DefaultConstraintSystem({constraint_degree}, {num_rows}, {num_cols}, {blowup}) \{
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
