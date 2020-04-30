import { waffle } from '@nomiclabs/buidler';
import chai from 'chai';
import { deployContract, solidity } from 'ethereum-waffle';

import FriTestingArtifact from '../artifacts/FriTesting.json';
import { FriTesting } from '../typechain/FriTesting';

import { txToEventsAsync } from './test_utils';

chai.use(solidity);
const { expect } = chai;

describe('Fri testing', () => {
    let fri_contract: FriTesting;

    const provider = waffle.provider;
    const [wallet] = provider.getWallets();

    before(async () => {
        fri_contract = (await deployContract(wallet, FriTestingArtifact)) as FriTesting;
    });

    it("Should correctly fold coset's of size 2 and three", async () => {
        // Case with 4 at top of folds
        let result = await txToEventsAsync(
            fri_contract.fold_coset_external(
                [
                    '0x0277280e06985e84a027013e4a1a7aebb7271a1a10e3fc280490771ba0942640',
                    '0x04f9766d81268f7db239bd7e7c3dac69300cf00f65a4f1dd4b4d956cf6169a00',
                    '0x04308d177deff86b75b83c933827a325b46d3908d6fe343a6a7c2b398b0f5f07',
                    '0x078142878e2b799ecdefaa16a63e9bf60ba92d18507da426726ed0d128102211',
                ],
                '0x0311eb11aee4210f24fa85a3ccaa7c78a43f33701461243e4712a3b7275f962b',
                1,
                8178,
                16384,
                {
                    eval_domain_generator: '0x0393a32b34832dbad650df250f673d7c5edd09f076fc314a3e5a42f0606082e1',
                    log_eval_domain_size: 14,
                    eval_domain_size: 16384,
                },
            ),
        );
        expect(result[result.length - 1].data).to.be.eq('0x07ce58276a6663f522711bf9cce5bf737f296d73d32d6c8da3e17deccd459a2b');

        // Case with 8 in a middle layer
        result = await txToEventsAsync(
            fri_contract.fold_coset_external(
                [
                    '0x03f20fe20206d860ed9bffb49511d31c66e2792ef31aa8d1673bb510dff5eba9',
                    '0x06dc714ebe46aa2af98c0ac88accc42d4dad81ce3e3deb9dbf2e62616b56974f',
                    '0x02ffdcceeedbd5aeeffd9a04e5dcb9747369acc8662acec1771629e26d598a6c',
                    '0x05215c3bad6794ebca49ba68a5a1ac1d5d1bf35a02cd332113d973f24b825331',
                    '0x04b22f9fbc5c4e6ed879d67f1eedd65a6ea08017ddc3d8cda107cfb8865327e2',
                    '0x0321c1d8ef32177c5e0ee959b2a5f6c64c409ae5b32d9d599c8421ac9f40cd3f',
                    '0x070c602a8d96d2f0c0db99e668c5c3777730188c48e5d4bd5af1a602a3e22b30',
                    '0x03efaced6dd3fc8d7ccdd0189997332f48e6feaae87a2b1ef17394aa97560121',
                ],
                '0x061b74e81cc787e67822e7093f4b0b3b656302ab54d232feca07eabb6a755a16',
                8,
                876,
                2048,
                {
                    eval_domain_generator: '0x0393a32b34832dbad650df250f673d7c5edd09f076fc314a3e5a42f0606082e1',
                    log_eval_domain_size: 14,
                    eval_domain_size: 16384,
                },
            ),
        );
        expect(result[result.length - 1].data).to.be.eq('0x0121a60f28684daf933d279d490f046286a4843b889a92902c90a7a69a7e5fab');
    });
});
