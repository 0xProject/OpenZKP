/* Generated by ts-generator ver. 0.0.8 */
/* tslint:disable */

import { Contract, ContractFactory, Signer } from "ethers";
import { Provider } from "ethers/providers";
import { UnsignedTransaction } from "ethers/utils/transaction";

import { Greeter } from "./Greeter";

export class GreeterFactory extends ContractFactory {
  constructor(signer?: Signer) {
    super(_abi, _bytecode, signer);
  }

  deploy(_greeting: string): Promise<Greeter> {
    return super.deploy(_greeting) as Promise<Greeter>;
  }
  getDeployTransaction(_greeting: string): UnsignedTransaction {
    return super.getDeployTransaction(_greeting);
  }
  attach(address: string): Greeter {
    return super.attach(address) as Greeter;
  }
  connect(signer: Signer): GreeterFactory {
    return super.connect(signer) as GreeterFactory;
  }
  static connect(
    address: string,
    signerOrProvider: Signer | Provider
  ): Greeter {
    return new Contract(address, _abi, signerOrProvider) as Greeter;
  }
}

const _abi = [
  {
    inputs: [
      {
        internalType: "string",
        name: "_greeting",
        type: "string"
      }
    ],
    payable: false,
    stateMutability: "nonpayable",
    type: "constructor"
  },
  {
    constant: true,
    inputs: [],
    name: "greet",
    outputs: [
      {
        internalType: "string",
        name: "",
        type: "string"
      }
    ],
    payable: false,
    stateMutability: "view",
    type: "function"
  },
  {
    constant: false,
    inputs: [
      {
        internalType: "string",
        name: "_greeting",
        type: "string"
      }
    ],
    name: "setGreeting",
    outputs: [],
    payable: false,
    stateMutability: "nonpayable",
    type: "function"
  }
];

const _bytecode =
  "0x60806040523480156200001157600080fd5b5060405162000cff38038062000cff833981810160405260208110156200003757600080fd5b81019080805160405193929190846401000000008211156200005857600080fd5b838201915060208201858111156200006f57600080fd5b82518660018202830111640100000000821117156200008d57600080fd5b8083526020830192505050908051906020019080838360005b83811015620000c3578082015181840152602081019050620000a6565b50505050905090810190601f168015620000f15780820380516001836020036101000a031916815260200191505b506040525050506200012860405180606001604052806022815260200162000cdd60229139826200014860201b6200062a1760201c565b80600090805190602001906200014092919062000377565b505062000426565b60006a636f6e736f6c652e6c6f6773ffffffffffffffffffffffffffffffffffffffff168383604051602401808060200180602001838103835285818151815260200191508051906020019080838360005b83811015620001b75780820151818401526020810190506200019a565b50505050905090810190601f168015620001e55780820380516001836020036101000a031916815260200191505b50838103825284818151815260200191508051906020019080838360005b838110156200022057808201518184015260208101905062000203565b50505050905090810190601f1680156200024e5780820380516001836020036101000a031916815260200191505b509450505050506040516020818303038152906040527f4b5c4277000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040518082805190602001908083835b60208310620003075780518252602082019150602081019050602083039250620002e2565b6001836020036101000a038019825116818451168082178552505050505050905001915050600060405180830381855afa9150503d806000811462000369576040519150601f19603f3d011682016040523d82523d6000602084013e6200036e565b606091505b50509050505050565b828054600181600116156101000203166002900490600052602060002090601f016020900481019282601f10620003ba57805160ff1916838001178555620003eb565b82800160010185558215620003eb579182015b82811115620003ea578251825591602001919060010190620003cd565b5b509050620003fa9190620003fe565b5090565b6200042391905b808211156200041f57600081600090555060010162000405565b5090565b90565b6108a780620004366000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c8063a41368621461003b578063cfae3217146100f6575b600080fd5b6100f46004803603602081101561005157600080fd5b810190808035906020019064010000000081111561006e57600080fd5b82018360208201111561008057600080fd5b803590602001918460018302840111640100000000831117156100a257600080fd5b91908080601f016020809104026020016040519081016040528093929190818152602001838380828437600081840152601f19601f820116905080830192505050505050509192919290505050610179565b005b6100fe610250565b6040518080602001828103825283818151815260200191508051906020019080838360005b8381101561013e578082015181840152602081019050610123565b50505050905090810190601f16801561016b5780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b6102366040518060600160405280602381526020016108506023913960008054600181600116156101000203166002900480601f01602080910402602001604051908101604052809291908181526020018280546001816001161561010002031660029004801561022b5780601f106102005761010080835404028352916020019161022b565b820191906000526020600020905b81548152906001019060200180831161020e57829003601f168201915b5050505050836102f2565b806000908051906020019061024c929190610585565b5050565b606060008054600181600116156101000203166002900480601f0160208091040260200160405190810160405280929190818152602001828054600181600116156101000203166002900480156102e85780601f106102bd576101008083540402835291602001916102e8565b820191906000526020600020905b8154815290600101906020018083116102cb57829003601f168201915b5050505050905090565b60006a636f6e736f6c652e6c6f6773ffffffffffffffffffffffffffffffffffffffff1684848460405160240180806020018060200180602001848103845287818151815260200191508051906020019080838360005b83811015610364578082015181840152602081019050610349565b50505050905090810190601f1680156103915780820380516001836020036101000a031916815260200191505b50848103835286818151815260200191508051906020019080838360005b838110156103ca5780820151818401526020810190506103af565b50505050905090810190601f1680156103f75780820380516001836020036101000a031916815260200191505b50848103825285818151815260200191508051906020019080838360005b83811015610430578082015181840152602081019050610415565b50505050905090810190601f16801561045d5780820380516001836020036101000a031916815260200191505b5096505050505050506040516020818303038152906040527f2ced7cef000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040518082805190602001908083835b6020831061051657805182526020820191506020810190506020830392506104f3565b6001836020036101000a038019825116818451168082178552505050505050905001915050600060405180830381855afa9150503d8060008114610576576040519150601f19603f3d011682016040523d82523d6000602084013e61057b565b606091505b5050905050505050565b828054600181600116156101000203166002900490600052602060002090601f016020900481019282601f106105c657805160ff19168380011785556105f4565b828001600101855582156105f4579182015b828111156105f35782518255916020019190600101906105d8565b5b5090506106019190610605565b5090565b61062791905b8082111561062357600081600090555060010161060b565b5090565b90565b60006a636f6e736f6c652e6c6f6773ffffffffffffffffffffffffffffffffffffffff168383604051602401808060200180602001838103835285818151815260200191508051906020019080838360005b8381101561069757808201518184015260208101905061067c565b50505050905090810190601f1680156106c45780820380516001836020036101000a031916815260200191505b50838103825284818151815260200191508051906020019080838360005b838110156106fd5780820151818401526020810190506106e2565b50505050905090810190601f16801561072a5780820380516001836020036101000a031916815260200191505b509450505050506040516020818303038152906040527f4b5c4277000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040518082805190602001908083835b602083106107e157805182526020820191506020810190506020830392506107be565b6001836020036101000a038019825116818451168082178552505050505050905001915050600060405180830381855afa9150503d8060008114610841576040519150601f19603f3d011682016040523d82523d6000602084013e610846565b606091505b5050905050505056fe4368616e67696e67206772656574696e672066726f6d202725732720746f2027257327a265627a7a72315820999ef1b160db239081c743645d82926f60cafa0af0739319215383d7822fc1cf64736f6c634300050f00324465706c6f79696e67206120477265657465722077697468206772656574696e673a";
