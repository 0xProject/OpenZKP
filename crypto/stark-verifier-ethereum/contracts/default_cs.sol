pragma solidity ^0.6.6;
pragma experimental ABIEncoderV2;

import './interfaces/ConstraintInterface.sol';
import './primefield.sol';
import './utils.sol';
import './trace.sol';
import './proof_types.sol';

abstract contract DefaultConstraintSystem is ConstraintSystem, Trace  {
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

    // This function calcluates the adjustments to each query point which are implied
    // by the offsets and degree of the constraint system
    // It returns the low degree polynomial points at the query indcies
    function get_polynomial_points(
        ProofTypes.OodsEvaluationData memory data,
        PrimeField.EvalX memory eval,
        uint256[] memory oods_coeffiecients,
        uint256[] memory queries,
        uint256 oods_point
    ) internal returns (uint256[] memory) {
        trace('oods_prepare_inverses', true);
        uint256[] memory inverses = oods_prepare_inverses(
            queries,
            eval,
            oods_point,
            data.log_trace_length + 4,
            data.log_trace_length
        );
        trace('oods_prepare_inverses', false);
        uint256[] memory results = new uint256[](queries.length);

        // Note that the oods coeffients are read from the data and assumed to be
        // in montgomery form, we remove that here to save gas.
        for (uint256 i = 0; i < oods_coeffiecients.length; i++) {
            oods_coeffiecients[i] = mulmod(oods_coeffiecients[i], PrimeField.MONTGOMERY_R_INV, PrimeField.MODULUS);
        }

        uint256[] memory layout = layout_col_major();
        for (uint256 i = 0; i < queries.length; i++) {
            uint256 result = 0;
            {
            trace('get_polynomial_points_loop_1', true);
            // These held pointers help soldity make the stack work
            uint256[] memory trace_oods_values = data.trace_oods_values;
            uint256[] memory trace_values = data.trace_values;

            // This function is an assembly implementation of the logic found
            // in commit 596a0ea670055de92d6c0240701ac4ec4aaa0f44 linked here:
            // https://github.com/0xProject/OpenZKP/blob/596a0ea670055de92d6c0240701ac4ec4aaa0f44/crypto/stark-verifier-ethereum/contracts/default_cs.sol#L56
            result = oods_row_adjustment(trace_oods_values, trace_values, oods_coeffiecients, layout, inverses, i);

            trace('get_polynomial_points_loop_1', false);
            }

            uint256 coeffiecients_index = data.trace_oods_values.length;

            trace('get_polynomial_points_loop_2', true);
            uint256 denominator_inv = inverses[i * (NUM_OFFSETS+1) + NUM_OFFSETS];
            uint256[] memory constraint_values = data.constraint_values;
            uint256[] memory constraint_oods_values = data.constraint_oods_values;

            for (uint256 j = 0; j < CONSTRAINT_DEGREE; j ++ ) {
                // Load the Oods coefficent
                uint256 coef = oods_coeffiecients[coeffiecients_index + j];

                // Get the constraint value, oods constraint value and use to get the numerator
                uint256 loaded_constraint_value = constraint_values[i * CONSTRAINT_DEGREE + j];
                uint256 loaded_oods_value = constraint_oods_values[j];
                uint256 numerator = addmod(loaded_constraint_value, PrimeField.MODULUS - loaded_oods_value, PrimeField.MODULUS);

                // Multiply numerator*denominator and add this to the result
                uint256 element = mulmod(numerator, denominator_inv, PrimeField.MODULUS);
                uint256 next_term = mulmod(element, coef, PrimeField.MODULUS);
                result = addmod(result, next_term, PrimeField.MODULUS);
            }
            trace('get_polynomial_points_loop_2', false);

            results[i] = result;
        }

        return results;
    }

    // TODO - Make batch invert a function
    // TODO - Attempt to make batch invert work in place
    // Note - This function should be auto generated along
    function oods_prepare_inverses(
        uint256[] memory queries,
        PrimeField.EvalX memory eval,
        uint256 oods_point,
        uint8 log_eval_domain_size,
        uint8 log_trace_len
    ) internal returns(uint256[] memory) {
        // The layout rows function gives us a listing of all of the row offset which
        // will be accessed for this calculation
        uint256[] memory trace_rows = layout_rows();
        oods_point = oods_point.from_montgomery();
        uint256 trace_generator = eval.eval_domain_generator.fpow(BLOWUP);
        uint256[] memory batch_in = new uint256[]((NUM_OFFSETS+1) * queries.length);
        // For each query we we invert several points used in the calculation of
        // the commited polynomial.
        {
        uint256 oods_constraint_power = oods_point.fpow(uint256(CONSTRAINT_DEGREE));
        uint256[] memory generator_powers = new uint256[](trace_rows.length);

        // NOTE - This consumes quite a bit of gas ~100k for larger constraint systems
        // However, removing it would break our abstractions in a way that is probably
        // not worth it.
        for (uint i = 0; i < trace_rows.length; i++) {
            generator_powers[i] = trace_generator.fpow(trace_rows[i]);
        }

        trace('query_shifting_loop', true);
        for (uint256 i = 0; i < queries.length; i++) {
            // Get the shifted eval point
            uint256 x;
            {
                uint256 query = queries[i];
                uint256 bit_reversed_query = query.bit_reverse(log_eval_domain_size);
                x = eval.lookup(bit_reversed_query);
                x = x.fmul(PrimeField.GENERATOR);
            }

            generator_power_adjustment(batch_in, generator_powers, oods_point, i, x);
            // This is the shifted x - oods_point^(degree)
            batch_in[i*(NUM_OFFSETS+1) + NUM_OFFSETS] = x.fsub(oods_constraint_power);
        }
        }
        trace('query_shifting_loop', false);

        trace('oods_batch_invert', true);
        uint256[] memory batch_out = new uint256[](batch_in.length);
        PrimeField.batch_invert(batch_in, batch_out);
        trace('oods_batch_invert', false);
        return batch_out;
    }

    function generator_power_adjustment(
        uint256[] memory output_memory_location,
        uint256[] memory generator_powers,
        uint256 oods_point,
        uint256 i,
        uint256 x
    ) internal {
        // We localize the number of offsets because assembly
        // doesn't understand immutables
        uint256 local_num_offsets = NUM_OFFSETS;
        assembly {
            // Pointer to the data in the generator array
            // Note - By copying the pointer we prevent corruption of
            // the underlying.
            let generator_pointer := add(generator_powers, 32)
            // The word location which one after the end of the data
            // array
            let generator_end := add(generator_pointer, mul(mload(generator_powers), 32))
            // We need to write to a memory data pointer in the output array
            let output_pointer := add(output_memory_location, 32)
            // Since we have been writing to this array before
            // we need to move the data pointer forward by i*(NUM_OFFSETS+1) words
            output_pointer := add(output_pointer, mul(mul(i, add(local_num_offsets, 1)), 32))

            // This loop increments the generator pointer and checks if it
            // is still before the end of the data on every loop.
            for {} lt(generator_pointer, generator_end) {generator_pointer := add(generator_pointer, 32)}
            {
                let loaded_gen_power := mload(generator_pointer)
                // Note - The local MODULUS const is declared below this function
                let shifted_oods := mulmod(oods_point, loaded_gen_power, MODULUS)
                // Add x to -shifted_oods mod P
                let x_sub_shifted_oods := addmod(x, sub(MODULUS, shifted_oods), MODULUS)
                // Store our computational result
                mstore(output_pointer, x_sub_shifted_oods)
                // Move our output pointer fowards
                output_pointer := add(output_pointer, 32)
            }
        }
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

    function oods_row_adjustment(uint256[] memory trace_oods_values, uint256[] memory trace_values, uint256[] memory oods_coeffiecients, uint256[] memory layout, uint256[] memory inverses, uint256 i) internal view returns(uint256 result) {
        // We want to get the pointers to memory before passing those into
        // the pure assembly function.
        uint256 trace_oods_values_ptr;
        uint256 trace_values_ptr;
        uint256 oods_coeffiecients_ptr;
        uint256 layout_ptr;
        uint256 inverses_ptr;
        // This assembly block copies the pointers of the memory objects.
        assembly {
            trace_oods_values_ptr := trace_oods_values
            trace_values_ptr := trace_values
            oods_coeffiecients_ptr := oods_coeffiecients
            layout_ptr := layout
            inverses_ptr := inverses
        }

        result = oods_row_adjustment_asm(trace_oods_values_ptr, trace_values_ptr, oods_coeffiecients_ptr, layout_ptr, inverses_ptr, i);
    }


    // We localize this constant so it can be used in assembly
    bytes32 constant MODULUS = 0x0800000000000011000000000000000000000000000000000000000000000001;

    // This pure assembly function takes in memory pointers and maniuplates them
    // Warning - Pass in a copy of the pointer as it will corrupt the pointers passed in
    // It then reads each of the oods values and divides out the polynomial terms needed
    // to make the result match an intermediate calculation of the polynomial point
    // commited too.
    function oods_row_adjustment_asm(uint256 trace_oods_values, uint256 trace_values, uint256 oods_coeffiecients, uint256 layout, uint256 inverses, uint256 i) internal view returns(uint256 result) {
        // We cannot access immutables in assembly
        uint256 inverseOffset = (NUM_OFFSETS+1)*i;
        uint256 rowOffset = NUM_COLUMNS*i;

        assembly {
            function read_array(ptr, offset) -> loaded {
                loaded :=  mload(add(add(ptr, 32), mul(offset, 32)))
            }
            // We record total length to use in the loop bound
            let bound := mload(trace_oods_values)
            // Then because the arrays are structured as [length][data start]
            // we move the pointers forward by one machine word.
            trace_oods_values := add(trace_oods_values, 32)
            // // Trace oods values is a special case, where we always want
            // // data at the rowOffset so we add that to this pointer
            trace_values := add(trace_values, 32)
            trace_values := add(trace_values, mul(rowOffset, 32))
            oods_coeffiecients := add(oods_coeffiecients, 32)
            layout := add(layout, 32)
            // Inverses is also a special case where we increment the data
            // pointer to a new location in the data memory
            inverses := add(inverses, 32)
            inverses := add(inverses, mul(32, inverseOffset))

            let trace_oods_values_end := add(trace_oods_values, mul(bound, 32))
            for {} lt(trace_oods_values, trace_oods_values_end) {} {
                let numerator
                {
                // Load directly from the data pointer
                let loaded_trace_data := mload(trace_oods_values)
                // We then move the data pointer foward by one word
                trace_oods_values := add(trace_oods_values, 32)
                // We load from the word in the trace values data range which
                // is at the forward location determined by layout's load
                let loaded_trace_value := mload(add(trace_values, mload(layout)))
                // We load the data pointer layout so now need to move it forward to the
                // next data location.
                layout := add(layout, 32)

                numerator := addmod(loaded_trace_value, sub(MODULUS, loaded_trace_data), MODULUS)
                }

                let denominator_inv
                {
                // We read from the layout data pointer
                let row := mload(layout)
                // Then we increment it to the next data location
                layout := add(layout, 32)
                // To read the demoninator inverse we want to read the
                // row-th element after the inverses data pointer,
                // so we load from inverse + row*32
                denominator_inv := mload(add(inverses, row))
                }

                let element := mulmod(numerator, denominator_inv, MODULUS)
                // We read right from the oods coeffiecient data pointer
                let coef := mload(oods_coeffiecients)
                // We then incrrement it by word size so the next loop can use it
                oods_coeffiecients := add(oods_coeffiecients, 32)

                let next_term := mulmod(element, coef, MODULUS)
                result := addmod(result, next_term, MODULUS)
            }
        }
        return result;
    }

    // Returns an array of all of the row offsets which are used
    function layout_rows() internal pure virtual returns(uint256[] memory);
    // Returns a trace layout in pairs ordered in coloum major form
    function layout_col_major() internal pure virtual returns(uint256[] memory);
}
