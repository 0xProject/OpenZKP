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

    it('Sould add correctly', async () => {
        const res = await field_contract.fadd_external(
            '0x0326c9b26c9b26d064d9364d9364d9364d9364d9364d9364d9364d9364d9364e',
            '0x0326c9b26c9b26d064d9364d9364d9364d9364d9364d9364d9364d9364d9364e',
        );
        expect(res).to.eq('2850941591070285198670617950317327962006084472685500430281830104834323410076');
    });

    it('Sould multiply correctly', async () => {
        const res = await field_contract.fmul_external(
            '0x0326c9b26c9b26d064d9364d9364d9364d9364d9364d9364d9364d9364d9364e',
            '0x0326c9b26c9b26d064d9364d9364d9364d9364d9364d9364d9364d9364d9364e',
        );
        expect(res).to.eq('2565182876813823045890113120798341709404076005726347706500667646774006611397');
    });

    it('Sould pow correctly', async () => {
        let res = await txToEventsAsync(field_contract.fpow_external('0x21', '0x2'));
        expect(res[0].data).to.eq('0x0000000000000000000000000000000000000000000000000000000000000441');
        res = await txToEventsAsync(field_contract.fpow_external('0x21', '0x21'));
        expect(res[0].data).to.eq('0x00000000000000000000005857366dce0162cb5ddcd1bf0fc7c03a6438304421');
    });

    it('Sould invert correctly', async () => {
        const res = await txToEventsAsync(field_contract.inverse_external('0x21'));
        expect(res[0].data).to.eq('0x0326c9b26c9b26d064d9364d9364d9364d9364d9364d9364d9364d9364d9364e');
    });
});
