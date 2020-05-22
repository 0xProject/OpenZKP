pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../interfaces/ConstraintInterface.sol';
import '../public_coin.sol';
import '../proof_types.sol';
import '../utils.sol';
import '../primefield.sol';
import '../iterator.sol';
import '../default_cs.sol';
import './starkdex_trace.sol';
import './starkdex_oods.sol';
import './starkdex_periodic0.sol';
import './starkdex_periodic1.sol';
import './starkdex_periodic2.sol';
import './starkdex_periodic3.sol';
import '@nomiclabs/buidler/console.sol';

// This contract is the constraint system for a dex
// It can only process transactions in the following batch
// sizes: 1
contract Starkdex is StarkdexTrace {
    using Iterators for Iterators.IteratorUint;
    using PrimeField for uint256;
    using PrimeField for PrimeField.EvalX;
    using Utils for *;

    StarkdexOodsPoly immutable oods;
    StarkdexPeriodic0 immutable periodic_column_0;
    StarkdexPeriodic1 immutable periodic_column_1;
    StarkdexPeriodic2 immutable periodic_column_2;
    StarkdexPeriodic3 immutable periodic_column_3;

    constructor(StarkdexOodsPoly constraint, address[4] memory addresses) public {
        oods = constraint;
        periodic_column_0 = (StarkdexPeriodic0)(addresses[0]);
        periodic_column_1 = (StarkdexPeriodic1)(addresses[1]);
        periodic_column_2 = (StarkdexPeriodic2)(addresses[2]);
        periodic_column_3 = (StarkdexPeriodic3)(addresses[3]);
    }

    // This stuct contains the public input for the stark proof
    // Inital root is the vault state root before and final root is it after
    // The packed modifcation array contains modifcation data
    // Packed in the following format: prev_amount (64b) + new_amount (64b) + vault_id (32b) + row (16b) + reserved (80b)
    // This format is likely to change as we support larger balances and other changes.
    struct PublicInput {
        uint256 number_of_transactions;
        uint256 inital_root;
        uint256 final_root;
        bytes32[] packed_modification_data;
        uint256[] token_ids;
        uint256[] public_keys;
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
        console.log("constraint_calculations start");
        ProofData memory data = ProofData(
            proof.trace_values,
            PrimeField.init_eval(params.log_trace_length + 4),
            proof.constraint_values, proof.trace_oods_values,
            proof.constraint_oods_values,
            params.log_trace_length);
        console.log("about to decode start");
        console.logBytes(proof.public_inputs);
        PublicInput memory input;
        bytes memory public_input_bytes = proof.public_inputs;
        {
          (uint256 p_0, uint256 p_1, uint256 p_2, bytes32[] memory p_3, uint256[] memory p_4, uint256[] memory p_5) = abi.decode(public_input_bytes, (uint256, uint256, uint256, bytes32[], uint256[], uint256[]));
          input = PublicInput(p_0, p_1, p_2, p_3, p_4, p_5);
        }
        console.log("decode end");
        uint256[] memory result = get_polynomial_points(data, oods_coeffiencts, queries, oods_point);
        console.log("get_polynomial_points");
        uint256 evaluated_point;
        if (params.log_trace_length == 16) {
            evaluated_point = evaluate_oods_point1(oods_point, constraint_coeffiencts, data.eval, input, data.trace_oods_values);
        } else {
            revert("Unsuported tx len");
        }
        console.log("constraint_calculations end");

        console.log("constraint_calculations result");
        console.logBytes32((bytes32)(evaluated_point.from_montgomery()));
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
        /* console.log("initalize_system start"); */
        /* PublicInput memory input = abi.decode(public_input, (PublicInput)); */
        /* console.log("initalize_system 1"); */
        PublicCoin.Coin memory coin = PublicCoin.Coin({
            // TODO - Potetially insecure, FIX BEFORE LAUNCH
            digest: keccak256(abi.encodePacked()),
            counter: 0
        });
        /* console.log("initalize_system 2"); */
        // The trace length is going to be the next power of two after index.
        // Note - Trace length is num_txn*65536 so
        // log_trace_length = num_bits(num_txn*65536) = num_bits(num_txn) + 15
        uint8 log_trace_length = 16; //Utils.num_bits((uint64)(input.number_of_transactions)) + 15;
        uint8[] memory fri_layout = default_fri_layout(log_trace_length);

        /* console.log("initalize_system 3"); */
        ProofTypes.ProofParameters memory params = ProofTypes.ProofParameters({
            number_of_columns: NUM_COLUMNS,
            log_trace_length: log_trace_length,
            number_of_constraints: 120,
            // TODO - Potentially non-fixed blowup
            log_blowup: 4,
            constraint_degree: CONSTRAINT_DEGREE,
            pow_bits: 0,
            number_of_queries: 3,
            fri_layout: fri_layout
        });
        /* console.log("initalize_system end"); */
        return (params, coin);
    }

    uint256 constant root_1_tx = 1;
    function evaluate_oods_point1(
        uint256 oods_point,
        uint256[] memory constraint_coeffiencts,
        PrimeField.EvalX memory eval,
        PublicInput memory public_input,
        uint256[] memory trace_oods_values
    ) internal returns (uint256) {
        /* console.log("evaluate_oods_point1 end"); */
        // TODO - Resize this to match, may cause reverts
        uint256[] memory call_context = new uint256[](382);
        uint256 non_mont_oods = oods_point.fmul_mont(1);
        call_context[0] = non_mont_oods;
        // Mason! this order is wrong. these should be at the end!
        call_context[1] = (uint256)(public_input.inital_root).from_montgomery();
        call_context[2] = (uint256)(public_input.final_root).from_montgomery();

        // Calculate the is_settlement polynomial
        uint256 is_settlement = is_settlement_polynomial(public_input.packed_modification_data, non_mont_oods, root_1_tx);
        // Calculate the is_modification polynomials
        call_context[3] = is_settlement;
        call_context[4] = is_modification_polynomial(non_mont_oods, public_input.number_of_transactions, is_settlement);
        console.log("is_settlement");
        console.logBytes32((bytes32)(call_context[3]));
        console.log("is_modification");
        console.logBytes32((bytes32)(call_context[4]));
        // Calculate the 'base', 'key', 'token', 'initial_amount', 'final_amount' and 'vault' polynomials
        // We use a single function call for for efficency and so we can reuse values
        // TODO - Further reuse could be accomplished by reusing denominators from is_settlement call, saving significant gas
        uint256[6] memory interpolated_value_polys = get_weighted_field(public_input, non_mont_oods, is_settlement, root_1_tx);
        call_context[5] = interpolated_value_polys[0];
        call_context[6] = interpolated_value_polys[1];
        call_context[7] = interpolated_value_polys[2];
        call_context[8] = interpolated_value_polys[3];
        call_context[9] = interpolated_value_polys[4];
        call_context[10] = interpolated_value_polys[5];
        console.log("interpolated_value_polys 0");
        console.logBytes32((bytes32)(call_context[5]));
        console.log("interpolated_value_polys 1");
        console.logBytes32((bytes32)(call_context[6]));
        console.log("interpolated_value_polys 2");
        console.logBytes32((bytes32)(call_context[7]));
        console.log("interpolated_value_polys 3");
        console.logBytes32((bytes32)(call_context[8]));
        console.log("interpolated_value_polys 4");
        console.logBytes32((bytes32)(call_context[9]));
        console.log("interpolated_value_polys 5");
        console.logBytes32((bytes32)(call_context[10]));

        // Next we add the perodic cols
        call_context[11] = periodic_column_0.evaluate(non_mont_oods);
        call_context[12] = periodic_column_1.evaluate(non_mont_oods);
        call_context[13] = periodic_column_2.evaluate(non_mont_oods);
        call_context[14] = periodic_column_3.evaluate(non_mont_oods);
        console.log("periodic_column_0");
        console.logBytes32((bytes32)(call_context[11]));
        console.log("periodic_column_1");
        console.logBytes32((bytes32)(call_context[12]));
        console.log("periodic_column_2");
        console.logBytes32((bytes32)(call_context[13]));
        console.log("periodic_column_3");
        console.logBytes32((bytes32)(call_context[14]));

        uint256 current_index = 15;
        // This array contains 240 elements, 2 for each constraint
        console.log("constraint_coeffiencts.length", constraint_coeffiencts.length);
        for (uint256 i = 0; i < constraint_coeffiencts.length; i ++) {
            if (i > 0) {
                call_context[current_index] = 0;
            } else {
                call_context[current_index] = constraint_coeffiencts[i];
                console.log("coefficient = ");
                console.logBytes32((bytes32)(call_context[current_index]));
                console.logBytes32((bytes32)(call_context[current_index].from_montgomery()));
            }
            current_index++;
        }
        // This array contains 127 elements, one for each trace offset in the layout
        console.log("trace_oods_values.length", trace_oods_values.length);
        for (uint256 i = 0; i < trace_oods_values.length; i++) {
            /* if (i < 1) {
                call_context[current_index] = 0;
            } else { */
                call_context[current_index] = trace_oods_values[i].fmul_mont(1);
                console.logBytes32((bytes32)(call_context[current_index].from_montgomery()));
            /* } */
            current_index++;
        }

        // The contract we are calling out to is a pure assembly contract
        // With its own hard coded memory structure so we use an assembly
        // call to send a non abi encoded array that will be loaded dirrectly
        // into memory
        uint256 result;
        {
        StarkdexOodsPoly local_contract_address = oods;
        assembly {
            let p := mload(0x40)
            // Note size is 382*32 because we have 15 public inputs, 240 constraint coeffiecents and 127 trace decommitments
            if iszero(call(not(0), local_contract_address, 0, add(call_context, 0x20), 0x2FC0, p, 0x20)) {
              revert(0, 0)
            }
            result := mload(p)
        }
        }
        return result;
    }

    // This function takes the listing of modification data, the non-montgomery oods point, and the root of order number of transactions
    // It returns an evaulation of an interpolating polynomial over the modification data to be used in the constraint calculation
    function is_settlement_polynomial(bytes32[] memory packed_modification_data, uint256 x, uint256 root) internal returns(uint256) {
        uint256 is_settlement = 1;

        for (uint256 i = 0; i < packed_modification_data.length; i++) {
            // The layout has the index as 2 bytes which are 20 bytes from the start and followed by 10 bytes of reserved space
            // The following divides out the extra bits an then cleans the bits on top with an and
            uint256 unpacked_index = ((uint256)(packed_modification_data[i]) /  0x100000000000000000000) & 0xFFFF;
            // Our polynomial is the product of the
            is_settlement = is_settlement.fmul(x.fsub(root.fpow(unpacked_index)));
        }

        return is_settlement;
    }

    // Cacluated the 'is_modifications' polynomial at the oods point
    function is_modification_polynomial(uint256 x, uint256 number_of_transactions, uint256 is_settlement)  internal returns(uint256) {
        uint256 denominator = is_settlement.inverse();
        uint256 numberator = (x.fpow(number_of_transactions)).fsub(1);

        return numberator.fmul(denominator);
    }

    // Calculates the 'base', 'key', 'token', 'initial_amount', 'final_amount' and 'vault' polynomials
    // at the oods point/ any input x, using the public inputs and the settlement polynomial and a root
    // of order number of transactions.
    function get_weighted_field(PublicInput memory public_input, uint256 x, uint256 is_settlement, uint256 root) internal returns(uint256[6] memory) {
        uint256[6] memory outputs;
        for( uint i = 0; i < 6; i++) {
            outputs[i] = 0;
        }

        for(uint256 i = 0; i < public_input.packed_modification_data.length; i++) {
            uint256 unpacked_index = ((uint256)(public_input.packed_modification_data[i]) /  0x100000000000000000000) & 0xFFFF;
            // TODO - Batch invert, will save quite a bit of gas
            // TODO - If we make this batch invert we should factor that out to the primefield libary
            // it's used many other places as well.
            uint256 weight = (x.fsub(root.fpow(unpacked_index))).inverse();

            // 'base' has 1 as accumulated data
            outputs[0] = outputs[0].fadd(weight.fmul(is_settlement));
            //  'key' has the public key's x cord as accumlated data
            outputs[1] = outputs[1].fadd(weight.fmul(public_input.public_keys[i].fmul(is_settlement)));
            // 'token' has the token data as accumlated public input
            outputs[2] = outputs[2].fadd(weight.fmul(public_input.token_ids[i].fmul(is_settlement)));
            // 'inital_amount' has the inital amount as accumulated data
            uint256 unpacked_inital_amount = (uint256)(public_input.packed_modification_data[i]) >> 192;
            outputs[3] = outputs[3].fadd(weight.fmul(unpacked_inital_amount.fmul(is_settlement)));
            // 'final_ammount' has the final amount as accumulated data
            uint256 unpacked_final_amount = ((uint256)(public_input.packed_modification_data[i]) >> 128) & ((1 << 64) - 1);
            outputs[4] = outputs[4].fadd(weight.fmul(unpacked_final_amount.fmul(is_settlement)));
            // 'vault' has the vault index as the accumulated data
            uint256 unpacked_vault = ((uint256)(public_input.packed_modification_data[i]) >> 96) & ((1 << 32) - 1);
            outputs[5] = outputs[5].fadd(weight.fmul(unpacked_vault.fmul(is_settlement)));
        }

        return outputs;
    }
}
