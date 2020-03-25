import { deployContract, solidity } from "ethereum-waffle";
import { waffle } from "@nomiclabs/buidler";
import chai from "chai";
import {tx_to_events} from './test_utils';

import MerkleVerifierTestArtifact from "../artifacts/MerkleVerifierTest.json";
import { MerkleVerifierTest } from "../typechain/MerkleVerifierTest";

chai.use(solidity);
const { expect } = chai;

describe("Merkle Testing testing", () => {
  let merkle_contract: any;
  const init_hex = "0x0123456789abcded";

  const provider = waffle.provider;
  let [wallet] = provider.getWallets();

  before(async function() {
    merkle_contract = await deployContract(wallet, MerkleVerifierTestArtifact) as MerkleVerifierTest;
  });

  it("It should verify a valid proof", async function() {
      let claimed_data = [
          "0x0000000000000000000000000000000000000000000000000000000000000533",
          "0x000000000000000000000000000000000000000000000000000000000000242d",
          "0x0000000000000000000000000000000000000000000000000000000000003600"];
      let data_indexes = [1 + 64, 11 + 64, 14 + 64];
      let root = "0xfd112f44bc944f33e2567f86eea202350913b11c000000000000000000000000";
      let decommitment = [
        "0x00000000000000000000000000000000000000000000000000000000000003e8",
        "0x0000000000000000000000000000000000000000000000000000000000001f40",
        "0x0000000000000000000000000000000000000000000000000000000000003d09",
        "0x4ea8b9bafb11dafcfe132a26f8e343eaef0651d9000000000000000000000000",
        "0x023a7ce535cadd222093be053ac26f9b800ee476000000000000000000000000",
        "0x70b0744af2583d10e7e3236c731d37605e196e06000000000000000000000000",
        "0x221aea6e87862ba2d03543d0aa82c6bffee310ae000000000000000000000000",
        "0x68b58e5131703684edb16d41b763017dfaa24a35000000000000000000000000",
        "0xe108b7dc670810e8588c67c2fde7ec4cc00165e8000000000000000000000000",
      ];

      let data = await merkle_contract.verify_merkle_proof_external(root, claimed_data, data_indexes, decommitment);
      expect(data);
  });

  it("It should verify a valid proof, with no decommitment", async function() {
    let claimed_data = [
        "0x00000000000000000000000000000000000000000000000000000000000003e8",
        "0x0000000000000000000000000000000000000000000000000000000000000533",
        "0x00000000000000000000000000000000000000000000000000000000000006c0",
        "0x0000000000000000000000000000000000000000000000000000000000000895",
        "0x0000000000000000000000000000000000000000000000000000000000000ab8",
        "0x0000000000000000000000000000000000000000000000000000000000000d2f",
        "0x0000000000000000000000000000000000000000000000000000000000001000",
        "0x0000000000000000000000000000000000000000000000000000000000001331"];
    let data_indexes = [0 + 8, 1 + 8, 2 + 8, 3 + 8, 4 + 8, 5 + 8, 6 + 8, 7 + 8];
    let root = "0xa438a228f242643e8accf6466333b760095bfe34000000000000000000000000";
    let decommitment: any[] = [];

    let data = await merkle_contract.verify_merkle_proof_external(root, claimed_data, data_indexes, decommitment);
    expect(data);
});

  it("It should fail invalid a valid proofs", async function() {
    let claimed_data = [
        "0x0000000000000000000000000000000000000000000000000000000000000533",
        "0x000000000000000000000000000000000000000000000000000000000000242d",
        "0x0000000000000000000000000000000000000000000000000000000000003600"];
    let data_indexes = [1 + 64, 11 + 64, 14 + 64];
    let root = "0xfd112f44bc944f33e2567f86eea202350913b11c000000000000000000000000";
    let decommitment = [
      "0x00000000000000000000000000000000000000000000000000000000000003e8",
      "0x0000000000000000000000000000000000000000000000000000000000001f40",
      "0x0000000000000000000000000000000000000000000000000000000000003d09",
      "0x4ea8b9bafb11dafcfe132a26f8e343eaef0651d9000000000000000000000000",
      "0x023a7ce535cadd222093be053ac26f9b800ee476000000000000000000000000",
      "0x70b0744af2583d10e7e3236c731d37605e196e06000000000000000000000000",
      "0x221aea6e87862ba2d03543d0aa82c6bffee310ae000000000000000000000000",
      "0x68b58e5131703684edb16d41b763017dfaa24a35000000000000000000000000",
      "0xe108b7dc670810e8588c67c2fde7ec4cc00165e8000000000000000000000000",
    ];

    // Fails with wrong root
    let data = await merkle_contract.verify_merkle_proof_external("0xad112f44bc944f33e2567f86eea202350913b11c000000000000000000000000", claimed_data, data_indexes, decommitment);
    expect(!data);
    // Fails with wrong decommitment
    data = await merkle_contract.verify_merkle_proof_external(root, claimed_data, data_indexes, decommitment.slice(1));
    expect(!data);
    // Fails with wrong values
    data_indexes[0] = 64;
    data = await merkle_contract.verify_merkle_proof_external(root, claimed_data, data_indexes, decommitment.slice(1));
    expect(!data);
    // Reverts when called with no data
    try {
        await merkle_contract.verify_merkle_proof_external(root, [], [], decommitment);
    } catch (err) {
        expect(err.message).to.be.eq("VM Exception while processing transaction: revert No claimed data");
    }
});
});
