import {waffle} from '@nomiclabs/buidler';
import chai from 'chai';
import {deployContract, solidity} from 'ethereum-waffle';
import {utils} from 'ethers';

import RecuranceArtifact from '../artifacts/Recurance.json';
import StarkDigestTestingArtifact from '../artifacts/StarkDigestTesting.json';
import {Recurance} from '../typechain/Recurance';
import {StarkDigestTesting} from '../typechain/StarkDigestTesting';

import recurance_proofs from './recurance_proofs.json';

chai.use(solidity);

describe('Recurance testing', function(this: any): void {
    // Disables the timeouts
    this.timeout(0);
    let constraint_contract: Recurance;
    let verifier_contract: StarkDigestTesting;

    const provider = waffle.provider;
    const [wallet] = provider.getWallets();

    before(async () => {
        constraint_contract = (await deployContract(wallet, RecuranceArtifact)) as Recurance;
        verifier_contract = (await deployContract(wallet, StarkDigestTestingArtifact)) as StarkDigestTesting;
    });

    // Note - This checks the proof of work, but not the whole proof yet
    it('It should validate a correct proof', async () => {
        for (let i = 19; i < recurance_proofs.length; i++) {
            // We ts-ignore because it's connivent to abi encode here not in rust
            // @ts-ignore
            recurance_proofs[i].public_inputs = utils.defaultAbiCoder.encode(
                ['uint256', 'uint64'],
                [recurance_proofs[i].public_inputs.value, recurance_proofs[i].public_inputs.index],
            );
            // NOTE - Typescript has a very very hard time with the ethers js internal array types in struct encoding
            // in this case it's best for the code to ignore it because this is how ethers js understands these types.
            // @ts-ignore
            const events = await (
                // @ts-ignore
                await verifier_contract.verify_proof(recurance_proofs[i], constraint_contract.address)
            ).wait();
            // TODO - Use better logging
            /* tslint:disable:no-console*/
            console.log('Proof verification gas used : ', events.gasUsed?.toNumber());
        }
    });
});
