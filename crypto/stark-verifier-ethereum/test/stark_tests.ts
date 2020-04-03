/* tslint:disable:custom-no-magic-numbers */
import {waffle} from '@nomiclabs/buidler';
import chai from 'chai';
import {deployContract, solidity} from 'ethereum-waffle';
import {utils} from 'ethers';

import StarkDigestTestingArtifact from '../artifacts/StarkDigestTesting.json';
import TrivialFibArtifact from '../artifacts/TrivialFib.json';
import {StarkDigestTesting} from '../typechain/StarkDigestTesting';
import {TrivialFib} from '../typechain/TrivialFib';

import {testing_proof} from './testing_proof';

chai.use(solidity);
const {expect} = chai;

describe('Stark Testing testing', () => {
    let constraint_contract: TrivialFib;
    let verifier_contract: StarkDigestTesting;

    const provider = waffle.provider;
    const [wallet] = provider.getWallets();

    before(async () => {
        constraint_contract = (await deployContract(wallet, TrivialFibArtifact)) as TrivialFib;
        verifier_contract = (await deployContract(wallet, StarkDigestTestingArtifact)) as StarkDigestTesting;
    });

    it('It should have the correct random digest after writing and reading', async () => {
        // NOTE - Typescript has a very very hard time with the ethers js internal array types in struct encoding
        // in this case it's best for the code to ignore it because this is how ethers js understands these types.
        // @ts-ignore
        const return_data = await verifier_contract.digest_read(testing_proof, constraint_contract.address);
        expect(return_data).to.be.eq('0xb8fa751e9886b6eccc725754333f339b7bc9024f38a44e75468fb4b16e1709cc');
    });

    it('It should have the correct queries after reading', async () => {
        // NOTE - Typescript has a very very hard time with the ethers js internal array types in struct encoding
        // in this case it's best for the code to ignore it because this is how ethers js understands these types.
        // @ts-ignore
        const return_data = await verifier_contract.queries_read(testing_proof, constraint_contract.address);
        const converted = return_data.map(x => x.toNumber());
        expect(converted).to.be.deep.equal([
            1089,
            1175,
            1750,
            2590,
            2747,
            4172,
            4304,
            4443,
            4534,
            5373,
            6525,
            7804,
            9317,
            9568,
            11715,
            12372,
            12762,
            14035,
            14823,
            15894,
        ]);
    });

    // Note - This checks the proof of work, but not the whole proof yet
    it('It should validate a correct proof', async () => {
        // NOTE - Typescript has a very very hard time with the ethers js internal array types in struct encoding
        // in this case it's best for the code to ignore it because this is how ethers js understands these types.
        // @ts-ignore
        const return_data = await verifier_contract.verify_proof(testing_proof, constraint_contract.address);
        expect(return_data);
    });
});
