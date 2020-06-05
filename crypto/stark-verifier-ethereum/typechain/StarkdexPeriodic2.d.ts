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

interface StarkdexPeriodic2Interface extends Interface {
  functions: {
    evaluate: TypedFunctionDescription<{ encode([x]: [BigNumberish]): string }>;
  };

  events: {};
}

export class StarkdexPeriodic2 extends Contract {
  connect(signerOrProvider: Signer | Provider | string): StarkdexPeriodic2;
  attach(addressOrName: string): StarkdexPeriodic2;
  deployed(): Promise<StarkdexPeriodic2>;

  on(event: EventFilter | string, listener: Listener): StarkdexPeriodic2;
  once(event: EventFilter | string, listener: Listener): StarkdexPeriodic2;
  addListener(
    eventName: EventFilter | string,
    listener: Listener
  ): StarkdexPeriodic2;
  removeAllListeners(eventName: EventFilter | string): StarkdexPeriodic2;
  removeListener(eventName: any, listener: Listener): StarkdexPeriodic2;

  interface: StarkdexPeriodic2Interface;

  functions: {
    evaluate(x: BigNumberish): Promise<BigNumber>;
  };

  evaluate(x: BigNumberish): Promise<BigNumber>;

  filters: {};

  estimate: {
    evaluate(x: BigNumberish): Promise<BigNumber>;
  };
}