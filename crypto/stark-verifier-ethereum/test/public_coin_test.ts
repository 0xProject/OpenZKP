import {waffle} from '@nomiclabs/buidler';
import chai from 'chai';
import {deployContract, solidity} from 'ethereum-waffle';

import PublicCoinTestingArtifact from '../artifacts/PublicCoinTesting.json';
import {PublicCoinTesting} from '../typechain/PublicCoinTesting';

import {txToEventsAsync} from './test_utils';

chai.use(solidity);
const {expect} = chai;

describe('Public coin testing', () => {
    let coin_contract: any;
    const init_hex = '0x0123456789abcded';

    const provider = waffle.provider;
    const [wallet] = provider.getWallets();

    before(async () => {
        coin_contract = (await deployContract(wallet, PublicCoinTestingArtifact)) as PublicCoinTesting;
    });

    it('Should read the correct data from multiple reads', async () => {
        const events = await txToEventsAsync(coin_contract.init_and_read(init_hex, 3));

        expect(events[0].data).to.eq('0x7d84f75ca3e9328b92123c1790834ee0084e02c09b379c6f95c5d2ae8739b9c8');
        expect(events[1].data).to.eq('0x4ed5f0fd8cffa8dec69beebab09ee881e7369d6d084b90208a079eedc67d2d45');
        expect(events[2].data).to.eq('0x2389a47fe0e1e5f9c05d8dcb27b069b67b1c7ec61a5c0a3f54d81aea83d2c8f0');
    });

    it('Should have the correct digest after a write', async () => {
        const events = await txToEventsAsync(
            coin_contract.init_and_write(
                init_hex,
                '0x7d84f75ca3e9328b92123c1790834ee0084e02c09b379c6f95c5d2ae8739b9c8',
            ),
        );
        expect(events[0].data).to.eq('0x3174a00d031bc8deff799e24a78ee347b303295a6cb61986a49873d9b6f13a0d');
    });
});
