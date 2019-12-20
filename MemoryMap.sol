pragma solidity ^0.5.0;

    contract MemoryMap {
        /*
            We store the state of the verifer in a contiguous chunk of memory.
            The offsets of the different fields are listed below.
            E.g. The offset of the i'th hash is [mm_hashes + i].
        */
    
        uint256 constant internal channel_state_size = 3;
        uint256 constant internal max_n_queries =  22;
        uint256 constant internal fri_queue_size = max_n_queries;
    
        uint256 constant internal max_supported_max_fri_step = 3;
    
        uint256 constant internal mm_eval_domain_size =                              0;
        uint256 constant internal mm_blow_up_factor =                                1;
        uint256 constant internal mm_log_eval_domain_size =                          2;
        uint256 constant internal mm_proof_of_work_bits =                            3;
        uint256 constant internal mm_eval_domain_generator =                         4;
        uint256 constant internal mm_public_input_ptr =                              5;
        uint256 constant internal mm_trace_commitment =                              6;
        uint256 constant internal mm_oods_commitment =                               7;
        uint256 constant internal mm_n_unique_queries =                              8;
        uint256 constant internal mm_channel =                                       9; // uint256[3]
        uint256 constant internal mm_merkle_queue =                                 12; // uint256[44]
        uint256 constant internal mm_fri_values =                                   56; // uint256[22]
        uint256 constant internal mm_fri_inv_points =                               78; // uint256[22]
        uint256 constant internal mm_queries =                                     100; // uint256[22]
        uint256 constant internal mm_fri_queries_delimiter =                       122;
        uint256 constant internal mm_fri_ctx =                                     123; // uint256[20]
        uint256 constant internal mm_fri_steps_ptr =                               143;
        uint256 constant internal mm_fri_eval_points =                             144; // uint256[10]
        uint256 constant internal mm_fri_commitments =                             154; // uint256[10]
        uint256 constant internal mm_fri_last_layer_deg_bound =                    164;
        uint256 constant internal mm_fri_last_layer_ptr =                          165;
    uint256 constant internal mm_batch_inverse_out =                            166;
    uint256 constant internal mm_batch_inverse_in =                            254;
    uint256 constant internal mm_constraint_poly_args_start =                  342;
    uint256 constant internal mm_oods_point =                                  342;
    uint256 constant internal mm_public0 =                            343;
    uint256 constant internal mm_public1 =                            344;
    uint256 constant internal mm_public2 =                            345;
    uint256 constant internal mm_periodic0 =                            346;
    uint256 constant internal mm_periodic1 =                            347;
    uint256 constant internal mm_periodic2 =                            348;
    uint256 constant internal mm_periodic3 =                            349;
    uint256 constant internal mm_coefficients =                                350;
    uint256 constant internal mm_oods_values =                                 408;
    uint256 constant internal mm_constraint_poly_args_end =                    424;
    uint256 constant internal mm_composition_oods_values =                     424;
    uint256 constant internal mm_oods_eval_points =                            427;
    uint256 constant internal mm_oods_coefficients =                           449;
    uint256 constant internal mm_trace_query_responses =                       467;
    uint256 constant internal mm_composition_query_responses =                 643;
    uint256 constant internal mm_trace_generator =                             687;
    uint256 constant internal mm_trace_length =                                688;
    uint256 constant internal mm_context_size =                                689;
}
