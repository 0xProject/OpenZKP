/* Generated by ts-generator ver. 0.0.8 */
/* tslint:disable */

import { Contract, ContractFactory, Signer } from "ethers";
import { Provider } from "ethers/providers";
import { UnsignedTransaction } from "ethers/utils/transaction";

import { TransactionOverrides } from ".";
import { ProofOfWorkTesting } from "./ProofOfWorkTesting";

export class ProofOfWorkTestingFactory extends ContractFactory {
  constructor(signer?: Signer) {
    super(_abi, _bytecode, signer);
  }

  deploy(overrides?: TransactionOverrides): Promise<ProofOfWorkTesting> {
    return super.deploy(overrides) as Promise<ProofOfWorkTesting>;
  }
  getDeployTransaction(overrides?: TransactionOverrides): UnsignedTransaction {
    return super.getDeployTransaction(overrides);
  }
  attach(address: string): ProofOfWorkTesting {
    return super.attach(address) as ProofOfWorkTesting;
  }
  connect(signer: Signer): ProofOfWorkTestingFactory {
    return super.connect(signer) as ProofOfWorkTestingFactory;
  }
  static connect(
    address: string,
    signerOrProvider: Signer | Provider
  ): ProofOfWorkTesting {
    return new Contract(address, _abi, signerOrProvider) as ProofOfWorkTesting;
  }
}

const _abi = [
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "init_digest",
        type: "bytes32"
      },
      {
        internalType: "bytes8",
        name: "pow_nonce",
        type: "bytes8"
      },
      {
        internalType: "uint8",
        name: "pow_bits",
        type: "uint8"
      }
    ],
    name: "check_proof_of_work_external",
    outputs: [
      {
        internalType: "bool",
        name: "",
        type: "bool"
      }
    ],
    stateMutability: "pure",
    type: "function"
  }
];

const _bytecode =
  "0x608060405234801561001057600080fd5b50610286806100206000396000f3fe608060405234801561001057600080fd5b506004361061002b5760003560e01c8063b8409c4914610030575b600080fd5b61007e6004803603606081101561004657600080fd5b5080359060208101357fffffffffffffffff00000000000000000000000000000000000000000000000016906040013560ff16610092565b604080519115158252519081900360200190f35b600061009c610239565b5060408051808201909152848152600060208201526100bc8185856100c5565b95945050505050565b8251604080517f0123456789abcded00000000000000000000000000000000000000000000000060208083019190915260288201939093527fff0000000000000000000000000000000000000000000000000000000000000060f885901b16604882015281516029818303018152604982018352805190840120606982018190527fffffffffffffffff0000000000000000000000000000000000000000000000008616608983015282516071818403018152609190920190925280519201919091206000919061019c868663ffffffff6101ce16565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60ff85161c10159150509392505050565b81516040805160208101929092527fffffffffffffffff00000000000000000000000000000000000000000000000083168282015280516028818403018152604890920190526000906102209061022e565b835250506000602090910152565b805160209091012090565b60408051808201909152600080825260208201529056fea264697066735822122045e59e7288228b8ac7a7233b29ecec1527adcc4e5d07d508b919df2cc13329d964736f6c63430006060033";
