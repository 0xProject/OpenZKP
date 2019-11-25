use crate::rational_expression::*;
use std::collections::HashMap;
use std::cmp::Ordering;

pub fn autogen_oods(public: &[&RationalExpression], constraints: &[RationalExpression]) {
    let mut traces = HashMap::new();
    let mut inverses = HashMap::new();
    let mut periodic = HashMap::new();
    for exp in constraints.iter() {
        traces.extend(exp.trace_search());
        inverses.extend(exp.inv_search());
        periodic.extend(exp.periodic_search());
    }

    let mut trace_keys : Vec<&RationalExpression> = traces.keys().collect();
    trace_keys.sort_by(|a, b| lexographic_compare(a, b));
    let inverse_keys : Vec<&RationalExpression> = inverses.keys().collect();
    // TODO - sorting periodic keys
    let periodic_keys : Vec<&RationalExpression> = periodic.keys().collect();

    let memory_map = setup_memory_layout(constraints.len(), public, trace_keys.as_slice(), inverse_keys.as_slice(), periodic_keys.as_slice());

    let mut coefficient_index = 1 + public.len() + trace_keys.len() + periodic_keys.len();
    let mut adjust_index = coefficient_index + 2*constraints.len() + 2*inverses.len();
    for exp in constraints.iter() {
        println!("{{");
        println!("let val := {}", exp.soldity_encode(&memory_map));
        println!("res := addmod(res, mulmod(val, add(mload({}), mulmod(mload({}), mload({}), PRIME)), PRIME),PRIME)", coefficient_index*32, (coefficient_index+1)*32, adjust_index*32);
        println!("}}");
        coefficient_index += 2;
        adjust_index += 1;
    }
    println!("mstore(0, res)\nreturn(0, 0x20)");
}

pub fn setup_memory_layout(num_constraints: usize, public_inputs: &[&RationalExpression], inverses: &[&RationalExpression], traces: &[&RationalExpression], periodic: &[&RationalExpression]) -> HashMap::<RationalExpression, String> {
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
    // Here we need to add an output which writes denominator storage and batch inversion
    index += inverses.len();
    for &exp in inverses.iter() {
        memory_lookups.insert(exp.clone(), format!("mload({})", index*32));
        index += 1;
    }

    memory_lookups
}

fn lexographic_compare(first: &RationalExpression, second: &RationalExpression) -> Ordering {
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
                _ => {panic!("The lexographic compare should only be used on traces");}
            }
        },
        _ => {panic!("The lexographic compare should only be used on traces");}
    }
}