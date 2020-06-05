/* Generated by ts-generator ver. 0.0.8 */
/* tslint:disable */

import { Contract, ContractTransaction, EventFilter, Signer } from "ethers";
import { Listener, Provider } from "ethers/providers";
import { Arrayish, BigNumber, BigNumberish, Interface } from "ethers/utils";
import {
  TransactionOverrides,
  TypedEventDescription,
  TypedFunctionDescription
} from ".";

interface MerkleVerifierTestInterface extends Interface {
  functions: {
    verify_merkle_proof_external: TypedFunctionDescription<{
      encode([root, data_points, indices, decommitment]: [
        Arrayish,
        Arrayish[],
        BigNumberish[],
        Arrayish[]
      ]): string;
    }>;
  };

  events: {
    LogTrace: TypedEventDescription<{
      encodeTopics([name, enter, gasLeft, allocated]: [
        null,
        null,
        null,
        null
      ]): string[];
    }>;

    log_bool: TypedEventDescription<{ encodeTopics([data]: [null]): string[] }>;
  };
}

export class MerkleVerifierTest extends Contract {
  connect(signerOrProvider: Signer | Provider | string): MerkleVerifierTest;
  attach(addressOrName: string): MerkleVerifierTest;
  deployed(): Promise<MerkleVerifierTest>;

  on(event: EventFilter | string, listener: Listener): MerkleVerifierTest;
  once(event: EventFilter | string, listener: Listener): MerkleVerifierTest;
  addListener(
    eventName: EventFilter | string,
    listener: Listener
  ): MerkleVerifierTest;
  removeAllListeners(eventName: EventFilter | string): MerkleVerifierTest;
  removeListener(eventName: any, listener: Listener): MerkleVerifierTest;

  interface: MerkleVerifierTestInterface;

  functions: {
    verify_merkle_proof_external(
      root: Arrayish,
      data_points: Arrayish[],
      indices: BigNumberish[],
      decommitment: Arrayish[],
      overrides?: TransactionOverrides
    ): Promise<ContractTransaction>;
  };

  verify_merkle_proof_external(
    root: Arrayish,
    data_points: Arrayish[],
    indices: BigNumberish[],
    decommitment: Arrayish[],
    overrides?: TransactionOverrides
  ): Promise<ContractTransaction>;

  filters: {
    LogTrace(
      name: null,
      enter: null,
      gasLeft: null,
      allocated: null
    ): EventFilter;

    log_bool(data: null): EventFilter;
  };

  estimate: {
    verify_merkle_proof_external(
      root: Arrayish,
      data_points: Arrayish[],
      indices: BigNumberish[],
      decommitment: Arrayish[]
    ): Promise<BigNumber>;
  };
}