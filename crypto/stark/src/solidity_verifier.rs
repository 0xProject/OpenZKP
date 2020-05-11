use crate::rational_expression::*;
use std::{
    cmp::Ordering, collections::BTreeMap, fs::File, io::prelude::*, path::Path, prelude::v1::*,
};
use zkp_u256::U256;

impl RationalExpression {
    #[cfg(feature = "std")]
    pub fn soldity_encode(&self, memory_layout: &BTreeMap<Self, String>) -> String {
        use RationalExpression::*;

        match memory_layout.get(self) {
            Some(s) => s.clone(),
            None => {
                match self {
                    X => "mload(0x0)".to_owned(),
                    Constant(c) => format!("0x{}", U256::from(c).to_string()),
                    Add(a, b) => {
                        format!(
                            "addmod({}, {}, PRIME)",
                            a.soldity_encode(memory_layout),
                            b.soldity_encode(memory_layout)
                        )
                    }
                    Neg(a) => format!("sub(PRIME , {})", a.soldity_encode(memory_layout)),
                    Mul(a, b) => {
                        format!(
                            "mulmod({}, {}, PRIME)",
                            a.soldity_encode(memory_layout),
                            b.soldity_encode(memory_layout)
                        )
                    }
                    Exp(a, e) => {
                        match e {
                            0 => "0x01".to_owned(),
                            1 => a.soldity_encode(memory_layout),
                            _ => {
                                // TODO - Check the gas to see what the real breaking point should
                                // be
                                if *e < 10 {
                                    format!(
                                        "small_expmod({}, {}, PRIME)",
                                        a.soldity_encode(memory_layout),
                                        e.to_string()
                                    )
                                } else {
                                    format!(
                                        "expmod({}, {}, PRIME)",
                                        a.soldity_encode(memory_layout),
                                        e.to_string()
                                    )
                                }
                            }
                        }
                    }
                    _ => panic!("This should not happen...."),
                }
            }
        }
    }

    // TODO - DRY this by writing a generic search over subtypes
    #[cfg(feature = "std")]
    pub fn trace_search(&self) -> BTreeMap<Self, bool> {
        use RationalExpression::*;

        match self {
            X | Constant(..) => BTreeMap::new(),
            Trace(..) => [(self.clone(), true)].iter().cloned().collect(),
            Add(a, b) | Mul(a, b) => {
                let mut first = a.trace_search();
                first.extend(b.trace_search());
                first
            }
            ClaimPolynomial(_, _, a) | Polynomial(_, a) | Inv(a) | Exp(a, _) | Neg(a) => {
                a.trace_search()
            }
        }
    }

    #[cfg(feature = "std")]
    pub fn inv_search(&self) -> BTreeMap<Self, bool> {
        use RationalExpression::*;

        match self {
            X | Constant(_) | Trace(..) => BTreeMap::new(),
            Add(a, b) | Mul(a, b) => {
                let mut first = a.inv_search();
                first.extend(b.inv_search());
                first
            }
            Inv(_) => [(self.clone(), true)].iter().cloned().collect(),
            ClaimPolynomial(_, _, a) | Polynomial(_, a) | Exp(a, _) | Neg(a) => a.inv_search(),
        }
    }

    #[cfg(feature = "std")]
    pub fn periodic_search(&self) -> BTreeMap<Self, bool> {
        use RationalExpression::*;

        match self {
            X | Constant(_) | Trace(..) => BTreeMap::new(),
            Polynomial(..) => [(self.clone(), true)].iter().cloned().collect(),
            ClaimPolynomial(..) => panic!("TODO"),
            Add(a, b) | Mul(a, b) => {
                let mut first = a.periodic_search();
                first.extend(b.periodic_search());
                first
            }
            Inv(a) | Exp(a, _) | Neg(a) => a.periodic_search(),
        }
    }
}

