pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../interfaces/ConstraintInterface.sol';
import '../public_coin.sol';
import '../proof_types.sol';
import '../utils.sol';
import '../primefield.sol';
import '../iterator.sol';
import '../default_cs.sol';
import './constant_trace.sol';
import './constant_oods.sol';


// This contract checks the recurance constraint system from the testing contract
contract Constant is ConstantTrace {
    using Iterators for Iterators.IteratorUint;
    using PrimeField for uint256;
    using PrimeField for PrimeField.EvalX;
    using Utils for *;

    ConstantOodsPoly immutable constraint256;

    constructor(ConstantOodsPoly constraint) public {
        constraint256 = constraint;
    }

    struct PublicInput {
        uint256 value;
    }

    // prettier-ignore
    function constraint_calculations(
        ProofTypes.StarkProof calldata proof,
        ProofTypes.ProofParameters calldata params,
        uint64[] calldata queries,
        uint256 oods_point,
        uint256[] calldata constraint_coeffiencts,
        uint256[] calldata oods_coeffiencts
    ) external override returns (uint256[] memory, uint256) {
        ProofData memory data = ProofData(
            proof.trace_values,
            PrimeField.init_eval(params.log_trace_length + 4),
            proof.constraint_values, proof.trace_oods_values,
            proof.constraint_oods_values,
            params.log_trace_length);
        PublicInput memory input = abi.decode(proof.public_inputs, (PublicInput));

        uint256[] memory result = get_polynomial_points(data, oods_coeffiencts, queries, oods_point);
        uint256 evaluated_point = evaluate_oods(oods_point, constraint_coeffiencts, data.eval, input, data.trace_oods_values);

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
            digest: keccak256(abi.encodePacked(input.value)),
            counter: 0
        });
        // trace length is always 2, because we can't handle trace lengths that are 1.
        uint8 log_trace_length = 1;
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

    function evaluate_oods(
        uint256 oods_point,
        uint256[] memory constraint_coeffiencts,
        PrimeField.EvalX memory eval,
        PublicInput memory public_input,
        uint256[] memory trace_oods_values
    ) internal returns (uint256) {
        uint256[] memory call_context = new uint256[](15);
        call_context[0] = oods_point.fmul_mont(1);
        call_context[1] = public_input.value.from_montgomery();
        uint256 current_index = 2;
        for (uint256 i = 0; i < constraint_coeffiencts.length; i ++) {
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
        ConstantOodsPoly local_contract_address = constraint256;
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
