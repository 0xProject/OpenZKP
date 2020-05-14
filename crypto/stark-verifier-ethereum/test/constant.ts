import {waffle} from '@nomiclabs/buidler';
import chai from 'chai';
import {deployContract, solidity} from 'ethereum-waffle';
import {utils} from 'ethers';
import fs from 'fs';

import ConstantOodsPolyArtifact from '../artifacts/ConstantOodsPoly.json';
import ConstantArtifact from '../artifacts/Constant.json';
import StarkDigestTestingArtifact from '../artifacts/StarkDigestTesting.json';
import {ConstantOodsPoly} from '../typechain/ConstantOodsPoly';
import {Constant} from '../typechain/Constant';
import {StarkDigestTesting} from '../typechain/StarkDigestTesting';

import constant_proof from './constant_proof.json';
import small_fib_proof from './small_fib_proof.json';


const INITIAL_GAS = 100000000;

chai.use(solidity);

// tslint:disable:space-before-function-paren typedef
describe('Recurrence testing', function(this: any) {
    this.timeout(0);
    let constant: Constant;
    let verifier_contract: StarkDigestTesting;
    let oods: ConstantOodsPoly;

    const provider = waffle.provider;
    const [wallet] = provider.getWallets();

    before(async () => {
        oods = (await deployContract(wallet, ConstantOodsPolyArtifact)) as ConstantOodsPoly;
        constant = (await deployContract(wallet, ConstantArtifact, [
            oods.address,
        ])) as Constant;
        verifier_contract = (await deployContract(wallet, StarkDigestTestingArtifact)) as StarkDigestTesting;
    });

    // Note - This checks the proof of work, but not the whole proof yet
    it('Should validate a correct proof', async () => {
      (await verifier_contract.verify_proof(constant_proof, constant.address)).wait();
    });
});
