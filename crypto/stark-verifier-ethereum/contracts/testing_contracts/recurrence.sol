pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../interfaces/ConstraintInterface.sol';
import '../public_coin.sol';
import '../proof_types.sol';
import '../utils.sol';
import '../primefield.sol';
import '../iterator.sol';
import '../trace.sol';


// This trivial Fibonacci system returns constant values which are true only for one proof
// It should only be used for testing purposes
contract Recurrence is Trace, ConstraintSystem {
    using Iterators for Iterators.IteratorUint;
    using PrimeField for uint256;
    using PrimeField for PrimeField.EvalX;
    using Utils for *;

    struct PublicInput {
        uint256 value;
        uint64 index;
    }

    struct ProofData {
        uint256[] trace_values;
        PrimeField.EvalX eval;
        uint256[] constraint_values;
        uint256[] trace_oods_values;
        uint256[] constraint_oods_values;
        uint8 log_trace_length;
    }

    uint8 NUM_COLUMNS = 2;
    uint8 CONSTRAINT_DEGREE = 2;
    // TODO - Move this to a util file or default implementation
    uint8 LOG2_TARGET = 8;

    // prettier-ignore
    function constraint_calculations(
        ProofTypes.StarkProof calldata proof,
        ProofTypes.ProofParameters calldata params,
        uint64[] calldata queries,
        uint256 oods_point,
        uint256[] calldata constraint_coeffiencts,
        uint256[] calldata oods_coeffiencts
    ) external override returns (uint256[] memory, uint256) {
        trace('constraint_calculations', true);
        ProofData memory data = ProofData(
            proof.trace_values,
            PrimeField.init_eval(params.log_trace_length + 4),
            proof.constraint_values, proof.trace_oods_values,
            proof.constraint_oods_values,
            params.log_trace_length);
        PublicInput memory input = abi.decode(proof.public_inputs, (PublicInput));
        uint256[] memory result = get_polynomial_points(data, oods_coeffiencts, queries, oods_point);

        uint256 evaluated_point = evaluate_oods_point(oods_point, constraint_coeffiencts, data.eval, input, data);
        trace('constraint_calculations', false);
        return (result, evaluated_point);
    }

    // These constants are derived from the small fib example in rust
    // TODO - The solidity prettier wants to delete all 'override' statements
    // We should remove this ignore statement when that changes.
    // prettier-ignore
    function initalize_system(bytes calldata public_input)
        external
        view
        override
        returns (ProofTypes.ProofParameters memory, PublicCoin.Coin memory)
    {
        PublicInput memory input = abi.decode(public_input, (PublicInput));
        PublicCoin.Coin memory coin = PublicCoin.Coin({
            digest: keccak256(abi.encodePacked(input.index, input.value, uint64(2))),
            counter: 0
        });
        // The trace length is going to be the next power of two after index.
        uint8 log_trace_length = Utils.num_bits(input.index) + 1;
        uint8[] memory fri_layout = default_fri_layout(log_trace_length);

        ProofTypes.ProofParameters memory params = ProofTypes.ProofParameters({
            number_of_columns: NUM_COLUMNS,
            log_trace_length: log_trace_length,
            number_of_constraints: 4,
            log_blowup: 4,
            constraint_degree: CONSTRAINT_DEGREE,
            pow_bits: 10,
            number_of_queries: 20,
            fri_layout: fri_layout
        });

        return (params, coin);
    }

    // (Trace(0, 1) - Trace(1, 0).pow(self.exponent)) * every_row(),
    // (Trace(1, 1) - Trace(0, 0) - Trace(1, 0)) * every_row(),
    // (Trace(0, 0) - 1.into()) * on_row(trace_length),
    // (Trace(0, 0) - (&self.value).into()) * on_row(self.index),
    // TODO - Use batch inversion
    function evaluate_oods_point(
        uint256 oods_point,
        uint256[] memory constraint_coeffiencts,
        PrimeField.EvalX memory eval,
        PublicInput memory public_input,
        ProofData memory data
    ) internal returns (uint256) {
        uint256 trace_length = uint256(1) << data.log_trace_length;
        // Note the blowup is fixed in this contract
        uint256 trace_generator = eval.eval_domain_generator.fpow(16);
        // NOTE - Constraint degree is fixed in this system
        uint256 target_degree = 2 * trace_length - 1; // 511
        uint256 non_mont_oods = oods_point.fmul_mont(1);

        uint256 result = 0;
        {
            //  (X.pow(trace_length) - 1.into())
            // Non mont form because the inverse is in native form
            // NOTE - Stack depth errors prevent this from being spread
            uint256 every_row_denom = (non_mont_oods.fpow(trace_length)).fsub(1);
            every_row_denom = every_row_denom.inverse();
            uint256 every_row_numb = non_mont_oods.fsub(trace_generator.fpow(trace_length - 1));
            // First constraint calculation block
            {
                uint256 adjustment;
                {
                    uint256 adjustment_every_row_2 = non_mont_oods.fpow(
                        degree_adjustment(target_degree, 2 * trace_length - 1, trace_length)
                    );
                    adjustment = constraint_coeffiencts[1].fmul(adjustment_every_row_2);
                    adjustment = adjustment.fadd(constraint_coeffiencts[0]);
                }
                uint256 cell_squared = data.trace_oods_values[2].fmul_mont(data.trace_oods_values[2]);
                uint256 constraint_eval = data.trace_oods_values[1].fsub(cell_squared);
                constraint_eval = constraint_eval.fmul(every_row_numb);
                constraint_eval = constraint_eval.fmul(every_row_denom);
                result = adjustment.fmul_mont(constraint_eval);
            }
            // Second constraint calculation block
            {
                uint256 adjustment;
                {
                    uint256 adjustment_every_row_1 = non_mont_oods.fpow(
                        degree_adjustment(target_degree, trace_length, trace_length)
                    );
                    adjustment = constraint_coeffiencts[3].fmul(adjustment_every_row_1);
                    adjustment = adjustment.fadd(constraint_coeffiencts[2]);
                }
                uint256 constraint_eval = data.trace_oods_values[3].fsub(data.trace_oods_values[0]);
                constraint_eval = constraint_eval.fsub(data.trace_oods_values[2]);
                constraint_eval = constraint_eval.fmul(every_row_numb);
                constraint_eval = constraint_eval.fmul(every_row_denom);
                result = result.fadd(adjustment.fmul_mont(constraint_eval));
            }
        }
        {
            uint256 adjustment_fixed_row = non_mont_oods.fpow(degree_adjustment(target_degree, trace_length - 1, 1));
            {
                uint256 constraint_eval = data.trace_oods_values[0].fsub((uint256(1).to_montgomery()));
                // TODO - Just make that one?
                uint256 last_row_denom = (non_mont_oods.fsub(trace_generator.fpow(trace_length))).inverse();
                constraint_eval = constraint_eval.fmul(last_row_denom);
                uint256 adjustment = constraint_coeffiencts[5].fmul(adjustment_fixed_row);
                adjustment = adjustment.fadd(constraint_coeffiencts[4]);
                result = result.fadd(adjustment.fmul_mont(constraint_eval));
            }

            {
                uint256 constraint_eval = data.trace_oods_values[0].fsub(public_input.value);
                uint256 index_row_denom = non_mont_oods.fsub(trace_generator.fpow(public_input.index));
                index_row_denom = index_row_denom.inverse();
                constraint_eval = constraint_eval.fmul(index_row_denom);
                uint256 adjustment = constraint_coeffiencts[7].fmul(adjustment_fixed_row);
                adjustment = adjustment.fadd(constraint_coeffiencts[6]);
                result = result.fadd(adjustment.fmul_mont(constraint_eval));
            }
        }
        return result;
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

        for (uint256 i = 0; i < queries.length; i++) {
            uint256 result = 0;
            // Num col * num_rows [note this relation won't hold in other contraint systems]
            uint256 len = NUM_COLUMNS * 2;
            for (uint256 j = 0; j < len; j++) {
                uint256 numberator = data.trace_values[i * 2 + j / 2].fsub(data.trace_oods_values[j]);
                uint256 denominator_inv = inverses[i * 3 + (j % 2)];
                uint256 element = numberator.fmul(denominator_inv);
                uint256 coef = coeffiecients.next();
                uint256 next_term = element.fmul_mont(coef);
                result = result.fadd(next_term);
            }

            uint256 denominator_inv = inverses[i * 3 + 2];
            len = CONSTRAINT_DEGREE;
            for (uint256 j = 0; j < len; j++) {
                uint256 numberator = data.constraint_values[i * len + j].fsub(data.constraint_oods_values[j]);
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
    // TODO - Make generic over a constant trace layout, will that work with complex systems?
    function oods_prepare_inverses(
        uint64[] memory queries,
        PrimeField.EvalX memory eval,
        uint256 oods_point,
        uint8 log_eval_domain_size,
        uint8 log_trace_len
    ) internal returns (uint256[] memory) {
        oods_point = oods_point.from_montgomery();
        uint256 trace_generator = eval.eval_domain_generator.fpow(16);
        uint256[] memory batch_in = new uint256[](3 * queries.length);
        // For each query we we invert several points used in the calculation of
        // the commited polynomial.
        for (uint256 i = 0; i < queries.length; i++) {
            // Get the shifted eval point
            uint256 x = eval.lookup(queries[i].bit_reverse(log_eval_domain_size)).fmul(PrimeField.GENERATOR);
            // Preparing denominator for row 0
            // This is the shifted x - trace_generator^(0)
            batch_in[3 * i + 0] = x.fsub(oods_point.fmul(uint256(1)));
            // Preparing denominator for row 1
            // This is the shifted x - trace_generator^(1)
            batch_in[3 * i + 1] = x.fsub(oods_point.fmul(trace_generator));
            // This is the shifted x - oods_point^(degree)
            batch_in[3 * i + 2] = x.fsub(oods_point.fmul(oods_point));
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

    function degree_adjustment(uint256 target_degree, uint256 numerator_degree, uint256 denominator_degree)
        internal
        pure
        returns (uint256)
    {
        return target_degree + denominator_degree - numerator_degree;
    }
}
