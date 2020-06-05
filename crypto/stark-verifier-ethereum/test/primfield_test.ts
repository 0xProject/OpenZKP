import {waffle} from '@nomiclabs/buidler';
import chai from 'chai';
import {deployContract, solidity} from 'ethereum-waffle';

import PrimeFieldTesterArtifact from '../artifacts/PrimeFieldTester.json';
import {PrimeFieldTester} from '../typechain/PrimeFieldTester';

import {txToEventsAsync} from './test_utils';

chai.use(solidity);
const {expect} = chai;

describe('Public coin testing', () => {
    let field_contract: any;

    const provider = waffle.provider;
    const [wallet] = provider.getWallets();

    before(async () => {
        field_contract = (await deployContract(wallet, PrimeFieldTesterArtifact)) as PrimeFieldTester;
    });

    it('Should add correctly', async () => {
        const res = await field_contract.fadd_external(
            '0x0326c9b26c9b26d064d9364d9364d9364d9364d9364d9364d9364d9364d9364e',
            '0x0326c9b26c9b26d064d9364d9364d9364d9364d9364d9364d9364d9364d9364e',
        );
        expect(res).to.eq('2850941591070285198670617950317327962006084472685500430281830104834323410076');
    });

    it('Should multiply correctly', async () => {
        const res = await field_contract.fmul_external(
            '0x0326c9b26c9b26d064d9364d9364d9364d9364d9364d9364d9364d9364d9364e',
            '0x0326c9b26c9b26d064d9364d9364d9364d9364d9364d9364d9364d9364d9364e',
        );
        expect(res).to.eq('2565182876813823045890113120798341709404076005726347706500667646774006611397');
    });

    it('Should pow correctly', async () => {
        let res = await txToEventsAsync(field_contract.fpow_external(0x21, 2));
        expect(res[0].data).to.eq('0x0000000000000000000000000000000000000000000000000000000000000441');
        res = await txToEventsAsync(
            field_contract.fpow_external('0x0393a32b34832dbad650df250f673d7c5edd09f076fc314a3e5a42f0606082e1', 13825),
        );
        expect(res[0].data).to.eq('0x07b29494e473ce930b6238d02250fdbde4f31c35b05d1e7026e082c068ece7e7');
    });

    it('Should invert correctly', async () => {
        const result = await (await field_contract.inverse_external('0x21')).wait();
        const res = result.events;
        expect(res[0].data).to.eq('0x0326c9b26c9b26d064d9364d9364d9364d9364d9364d9364d9364d9364d9364e');
        // TODO - Use better logging
        /* tslint:disable:no-console*/
        console.log('Inverse gas used : ', result.gasUsed.sub(22592).toNumber());
    });
});
