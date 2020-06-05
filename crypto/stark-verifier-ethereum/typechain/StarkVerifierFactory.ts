/* Generated by ts-generator ver. 0.0.8 */
/* tslint:disable */

import { Contract, ContractFactory, Signer } from "ethers";
import { Provider } from "ethers/providers";
import { UnsignedTransaction } from "ethers/utils/transaction";

import { TransactionOverrides } from ".";
import { StarkVerifier } from "./StarkVerifier";

export class StarkVerifierFactory extends ContractFactory {
  constructor(signer?: Signer) {
    super(_abi, _bytecode, signer);
  }

  deploy(overrides?: TransactionOverrides): Promise<StarkVerifier> {
    return super.deploy(overrides) as Promise<StarkVerifier>;
  }
  getDeployTransaction(overrides?: TransactionOverrides): UnsignedTransaction {
    return super.getDeployTransaction(overrides);
  }
  attach(address: string): StarkVerifier {
    return super.attach(address) as StarkVerifier;
  }
  connect(signer: Signer): StarkVerifierFactory {
    return super.connect(signer) as StarkVerifierFactory;
  }
  static connect(
    address: string,
    signerOrProvider: Signer | Provider
  ): StarkVerifier {
    return new Contract(address, _abi, signerOrProvider) as StarkVerifier;
  }
}

const _abi = [
  {
    anonymous: false,
    inputs: [
      {
        indexed: false,
        internalType: "bytes32",
        name: "name",
        type: "bytes32"
      },
      {
        indexed: false,
        internalType: "bool",
        name: "enter",
        type: "bool"
      },
      {
        indexed: false,
        internalType: "uint256",
        name: "gasLeft",
        type: "uint256"
      },
      {
        indexed: false,
        internalType: "uint256",
        name: "allocated",
        type: "uint256"
      }
    ],
    name: "LogTrace",
    type: "event"
  },
  {
    inputs: [
      {
        components: [
          {
            internalType: "bytes",
            name: "public_inputs",
            type: "bytes"
          },
          {
            internalType: "uint256[]",
            name: "trace_values",
            type: "uint256[]"
          },
          {
            internalType: "bytes32",
            name: "trace_commitment",
            type: "bytes32"
          },
          {
            internalType: "uint256[]",
            name: "constraint_values",
            type: "uint256[]"
          },
          {
            internalType: "bytes32",
            name: "constraint_commitment",
            type: "bytes32"
          },
          {
            internalType: "uint256[]",
            name: "trace_oods_values",
            type: "uint256[]"
          },
          {
            internalType: "uint256[]",
            name: "constraint_oods_values",
            type: "uint256[]"
          },
          {
            internalType: "bytes8",
            name: "pow_nonce",
            type: "bytes8"
          },
          {
            internalType: "bytes32[]",
            name: "trace_decommitment",
            type: "bytes32[]"
          },
          {
            internalType: "bytes32[]",
            name: "constraint_decommitment",
            type: "bytes32[]"
          },
          {
            internalType: "uint256[][]",
            name: "fri_values",
            type: "uint256[][]"
          },
          {
            internalType: "bytes32[]",
            name: "fri_commitments",
            type: "bytes32[]"
          },
          {
            internalType: "bytes32[][]",
            name: "fri_decommitments",
            type: "bytes32[][]"
          },
          {
            internalType: "uint256[]",
            name: "last_layer_coefficients",
            type: "uint256[]"
          }
        ],
        internalType: "struct ProofTypes.StarkProof",
        name: "proof",
        type: "tuple"
      },
      {
        internalType: "contract ConstraintSystem",
        name: "constraints",
        type: "address"
      }
    ],
    name: "verify_proof",
    outputs: [
      {
        internalType: "bool",
        name: "",
        type: "bool"
      }
    ],
    stateMutability: "nonpayable",
    type: "function"
  }
];

