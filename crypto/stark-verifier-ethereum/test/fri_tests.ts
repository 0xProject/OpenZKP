import {waffle} from '@nomiclabs/buidler';
import chai from 'chai';
import {deployContract, solidity} from 'ethereum-waffle';

import FriTestingArtifact from '../artifacts/FriTesting.json';
import {FriTesting} from '../typechain/FriTesting';

import {txToEventsAsync} from './test_utils';

chai.use(solidity);
const {expect} = chai;

describe('Public coin testing', () => {
    let fri_contract: FriTesting;

    const provider = waffle.provider;
    const [wallet] = provider.getWallets();

    before(async () => {
        fri_contract = (await deployContract(wallet, FriTestingArtifact)) as FriTesting;
    });

    it('Should correctly fold coset\'s of size 2 and three', async () => {
        // Case with 4 at top of folds
        let result = await txToEventsAsync(fri_contract.fold_coset_external(
            ["0x025906063d19f162d82b9537e4d01399ffafb20c43b6b6779642c08c03d7a026",
             "0x07d96bae76b2cd64c07304d048126543e0aa031944ba6574e4c9a24ed85b3b7f",
             "0x0230728dd5f0aabd0345ab8205a98aab01209070fea638dd316b550fecc60d27",
             "0x00ced4dae0d7364dc914d3cd14216cc5a2b86b163353b78806308adc1e679862"],
             "0x0311eb11aee4210f24fa85a3ccaa7c78a43f33701461243e4712a3b7275f962b",
            1,
            8178,
            16384,
            {
                eval_domain_generator: "0x0393a32b34832dbad650df250f673d7c5edd09f076fc314a3e5a42f0606082e1",
                log_eval_domain_size: 14,
                eval_domain_size: 16384
            }
        ));
        expect(result[0].data).to.be.eq("0x076d98ca545a7bb2f6804220f03a1aa0f2160c1425dc88fa20b5a82f08e0a773");

        // Case with 8 in a middle layer
        result = await txToEventsAsync(fri_contract.fold_coset_external(
            ["0x00f827b1e6ea6e70cfe77a0d0d821cc0556cf138a23825d69fbc1639543b1abb",
             "0x0119c0b94b13cb12f6ed296945b91d330345dca0567509a0e980ee3ad288a5cc",
             "0x022bc759d2a0b2e08587b20f6e4d2d826c8d7846792658ce4a42d56223bd78d4",
             "0x06a27ab6dab1ff98c2f276bd7511f947a631bf72302d2873c8860b663e038835",
             "0x05e694dd09794ac92338870028c76212cc73bd8e3346a5383a4f3fec81cc011a",
             "0x02cd23d4ed20dc5388fbad2d183ef5268ea07c33f5612fea22d195b1b4fe86a3",
             "0x0417fd3012a37062acf329586808911775b146eea896d66b2af6d17f139dce14",
             "0x0065e99334f710b5f4a9ea4fc836c7423c0df66875b1c883311d7a013b2872e9"],
             "0x061b74e81cc787e67822e7093f4b0b3b656302ab54d232feca07eabb6a755a16",
            8,
            876,
            2048,
            {
                eval_domain_generator: "0x0393a32b34832dbad650df250f673d7c5edd09f076fc314a3e5a42f0606082e1",
                log_eval_domain_size: 14,
                eval_domain_size: 16384
            }
        ));
        expect(result[0].data).to.be.eq("0x0027661922b9d4ec7003e8502d5de4544140930ba4b55bf40166c5b21805a4b1");
    });
});
