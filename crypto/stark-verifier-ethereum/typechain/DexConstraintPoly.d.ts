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

interface DexConstraintPolyInterface extends Interface {
  functions: {};

  events: {};
}

export class DexConstraintPoly extends Contract {
  connect(signerOrProvider: Signer | Provider | string): DexConstraintPoly;
  attach(addressOrName: string): DexConstraintPoly;
  deployed(): Promise<DexConstraintPoly>;

  on(event: EventFilter | string, listener: Listener): DexConstraintPoly;
  once(event: EventFilter | string, listener: Listener): DexConstraintPoly;
  addListener(
    eventName: EventFilter | string,
    listener: Listener
  ): DexConstraintPoly;
  removeAllListeners(eventName: EventFilter | string): DexConstraintPoly;
  removeListener(eventName: any, listener: Listener): DexConstraintPoly;

  interface: DexConstraintPolyInterface;

  functions: {};

  filters: {};

  estimate: {};
}