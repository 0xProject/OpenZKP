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

import starkdex_proof from './starkdex_proof.json';

const INITIAL_GAS = 10000000000000;

chai.use(solidity);

// tslint:disable:space-before-function-paren typedef
describe('StarkdexVerifier', function(this: any) {
    this.timeout(0);
    let starkdex: Starkdex;
    let verifier_contract: StarkDigestTesting;
    let oods: StarkdexOodsPoly;

    const provider = waffle.provider;
    const [wallet] = provider.getWallets();

    before(async () => {
        console.log("deploying...");
        oods = (await deployContract(wallet, StarkdexOodsPolyArtifact)) as StarkdexOodsPoly;
        console.log("oods deployed");
        starkdex = (await deployContract(wallet, StarkdexArtifact, [
            oods.address,
        ])) as Starkdex;
        console.log("starkdex deployed");
        verifier_contract = (await deployContract(wallet, StarkDigestTestingArtifact)) as StarkDigestTesting;
        console.log("verifier deployed");
    });

    // Note - This checks the proof of work, but not the whole proof yet
    it('validates a correct proof', async () => {
      // console.log(oods.address);
      // console.log(starkdex.address);
      // console.log(verifier_contract.address);

      // @ts-ignore
      starkdex_proof.public_inputs = utils.defaultAbiCoder.encode(
          ['uint256', 'uint256', 'uint256'],
          [1, "0x010bbb3b97e81273d77cfdf9519ac52f9f9f73377df761b41656214d346a3d6f", "0x010bbb3b97e81273d77cfdf9519ac52f9f9f73377df761b41656214d346a3d6f"],
      );
      // @ts-ignore
      (await verifier_contract.verify_proof(constant_proof, constant.address)).wait();
    });
});
