pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../interfaces/ConstraintInterface.sol';
import '../public_coin.sol';
import '../proof_types.sol';
import '../utils.sol';
import '../primefield.sol';
import '../iterators.sol';


// This trivial Fibonacci system returns constant values which are true only for one proof
// It should only be used for testing purposes
contract Recurance is ConstraintSystem {
    using Iterators for Iterators.IteratorUint;
    using PrimeField for uint256;
    using PrimeField for PrimeField.EvalX;
    using Utils for *;

    struct PublicInput {
        uint256 value;
        uint64 index;
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
            digest: keccak256(abi.encodePacked(input.index, input.value, uint256(2))),
            counter: 0
        });

        // The trace length is going to be the next power of two after index.
        uint8 log_trace_length = Utils.num_bits(input.index) + 1;
        uint8[] memory fri_layout = default_fri_layout(log_trace_length);

        ProofTypes.ProofParameters memory params = ProofTypes.ProofParameters({
            number_of_columns: 2,
            log_trace_length: log_trace_length,
            number_of_constraints: 4,
            log_blowup: 4,
            constraint_degree: 2,
            pow_bits: 10,
            number_of_queries: 20,
            fri_layout: fri_layout
        });

        return (params, coin);
    }

    // TODO - Move this to a util file or default implementation
    uint8 LOG2_TARGET = 8;

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

    // prettier-ignore
    function constraint_calculations(
        ProofTypes.StarkProof calldata proof,
        ProofTypes.ProofParameters calldata params,
        uint64[] calldata queries,
        uint256 oods_point,
        uint256[] calldata constraint_coeffiencts
    ) external view override returns (uint256[] memory, uint256) {
        uint256[20] memory data = [
            0x0278847872d28b671420b700e8472b61d6846def99dbf99a7a5399322e5a2b25,
            0x067cda05602e614e2c1b223c79da8baebac06b2d292fefb80ea4e86e18f943bc,
            0x00c675c55829b7a6c183dde1223e93478d6b70c26bcc2201889e1b7887fe6aed,
            0x020c9a64e2b4c00045aea9a87b1164a50466eabd2cfe73384cf9bebd790b20b0,
            0x017228952dfaed74882b395e72518fc0a62f850a1988d663d3bc71be2ac0fa17,
            0x052108c7d4c28ce004ab79e110fb3cdc47d2ac50fb98de8ce04472ac67198a1d,
            0x05f492f33d6193afbb51b02b931c1aa08ae75af0893b20a46e5061fab952098d,
            0x06a0f45dfaf230e64f2bf379bd3c98f21420bddd9b8fef9e9c65f7486c6d5046,
            0x00262047cab1f998fb5707c6eee44b246e4ef011d2832eb289b708be2c1368d6,
            0x04ab8123e26adcb3dbd198991ab1e9b435712c26a246ef5396911e1e29d55d33,
            0x02fb6d73ed2f683e39d10a47b8419dcc4a8fd38826e84ec526768fae221e71a9,
            0x00e969f57d6c8591abe24b6e44060e01ac72555c00f7f9c5811dd02d857435a3,
            0x0069ebcf5161ea303c183fe4a92d6ff06343bca3cd382792aeb4a1b43c6610f5,
            0x01f7f5804a45c9da2940a323ca6edf93dab8b19a08917392951306162f45cfaa,
            0x07ffc4537e1b3c8f709413fbec183286e663878fb43cb6c58d15f618f91dcae8,
            0x06cfe5f951759fdfaf0affdef0fc822396baae1090e579bc18a20c154c4dd97b,
            0x033028e4c3a950389e9e219d624acecc8d8c201f27005335ec487fb8decca1e8,
            0x00624958789bc7d55270e20d9abe2ab66e58f9713af2cf0a3a6feba91c514506,
            0x02ba418d91252465917e8e1f6126194005b06abc1739036c1068ebef512a7536,
            0x0714fd690cb3ef6d859113829d892187b2a6300949e3fca261214473632e5559
        ];

        // Soldity really needs better conversions
        uint256[] memory result = new uint256[](20);
        for (uint256 i = 0; i < 20; i++) {
            result[i] = data[i];
        }
        return (result, 0x01e94b626dcff9d77c33c75b33d8457ba91534da30442d41d717a06e3f65211d);
    }

    function get_polynomial_points(
        uint256[] memory trace_oods_values,
        uint256[] memory constraint_oods_values,
        uint256[] query_trace_values,
        uint256[] memory query_constraint_values,
        uint256[] memory oods_coeffiecients,
        uint64[] memory queries,
        uint256 oods_point,
        uint8 log_eval_domain_size
    ) internal pure returns(uint256[] memory) {
        Iterators.IteratorUint memory inverses = Iterators.init_iterator(oods_prepare_inverses(queries, oods_point, log_eval_domain_size));
        uint256[] memory result = new uint256[](queries.length);
        // Init an iterator over the oods coeffiecients
        Iterators.IteratorUint memory coeffiecients = Iterators.init_iterator(oods_coeffiecients);

        for (uint256 i = 0; i < queries.length; i++) {
            uint256 result = 0;
            for (uint256 j = 0; j < trace_oods_values.length; j++) {

            }
        }
    }

    // TODO - Make batch invert a function
    // TODO - Attempt to make batch invert work in place
    function oods_prepare_inverses(uint64[] memory queries, uint256 oods_point, uint8 log_eval_domain_size)
        internal
        returns (uint256[] memory)
    {
        PrimeField.EvalX memory eval = PrimeField.init_eval(log_eval_domain_size);
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
            batch_in[3 * i + 1] = x.fsub(oods_point.fmul(eval.eval_domain_generator));
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
            batch_out[i] = inv_prod.fmul(batch_in[i - 1]);
            inv_prod = inv_prod.fmul(batch_in[i]);
        }
        batch_out[0] = inv_prod;
        return batch_out;
    }
}