pub fn generate(
    trace_len: usize,
    public: &[&RationalExpression],
    constraints: &[RationalExpression],
    n_cols: usize,
    blowup: usize,
) -> Result<(), std::io::Error> {
    let mut traces = BTreeMap::new();
    let mut inverses = BTreeMap::new();
    let mut periodic = BTreeMap::new();
    for exp in constraints.iter() {
        traces.extend(exp.trace_search());
        inverses.extend(exp.inv_search());
        periodic.extend(exp.periodic_search());
    }

    let path = Path::new("contracts/ConstraintPoly.sol");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
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

    autogen_oods_contract(constraints, n_cols, blowup);
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

// Please note this function assumes a rational expression which is a polynomial
// over X^len_poly
fn autogen_periodic(
    periodic: &RationalExpression,
    index: usize,
    name: &str,
) -> Result<(), std::io::Error> {
    let name = format!("contracts/{}.sol", name);
    let path = Path::new(&name);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

    println!("{}: {:?}", index, periodic);

    if let RationalExpression::Polynomial(poly, _) = periodic {
        writeln!(
            &mut file,
            "pragma solidity ^0.6.6;

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

fn autogen_oods_contract(constraints: &[RationalExpression], n_cols: usize, blowup: usize) {
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
pragma solidity ^0.6.6;
pragma experimental ABIEncoderV2;

import '../interfaces/ConstraintInterface.sol';
import '../default_cs.sol';

// The linter doesn't understand 'abstract' and thinks it's indentation

// solhint-disable-next-line indent
abstract contract RecurrenceTrace is DefaultConstraintSystem({}, {}, {}, {}) {{",
        constraint_degree,
        rows.len(),
        n_cols,
        blowup
    );

    // This specifies the lookup function
    trace_layout_contract.push_str(
        "\n    // This lets us map rows -> inverse index,
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

#[allow(clippy::too_many_lines)]
fn setup_call_memory(
    file: &mut File,
    num_constraints: usize,
    public_inputs: &[&RationalExpression],
    inverses: &[&RationalExpression],
    traces: &[&RationalExpression],
    periodic: &[&RationalExpression],
    adjustment_degrees: &[usize],
) -> Result<BTreeMap<RationalExpression, String>, std::io::Error> {
    let mut index = 1; // Note index 0 is taken by the oods_point
    let mut memory_lookups: BTreeMap<RationalExpression, String> = BTreeMap::new();
    for &exp in public_inputs.iter() {
        let _ = memory_lookups.insert(exp.clone(), format!("mload({})", index * 32));
        index += 1;
    }
    for &exp in periodic.iter() {
        let _ = memory_lookups.insert(exp.clone(), format!("mload({})", index * 32));
        index += 1;
    }
    // Layout the constraints
    index += num_constraints * 2;
    // Note that the trace values must be the last inputs from the contract to make
    // the memory layout defaults work.
    for &exp in traces.iter() {
        let _ = memory_lookups.insert(exp.clone(), format!("mload({})", index * 32));
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
            let _ = memory_lookups.insert(implied_expression, format!("mload({})", index * 32));
            index += 1;
        }
    }

    let inverse_start_index = index;
    index += inverses.len();

    writeln!(
        file,
        "pragma solidity ^0.6.6;

contract OodsPoly {{
    fallback() external {{
          assembly {{
            let res := 0
            let PRIME := 0x800000000000011000000000000000000000000000000000000000000000001
            // NOTE - If compilation hits a stack depth error on variable PRIME,
            // then uncomment the following line and globally replace PRIME with mload({})
            // mstore({}, 0x800000000000011000000000000000000000000000000000000000000000001)
            // Copy input from calldata to memory.
            calldatacopy(0x0, 0x0, /*input_data_size*/ {})

            function expmod(base, exponent, modulus) -> result {{
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
                result := mload(p)
            }}

            function degree_adjustment(composition_polynomial_degree_bound, constraint_degree, \
         numerator_degree,
                denominator_degree) -> result {{
                    result := sub(sub(composition_polynomial_degree_bound, 1),
                       sub(add(constraint_degree, numerator_degree), denominator_degree))
                    }}

            function small_expmod(x, num, prime) -> result {{
                result := 1
                for {{ let ind := 0 }} lt(ind, num) {{ ind := add(ind, 1) }} {{
                    result := mulmod(result, x, prime)
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
        let _ = memory_lookups.insert(exp.clone(), format!("mload({})", inverse_position * 32));
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
        let products_to_values := {}
        let prod := 1
        let partial_product_end_ptr := {}
        for {{ let partial_product_ptr := {} }}
            lt(partial_product_ptr, partial_product_end_ptr)
            {{ partial_product_ptr := add(partial_product_ptr, 0x20) }} {{
            mstore(partial_product_ptr, prod)
            // prod *= d_{{i}}.
            prod := mulmod(prod,
                           mload(add(partial_product_ptr, products_to_values)),
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
                               mload(add(current_partial_product_ptr, products_to_values)),
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
