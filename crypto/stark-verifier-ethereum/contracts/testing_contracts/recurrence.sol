pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../interfaces/ConstraintInterface.sol';
import '../public_coin.sol';
import '../proof_types.sol';
import '../utils.sol';
import '../primefield.sol';
import '../iterator.sol';
import '../default_cs.sol';
import './recurrence_trace.sol';
import './recurence_constraint_256.sol';

// This contract checks the recurance constraint system from the testing contract
contract Recurrence is RecurrenceTrace {
    using Iterators for Iterators.IteratorUint;
    using PrimeField for uint256;
    using PrimeField for PrimeField.EvalX;
    using Utils for *;

    ConstraintPolyLen256 immutable constraint256;

    constructor(ConstraintPolyLen256 constraint) public {
        constraint256 = constraint;
    }

    struct PublicInput {
        uint256 value;
        uint64 index;
    }

    struct StackDepthSaver {
        PrimeField.EvalX eval;
        ProofTypes.OodsEvaluationData data;
    }

    // prettier-ignore
    function constraint_calculations(
        ProofTypes.OodsEvaluationData memory oods_eval_data,
        uint256[] memory queries,
        uint256 oods_point,
        uint256[] memory constraint_coeffiencts,
        uint256[] memory oods_coeffiencts
    ) public override returns (uint256[] memory, uint256) {
        PublicInput memory input = abi.decode(oods_eval_data.public_inputs, (PublicInput));
        PrimeField.EvalX memory eval = PrimeField.init_eval(oods_eval_data.log_trace_length + 4);
        uint256[] memory result = get_polynomial_points(oods_eval_data, eval, oods_coeffiencts, queries, oods_point);

        uint256 evaluated_point;
        {
            if (oods_eval_data.log_trace_length == 8 && input.index == 150) {
                uint256[] memory preloaded_trace_values =  oods_eval_data.trace_oods_values;
                evaluated_point = evaluate_oods_point256(oods_point, constraint_coeffiencts, eval, input, preloaded_trace_values);
            } else {
                StackDepthSaver memory saver = StackDepthSaver(
                    eval,
                    oods_eval_data
                );
                evaluated_point = evaluate_oods_point(oods_point, constraint_coeffiencts, input, saver);
            }
        }
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
        PublicInput memory public_input,
        StackDepthSaver memory saver
    ) internal returns (uint256) {
        uint256 trace_length = uint256(1) << saver.data.log_trace_length;
        // Note the blowup is fixed in this contract
        uint256 trace_generator = saver.eval.eval_domain_generator.fpow(16);
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
                uint256 cell_squared = saver.data.trace_oods_values[2].fmul_mont(saver.data.trace_oods_values[2]);
                uint256 constraint_eval = saver.data.trace_oods_values[1].fsub(cell_squared);
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
                uint256 constraint_eval = saver.data.trace_oods_values[3].fsub(saver.data.trace_oods_values[0]);
                constraint_eval = constraint_eval.fsub(saver.data.trace_oods_values[2]);
                constraint_eval = constraint_eval.fmul(every_row_numb);
                constraint_eval = constraint_eval.fmul(every_row_denom);
                result = result.fadd(adjustment.fmul_mont(constraint_eval));
            }
        }
        {
            uint256 adjustment_fixed_row = non_mont_oods.fpow(degree_adjustment(target_degree, trace_length - 1, 1));
            {
                uint256 constraint_eval = saver.data.trace_oods_values[0].fsub((uint256(1).to_montgomery()));
                // TODO - Just make that one?
                uint256 last_row_denom = (non_mont_oods.fsub(trace_generator.fpow(trace_length))).inverse();
                constraint_eval = constraint_eval.fmul(last_row_denom);
                uint256 adjustment = constraint_coeffiencts[5].fmul(adjustment_fixed_row);
                adjustment = adjustment.fadd(constraint_coeffiencts[4]);
                result = result.fadd(adjustment.fmul_mont(constraint_eval));
            }

            {
                uint256 constraint_eval = saver.data.trace_oods_values[0].fsub(public_input.value);
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

    function evaluate_oods_point256(
        uint256 oods_point,
        uint256[] memory constraint_coeffiencts,
        PrimeField.EvalX memory eval,
        PublicInput memory public_input,
        uint256[] memory trace_oods_values
    ) internal returns (uint256) {
        uint256[] memory call_context = new uint256[](15);
        call_context[0] = oods_point.fmul_mont(1);
        call_context[1] = public_input.index;
        call_context[2] = public_input.value.from_montgomery();
        uint256 current_index = 3;
        for (uint256 i = 0; i < constraint_coeffiencts.length; i++) {
            call_context[current_index] = constraint_coeffiencts[i];
            current_index++;
        }
        for (uint256 i = 0; i < trace_oods_values.length; i++) {
            call_context[current_index] = trace_oods_values[i].fmul_mont(1);
            current_index++;
        }

        // The contract we are calling out to is a pure assembly contract
        // With its own hard coded memory structure so we use an assembly
        // call to send a non abi encoded array that will be loaded dirrectly
        // into memory
        uint256 result;
        ConstraintPolyLen256 local_contract_address = constraint256;
        assembly {
            let p := mload(0x40)
            if iszero(call(not(0), local_contract_address, 0, add(call_context, 0x20), 0x1E0, p, 0x20)) {
                revert(0, 0)
            }
            result := mload(p)
        }
        return result;
    }

    function degree_adjustment(
        uint256 target_degree,
        uint256 numerator_degree,
        uint256 denominator_degree
    ) internal pure returns (uint256) {
        return target_degree + denominator_degree - numerator_degree;
    }
}
