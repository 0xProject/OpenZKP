pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;


contract ProofTypes {
    // This struct contains all of the components of the STARK proof.
    // Please note that any input which would be a 'FieldElement' in rust
    // should be the montgomery bytes encoded field element
    // TODO - Add more structure
    struct StarkProof {
        // An array with the public inputs to the STARK
        bytes public_inputs;
        // An array with the flattened trace table decommitments
        // For a trace table with n coloums it will be length num_queries*n
        // and it will be laid out as:
        // [[query 1 col 1][query 1 col 2]...[query 1 col n]]...[[query q col 1]...[query q col n]]
        uint256[] trace_values;
        // The commitment to the trace table
        bytes32 trace_commitment;
        // The trace table evaluated constraint values at the the query indices.
        // This is also stored as a flattened array
        uint256[] constraint_values;
        // The commitment to the evaluated constraints
        bytes32 constraint_commitment;
        // The trace values used for the oods point constraint evaluation
        uint256[] trace_oods_values;
        // The constraint values used for the oods point constraint evaluation
        uint256[] constraint_oods_values;
        // The nonce used for the proof of work
        bytes8 pow_nonce;
        // The merkle decomitment for the trace values
        bytes32[] trace_decommitment;
        // The merkle decomitment for the constraint evaluated queries
        bytes32[] constraint_decommitment;
        // The values to complete each coset of fri at each layer.
        uint256[][] fri_values;
        // The roots for each fri decommitment
        bytes32[] fri_commitments;
        // The merkle proof decommitment at each fri layer
        bytes32[][] fri_decommitments;
        // The coeffiencts of the last fri layer
        uint256[] last_layer_coeffiencts;
    }

    // This struct contains the relevent information about the constraint system
    // It will be returned from a callout to the constraint system contract.
    struct ProofParameters {
        uint8 number_of_columns;
        uint8 log_trace_length;
        uint64 number_of_constraints;
        uint8 log_blowup;
        uint8 constraint_degree;
        uint8 pow_bits;
        uint8 number_of_queries;
        // TODO - Does the smaller size give us a real advantage
        uint8[] fri_layout;
    }
}
