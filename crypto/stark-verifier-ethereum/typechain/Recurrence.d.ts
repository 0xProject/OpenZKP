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

interface RecurrenceInterface extends Interface {
  functions: {
    constraint_calculations: TypedFunctionDescription<{
      encode([
        proof,
        params,
        queries,
        oods_point,
        constraint_coeffiencts,
        oods_coeffiencts
      ]: [
        {
          public_inputs: Arrayish;
          trace_values: BigNumberish[];
          trace_commitment: Arrayish;
          constraint_values: BigNumberish[];
          constraint_commitment: Arrayish;
          trace_oods_values: BigNumberish[];
          constraint_oods_values: BigNumberish[];
          pow_nonce: Arrayish;
          trace_decommitment: Arrayish[];
          constraint_decommitment: Arrayish[];
          fri_values: BigNumberish[][];
          fri_commitments: Arrayish[];
          fri_decommitments: Arrayish[][];
          last_layer_coefficients: BigNumberish[];
        },
        {
          number_of_columns: BigNumberish;
          log_trace_length: BigNumberish;
          number_of_constraints: BigNumberish;
          log_blowup: BigNumberish;
          constraint_degree: BigNumberish;
          pow_bits: BigNumberish;
          number_of_queries: BigNumberish;
          fri_layout: BigNumberish[];
        },
        BigNumberish[],
        BigNumberish,
        BigNumberish[],
        BigNumberish[]
      ]): string;
    }>;

    initalize_system: TypedFunctionDescription<{
      encode([public_input]: [Arrayish]): string;
    }>;
  };

  events: {};
}

export class Recurrence extends Contract {
  connect(signerOrProvider: Signer | Provider | string): Recurrence;
  attach(addressOrName: string): Recurrence;
  deployed(): Promise<Recurrence>;

  on(event: EventFilter | string, listener: Listener): Recurrence;
  once(event: EventFilter | string, listener: Listener): Recurrence;
  addListener(eventName: EventFilter | string, listener: Listener): Recurrence;
  removeAllListeners(eventName: EventFilter | string): Recurrence;
  removeListener(eventName: any, listener: Listener): Recurrence;

  interface: RecurrenceInterface;

  functions: {
    constraint_calculations(
      proof: {
        public_inputs: Arrayish;
        trace_values: BigNumberish[];
        trace_commitment: Arrayish;
        constraint_values: BigNumberish[];
        constraint_commitment: Arrayish;
        trace_oods_values: BigNumberish[];
        constraint_oods_values: BigNumberish[];
        pow_nonce: Arrayish;
        trace_decommitment: Arrayish[];
        constraint_decommitment: Arrayish[];
        fri_values: BigNumberish[][];
        fri_commitments: Arrayish[];
        fri_decommitments: Arrayish[][];
        last_layer_coefficients: BigNumberish[];
      },
      params: {
        number_of_columns: BigNumberish;
        log_trace_length: BigNumberish;
        number_of_constraints: BigNumberish;
        log_blowup: BigNumberish;
        constraint_degree: BigNumberish;
        pow_bits: BigNumberish;
        number_of_queries: BigNumberish;
        fri_layout: BigNumberish[];
      },
      queries: BigNumberish[],
      oods_point: BigNumberish,
      constraint_coeffiencts: BigNumberish[],
      oods_coeffiencts: BigNumberish[],
      overrides?: TransactionOverrides
    ): Promise<ContractTransaction>;

    initalize_system(
      public_input: Arrayish
    ): Promise<{
      0: {
        number_of_columns: number;
        log_trace_length: number;
        number_of_constraints: BigNumber;
        log_blowup: number;
        constraint_degree: number;
        pow_bits: number;
        number_of_queries: number;
        fri_layout: number[];
      };
      1: { digest: string; counter: BigNumber };
    }>;
  };

  constraint_calculations(
    proof: {
      public_inputs: Arrayish;
      trace_values: BigNumberish[];
      trace_commitment: Arrayish;
      constraint_values: BigNumberish[];
      constraint_commitment: Arrayish;
      trace_oods_values: BigNumberish[];
      constraint_oods_values: BigNumberish[];
      pow_nonce: Arrayish;
      trace_decommitment: Arrayish[];
      constraint_decommitment: Arrayish[];
      fri_values: BigNumberish[][];
      fri_commitments: Arrayish[];
      fri_decommitments: Arrayish[][];
      last_layer_coefficients: BigNumberish[];
    },
    params: {
      number_of_columns: BigNumberish;
      log_trace_length: BigNumberish;
      number_of_constraints: BigNumberish;
      log_blowup: BigNumberish;
      constraint_degree: BigNumberish;
      pow_bits: BigNumberish;
      number_of_queries: BigNumberish;
      fri_layout: BigNumberish[];
    },
    queries: BigNumberish[],
    oods_point: BigNumberish,
    constraint_coeffiencts: BigNumberish[],
    oods_coeffiencts: BigNumberish[],
    overrides?: TransactionOverrides
  ): Promise<ContractTransaction>;

  initalize_system(
    public_input: Arrayish
  ): Promise<{
    0: {
      number_of_columns: number;
      log_trace_length: number;
      number_of_constraints: BigNumber;
      log_blowup: number;
      constraint_degree: number;
      pow_bits: number;
      number_of_queries: number;
      fri_layout: number[];
    };
    1: { digest: string; counter: BigNumber };
  }>;

  filters: {};

  estimate: {
    constraint_calculations(
      proof: {
        public_inputs: Arrayish;
        trace_values: BigNumberish[];
        trace_commitment: Arrayish;
        constraint_values: BigNumberish[];
        constraint_commitment: Arrayish;
        trace_oods_values: BigNumberish[];
        constraint_oods_values: BigNumberish[];
        pow_nonce: Arrayish;
        trace_decommitment: Arrayish[];
        constraint_decommitment: Arrayish[];
        fri_values: BigNumberish[][];
        fri_commitments: Arrayish[];
        fri_decommitments: Arrayish[][];
        last_layer_coefficients: BigNumberish[];
      },
      params: {
        number_of_columns: BigNumberish;
        log_trace_length: BigNumberish;
        number_of_constraints: BigNumberish;
        log_blowup: BigNumberish;
        constraint_degree: BigNumberish;
        pow_bits: BigNumberish;
        number_of_queries: BigNumberish;
        fri_layout: BigNumberish[];
      },
      queries: BigNumberish[],
      oods_point: BigNumberish,
      constraint_coeffiencts: BigNumberish[],
      oods_coeffiencts: BigNumberish[]
    ): Promise<BigNumber>;

    initalize_system(public_input: Arrayish): Promise<BigNumber>;
  };
}
