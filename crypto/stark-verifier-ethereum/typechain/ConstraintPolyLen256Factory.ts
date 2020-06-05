/* Generated by ts-generator ver. 0.0.8 */
/* tslint:disable */

import { Contract, ContractFactory, Signer } from "ethers";
import { Provider } from "ethers/providers";
import { UnsignedTransaction } from "ethers/utils/transaction";

import { TransactionOverrides } from ".";
import { ConstraintPolyLen256 } from "./ConstraintPolyLen256";

export class ConstraintPolyLen256Factory extends ContractFactory {
  constructor(signer?: Signer) {
    super(_abi, _bytecode, signer);
  }

  deploy(overrides?: TransactionOverrides): Promise<ConstraintPolyLen256> {
    return super.deploy(overrides) as Promise<ConstraintPolyLen256>;
  }
  getDeployTransaction(overrides?: TransactionOverrides): UnsignedTransaction {
    return super.getDeployTransaction(overrides);
  }
  attach(address: string): ConstraintPolyLen256 {
    return super.attach(address) as ConstraintPolyLen256;
  }
  connect(signer: Signer): ConstraintPolyLen256Factory {
    return super.connect(signer) as ConstraintPolyLen256Factory;
  }
  static connect(
    address: string,
    signerOrProvider: Signer | Provider
  ): ConstraintPolyLen256 {
    return new Contract(
      address,
      _abi,
      signerOrProvider
    ) as ConstraintPolyLen256;
  }
}

const _abi = [
  {
    stateMutability: "nonpayable",
    type: "fallback"
  }
];

const _bytecode =
  "0x608060405234801561001057600080fd5b50610337806100206000396000f3fe608060405234801561001057600080fd5b506101e06000818182377f080000000000001100000000000000000000000000000000000000000000000161010061004a8282855161027d565b845261005a826101ff855161027d565b61020081815261006e84610101875161027d565b6102209081527ec92ecbfd121b58bc7924d5dd51a717c4d78992d0e87cd59f1bc8d7ff0cb3479250846100a38160968661027d565b86038751086102a0818152866100ba88888861027d565b88038951086102c0527f0800000000000011000000000000000000000000000000000000000000000000915086826100f489898c5161027d565b086102e05260016102405b8281101561011c57818152886060820151830991506020016100ff565b50610148887f0800000000000010ffffffffffffffffffffffffffffffffffffffffffffffff8361027d565b8291505b819250610240821161015d57610198565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe082019150888183510982528860408401518209905061014c565b505050610280868151886101ae8a60ff8a61027d565b8a038b5108096101a088828a6101c78c600286516102dd565b8c03610180510809898a8b8e51608051096060510183098c089250508883518a6101f38c60ff8c61027d565b8c038d5108099a50805189039650506101609150878a89888b86518d036101c05108080988898a885160c0510960a05101830983089a50505086610260518884845108099450868788855189510960e0510187098a0893508661024051886040518a0384510809985050508485868351610140510961012051018909830886525050505050602081f35b60006103006020815260206103205260206103405282610360528361038052846103a05260208160c0838560057ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff16102d4578182fd5b51949350505050565b60018060005b848110156102f85785848409925081016102e3565b5050939250505056fea26469706673582212206b4f5f423d6e59adee12dd89093233e8d753057daeccdf3dbe9e4a013ccce04a64736f6c63430006060033";