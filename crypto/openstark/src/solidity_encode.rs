use crate::rational_expression::*;
use std::collections::HashMap;
use std::cmp::Ordering;

pub fn autogen_oods(trace_len: usize, public: &[&RationalExpression], constraints: &[RationalExpression]) {
    let mut traces = HashMap::new();
    let mut inverses = HashMap::new();
    let mut periodic = HashMap::new();
    for exp in constraints.iter() {
        traces.extend(exp.trace_search());
        inverses.extend(exp.inv_search());
        periodic.extend(exp.periodic_search());
    }

    // Note that because the hash map strategy used here is they keys are in arbitrary orders.
    // We want to enforce the restriction that the trace ones be in lexicographic order though.
    let mut trace_keys : Vec<&RationalExpression> = traces.keys().collect();
    trace_keys.sort_by(|a, b| lexicographic_compare(a, b));
    let inverse_keys : Vec<&RationalExpression> = inverses.keys().collect();
    // TODO - sorting periodic keys
    let periodic_keys : Vec<&RationalExpression> = periodic.keys().collect();
    let max_degree = constraints.iter().map(|c| {
        let (numerator_degree, denominator_degree) = c.trace_degree();
        numerator_degree - denominator_degree
    }).max().expect("No constraints");
    let target_degree = trace_len*max_degree -1;
    let adjustment_degrees : Vec<usize> = constraints.iter().map(|x| {
        let (num, den) = x.degree(trace_len - 1);
        target_degree + den - num
    }).collect();

    let memory_map = setup_memory_layout(constraints.len(), public, inverse_keys.as_slice(), trace_keys.as_slice(), periodic_keys.as_slice(), adjustment_degrees.as_slice());

    let mut coefficient_index = 1 + public.len() + trace_keys.len() + periodic_keys.len();
    let mut adjust_index = coefficient_index + 2*constraints.len() + 2*inverses.len();
    for (exp, &degree) in constraints.iter().zip(adjustment_degrees.iter()) {
        println!("      {{");
        println!("        let val := {}", exp.soldity_encode(&memory_map));
        println!("        res := addmod(res, mulmod(val, add(mload({}), mulmod(mload({}), {}, PRIME)), PRIME),PRIME)", coefficient_index*32, (coefficient_index+1)*32, memory_map.get(&RationalExpression::Exp(RationalExpression::X.into(), degree)).unwrap());
        println!("      }}");
        coefficient_index += 2;
        adjust_index += 1;
    }
    println!("      mstore(0, res)\n      return(0, 0x20)\n}}");
}

pub fn setup_memory_layout(num_constraints: usize, public_inputs: &[&RationalExpression], inverses: &[&RationalExpression], traces: &[&RationalExpression], periodic: &[&RationalExpression], adjustment_degrees: &[usize]) -> HashMap::<RationalExpression, String> {
    let mut index = 1; // Note index 0 is taken by the oods_point
    let mut memory_lookups :  HashMap::<RationalExpression, String> = HashMap::new();
    for &exp in public_inputs.iter() {
        memory_lookups.insert(exp.clone(), format!("mload({})", index*32));
        index += 1;
    }
    for &exp in traces.iter() {
        memory_lookups.insert(exp.clone(), format!("mload({})", index*32));
        index += 1;
    }
    for &exp in periodic.iter() {
        memory_lookups.insert(exp.clone(), format!("mload({})", index*32));
        index += 1;
    }
    index += num_constraints*2;
    let in_data_size = index;
    // Here we need to add an output which writes denominator storage and batch inversion

    let mut held = "".to_owned();
    // We put the degree adjustment calculation into the memory map: Note this means that if the exp used
    // is used in non adjustment places in the constraints those will now load this [in some cases]
    for &degree in adjustment_degrees {
        let implied_expression = RationalExpression::Exp(RationalExpression::X.into(), degree);
        if !memory_lookups.contains_key(&implied_expression) {
            held.extend(format!("        mstore({},expmod(mload(0), {}, PRIME))\n", index*32, degree).chars());
            memory_lookups.insert(implied_expression, format!("mload({})", index*32));
            index += 1;
        }
    }

    let inverse_start_index = index;
    index += inverses.len();

    println!("function() external {{
          uint256 res;
          assembly {{
            let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001
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
    
            function degree_adjustment(composition_polynomial_degree_bound, constraint_degree, numerator_degree,
                denominator_degree) -> res {{
                                        res := sub(sub(composition_polynomial_degree_bound, 1),
                       sub(add(constraint_degree, numerator_degree), denominator_degree))
                    }}
    
            function small_expmod(x, num, prime) -> res {{
                res := 1
                for {{ let ind := 0 }} lt(ind, num) {{ ind := add(ind, 1) }} {{
                       res := mulmod(res, x, prime)
                }}
            }}", in_data_size*32, (index + inverses.len())*32);
    
    println!("// Store adjustment degrees");
    println!("{}", held);
    println!("// Store the values which will be batch inverted")
    for &exp in inverses.iter() {
        match exp {
            RationalExpression::Inv(a) => {println!("        mstore({}, {})", index*32, a.soldity_encode(&memory_lookups));},
            _ => {panic!("Inverse search returned a non inverse");},
        }
        memory_lookups.insert(exp.clone(), format!("mload({})", index*32));
        index += 1;
    }

    println!("      {{
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
      }}", inverses.len()*32, (inverse_start_index+inverses.len())*32, (inverse_start_index)*32, inverse_start_index*32, (inverse_start_index+inverses.len())*32);


    memory_lookups
}

fn lexicographic_compare(first: &RationalExpression, second: &RationalExpression) -> Ordering {
    match first {
        RationalExpression::Trace(i, j) => {
            match second {
                RationalExpression::Trace(x, y) => {
                    if x > i {
                        Ordering::Less
                    } else if x < i {
                        Ordering::Greater
                    } else {
                        if y > j {
                            Ordering::Less
                        } else if y < j {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    }
                },
                _ => {panic!("The lexicographic compare should only be used on traces");}
            }
        },
        _ => {panic!("The lexicographic compare should only be used on traces");}
    }
}