// TODO - Format the hash map returns to remove this warning
#![allow(unused_results)]
use crate::rational_expression::*;
use std::{
    cmp::Ordering, collections::BTreeMap, fs::File, io::prelude::*, path::Path, prelude::v1::*,
};

pub fn autogen2(
    _trace_len: usize,
    _public: &[&RationalExpression],
    constraints: &[RationalExpression],
    _n_rows: usize,
    n_cols: usize,
    blowup: usize,
) {
    let mut traces = BTreeMap::new();

    for exp in constraints.iter() {
        traces.extend(exp.trace_search());
    }

    // Note that because the hash map strategy used here is they keys are in
    // arbitrary orders. We want to enforce the restriction that the trace ones
    // be in lexicographic order though.
    let mut trace_keys: Vec<&RationalExpression> = traces.keys().collect();
    trace_keys.sort_by(|a, b| lexicographic_compare(a, b));

    let max_degree = constraints
        .iter()
        .map(|c| {
            let (numerator_degree, denominator_degree) = c.trace_degree();
            numerator_degree - denominator_degree
        })
        .max()
        .expect("No constraints");

    let trace_contract = autogen_trace_layout(&trace_keys, n_cols, max_degree, blowup);
    println!("{}", &trace_contract);

    // TODO - Variable naming
    let name = format!("contracts/{}.sol", "autogenerated_trace");
    let path = Path::new(&name);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    let _ = writeln!(&mut file, "{}", trace_contract);
}

// TODO - Support negative row offsets
fn autogen_trace_layout(
    trace_keys: &[&RationalExpression],
    n_cols: usize,
    constraint_degree: usize,
    blowup: usize,
) -> String {
    // We map each trace to the row it contains
    let mut rows = trace_keys
        .iter()
        .map(|expression| {
            match expression {
                // TODO - Remove this when we support negatives
                #[allow(clippy::cast_sign_loss)]
                RationalExpression::Trace(_, j) => *j as usize,
                _ => panic!("Non trace in trace array"),
            }
        })
        .collect::<Vec<_>>();
    // We sort
    rows.sort_unstable();
    // Then we remove duplicate items
    rows.dedup();

    // We now can create a solidity function which returns the right vector
    let mut trace_layout_contract = format!(
        "
pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../interfaces/ConstraintInterface.sol';
import '../default_cs.sol';

abstract contract RecurrenceTrace is DefaultConstraintSystem({}, {}, {}, {}) {{",
        constraint_degree,
        rows.len(),
        n_cols,
        blowup
    );

    // This specifies the lookup function
    trace_layout_contract.push_str(
        "    // This lets us map rows -> inverse index,
    // In complex systems use a autogen binary search.
    function row_to_offset(uint256 row) internal pure override returns(uint256) {",
    );

    // TODO  -this doesn't support negative rows
    if rows.iter().enumerate().all(|(index, item)| index == *item) {
        trace_layout_contract.push_str(
            "     return row;
         }",
        );
    } else {
        trace_layout_contract.push_str(&binary_row_search_string(rows.as_slice()));
        trace_layout_contract.push_str("}");
    }

    // This defines the trace layout function in solidity
    trace_layout_contract.push_str("\n");
    trace_layout_contract.push_str(&format!(
        "
    function layout_col_major() internal pure override returns(uint256[] memory) {{
        uint256[] memory result = new uint256[]({});",
        2 * trace_keys.len()
    ));
    for k in (0..2 * trace_keys.len()).step_by(2) {
        let (i, j) = match trace_keys[k / 2] {
            RationalExpression::Trace(i, j) => (i, j),
            _ => panic!("Non trace rational expression in rows"),
        };
        trace_layout_contract.push_str(&format!(
            "    (result[{}], result[{}]) = ({}, {});",
            k,
            k + 1,
            i,
            j
        ));
    }
    trace_layout_contract.push_str(
        "        return result;
    }",
    );

    // This prints out the function called rows in solidity
    trace_layout_contract.push_str(&format!(
        "
    function layout_rows() internal pure override returns(uint256[] memory) {{
        uint256[] memory result = new uint256[]({});",
        rows.len()
    ));
    for (index, row) in rows.iter().enumerate() {
        trace_layout_contract.push_str(&format!("        result[{}] = {};", index, row));
    }
    trace_layout_contract.push_str(
        "
        return result;
    }
}
    ",
    );
    trace_layout_contract
}

// TODO - This needs testing
fn binary_row_search_string(rows: &[usize]) -> String {
    if rows.len() == 1 {
        return format!("return {}", rows[0]);
    }
    format!(
        "
    if (row > {}) {{
        {}
    }} else {{
        {}
    }}
    return {};
    ",
        rows[rows.len() / 2],
        binary_row_search_string(rows.get(0..(rows.len()) / 2).unwrap()),
        binary_row_search_string(rows.get((rows.len() / 2)..rows.len()).unwrap()),
        rows[rows.len() / 2]
    )
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
