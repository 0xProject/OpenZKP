/* tslint:disable:custom-no-magic-numbers */
import {waffle} from '@nomiclabs/buidler';
import chai from 'chai';
import {deployContract, solidity} from 'ethereum-waffle';
import {utils} from 'ethers';

import ProofOfWorkTestingArtifact from '../artifacts/ProofOfWorkTesting.json';
import {ProofOfWorkTesting} from '../typechain/ProofOfWorkTesting';

chai.use(solidity);
const {expect} = chai;

describe('Proof of work testing', () => {
    let pow_contract: any;

    const provider = waffle.provider;
    const [wallet] = provider.getWallets();

    before(async () => {
        pow_contract = (await deployContract(wallet, ProofOfWorkTestingArtifact)) as ProofOfWorkTesting;
    });

    it('Should validate a correct proof of work', async () => {
        expect(
            await pow_contract.check_proof_of_work_external(
                '0x0123456789abcded0123456789abcded0123456789abcded0123456789abcded',
                '0x000000000000008A',
                8,
            ),
        ).to.eq(true);
    });

    it('Should fail an incorrect proof of work', async () => {
        expect(
            await pow_contract.check_proof_of_work_external(
                '0x0123456789abcded0123456789abcded0123456789abcded0123456789abcded',
                '0x000000000000008A',
                9,
            ),
        ).to.equal(false);
    });

    it('Should always pass a 0 bit proof of work', async () => {
        expect(await pow_contract.check_proof_of_work_external(utils.randomBytes(32), '0x000000000000008A', 0)).to.eq(
            true,
        );
    });
});
