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

interface StarkdexPeriodic0Interface extends Interface {
  functions: {
    evaluate: TypedFunctionDescription<{ encode([x]: [BigNumberish]): string }>;
  };

  events: {};
}

export class StarkdexPeriodic0 extends Contract {
  connect(signerOrProvider: Signer | Provider | string): StarkdexPeriodic0;
  attach(addressOrName: string): StarkdexPeriodic0;
  deployed(): Promise<StarkdexPeriodic0>;

  on(event: EventFilter | string, listener: Listener): StarkdexPeriodic0;
  once(event: EventFilter | string, listener: Listener): StarkdexPeriodic0;
  addListener(
    eventName: EventFilter | string,
    listener: Listener
  ): StarkdexPeriodic0;
  removeAllListeners(eventName: EventFilter | string): StarkdexPeriodic0;
  removeListener(eventName: any, listener: Listener): StarkdexPeriodic0;

  interface: StarkdexPeriodic0Interface;

  functions: {
    evaluate(x: BigNumberish): Promise<BigNumber>;
  };

  evaluate(x: BigNumberish): Promise<BigNumber>;

  filters: {};

  estimate: {
    evaluate(x: BigNumberish): Promise<BigNumber>;
  };
}