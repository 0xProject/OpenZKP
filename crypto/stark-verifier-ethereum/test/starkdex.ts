import {waffle} from '@nomiclabs/buidler';
import chai from 'chai';
import {deployContract, solidity} from 'ethereum-waffle';
import {utils} from 'ethers';
import fs from 'fs';

import StarkDigestTestingArtifact from '../artifacts/StarkDigestTesting.json';
import {StarkDigestTesting} from '../typechain/StarkDigestTesting';

import StarkdexOodsPolyArtifact from '../artifacts/StarkdexOodsPoly.json';
import StarkdexArtifact from '../artifacts/Starkdex.json';
import {StarkdexOodsPoly} from '../typechain/StarkdexOodsPoly';
import {Starkdex} from '../typechain/Starkdex';
import StarkdexPeriodic0Artifact from '../artifacts/StarkdexPeriodic0.json';
import StarkdexPeriodic1Artifact from '../artifacts/StarkdexPeriodic1.json';
import StarkdexPeriodic2Artifact from '../artifacts/StarkdexPeriodic2.json';
import StarkdexPeriodic3Artifact from '../artifacts/StarkdexPeriodic3.json';
import {StarkdexPeriodic0} from '../typechain/StarkdexPeriodic0';
import {StarkdexPeriodic1} from '../typechain/StarkdexPeriodic1';
import {StarkdexPeriodic2} from '../typechain/StarkdexPeriodic2';
import {StarkdexPeriodic3} from '../typechain/StarkdexPeriodic3';

import starkdex_proof from './starkdex_proof.json';

const INITIAL_GAS = 10000000000000;

chai.use(solidity);

// tslint:disable:space-before-function-paren typedef
describe('StarkdexVerifier', function(this: any) {
    this.timeout(0);
    let periodic_column_0: StarkdexPeriodic0;
    let periodic_column_1: StarkdexPeriodic1;
    let periodic_column_2: StarkdexPeriodic2;
    let periodic_column_3: StarkdexPeriodic3;
    let starkdex: Starkdex;
    let verifier_contract: StarkDigestTesting;
    let oods: StarkdexOodsPoly;

    const provider = waffle.provider;
    const [wallet] = provider.getWallets();

    before(async () => {
        const overrideOptions = {gasLimit: 800000000, gasPrice: 100000000};
        console.log("deploying...");
        periodic_column_0 = (await deployContract(wallet, StarkdexPeriodic0Artifact, [], overrideOptions)) as StarkdexPeriodic0;
        console.log("deployed periodic_column_0");
        periodic_column_1 = (await deployContract(wallet, StarkdexPeriodic1Artifact, [], overrideOptions)) as StarkdexPeriodic1;
        console.log("deployed periodic_column_1");
        periodic_column_2 = (await deployContract(wallet, StarkdexPeriodic2Artifact, [], overrideOptions)) as StarkdexPeriodic2;
        console.log("deployed periodic_column_2");
        periodic_column_3 = (await deployContract(wallet, StarkdexPeriodic3Artifact, [], overrideOptions)) as StarkdexPeriodic3;
        console.log("deployed periodic_column_3");

        oods = (await deployContract(wallet, StarkdexOodsPolyArtifact, [], overrideOptions)) as StarkdexOodsPoly;
        console.log("oods deployed");
        starkdex = (await deployContract(wallet, StarkdexArtifact, [
            oods.address, [periodic_column_0.address, periodic_column_1.address, periodic_column_2.address, periodic_column_3.address]
        ], overrideOptions)) as Starkdex;
        console.log("starkdex deployed");
        verifier_contract = (await deployContract(wallet, StarkDigestTestingArtifact)) as StarkDigestTesting;
        console.log("verifier deployed");
    });

    // Note - This checks the proof of work, but not the whole proof yet
    it('validates a correct proof', async () => {
      console.log(oods.address);
      console.log(starkdex.address);
      console.log(verifier_contract.address);

      // @ts-ignore
      starkdex_proof.public_inputs = utils.defaultAbiCoder.encode(
          ['uint256', 'uint256', 'uint256', 'bytes32[]', 'uint256[]', 'uint256[]'],
          [1, "0x010bbb3b97e81273d77cfdf9519ac52f9f9f73377df761b41656214d346a3d6f", "0x010bbb3b97e81273d77cfdf9519ac52f9f9f73377df761b41656214d346a3d6f", [], [], []],
      );
      // @ts-ignore
      (await verifier_contract.verify_proof(starkdex_proof, starkdex.address)).wait();
    });
});
