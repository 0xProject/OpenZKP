pragma solidity ^0.6.6;
pragma experimental ABIEncoderV2;

import './interfaces/ConstraintInterface.sol';
import './primefield.sol';
import './iterator.sol';
import './utils.sol';

abstract contract DefaultConstraintSystem is ConstraintSystem  {
    using Iterators for Iterators.IteratorUint;
    using PrimeField for uint256;
    using PrimeField for PrimeField.EvalX;
    using Utils for *;

    uint8 immutable CONSTRAINT_DEGREE;
    uint8 immutable NUM_OFFSETS;
    uint8 immutable NUM_COLUMNS;
    uint8 immutable BLOWUP;

    constructor(uint8 constraint_degree, uint8 num_offests, uint8 num_col, uint8 blowup) public {
        CONSTRAINT_DEGREE = constraint_degree;
        NUM_OFFSETS = num_offests;
        NUM_COLUMNS = num_col;
        BLOWUP = blowup;
    }

    struct ProofData {
        uint256[] trace_values;
        PrimeField.EvalX eval;
        uint256[] constraint_values;
        uint256[] trace_oods_values;
        uint256[] constraint_oods_values;
        uint8 log_trace_length;
    }
    // This function calcluates the adjustments to each query point which are implied
    // by the offsets and degree of the constraint system
    // It returns the low degree polynomial points at the query indcies
    function get_polynomial_points(
        ProofData memory data,
        uint256[] memory oods_coeffiecients,
        uint64[] memory queries,
        uint256 oods_point
    ) internal returns (uint256[] memory) {
        uint256[] memory inverses = oods_prepare_inverses(
            queries,
            data.eval,
            oods_point,
            data.log_trace_length + 4,
            data.log_trace_length
        );
        uint256[] memory results = new uint256[](queries.length);

        // Init an iterator over the oods coeffiecients
        Iterators.IteratorUint memory coeffiecients = Iterators.init_iterator(oods_coeffiecients);
        uint256[] memory layout = layout_col_major();
        for (uint256 i = 0; i < queries.length; i++) {
            uint256 result = 0;
            {
            // These held pointers help soldity make the stack work
            uint256[] memory trace_oods_value = data.trace_oods_values;
            uint256[] memory trace_values = data.trace_values;
            for (uint256 j = 0; j < trace_oods_value.length; j++) {
                uint256 loaded_trace_data = trace_oods_value[j];
                // J*2 is the col index when the layout is in coloum major form
                // NUM_COLUMNS*i idenifes the start of this querry's row values
                uint256 calced_index = NUM_COLUMNS*i + layout[j*2];
                uint256 numberator = trace_values[calced_index].fsub(loaded_trace_data);

                // We are in col major form so we need to lookup the row offset
                uint256 row = layout[j*2+1];
                // We then use the row to offset function to lookup the offest of the
                // row's inverse.
                calced_index = (NUM_OFFSETS+1)*i + row_to_offset(row);
                uint256 denominator_inv = inverses[calced_index];

                uint256 element = numberator.fmul(denominator_inv);
                uint256 coef = coeffiecients.next();
                uint256 next_term = element.fmul_mont(coef);
                result = result.fadd(next_term);
            }
            }

            uint256 denominator_inv = inverses[i * (NUM_OFFSETS+1) + NUM_OFFSETS];
            uint256 len = CONSTRAINT_DEGREE;
            uint256[] memory constraint_values = data.constraint_values;
            uint256[] memory constraint_oods_values = data.constraint_oods_values;
            for (uint256 j = 0; j < len; j++) {
                uint256 loaded_constraint_value = constraint_values[i * len + j];
                uint256 loaded_oods_value = constraint_oods_values[j];
                uint256 numberator = loaded_constraint_value.fsub(loaded_oods_value);
                uint256 element = numberator.fmul(denominator_inv);
                uint256 coef = coeffiecients.next();
                uint256 next_term = element.fmul_mont(coef);
                result = result.fadd(next_term);
            }

            results[i] = result;
            // This resets the iterator to start from the begining again
            coeffiecients.index = 0;
        }

        return results;
    }

    // TODO - Make batch invert a function
    // TODO - Attempt to make batch invert work in place
    // Note - This function should be auto generated along
    function oods_prepare_inverses(
        uint64[] memory queries,
        PrimeField.EvalX memory eval,
        uint256 oods_point,
        uint8 log_eval_domain_size,
        uint8 log_trace_len
    ) internal returns (uint256[] memory) {
        // The layout rows function gives us a listing of all of the row offset which
        // will be accessed for this calculation
        uint256[] memory trace_rows = layout_rows();
        oods_point = oods_point.from_montgomery();
        uint256 trace_generator = eval.eval_domain_generator.fpow(BLOWUP);
        uint256[] memory batch_in = new uint256[]((NUM_OFFSETS+1) * queries.length);
        // For each query we we invert several points used in the calculation of
        // the commited polynomial.
        for (uint256 i = 0; i < queries.length; i++) {
            // Get the shifted eval point
            uint256 x = eval.lookup(queries[i].bit_reverse(log_eval_domain_size)).fmul(PrimeField.GENERATOR);


            for (uint j = 0; j < trace_rows.length; j ++) {
                batch_in[i*(NUM_OFFSETS+1) + j] = x.fsub(oods_point.fmul(trace_generator.fpow(trace_rows[j])));
            }
            // This is the shifted x - oods_point^(degree)
            batch_in[i*(NUM_OFFSETS+1) + NUM_OFFSETS] = x.fsub(oods_point.fpow(uint256(CONSTRAINT_DEGREE)));
        }

        uint256[] memory batch_out = new uint256[](batch_in.length);
        uint256 carried = 1;
        for (uint256 i = 0; i < batch_in.length; i++) {
            carried = carried.fmul(batch_in[i]);
            batch_out[i] = carried;
        }

        uint256 inv_prod = carried.inverse();

        for (uint256 i = batch_out.length - 1; i > 0; i--) {
            batch_out[i] = inv_prod.fmul(batch_out[i - 1]);
            inv_prod = inv_prod.fmul(batch_in[i]);
        }
        batch_out[0] = inv_prod;
        return batch_out;
    }

    // TODO - Move this to a util file or default implementation
    uint8 constant LOG2_TARGET = 8;
    // This function produces the default fri layout from the trace length
    function default_fri_layout(uint8 log_trace_len) internal view returns (uint8[] memory) {
        uint256 num_reductions;
        if (log_trace_len > LOG2_TARGET) {
            num_reductions = log_trace_len - LOG2_TARGET;
        } else {
            num_reductions = log_trace_len;
        }

        uint8[] memory result;
        if (num_reductions % 3 != 0) {
            result = new uint8[](1 + (num_reductions / 3));
            result[result.length - 1] = uint8(num_reductions % 3);
        } else {
            result = new uint8[](num_reductions / 3);
        }
        for (uint256 i = 0; i < (num_reductions / 3); i++) {
            result[i] = 3;
        }
        return result;
    }

    // Returns an array of all of the row offsets which are used
    function layout_rows() internal pure virtual returns(uint256[] memory);
    // Returns a trace layout in pairs ordered in coloum major form
    function layout_col_major() internal pure virtual returns(uint256[] memory);
    // A function which converts a row offset to where it is in the array of rows
    // This lets us map rows -> inverse index
    function row_to_offset(uint256 row) internal pure virtual returns(uint256);
}
