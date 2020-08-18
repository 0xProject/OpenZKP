import {waffle} from '@nomiclabs/buidler';
import chai from 'chai';
import {deployContract, solidity} from 'ethereum-waffle';
import {utils} from 'ethers';
import fs from 'fs';

import ConstraintPolyLen256Artifact from '../artifacts/ConstraintPolyLen256.json';
import RecurrenceArtifact from '../artifacts/Recurrence.json';
import StarkDigestTestingArtifact from '../artifacts/StarkDigestTesting.json';
import {ConstraintPolyLen256} from '../typechain/ConstraintPolyLen256';
import {Recurrence} from '../typechain/Recurrence';
import {StarkDigestTesting} from '../typechain/StarkDigestTesting';

import recurrence_proofs from './recurrence_proofs.json';

const INITIAL_GAS = 100000000;

chai.use(solidity);

// tslint:disable:space-before-function-paren typedef
describe('Recurrence testing', function(this: any) {
    // Disables the timeouts
    this.timeout(0);
    let constraint_contract: Recurrence;
    let verifier_contract: StarkDigestTesting;
    let constraint256Contract: ConstraintPolyLen256;

    const provider = waffle.provider;
    const [wallet] = provider.getWallets();

    before(async () => {
        constraint256Contract = (await deployContract(wallet, ConstraintPolyLen256Artifact)) as ConstraintPolyLen256;
        constraint_contract = (await deployContract(wallet, RecurrenceArtifact, [
            constraint256Contract.address,
        ])) as Recurrence;
        verifier_contract = (await deployContract(wallet, StarkDigestTestingArtifact)) as StarkDigestTesting;
    });

    // Note - This checks the proof of work, but not the whole proof yet
    it('Should validate a correct proof', async () => {
        for (let i = 0; i < recurrence_proofs.length; i++) {
            // We ts-ignore because it's connivent to abi encode here not in rust
            // @ts-ignore
            recurrence_proofs[i].public_inputs = utils.defaultAbiCoder.encode(
                ['uint256', 'uint64'],
                [recurrence_proofs[i].public_inputs.value, recurrence_proofs[i].public_inputs.index],
            );
            // NOTE - Typescript has a very very hard time with the ethers js internal array types in struct encoding
            // in this case it's best for the code to ignore it because this is how ethers js understands these types.
            const receipt = await
            (
                // @ts-ignore
                await verifier_contract.verify_proof(recurrence_proofs[i], constraint_contract.address, {
                    gasLimit: INITIAL_GAS,
                })
            ).wait();

            // Compute calldata cost
            const call_data = utils.arrayify(
                verifier_contract.interface.functions.verify_proof.encode([
                    // @ts-ignore
                    recurrence_proofs[i],
                    constraint_contract.address,
                ]),
            );
            const call_data_length = call_data.length;
            const call_data_zeros = call_data.filter(byte => byte === 0).length;
            const call_data_zeros_cost = call_data_zeros * 4;
            const calldata_cost = (call_data_length - call_data_zeros) * 16 + call_data_zeros_cost;

            // Log gas consumption
            let gas_log = '';
            gas_log += `ENTER transaction ${INITIAL_GAS} 0\n`;
            gas_log += `ENTER calldata ${INITIAL_GAS} 0\n`;
            gas_log += `ENTER calldata_zeros ${INITIAL_GAS - calldata_cost + call_data_zeros_cost} 0\n`;
            gas_log += `LEAVE calldata_zeros ${INITIAL_GAS - calldata_cost} 0\n`;
            gas_log += `LEAVE calldata ${INITIAL_GAS - calldata_cost} 0\n`;
            let last_alloc = 0;
            for (const event of receipt.events) {
                if (event.event !== 'LogTrace') {
                    continue;
                }
                const direction = event.args.enter ? 'ENTER' : 'LEAVE';
                const name = utils.parseBytes32String(event.args.name);
                gas_log += `${direction} ${name} ${event.args.gasLeft} ${event.args.allocated}\n`;
                last_alloc = event.args.allocated;
            }
            gas_log += `LEAVE transaction ${INITIAL_GAS - receipt.gasUsed?.toNumber()} ${last_alloc}\n`;
            fs.writeFile(`gas-${i}.log`, gas_log, err => {
                if (err) {
                    // tslint:disable:no-console
                    console.error(err);
                }
            });

            // TODO - Use better logging
            // tslint:disable:no-console
            console.log('Proof verification gas used : ', receipt.gasUsed?.toNumber());
        }
    });
});
