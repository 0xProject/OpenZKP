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

interface GreeterInterface extends Interface {
  functions: {
    greet: TypedFunctionDescription<{ encode([]: []): string }>;

    setGreeting: TypedFunctionDescription<{
      encode([_greeting]: [string]): string;
    }>;
  };

  events: {};
}

export class Greeter extends Contract {
  connect(signerOrProvider: Signer | Provider | string): Greeter;
  attach(addressOrName: string): Greeter;
  deployed(): Promise<Greeter>;

  on(event: EventFilter | string, listener: Listener): Greeter;
  once(event: EventFilter | string, listener: Listener): Greeter;
  addListener(eventName: EventFilter | string, listener: Listener): Greeter;
  removeAllListeners(eventName: EventFilter | string): Greeter;
  removeListener(eventName: any, listener: Listener): Greeter;

  interface: GreeterInterface;

  functions: {
    greet(): Promise<string>;

    setGreeting(
      _greeting: string,
      overrides?: TransactionOverrides
    ): Promise<ContractTransaction>;
  };

  greet(): Promise<string>;

  setGreeting(
    _greeting: string,
    overrides?: TransactionOverrides
  ): Promise<ContractTransaction>;

  filters: {};

  estimate: {
    greet(): Promise<BigNumber>;

    setGreeting(_greeting: string): Promise<BigNumber>;
  };
}
