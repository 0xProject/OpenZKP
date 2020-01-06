// TODO - Format the hash map returns to remove this warning
#![allow(unused_results)]
use crate::rational_expression::*;
use std::{
    cmp::Ordering, collections::HashMap, error::Error, fs::File, io::prelude::*, path::Path,
    prelude::v1::*,
};
use zkp_primefield::{FieldElement, Pow, Root};
use zkp_u256::U256;

// Contains the offsets from the context data array pointer to the named values
#[derive(Debug)]
pub struct ContextArray {
    pub n_unique_queries:    usize,
    pub fri_inverses:        usize,
    pub fri_values:          usize,
    pub trace_queries:       usize,
    pub composition_queries: usize,
    pub denominators:        usize,
    pub oods_coefficients:   usize,
    pub oods_values:         usize,
    pub composition_oods:    usize,
}

impl ContextArray {
    fn new() -> Self {
        Self {
            n_unique_queries:    0,
            fri_inverses:        0,
            fri_values:          0,
            trace_queries:       0,
            composition_queries: 0,
            denominators:        0,
            oods_coefficients:   0,
            oods_values:         0,
            composition_oods:    0,
        }
    }
}

// Please note this function assumes a rational expression which is a polynomial
// over X^len_poly
pub fn autogen_periodic(
    periodic: &RationalExpression,
    index: usize,
    name: &str,
) -> Result<(), std::io::Error> {
    let name = format!("contracts/{}.sol", name);
    let path = Path::new(&name);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    println!("{}: {:?}", index, periodic);

    if let RationalExpression::Polynomial(poly, _) = periodic {
        writeln!(
            &mut file,
            "pragma solidity ^0.5.11;

contract perodic{} {{
    function evaluate(uint x) external pure returns (uint y){{
        assembly {{
                let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001
                y := 0x0",
            index
        )?;
        for coef in poly.coefficients().iter().rev() {
            writeln!(
                &mut file,
                "                y := addmod(mulmod(x, y, PRIME), 0x{}, PRIME)",
                U256::from(coef).to_string()
            )?;
        }
        writeln!(
            &mut file,
            "        }}
    }}
}}"
        )?;
    }
    Ok(())
}

pub fn autogen(
    trace_len: usize,
    public: &[&RationalExpression],
    constraints: &[RationalExpression],
    n_rows: usize,
    n_cols: usize,
) -> Result<(), std::io::Error> {
    let generator = FieldElement::root(trace_len).unwrap();
    let mut traces = HashMap::new();
    let mut inverses = HashMap::new();
    let mut periodic = HashMap::new();
    for exp in constraints.iter() {
        traces.extend(exp.trace_search());
        inverses.extend(exp.inv_search());
        periodic.extend(exp.periodic_search());
    }

    let path = Path::new("contracts/ConstraintPoly.sol");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Note that because the hash map strategy used here is they keys are in
    // arbitrary orders. We want to enforce the restriction that the trace ones
    // be in lexicographic order though.
    let mut trace_keys: Vec<&RationalExpression> = traces.keys().collect();
    trace_keys.sort_by(|a, b| lexicographic_compare(a, b));
    let inverse_keys: Vec<&RationalExpression> = inverses.keys().collect();
    // TODO - sorting periodic keys
    let periodic_keys: Vec<&RationalExpression> = periodic.keys().collect();
    for (index, col) in periodic_keys.iter().enumerate() {
        autogen_periodic(col, index, &format!("periodic{}", index))?;
    }
    let max_degree = constraints
        .iter()
        .map(|c| {
            let (numerator_degree, denominator_degree) = c.trace_degree();
            numerator_degree - denominator_degree
        })
        .max()
        .expect("No constraints");
    let target_degree = trace_len * max_degree - 1;
    let adjustment_degrees: Vec<usize> = constraints
        .iter()
        .map(|x| {
            let (num, den) = x.degree(trace_len - 1);
            target_degree + den - num
        })
        .collect();

    let ctx_layout = autogen_memory_layout(
        constraints.len(),
        public.len(),
        periodic_keys.len(),
        n_rows,
        n_cols,
        trace_keys.len(),
        max_degree,
    )?;
    autogen_oods(trace_keys.as_slice(), &generator, max_degree, &ctx_layout)?;
    let memory_map = setup_call_memory(
        &mut file,
        constraints.len(),
        public,
        inverse_keys.as_slice(),
        trace_keys.as_slice(),
        periodic_keys.as_slice(),
        adjustment_degrees.as_slice(),
    )?;

    let mut coefficient_index = 1 + public.len() + periodic_keys.len();
    for (exp, &degree) in constraints.iter().zip(adjustment_degrees.iter()) {
        writeln!(&mut file, "      {{")?;
        writeln!(
            &mut file,
            "        let val := {}",
            exp.soldity_encode(&memory_map)
        )?;
        writeln!(
            &mut file,
            "        res := addmod(res, mulmod(val, add(mload({}), mulmod(mload({}), {}, PRIME)), \
             PRIME),PRIME)",
            coefficient_index * 32,
            (coefficient_index + 1) * 32,
            memory_map
                .get(&RationalExpression::Exp(
                    RationalExpression::X.into(),
                    degree
                ))
                .unwrap()
        )?;
        writeln!(&mut file, "      }}")?;
        coefficient_index += 2;
    }
    writeln!(
        &mut file,
        "      mstore(0, res)\n      return(0, 0x20)\n  }}\n  }}\n}}"
    )?;
    Ok(())
}

#[allow(clippy::too_many_lines)]
pub fn autogen_memory_layout(
    num_constraints: usize,
    n_public_in: usize,
    n_perodic: usize,
    n_rows: usize,
    n_cols: usize,
    mask_size: usize,
    degree_bound: usize,
) -> Result<ContextArray, std::io::Error> {
    let mut ctx = ContextArray::new();
    ctx.n_unique_queries = 8;
    ctx.fri_inverses = 78;
    ctx.fri_values = 56;

    let max_query_size = 22;
    let path = Path::new("contracts/MemoryMap.sol");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };
    writeln!(
        &mut file,
        "pragma solidity ^0.5.11;

    contract MemoryMap {{
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
        uint256 constant internal mm_channel =                                       9; // \
         uint256[3]
        uint256 constant internal mm_merkle_queue =                                 12; // \
         uint256[44]
        uint256 constant internal mm_fri_values =                                   56; // \
         uint256[22]
        uint256 constant internal mm_fri_inv_points =                               78; // \
         uint256[22]
        uint256 constant internal mm_queries =                                     100; // \
         uint256[22]
        uint256 constant internal mm_fri_queries_delimiter =                       122;
        uint256 constant internal mm_fri_ctx =                                     123; // \
         uint256[20]
        uint256 constant internal mm_fri_steps_ptr =                               143;
        uint256 constant internal mm_fri_eval_points =                             144; // \
         uint256[10]
        uint256 constant internal mm_fri_commitments =                             154; // \
         uint256[10]
        uint256 constant internal mm_fri_last_layer_deg_bound =                    164;
        uint256 constant internal mm_fri_last_layer_ptr =                          165;"
    )?;

    let mut index = 166;
    ctx.denominators = index;
    writeln!(
        &mut file,
        "    uint256 constant internal mm_batch_inverse_out =                            {};",
        index
    )?;
    index += max_query_size * (n_rows + 2);
    writeln!(
        &mut file,
        "    uint256 constant internal mm_batch_inverse_in =                            {};",
        index
    )?;
    index += max_query_size * (n_rows + 2);

    // Formats memory for the constraint calculation call
    writeln!(
        &mut file,
        "    uint256 constant internal mm_constraint_poly_args_start =                  {};
    uint256 constant internal mm_oods_point =                                  {};",
        index, index
    )?;
    index += 1;

    for i in 0..n_public_in {
        writeln!(
            &mut file,
            "    uint256 constant internal mm_public{} =                            {};",
            i, index
        )?;
        index += 1;
    }

    for i in 0..n_perodic {
        writeln!(
            &mut file,
            "    uint256 constant internal mm_periodic{} =                            {};",
            i, index
        )?;
        index += 1;
    }

    writeln!(
        &mut file,
        "    uint256 constant internal mm_coefficients =                                {};",
        index
    )?;
    index += num_constraints * 2;
    ctx.oods_values = index;
    writeln!(
        &mut file,
        "    uint256 constant internal mm_oods_values =                                 {};",
        index
    )?;
    index += mask_size;
    writeln!(
        &mut file,
        "    uint256 constant internal mm_constraint_poly_args_end =                    {};",
        index
    )?;

    // Formats memory used for oods virtual oracle
    ctx.composition_oods = index;
    writeln!(
        &mut file,
        "    uint256 constant internal mm_composition_oods_values =                     {};",
        index
    )?;
    index += degree_bound + 1;
    writeln!(
        &mut file,
        "    uint256 constant internal mm_oods_eval_points =                            {};",
        index
    )?;
    index += max_query_size;
    ctx.oods_coefficients = index;
    writeln!(
        &mut file,
        "    uint256 constant internal mm_oods_coefficients =                           {};",
        index
    )?;
    index += mask_size + degree_bound;
    ctx.trace_queries = index;
    writeln!(
        &mut file,
        "    uint256 constant internal mm_trace_query_responses =                       {};",
        index
    )?;
    index += max_query_size * n_cols;
    ctx.composition_queries = index;
    writeln!(
        &mut file,
        "    uint256 constant internal mm_composition_query_responses =                 {};",
        index
    )?;
    index += degree_bound * max_query_size;
    writeln!(
        &mut file,
        "    uint256 constant internal mm_trace_generator =                             {};
    uint256 constant internal mm_trace_length =                                {};
    uint256 constant internal mm_context_size =                                {};",
        index,
        index + 1,
        index + 2
    )?;
    writeln!(&mut file, "}}")?;

    Ok(ctx)
}

#[allow(clippy::too_many_lines)]
pub fn setup_call_memory(
    file: &mut File,
    num_constraints: usize,
    public_inputs: &[&RationalExpression],
    inverses: &[&RationalExpression],
    traces: &[&RationalExpression],
    periodic: &[&RationalExpression],
    adjustment_degrees: &[usize],
) -> Result<HashMap<RationalExpression, String>, std::io::Error> {
    let mut index = 1; // Note index 0 is taken by the oods_point
    let mut memory_lookups: HashMap<RationalExpression, String> = HashMap::new();
    for &exp in public_inputs.iter() {
        memory_lookups.insert(exp.clone(), format!("mload({})", index * 32));
        index += 1;
    }
    for &exp in periodic.iter() {
        memory_lookups.insert(exp.clone(), format!("mload({})", index * 32));
        index += 1;
    }
    // Layout the constraints
    index += num_constraints * 2;
    // Note that the trace values must be the last inputs from the contract to make
    // the memory layout defaults work.
    for &exp in traces.iter() {
        memory_lookups.insert(exp.clone(), format!("mload({})", index * 32));
        index += 1;
    }
    let in_data_size = index;
    // Here we need to add an output which writelns denominator storage and batch
    // inversion

    let mut held = "".to_owned();
    // We put the degree adjustment calculation into the memory map: Note this means
    // that if the exp used is used in non adjustment places in the constraints
    // those will now load this [in some cases]
    for &degree in adjustment_degrees {
        let implied_expression = RationalExpression::Exp(RationalExpression::X.into(), degree);
        // TODO - Clean this pattern
        #[allow(clippy::map_entry)]
        let flag = !memory_lookups.contains_key(&implied_expression);
        if flag {
            held.extend(
                format!(
                    "        mstore({},expmod(mload(0x0), {}, PRIME))\n",
                    index * 32,
                    degree
                )
                .chars(),
            );
            memory_lookups.insert(implied_expression, format!("mload({})", index * 32));
            index += 1;
        }
    }

    let inverse_start_index = index;
    index += inverses.len();

    writeln!(
        file,
        "pragma solidity ^0.5.11;

contract OodsPoly {{
    function() external {{
          uint256 res;
          assembly {{
            PRIME := 0x800000000000011000000000000000000000000000000000000000000000001
            // NOTE - If compilation hits a stack depth error on variable PRIME,
            // then uncomment the following line and globally replace PRIME with mload({})
            // mstore({}, 0x800000000000011000000000000000000000000000000000000000000000001)
            // Copy input from calldata to memory.
            calldatacopy(0x0, 0x0, /*input_data_size*/ {})

            function expmod(base, exponent, modulus) -> res {{
                let p := /*expmod_context*/ {}
                mstore(p, 0x20)                 // Length of Base
                mstore(add(p, 0x20), 0x20)      // Length of Exponent
                mstore(add(p, 0x40), 0x20)      // Length of Modulus
                mstore(add(p, 0x60), base)      // Base
                mstore(add(p, 0x80), exponent)  // Exponent
                mstore(add(p, 0xa0), modulus)   // Modulus
                // call modexp precompile
                if iszero(call(not(0), 0x05, 0, p, 0xc0, p, 0x20)) {{
                    revert(0, 0)
                }}
                res := mload(p)
            }}
    
            function degree_adjustment(composition_polynomial_degree_bound, constraint_degree, \
         numerator_degree,
                denominator_degree) -> res {{
                                        res := sub(sub(composition_polynomial_degree_bound, 1),
                       sub(add(constraint_degree, numerator_degree), denominator_degree))
                    }}
    
            function small_expmod(x, num, prime) -> res {{
                res := 1
                for {{ let ind := 0 }} lt(ind, num) {{ ind := add(ind, 1) }} {{
                       res := mulmod(res, x, prime)
                }}
            }}",
        (index + inverses.len() + 6) * 32,
        (index + inverses.len() + 6) * 32,
        in_data_size * 32,
        (index + inverses.len()) * 32
    )?;

    writeln!(file, "        // Store adjustment degrees")?;
    writeln!(file, "{}", held)?;
    writeln!(
        file,
        "        // Store the values which will be batch inverted"
    )?;
    let mut inverse_position = inverse_start_index;
    for &exp in inverses.iter() {
        if let RationalExpression::Inv(a) = exp {
            writeln!(
                file,
                "        mstore({}, {})",
                index * 32,
                a.soldity_encode(&memory_lookups)
            )?;
        } else {
            panic!("Inverse search returned a non inverse");
        }
        // Out batch inversion will place the final inverted product before the
        // calculated denom
        memory_lookups.insert(exp.clone(), format!("mload({})", inverse_position * 32));
        inverse_position += 1;
        index += 1;
    }

    writeln!(
        file,
        "      {{
        // Compute the inverses of the denominators into denominator_invs using batch inverse.

        // Start by computing the cumulative product.
        // Let (d_0, d_1, d_2, ..., d_{{n-1}}) be the values in denominators. Then after this loop
        // denominator_invs will be (1, d_0, d_0 * d_1, ...) and prod will contain the value of
        // d_0 * ... * d_{{n-1}}.
        // Compute the offset between the partial_products array and the input values array.
        let products_to_values_offset := {}
        let prod := 1
        let partial_product_end_ptr := {}
        for {{ let partial_product_ptr := {} }}
            lt(partial_product_ptr, partial_product_end_ptr)
            {{ partial_product_ptr := add(partial_product_ptr, 0x20) }} {{
            mstore(partial_product_ptr, prod)
            // prod *= d_{{i}}.
            prod := mulmod(prod,
                           mload(add(partial_product_ptr, products_to_values_offset)),
                           PRIME)
        }}

        let first_partial_product_ptr := {}
        // Compute the inverse of the product.
        let prod_inv := expmod(prod, sub(PRIME, 2), PRIME)

        // Compute the inverses.
        // Loop over denominator_invs in reverse order.
        // current_partial_product_ptr is initialized to one past the end.
        for {{ let current_partial_product_ptr := {}
            }} gt(current_partial_product_ptr, first_partial_product_ptr) {{ }} {{
            current_partial_product_ptr := sub(current_partial_product_ptr, 0x20)
            // Store 1/d_{{i}} = (d_0 * ... * d_{{i-1}}) * 1/(d_0 * ... * d_{{i}}).
            mstore(current_partial_product_ptr,
                   mulmod(mload(current_partial_product_ptr), prod_inv, PRIME))
            // Update prod_inv to be 1/(d_0 * ... * d_{{i-1}}) by multiplying by d_i.
            prod_inv := mulmod(prod_inv,
                               mload(add(current_partial_product_ptr, products_to_values_offset)),
                               PRIME)
        }}
      }}",
        inverses.len() * 32,
        (inverse_start_index + inverses.len()) * 32,
        (inverse_start_index) * 32,
        inverse_start_index * 32,
        (inverse_start_index + inverses.len()) * 32
    )?;

    Ok(memory_lookups)
}

#[allow(clippy::too_many_lines)]
pub fn autogen_oods(
    trace: &[&RationalExpression],
    generator: &FieldElement,
    degree_bound: usize,
    ctx: &ContextArray,
) -> Result<(), std::io::Error> {
    let path = Path::new("contracts/Odds.sol");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    writeln!(
        &mut file,
        "pragma solidity ^0.5.11;

        import \"./MemoryMap.sol\";
        import \"./StarkParameters.sol\";
        
        contract Oods is MemoryMap, StarkParameters {{
          // For each query point we want to invert (2 + n_rows_in_mask) items:
          //  The query point itself (x).
          //  The denominator for the constraint polynomial (x-z^constraint_degree)
          //  [(x-(g^row_number)z) for row_number in mask].
          uint256 constant internal batch_inverse_chunk = (2 + n_rows_in_mask);
          uint256 constant internal batch_inverse_size = max_n_queries * batch_inverse_chunk;
          function oods_prepare_inverses(uint256[] memory context) internal pure {{
        uint trace_generator = 0x{};
        uint oods_point = context[mm_oods_point];
        for (uint i = 0; i < context[mm_n_unique_queries]; i ++) {{
            // Get the shifted eval point
            uint x = fmul(context[i + mm_oods_eval_points], PrimeFieldElement0.generator_val);",
        U256::from(generator)
    )?;

    let mut last_seen_row = isize::max_value();
    let mut counter = 0;
    let mut index_to_offset = HashMap::new();

    let mut row_sorted_trace = trace.to_vec();
    row_sorted_trace.sort_by(|a, b| back_lexicographic_compare(a, b));

    // Please note this relies heavily on the sortedness of the trace array.
    for element in &row_sorted_trace {
        match element {
            RationalExpression::Trace(_, j) => {
                if *j != last_seen_row {
                    writeln!(
                        &mut file,
                        "            // Preparing denominator for row {}",
                        j
                    )?;
                    writeln!(
                        &mut file,
                        "        context[batch_inverse_chunk*i + {} + mm_batch_inverse_in] = \
                         fsub(x, fmul(oods_point, 0x{}));",
                        counter,
                        U256::from(generator.pow(*j).unwrap())
                    )?;
                    index_to_offset.insert(j, counter * 32);
                    counter += 1;
                    last_seen_row = *j;
                }
            }
            _ => {
                panic!(
                    "Expected that the trace array was composed of only RationalExpression::Trace"
                )
            }
        }
    }

    writeln!(
        &mut file,
        "        context[batch_inverse_chunk*i + {} + mm_batch_inverse_in] = fsub(x, \
         fpow2(oods_point, {}));",
        counter, degree_bound
    )?;
    writeln!(
        &mut file,
        "        context[batch_inverse_chunk*i + {} + mm_batch_inverse_in] = context[i + \
         mm_oods_eval_points];",
        counter + 1
    )?;

    writeln!(
        &mut file,
        "}}

    uint carried = 1;
    for (uint i = 0; i < context[mm_n_unique_queries]*batch_inverse_chunk; i ++) {{
        carried = fmul(carried, context[mm_batch_inverse_in+i]);
        context[mm_batch_inverse_out+i] = carried;
    }}

    uint inv_prod = inverse2(carried);

    for (uint i = context[mm_n_unique_queries]*batch_inverse_chunk - 1; i > 0; i--) {{
        context[mm_batch_inverse_out + i] = fmul(inv_prod, context[mm_batch_inverse_out + i - 1]);
        inv_prod = fmul(inv_prod, context[mm_batch_inverse_in + i]);
    }}
    context[mm_batch_inverse_out] = inv_prod;
  }}"
    )?;

    writeln!(
        &mut file,
        "  function oods_virtual_oracle(uint256[] memory ctx)
  internal {{
  oods_prepare_inverses(ctx);

  uint k_montgomery_r_inv_ = PrimeFieldElement0.k_montgomery_r_inv;
  assembly {{
      let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001
      let k_montgomery_r_inv := k_montgomery_r_inv_
      let context := ctx
      let fri_values := /*fri_values*/ add(context, {})
      let fri_values_end := add(fri_values,  mul(/*n_unique_queries*/ mload(add(context, {})), \
         0x20))
      let fri_inv_points := /*fri_inv_points*/ add(context, {})
      let trace_query_responses := /*trace_query_responses*/ add(context, {})

      let composition_query_responses := /*composition_query_responses*/ add(context, {})

      // Set denominators_ptr to point to the batch_inverse_out array.
      // The content of batch_inverse_out is described in oods_prepare_inverses.
      let denominators_ptr := /*batch_inverse_out*/ add(context, {})
      for {{}} lt(fri_values, fri_values_end) {{fri_values := add(fri_values, 0x20)}} {{
        let res := 0",
        (ctx.fri_values + 1) * 32,
        (ctx.n_unique_queries + 1) * 32,
        (ctx.fri_inverses + 1) * 32,
        (ctx.trace_queries + 1) * 32,
        (ctx.composition_queries + 1) * 32,
        (ctx.denominators + 1) * 32
    )?;

    let mut seen_col = usize::max_value();
    let mut oods_value_ptr = (ctx.oods_values + 1) * 32;
    let mut oods_coefficients_ptr = (ctx.oods_coefficients + 1) * 32;
    let mut composition_ptr = (ctx.composition_oods + 1) * 32;
    let mut col_read = 0;

    // This logic depends on sorting on the col instead of on row
    // We also assume some constraint applies to each col somewhere so that we don't
    // have to index
    for mask_item in trace.iter() {
        match mask_item {
            RationalExpression::Trace(i, j) => {
                if *i != seen_col {
                    if seen_col != usize::max_value() {
                        writeln!(&mut file, "                }}")?;
                    }

                    writeln!(
                        &mut file,
                        "              // Mask items for column #{}.
                    {{
                    // Read the next element.
                    let column_value := mulmod(mload(add(trace_query_responses, {})), \
                         k_montgomery_r_inv, PRIME)",
                        i,
                        i * 32
                    )?;
                    seen_col = *i;
                    col_read += 1;
                }
                writeln!(
                    &mut file,
                    "              res := addmod(
                    res,
                    mulmod(mulmod( /* denom for {}th row */ mload(add(denominators_ptr, {})),
                                  /*oods_coefficients*/ mload(add(context, {})),
                                  PRIME),
                           add(column_value, sub(PRIME, /*oods_values*/ mload(add(context, {})))),
                           PRIME),
                    PRIME)",
                    j,
                    index_to_offset.get(j).unwrap(),
                    oods_coefficients_ptr,
                    oods_value_ptr
                )?;
                oods_value_ptr += 32;
                oods_coefficients_ptr += 32;
            }
            _ => {
                panic!(
                    "Expected that the trace array was composed of only RationalExpression::Trace"
                )
            }
        }
    }
    writeln!(&mut file, "                }}")?;

    writeln!(
        &mut file,
        "              trace_query_responses := add(trace_query_responses, {})",
        col_read * 32
    )?;
    writeln!(&mut file, "         // Composition constraints.")?;

    for i in 0..degree_bound {
        writeln!(
            &mut file,
            "              {{
            // Read the next element.
            let column_value := mulmod(mload(add(composition_query_responses, {})), \
             k_montgomery_r_inv, PRIME)
            res := addmod(
                res,
                mulmod(mulmod(mload(add(denominators_ptr, {})),
                              mload(/*oods_coefficients*/ add(context, {})),
                              PRIME),
                       add(column_value,
                           sub(PRIME, /*composition_oods_values*/ mload(add(context, {})))),
                       PRIME),
                PRIME)
            }}",
            i * 32,
            index_to_offset.len() * 32,
            oods_coefficients_ptr,
            composition_ptr
        )?;
        oods_coefficients_ptr += 32;
        composition_ptr += 32;
    }

    writeln!(
        &mut file,
        "              // Advance the composition_query_responses by the the amount we've read \
         (0x20 * constraint_degree).
      composition_query_responses := add(composition_query_responses, {})

      // Append the sum of the trace boundary constraints to the fri_values array.
      // Note that we need to add the sum of the composition boundary constraints to those
      // values before running fri.
      mstore(fri_values, res)

      // Append the fri_inv_point of the current query to the fri_inv_points array.
      mstore(fri_inv_points, mload(add(denominators_ptr, {})))
      fri_inv_points := add(fri_inv_points, 0x20)

      // Advance denominators_ptr by chunk size (0x20 * (2+n_rows_in_mask)).
      denominators_ptr := add(denominators_ptr, {})
      }}
    }}
  }}
}}",
        degree_bound * 32,
        (index_to_offset.len() + 1) * 32,
        (index_to_offset.len() + 2) * 32
    )?;
    Ok(())
}

fn lexicographic_compare(first: &RationalExpression, second: &RationalExpression) -> Ordering {
    if let RationalExpression::Trace(i, j) = first {
        if let RationalExpression::Trace(x, y) = second {
            if x > i {
                Ordering::Less
            } else if x < i {
                Ordering::Greater
            } else if y > j {
                Ordering::Less
            } else if y < j {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        } else {
            panic!("The lexicographic compare should only be used on traces");
        }
    } else {
        panic!("The lexicographic compare should only be used on traces");
    }
}

fn back_lexicographic_compare(first: &RationalExpression, second: &RationalExpression) -> Ordering {
    if let RationalExpression::Trace(i, j) = first {
        if let RationalExpression::Trace(x, y) = second {
            if y > j {
                Ordering::Less
            } else if y < j {
                Ordering::Greater
            } else if x > i {
                Ordering::Less
            } else if x < i {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        } else {
            panic!("The lexicographic compare should only be used on traces");
        }
    } else {
        panic!("The lexicographic compare should only be used on traces");
    }
}