const _bytecode =
  "0x608060405234801561001057600080fd5b50613dd6806100206000396000f3fe608060405234801561001057600080fd5b506004361061002b5760003560e01c80632c912efc14610030575b600080fd5b61004361003e36600461330a565b610059565b6040516100509190613849565b60405180910390f35b60006100867f7665726966795f70726f6f6600000000000000000000000000000000000000006001610547565b6100b17f696e6974616c697a655f73797374656d000000000000000000000000000000006001610547565b6100b9612de2565b6100c1612e28565b84516040517ff837656b00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff86169163f837656b916101149190600401613878565b60006040518083038186803b15801561012c57600080fd5b505afa158015610140573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052610186919081019061320b565b915091506101b57f696e6974616c697a655f73797374656d000000000000000000000000000000006000610547565b6101e07f77726974655f646174615f616e645f726561645f72616e646f6d0000000000006001610547565b606060006060806101f289878761058c565b93509350935093506102257f77726974655f646174615f616e645f726561645f72616e646f6d0000000000006000610547565b610238858a60e001518860a0015161070c565b610277576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161026e90613af9565b60405180910390fd5b60608601516020870151016102ad7f6765745f717565726965730000000000000000000000000000000000000000006001610547565b60606102be87838a60c001516107cd565b90506102eb7f6765745f717565726965730000000000000000000000000000000000000000006000610547565b6103167f636f6e73747261696e745f63616c63756c6174696f6e730000000000000000006001610547565b606060008b73ffffffffffffffffffffffffffffffffffffffff1663cdb7ef438e8c868b8d8c6040518763ffffffff1660e01b815260040161035d96959493929190613b30565b600060405180830381600087803b15801561037757600080fd5b505af115801561038b573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526103d1919081019061316e565b915091506104007f636f6e73747261696e745f63616c63756c6174696f6e730000000000000000006000610547565b60608a015160208b0151016104178e8c86846108e2565b6104298e8c60e0015188848888610ba0565b6104676040518060400160405280600a81526020017f6f6f64735f706f696e7400000000000000000000000000000000000000000000815250610c5d565b61047861047389610d5a565b610da7565b6104b66040518060400160405280601f81526020017f636f6e73747261696e745f6576616c75617465645f6f6f64735f706f696e7400815250610c5d565b6104c261047383610d5a565b6104cd8e8984610e5d565b61050b6040518060400160405280600981526020017f6f6f647320646f6e650000000000000000000000000000000000000000000000815250610c5d565b6105367f7665726966795f70726f6f6600000000000000000000000000000000000000006000610547565b505050505050505050505092915050565b60005a6040519091507f7c410723af298384134622201acb6634aca1f7fd03cf697165325fac19a4078161057e858585858061385d565b60405180910390a150505050565b606060006060806105aa876040015186610ff590919063ffffffff16565b6105ce866040015160020267ffffffffffffffff168661101390919063ffffffff16565b93506105e7876080015186610ff590919063ffffffff16565b6105f085611097565b92506106098760a001518661112190919063ffffffff16565b60c087015161061f90869063ffffffff61112116565b60c08701515160a08801515161063d9187910163ffffffff61101316565b91508560e001515167ffffffffffffffff8111801561065b57600080fd5b50604051908082528060200260200182016040528015610685578160200160208202803683370190505b50905060005b8660e00151518110156106eb576106c388610160015182815181106106ac57fe5b602002602001015187610ff590919063ffffffff16565b6106cc86611097565b8282815181106106d857fe5b602090810291909101015260010161068b565b506101a087015161070390869063ffffffff61116016565b93509350935093565b6000808460000151836040516020016107269291906137ee565b604051602081830303815290604052805190602001209050600081856040516020016107539291906137a2565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190528051602090910120905061079b868663ffffffff61119e16565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60ff85161c10159150509392505050565b6060808260ff1667ffffffffffffffff811180156107ea57600080fd5b50604051908082528060200260200182016040528015610814578160200160208202803683370190505b5090507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60ff851660020a0160005b600460ff86160460ff1681116108cf57600061085e886111ba565b905060005b60048110156108c5578660ff1681846004020110156108bd57838260c01c16858285600402018151811061089357fe5b602002602001019067ffffffffffffffff16908167ffffffffffffffff1681525050604082901b91505b600101610863565b5050600101610843565b506108d9826111fa565b50949350505050565b61090d7f636865636b5f636f6d6d69746d656e74730000000000000000000000000000006001610547565b60608360c0015160ff1667ffffffffffffffff8111801561092d57600080fd5b50604051908082528060200260200182016040528015610957578160200160208202803683370190505b5090506060835167ffffffffffffffff8111801561097457600080fd5b5060405190808252806020026020018201604052801561099e578160200160208202803683370190505b50905060008360ff1660020a90506109ea6040518060400160405280600c81526020017f707265706172696e672e2e2e0000000000000000000000000000000000000000815250610c5d565b610a038760200151876000015160ff16878487876112f5565b610a416040518060400160405280600e81526020017f646f6e6520707265706172696e67000000000000000000000000000000000000815250610c5d565b610a56876040015184848a61010001516114d4565b610a8c576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161026e90613a54565b610aca6040518060400160405280600e81526020017f6d6f726520707265706172696e67000000000000000000000000000000000000815250610c5d565b610ae38760600151876080015160ff16878487876112f5565b610af8876080015184848a61012001516114d4565b610b2e576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161026e9061391b565b610b6c6040518060400160405280601381526020017f646f6e65206d6f726520707265706172696e6700000000000000000000000000815250610c5d565b610b977f636865636b5f636f6d6d69746d656e74730000000000000000000000000000006000610547565b50505050505050565b610bcb7f6672695f636865636b00000000000000000000000000000000000000000000006001610547565b610c2a6040518061012001604052808861014001518152602001886101600151815260200188610180015181526020018781526020018681526020018560ff168152602001848152602001838152602001886101a00151815250611666565b610c557f6672695f636865636b00000000000000000000000000000000000000000000006000610547565b505050505050565b60006a636f6e736f6c652e6c6f6773ffffffffffffffffffffffffffffffffffffffff1682604051602401610c929190613878565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529181526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f41304fac0000000000000000000000000000000000000000000000000000000017905251610d1391906137d2565b600060405180830381855afa9150503d8060008114610d4e576040519150601f19603f3d011682016040523d82523d6000602084013e610d53565b606091505b5050505050565b60007f08000000000000110000000000000000000000000000000000000000000000017e40000000000001100000000000012100000000000000000000000000000000830990505b919050565b60006a636f6e736f6c652e6c6f6773ffffffffffffffffffffffffffffffffffffffff1682604051602401610ddc9190613854565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529181526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f27b7cf850000000000000000000000000000000000000000000000000000000017905251610d1391906137d2565b610e887f636865636b5f6f75745f6f665f646f6d61696e5f73616d706c650000000000006001610547565b600080610e956001611dcb565b905060005b8560c0015151811015610f02576000610ed3838860c001518481518110610ebd57fe5b6020026020010151611e1790919063ffffffff16565b9050610ee5848263ffffffff611e5216565b9350610ef7838763ffffffff611e1716565b925050600101610e9a565b50610f416040518060400160405280600681526020017f726573756c740000000000000000000000000000000000000000000000000000815250610c5d565b610f4a82610da7565b610f886040518060400160405280601481526020017f6576616c75617465645f6f6f64735f706f696e74000000000000000000000000815250610c5d565b610f9183610da7565b828214610fca576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161026e90613a8b565b610d537f636865636b5f6f75745f6f665f646f6d61696e5f73616d706c650000000000006000610547565b6000611005836000015183611e7f565b600060208501529092525050565b6060808267ffffffffffffffff8111801561102d57600080fd5b50604051908082528060200260200182016040528015611057578160200160208202803683370190505b50905060005b8381101561108d5761106e85611097565b82828151811061107a57fe5b602090810291909101015260010161105d565b5090505b92915050565b6000806110a3836111ba565b7f0fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff1690505b7f08000000000000110000000000000000000000000000000000000000000000018110611091576110f8836111ba565b7f0fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff1690506110c8565b60005b815181101561115b57600082828151811061113b57fe5b602002602001015160001b90506111528482610ff5565b50600101611124565b505050565b600061119083600001518360405160200161117c929190613764565b604051602081830303815290604052611eb2565b835250506000602090910152565b600061119083600001518360405160200161117c9291906137a2565b6000806111dc8360000151846020015167ffffffffffffffff1660001b611e7f565b602093909301805160010167ffffffffffffffff1690525090919050565b60005b81518110156112f157805b600081118015611254575082600182038151811061122257fe5b602002602001015167ffffffffffffffff1683828151811061124057fe5b602002602001015167ffffffffffffffff16105b156112e85782600182038151811061126857fe5b602002602001015183828151811061127c57fe5b602002602001015184838151811061129057fe5b602002602001018560018503815181106112a657fe5b67ffffffffffffffff9384166020918202929092010152911690527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff01611208565b506001016111fd565b5050565b6113336040518060400160405280601a81526020017f707265706172655f6861736865735f616e645f71756572696573000000000000815250610c5d565b60608567ffffffffffffffff8111801561134c57600080fd5b50604051908082528060200260200182016040528015611376578160200160208202803683370190505b5090506113b96040518060400160405280601281526020017f646174615f67726f7570732e6c656e67746800000000000000000000000000008152508851611ebd565b6113f86040518060400160405280600f81526020017f646174615f67726f75705f73697a65000000000000000000000000000000000081525087611ebd565b60005b8688518161140557fe5b048110156114785760005b8781101561144f578881898402018151811061142857fe5b602002602001015183828151811061143c57fe5b6020908102919091010152600101611410565b5061145982611fb0565b84828151811061146557fe5b60209081029190910101526001016113fb565b50611489858363ffffffff61200b16565b60005b85518110156114ca57848382815181106114a257fe5b6020026020010151018382815181106114b757fe5b602090810291909101015260010161148c565b5050505050505050565b60006115017f7665726966795f6d65726b6c655f70726f6f66000000000000000000000000006001610547565b825184511461153c576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161026e90613ac2565b6000845111611577576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161026e906139e6565b825160209485019493840193929092019160051b6000805b81860151828801516001821660051b52836020840106925060018114156115bd578860205114945050611630565b86830151600182171480156115e0578389015160205284602085010693506115f2565b8651600183191660051b526020870196505b5060011c8682015260406000207fffffffffffffffffffffffffffffffffffffffff000000000000000000000000168782015260200182900661158f565b50505061165e7f7665726966795f6d65726b6c655f70726f6f66000000000000000000000000006000610547565b949350505050565b6116917f666f6c645f616e645f636865636b5f6672695f6c6179657273000000000000006001610547565b611699612e3f565b604080516101c08101909152600160c08083019182527f080000000000001100000000000000000000000000000000000000000000000060e08401527f0625023929a2995b533120664329f8c7c5268e56ac8320da2a616626f41337e36101008401527f01dafdc6d65d66b5accedf99bcd607383ad971a9537cdf25d59e99d90becc81e6101208401527f063365fe0de874d9c90adb1e2f9c676e98c62155e4412e873ada5e1dee6feebb6101408401527f01cc9a01f2178b3736f524e1d06398916739deaa1bbed178c525a1e2119011466101608401527f03b912c31d6a226e4a15988c6b7ec1915474043aac68553537192090b43635cd6101808401527f0446ed3ce295dda2b5ea677394813e6eab8bfbc55397aacac8e6df6f4bc9ca346101a084015290825283015151602082019067ffffffffffffffff811180156117e157600080fd5b5060405190808252806020026020018201604052801561180b578160200160208202803683370190505b5081526020018360a0015160ff166001901b81526020018360a0015160ff168152602001836060015160008151811061184057fe5b602002602001015160ff166001901b8152602001600081525090506118688160400151612054565b60a082015260c08201515160609067ffffffffffffffff8111801561188c57600080fd5b506040519080825280602002602001820160405280156118b6578160200160208202803683370190505b50905060608360c001515167ffffffffffffffff811180156118d757600080fd5b50604051908082528060200260200182016040528015611901578160200160208202803683370190505b50905061192f7f696e69745f785f696e76000000000000000000000000000000000000000000006001610547565b60005b8460c00151518110156119bf5760008560c00151828151811061195157fe5b602002602001015167ffffffffffffffff16905061197c85606001518261210c90919063ffffffff16565b604086015160a0870151919003915061199b908263ffffffff61217d16565b856020015183815181106119ab57fe5b602090810291909101015250600101611932565b506119eb7f696e69745f785f696e76000000000000000000000000000000000000000000006000610547565b60005b846060015151811015611c0c5784606001518181518110611a0b57fe5b602002602001015160ff166001901b846080018181525050600884608001511115611a62576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161026e90613978565b611aae8560e001518660c0015187600001518481518110611a7f57fe5b6020026020010151611aa789608001518681518110611a9a57fe5b6020026020010151610d5a565b88876121aa565b60c0850151611ac3908463ffffffff61200b16565b82518560c001515114611af85760c085015151611ae6848263ffffffff6124c016565b611af6838263ffffffff6124c016565b505b60008460800151856040015181611b0b57fe5b04905060005b8451811015611b405781858281518110611b2757fe5b6020908102919091010180519091019052600101611b11565b50611b7b86602001518381518110611b5457fe5b6020026020010151848689604001518681518110611b6e57fe5b60200260200101516114d4565b611bb1576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161026e906138ad565b604085018190526060860151805183908110611bc957fe5b602002602001015160ff16856060018181510391508181525050611bfe85608001518660a0015161217d90919063ffffffff16565b60a0860152506001016119ee565b50611c387f6c6173745f6c61796572000000000000000000000000000000000000000000006001610547565b60005b8460e0015151811015611d6e5760008560c001518281518110611c5a57fe5b602002602001015167ffffffffffffffff169050611c8585606001518261210c90919063ffffffff16565b90506000611ca0828760a0015161217d90919063ffffffff16565b9050611ccd7f686f726e65725f6576616c0000000000000000000000000000000000000000006001610547565b610100870151600090611ce6908363ffffffff6124ff16565b9050611d137f686f726e65725f6576616c0000000000000000000000000000000000000000006000610547565b8760e001518481518110611d2357fe5b60200260200101518114611d63576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161026e90613a1d565b505050600101611c3b565b50611d9a7f6c6173745f6c61796572000000000000000000000000000000000000000000006000610547565b611dc57f666f6c645f616e645f636865636b5f6672695f6c6179657273000000000000006000610547565b50505050565b60007f08000000000000110000000000000000000000000000000000000000000000017f07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1830992915050565b6000611e4b611e268484612858565b7e40000000000001100000000000012100000000000000000000000000000000612858565b9392505050565b60007f08000000000000110000000000000000000000000000000000000000000000018284089392505050565b60008282604051602001611e94929190613794565b60405160208183030381529060405280519060200120905092915050565b805160209091012090565b60006a636f6e736f6c652e6c6f6773ffffffffffffffffffffffffffffffffffffffff168383604051602401611ef492919061388b565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529181526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f9710a9d00000000000000000000000000000000000000000000000000000000017905251611f7591906137d2565b600060405180830381855afa9150503d8060008114610c55576040519150601f19603f3d011682016040523d82523d6000602084013e610c55565b6000815160011415611fdb5781600081518110611fc957fe5b602002602001015160001b9050610da2565b50805160209081029101207fffffffffffffffffffffffffffffffffffffffff0000000000000000000000001690565b60005b825181101561115b5782818151811061202357fe5b602002602001015167ffffffffffffffff1682828151811061204157fe5b602090810291909101015260010161200e565b6000817f08000000000000110000000000000000000000000000000000000000000000008161207f57fe5b06156120b7576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161026e906138e4565b6110916003837f0800000000000011000000000000000000000000000000000000000000000000816120e557fe5b047f0800000000000011000000000000000000000000000000000000000000000001612885565b6103ff60ff80841664020202020290810265010884422010908116849006600890811b87821c85168402831686900617811b601088901c85168402831686900617811b601888901c94909416830282168590069390931790921b60209590951c021606919091176028919091031c90565b6000611e4b83837f0800000000000011000000000000000000000000000000000000000000000001612885565b6121d57f666f6c645f6c61796572000000000000000000000000000000000000000000006001610547565b60808201516000908190819060609067ffffffffffffffff811180156121fa57600080fd5b50604051908082528060200260200182016040528015612224578160200160208202803683370190505b5090505b895184101561246957600089858151811061223f57fe5b602002602001015167ffffffffffffffff16905060008760800151828161226257fe5b0682039050600088608001518201905060008960200151888151811061228457fe5b602002602001015190506122b58a60000151848603600881106122a357fe5b6020020151829063ffffffff61285816565b90506122e27f666f6c645f6c617965725f636f6c6c65637400000000000000000000000000006001610547565b825b828110156123945780851415612357578e898151811061230057fe5b6020026020010151868583038151811061231657fe5b6020026020010181815250506001890198508d51891015612352578d898151811061233d57fe5b602002602001015167ffffffffffffffff1694505b61238c565b8c878151811061236357fe5b6020026020010151868583038151811061237957fe5b6020026020010181815250506001870196505b6001016122e4565b506123c07f666f6c645f6c617965725f636f6c6c65637400000000000000000000000000006000610547565b6123c985611fb0565b8988815181106123d557fe5b6020026020010181815250506123ec85828d6128cd565b8f89815181106123f857fe5b602002602001018c602001518a8151811061240f57fe5b60209081029190910101919091525260808a0151838161242b57fe5b048d888151811061243857fe5b602002602001019067ffffffffffffffff16908167ffffffffffffffff168152505060018701965050505050612228565b6124798a8463ffffffff6124c016565b612489898463ffffffff6124c016565b6124b47f666f6c645f6c61796572000000000000000000000000000000000000000000006000610547565b50505050505050505050565b80825110156124fb576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161026e906139af565b9052565b81516000907f080000000000001100000000000000000000000000000000000000000000000190801561285057602085018160051b8101602081035b60088411156127c6577f080000000000001100000000000000000000000000000000000000000000000187870981510195507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0017f080000000000001100000000000000000000000000000000000000000000000187870981510195507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0017f080000000000001100000000000000000000000000000000000000000000000187870981510195507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0017f080000000000001100000000000000000000000000000000000000000000000187870981510195507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0017f080000000000001100000000000000000000000000000000000000000000000187870981510195507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0017f080000000000001100000000000000000000000000000000000000000000000187870981510195507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0017f080000000000001100000000000000000000000000000000000000000000000187870981510195507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0017f080000000000001100000000000000000000000000000000000000000000000187870981510195507ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8909301927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe00161253b565b5b8281111561281f577f080000000000001100000000000000000000000000000000000000000000000187870981510195507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0016127c7565b50507f0800000000000011000000000000000000000000000000000000000000000001858509935082815185089350505b505092915050565b60007f08000000000000110000000000000000000000000000000000000000000000018284099392505050565b600060405160208152602080820152602060408201528460608201528360808201528260a082015260208160c08360006005600019f16128c457600080fd5b51949350505050565b6000806128fb7f666f6c645f636f736574000000000000000000000000000000000000000000006001610547565b60007f08000000000000110000000000000000000000000000000000000000000000018585099050855160081415612c185760006129628760008151811061293f57fe5b60200260200101518860018151811061295457fe5b602002602001015184612d86565b905060006129e38860028151811061297657fe5b60200260200101518960038151811061298b57fe5b60200260200101517f0800000000000011000000000000000000000000000000000000000000000001806129bb57fe5b7f01dafdc6d65d66b5accedf99bcd607383ad971a9537cdf25d59e99d90becc81e8709612d86565b90506000612a64896004815181106129f757fe5b60200260200101518a600581518110612a0c57fe5b60200260200101517f080000000000001100000000000000000000000000000000000000000000000180612a3c57fe5b7f0446ed3ce295dda2b5ea677394813e6eab8bfbc55397aacac8e6df6f4bc9ca348809612d86565b90506000612ae58a600681518110612a7857fe5b60200260200101518b600781518110612a8d57fe5b60200260200101517f080000000000001100000000000000000000000000000000000000000000000180612abd57fe5b7f01cc9a01f2178b3736f524e1d06398916739deaa1bbed178c525a1e2119011468909612d86565b90507f08000000000000110000000000000000000000000000000000000000000000018586099450612b18848487612d86565b9350612b6882827f08000000000000110000000000000000000000000000000000000000000000017f01dafdc6d65d66b5accedf99bcd607383ad971a9537cdf25d59e99d90becc81e8909612d86565b92507f08000000000000110000000000000000000000000000000000000000000000018586099450612b9b848487612d86565b96507f0800000000000011000000000000000000000000000000000000000000000001898a0998507f0800000000000011000000000000000000000000000000000000000000000001898a0998507f0800000000000011000000000000000000000000000000000000000000000001898a09955050505050612d52565b855160041415612cd0576000612c348760008151811061293f57fe5b90506000612c488860028151811061297657fe5b90507f08000000000000110000000000000000000000000000000000000000000000018384099250612c7b828285612d86565b94507f080000000000001100000000000000000000000000000000000000000000000187880996507f080000000000001100000000000000000000000000000000000000000000000187880993505050612d52565b855160021415612d3a57612d0d86600081518110612cea57fe5b602002602001015187600181518110612cff57fe5b602002602001015183612d86565b92507f08000000000000110000000000000000000000000000000000000000000000018586099150612d52565b85600081518110612d4757fe5b602002602001015192505b612d7d7f666f6c645f636f736574000000000000000000000000000000000000000000006000610547565b50935093915050565b60008284017f08000000000000110000000000000000000000000000000000000000000000018486038101907f080000000000001100000000000000000000000000000000000000000000000182860983089695505050505050565b604080516101008101825260008082526020820181905291810182905260608082018390526080820183905260a0820183905260c082019290925260e081019190915290565b604080518082019091526000808252602082015290565b6040518060c00160405280612e52612e7b565b815260200160608152602001600081526020016000815260200160008152602001600081525090565b6040518061010001604052806008906020820280368337509192915050565b600082601f830112612eaa578081fd5b8135612ebd612eb882613d54565b613d2d565b818152915060208083019084810160005b84811015612ef757612ee5888484358a0101612f5a565b84529282019290820190600101612ece565b505050505092915050565b600082601f830112612f12578081fd5b8135612f20612eb882613d54565b818152915060208083019084810160005b84811015612ef757612f48888484358a0101612f5a565b84529282019290820190600101612f31565b600082601f830112612f6a578081fd5b8135612f78612eb882613d54565b818152915060208083019084810181840286018201871015612f9957600080fd5b60005b84811015612ef757813584529282019290820190600101612f9c565b600082601f830112612fc8578081fd5b8151612fd6612eb882613d54565b818152915060208083019084810181840286018201871015612ff757600080fd5b6000805b8581101561302557825160ff81168114613013578283fd5b85529383019391830191600101612ffb565b50505050505092915050565b80357fffffffffffffffff0000000000000000000000000000000000000000000000008116811461109157600080fd5b600082601f830112613071578081fd5b813567ffffffffffffffff811115613087578182fd5b6130b860207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601613d2d565b91508082528360208285010111156130cf57600080fd5b8060208401602084013760009082016020015292915050565b803573ffffffffffffffffffffffffffffffffffffffff8116811461109157600080fd5b60006040828403121561311d578081fd5b6131276040613d2d565b90508151815261313a8360208401613145565b602082015292915050565b805167ffffffffffffffff8116811461109157600080fd5b805160ff8116811461109157600080fd5b60008060408385031215613180578182fd5b825167ffffffffffffffff811115613196578283fd5b80840185601f8201126131a7578384fd5b805191506131b7612eb883613d54565b8083825260208083019250808401898283880287010111156131d7578788fd5b8794505b858510156131f95780518452600194909401939281019281016131db565b50969096015195979596505050505050565b6000806060838503121561321d578182fd5b825167ffffffffffffffff80821115613234578384fd5b610100918501808703831315613248578485fd5b61325183613d2d565b61325b888361315d565b815261326a886020840161315d565b602082015261327c8860408401613145565b604082015261328e886060840161315d565b60608201526132a0886080840161315d565b60808201526132b28860a0840161315d565b60a08201526132c48860c0840161315d565b60c082015260e08201519350828411156132dc578586fd5b6132e888858401612fb8565b60e082015280955050505050613301846020850161310c565b90509250929050565b6000806040838503121561331c578182fd5b823567ffffffffffffffff80821115613333578384fd5b6101c0918501808703831315613347578485fd5b61335083613d2d565b8135935082841115613360578586fd5b61336c88858401613061565b81526020820135935082841115613381578586fd5b61338d88858401612f5a565b60208201526040820135604082015260608201359350828411156133af578586fd5b6133bb88858401612f5a565b60608201526080820135608082015260a08201359350828411156133dd578586fd5b6133e988858401612f5a565b60a082015260c0820135935082841115613401578586fd5b61340d88858401612f5a565b60c082015261341f8860e08401613031565b60e082015261010093508382013583811115613439578687fd5b61344589828501612f5a565b85830152506101209350838201358381111561345f578687fd5b61346b89828501612f5a565b858301525061014093508382013583811115613485578687fd5b61349189828501612f02565b8583015250610160935083820135838111156134ab578687fd5b6134b789828501612f5a565b8583015250610180935083820135838111156134d1578687fd5b6134dd89828501612e9a565b85830152506101a0935083820135838111156134f7578687fd5b61350389828501612f5a565b85830152508095505050505061330184602085016130e8565b6000815180845260208085018081965082840281019150828601855b858110156135625782840389526135508483516135b5565b98850198935090840190600101613538565b5091979650505050505050565b6000815180845260208085018081965082840281019150828601855b858110156135625782840389526135a38483516135b5565b9885019893509084019060010161358b565b6000815180845260208085019450808401835b838110156135e4578151875295820195908201906001016135c8565b509495945050505050565b6000815180845260208085019450808401835b838110156135e457815167ffffffffffffffff1687529582019590820190600101613602565b6000815180845260208085019450808401835b838110156135e457815160ff168752958201959082019060010161363b565b7fffffffffffffffff000000000000000000000000000000000000000000000000169052565b60008151808452613698816020860160208601613d74565b601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169290920160200192915050565b600061010060ff835116845260ff602084015116602085015267ffffffffffffffff604084015116604085015260ff60608401511660608501526080830151613716608086018261375d565b5060a083015161372960a086018261375d565b5060c083015161373c60c086018261375d565b5060e08301518160e086015261375482860182613628565b95945050505050565b60ff169052565b600083825260208083018451819150828601845b8281101561356257815184529284019290840190600101613778565b918252602082015260400190565b9182527fffffffffffffffff00000000000000000000000000000000000000000000000016602082015260280190565b600082516137e4818460208701613d74565b9190910192915050565b7f0123456789abcded0000000000000000000000000000000000000000000000008152600881019290925260f81b7fff0000000000000000000000000000000000000000000000000000000000000016602882015260290190565b901515815260200190565b90815260200190565b93845291151560208401526040830152606082015260800190565b600060208252611e4b6020830184613680565b60006040825261389e6040830185613680565b90508260208301529392505050565b6020808252601e908201527f467269206d65726b6c6520766572696669636174696f6e206661696c65640000604082015260600190565b60208082526010908201527f526f6f7420756e617661696c61626c6500000000000000000000000000000000604082015260600190565b60208082526022908201527f436f6e73747261696e7420636f6d6d69746d656e742070726f6f66206661696c60408201527f6564000000000000000000000000000000000000000000000000000000000000606082015260800190565b6020808252600f908201527f436f73657420746f6f206c617267650000000000000000000000000000000000604082015260600190565b6020808252600d908201527f536872696e6b204661696c656400000000000000000000000000000000000000604082015260600190565b6020808252600f908201527f4e6f20636c61696d656420646174610000000000000000000000000000000000604082015260600190565b6020808252601e908201527f4c617374206c6179657220636f65666669656e7473206d69736d617463680000604082015260600190565b6020808252601d908201527f547261636520636f6d6d69746d656e742070726f6f66206661696c6564000000604082015260600190565b6020808252600d908201527f4f6f6473206d69736d6174636800000000000000000000000000000000000000604082015260600190565b6020808252600d908201527f496e76616c696420696e70757400000000000000000000000000000000000000604082015260600190565b6020808252600a908201527f504f57204661696c656400000000000000000000000000000000000000000000604082015260600190565b600060c0825287516101c060c0840152613b4e610280840182613680565b60208a015191507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff40808583030160e0860152613b8a82846135b5565b60408c015161010087015260608c015186820383016101208801529350613bb181856135b5565b92505060808b01519250610140838187015260a08c01519350610160828785030181880152613be084866135b5565b60c08e015195508388820301610180890152613bfc81876135b5565b94505060e08d015194506101a0613c158189018761365a565b6101008e0151955083888603016101c0890152613c3285876135b5565b6101208f0151965084898203016101e08a0152613c4f81886135b5565b955050828e015195508388860301610200890152613c6d858761356f565b9250818e015195508388840301610220890152613c8a83876135b5565b94506101808e015195508388860301610240890152613ca9858761351c565b9250808e0151955050508186820301610260870152613cc881856135b5565b925050508381036020850152613cde818a6136ca565b9150508281036040840152613cf381886135ef565b8660608501528381036080850152613d0b81876135b5565b91505082810360a0840152613d2081856135b5565b9998505050505050505050565b60405181810167ffffffffffffffff81118282101715613d4c57600080fd5b604052919050565b600067ffffffffffffffff821115613d6a578081fd5b5060209081020190565b60005b83811015613d8f578181015183820152602001613d77565b83811115611dc5575050600091015256fea264697066735822122095aa76112707c476530ab0f1d3016593a11a2f36c2cb4cbda79071d3c2c2447364736f6c63430006060033";
