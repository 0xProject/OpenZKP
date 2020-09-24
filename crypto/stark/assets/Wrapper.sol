pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../interfaces/ConstraintInterface.sol';
import '../public_coin.sol';
import '../proof_types.sol';
import '../utils.sol';
import '../primefield.sol';
import '../iterator.sol';
import '../default_cs.sol';
import './{name}Trace.sol';
import './{name}ContraintPoly.sol';


contract {name} is {name}Trace \{
    using Iterators for Iterators.IteratorUint;
    using PrimeField for uint256;
    using PrimeField for PrimeField.EvalX;
    using Utils for *;

    OddsPoly immutable constraint_poly;
    // FIX ME - Add polynomials immutable variables

    // FIX ME - The constructor should also be setting any
    // periodic colum contracts to immutables
    constructor(OddsPoly constraint) public\{
        constraint_poly = constraint;
    }

    struct PublicInput \{
        // Please add the public input fields to this struct
    }

    // prettier-ignore
    function constraint_calculations(
        ProofTypes.StarkProof calldata proof,
        ProofTypes.ProofParameters calldata params,
        uint64[] calldata queries,
        uint256 oods_point,
        uint256[] calldata constraint_coeffiencts,
        uint256[] calldata oods_coeffiencts
    ) external override returns (uint256[] memory, uint256) \{
        ProofData memory data = ProofData(
            proof.trace_values,
            PrimeField.init_eval(params.log_trace_length + 4),
            proof.constraint_values, proof.trace_oods_values,
            proof.constraint_oods_values,
            params.log_trace_length);
        // FIX ME - You may need to customize this decoding
        PublicInput memory input = abi.decode(proof.public_inputs, (PublicInput));
        uint256[] memory result = get_polynomial_points(data, oods_coeffiencts, queries, oods_point);

        // Fix Me - This may need several internal functions
        uint256 evaluated_point = evaluate_oods_point(oods_point, constraint_coeffiencts, data.eval, input, data);

        return (result, evaluated_point);
    }

    // TODO - The solidity prettier wants to delete all 'override' statements
    // We should remove this ignore statement when that changes.
    // prettier-ignore
    function initalize_system(bytes calldata public_input)
        external
        view
        override
        returns (ProofTypes.ProofParameters memory, PublicCoin.Coin memory)
   \{
        // FIX ME - You may need to customize this decoding
        PublicInput memory input = abi.decode(public_input, (PublicInput));
        PublicCoin.Coin memory coin = PublicCoin.Coin(\{
            // FIX ME - Please add a public input hash here
            digest: // I'm just a robot I don't know what goes here ¯\_(ツ)_/¯.
            ,
            counter: 0
        });
        // The trace length is going to be the next power of two after index.
        // FIX ME - This need a trace length set, based on public input
        uint8 log_trace_length = 0;
        uint8[] memory fri_layout = default_fri_layout(log_trace_length);

        ProofTypes.ProofParameters memory params = ProofTypes.ProofParameters(\{
            number_of_columns: NUM_COLUMNS,
            log_trace_length: log_trace_length,
            number_of_constraints: {number_of_constraints},
            log_blowup: {log_blowup},
            constraint_degree: CONSTRAINT_DEGREE,
            pow_bits: {pow_bits},
            number_of_queries: {number_of_queries},
            fri_layout: fri_layout
        });

        return (params, coin);
    }

    function evaluate_oods_point(
        uint256 oods_point,
        uint256[] memory constraint_coeffiencts,
        PrimeField.EvalX memory eval,
        PublicInput memory public_input,
        ProofData memory data
    ) internal returns (uint256) \{
        uint256[] memory call_context = new uint256[]({total_input_memory_size});
        uint256 non_mont_oods = oods_point.fmul_mont(1);
        call_context[0] = non_mont_oods;

        {{ for pi in public_input_names -}}
        {{ if pi -}}
        call_context[1 + {@index}] = 0; // This public input is named: {pi}
        {{ else -}}
        call_context[1 + {@index}] = 0; // This public input is not named,
                                      // please give it a name in Rust
        {{ endif -}}
        {{ endfor -}}
        {{ for pc in periodic_column_evaluations -}}
        call_context[{pc.index}] = {pc.name}.evaluate(non_mont_oods.fpow({pc.exponent}));
        {{ endfor }}

        uint256 current_index = {coefficient_offset};
        // This array contains 2 * {number_of_constraints} elements, 2 for each constraint
        for (uint256 i = 0; i < constraint_coeffiencts.length; i ++) \{
            call_context[current_index] = constraint_coeffiencts[i];
            current_index++;
        }
        // This array contains {trace_layout_len} elements, one for each trace offset in the layout
        for (uint256 i = 0; i < trace_oods_values.length; i++) \{
            call_context[current_index] = trace_oods_values[i].fmul_mont(1);
            current_index++;
        }

        // The contract we are calling out to is a pure assembly contract
        // With its own hard coded memory structure so we use an assembly
        // call to send a non abi encoded array that will be loaded directly
        // into memory
        uint256 result;
        \{
        OddsPoly local_contract_address = constraint_poly;
            assembly \{
                let p := mload(0x40)
                // Note size is {constraint_input_size} because we have {number_of_public_inputs} public inputs, ?? periodic evaluations,
                // 2 * {number_of_constraints} constraint coefficients and {trace_layout_len} trace decommitments,
                // each 32 bytes.
                if iszero(call(not(0), local_contract_address, 0, add(call_context, 0x20), {constraint_input_size}, p, 0x20)) \{
                revert(0, 0)
                }
                result := mload(p)
            }
        }
        return result;
    }
}
