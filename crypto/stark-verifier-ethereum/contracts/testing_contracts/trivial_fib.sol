pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../interfaces/ConstraintInterface.sol';
import '../public_coin.sol';
import '../proof_types.sol';


// This trivial Fibonacci system returns constant values which are true only for one proof
// It should only be used for testing purposes
contract TrivialFib is ConstraintSystem {
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
        PublicCoin.Coin memory coin = PublicCoin.Coin({
            digest: 0xc891a11ddbc6c425fad523a7a4aeafa505d7aa1638cfffbd5b747100bc69e367,
            counter: 0
        });
        uint8[] memory fri_layout = new uint8[](3);
        fri_layout[0] = 3;
        fri_layout[1] = 3;
        fri_layout[2] = 2;

        ProofTypes.ProofParameters memory params = ProofTypes.ProofParameters({
            number_of_columns: 2,
            log_trace_length: 10,
            number_of_constraints: 4,
            log_blowup: 4,
            constraint_degree: 1,
            pow_bits: 10,
            number_of_queries: 20,
            fri_layout: fri_layout
        });

        return (params, coin);
    }

    // prettier-ignore
    function constraint_calculations(
        ProofTypes.OodsEvaluationData memory oods_eval_data,
        uint256[] memory queries,
        uint256 oods_point,
        uint256[] memory constraint_coeffiencts,
        uint256[] memory oods_coeffiencts
    ) public override returns (uint256[] memory, uint256) {
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
}
