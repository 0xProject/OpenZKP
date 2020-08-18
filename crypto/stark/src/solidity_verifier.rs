use crate::{
    constraints::Constraints, polynomial::DensePolynomial, rational_expression::RationalExpression,
};
use std::{
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, BTreeMap, BTreeSet},
    convert::TryInto,
    fs::File,
    hash::{Hash, Hasher},
    io::prelude::*,
    iter::once,
    path::Path,
    prelude::v1::*,
};
use zkp_macros_decl::field_element;
use zkp_primefield::FieldElement;
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
            ClaimPolynomial(_, _, a, _) | Polynomial(_, a) | Inv(a) | Exp(a, _) | Neg(a) => {
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
            ClaimPolynomial(_, _, a, _) | Polynomial(_, a) | Exp(a, _) | Neg(a) => a.inv_search(),
        }
    }

    #[cfg(feature = "std")]
    pub fn periodic_search(&self) -> BTreeMap<Self, bool> {
        use RationalExpression::*;

        match self {
            X | Constant(_) | Trace(..) | ClaimPolynomial(..) => BTreeMap::new(),
            Polynomial(..) => [(self.clone(), true)].iter().cloned().collect(),
            Add(a, b) | Mul(a, b) => {
                let mut first = a.periodic_search();
                first.extend(b.periodic_search());
                first
            }
            Inv(a) | Exp(a, _) | Neg(a) => a.periodic_search(),
        }
    }

    #[cfg(feature = "std")]
    pub fn claim_polynomial_search(&self) -> BTreeSet<Self> {
        use RationalExpression::*;

        match self {
            ClaimPolynomial(..) => once(self).cloned().collect(),
            X | Constant(_) | Trace(..) | Polynomial(..) => BTreeSet::new(),
            Add(a, b) | Mul(a, b) => {
                let mut first = a.claim_polynomial_search();
                first.extend(b.claim_polynomial_search());
                first
            }
            Inv(a) | Exp(a, _) | Neg(a) => a.claim_polynomial_search(),
        }
    }
}

#[cfg(feature = "std")]
fn get_hash(r: &DensePolynomial) -> u64 {
    let mut hasher = DefaultHasher::new();
    r.hash(&mut hasher);
    hasher.finish()
}

impl Hash for DensePolynomial {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let x = field_element!("754ed488ec9208d1c552bb254c0890042078a9e1f7e36072ebff1bf4e193d11b");
        self.evaluate(&x).hash(state);
    }
}

#[cfg(feature = "std")]
impl PartialOrd for DensePolynomial {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "std")]
impl Ord for DensePolynomial {
    fn cmp(&self, other: &Self) -> Ordering {
        get_hash(self).cmp(&get_hash(other))
    }
}

// This function takes in the constraint system
// [which should still contain claim polynomials]
// The output directory where the files should be written too
// and a name for the constraint system
// It produces a set of files which should be manually edited
// to end up with a solidity verifier

pub fn generate(
    constraints: &Constraints,
    output_directory: &str,
    system_name: &str,
) -> Result<(), std::io::Error> {
    let blowup = constraints.blowup;
    let n_cols = constraints.trace_ncolumns();
    let trace_len = constraints.trace_nrows();
    let constraint_expressions = constraints.expressions();

    let mut traces = BTreeMap::new();
    let mut inverses = BTreeMap::new();
    let mut periodic = BTreeMap::new();
    let mut claim_polynomials = BTreeSet::new();
    for exp in constraint_expressions.iter() {
        traces.extend(exp.trace_search());
        inverses.extend(exp.inv_search());
        periodic.extend(exp.periodic_search());
        claim_polynomials.extend(exp.claim_polynomial_search());
    }

    let name = format!("{}/{}ConstraintPoly.sol", output_directory, system_name);
    let path = Path::new(&name);
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
    let claim_polynomial_keys: Vec<RationalExpression> =
        claim_polynomials.iter().cloned().collect();
    // TODO - sorting periodic keys
    let periodic_keys: Vec<&RationalExpression> = periodic.keys().collect();

    autogen_wrapper_contract(
        claim_polynomial_keys.as_slice(),
        periodic_keys.as_slice(),
        &constraints,
        system_name,
        output_directory,
        trace_keys.len(),
    )?;

    let max_degree = constraint_expressions
        .iter()
        .map(|c| {
            let (numerator_degree, denominator_degree) = c.trace_degree();
            numerator_degree - denominator_degree
        })
        .max()
        .expect("No constraints");
    let target_degree = trace_len * max_degree - 1;
    let adjustment_degrees: Vec<usize> = constraint_expressions
        .iter()
        .map(|x| {
            let (num, den) = x.degree(trace_len - 1);
            target_degree + den - num
        })
        .collect();
    autogen_oods_contract(
        constraint_expressions,
        n_cols,
        blowup,
        output_directory,
        system_name,
    );
    let memory_map = setup_call_memory(
        &mut file,
        constraint_expressions.len(),
        &claim_polynomial_keys,
        inverse_keys.as_slice(),
        trace_keys.as_slice(),
        periodic_keys.as_slice(),
        adjustment_degrees.as_slice(),
    )?;

    let mut coefficient_index = 1 + claim_polynomial_keys.len() + periodic_keys.len();
    for (exp, &degree) in constraint_expressions.iter().zip(adjustment_degrees.iter()) {
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

// We declare these macros so that the code isn't inlined in the function
macro_rules! wrapper_contract_start {
    () => {
        "
pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../interfaces/ConstraintInterface.sol';
import '../public_coin.sol';
import '../proof_types.sol';
import '../utils.sol';
import '../primefield.sol';
import '../iterator.sol';
import '../default_cs.sol';
import './{}Trace.sol';
import './{}ContraintPoly.sol';


contract {} is {}Trace {{
    using Iterators for Iterators.IteratorUint;
    using PrimeField for uint256;
    using PrimeField for PrimeField.EvalX;
    using Utils for *;

    OddsPoly immutable constraint_poly;
    // FIX ME - Add polynomials immutable variables

    // FIX ME - The constructor should also be setting any
    // periodic colum contracts to immutables
    constructor(OddsPoly constraint) public {{
        constraint_poly = constraint;
    }}

    struct PublicInput {{
        // Please add the public input fields to this struct
    }}

    // prettier-ignore
    function constraint_calculations(
        ProofTypes.StarkProof calldata proof,
        ProofTypes.ProofParameters calldata params,
        uint64[] calldata queries,
        uint256 oods_point,
        uint256[] calldata constraint_coeffiencts,
        uint256[] calldata oods_coeffiencts
    ) external override returns (uint256[] memory, uint256) {{
        ProofData memory data = ProofData(
            proof.trace_values,
            PrimeField.init_eval(params.log_trace_length + 4),
            proof.constraint_values, proof.trace_oods_values,
            proof.constraint_oods_values,
            params.log_trace_length);
        // FIX ME - You may need to customize this decoding
        PublicInput memory input = abi.decode(proof.public_inputs, (PublicInput));
        uint256[] memory result = get_polynomial_points(data, oods_coeffiencts, queries, \
     oods_point);

        // Fix Me - This may need several internal functions
        uint256 evaluated_point = evaluate_oods_point(oods_point, constraint_coeffiencts, \
     data.eval, input, data);

        return (result, evaluated_point);
    }}

    // TODO - The solidity prettier wants to delete all 'override' statements
    // We should remove this ignore statement when that changes.
    // prettier-ignore
    function initalize_system(bytes calldata public_input)
        external
        view
        override
        returns (ProofTypes.ProofParameters memory, PublicCoin.Coin memory)
    {{
        // FIX ME - You may need to customize this decoding
        PublicInput memory input = abi.decode(public_input, (PublicInput));
        PublicCoin.Coin memory coin = PublicCoin.Coin({{
            // FIX ME - Please add a public input hash here
            digest: // I'm just a robot I don't know what goes here ¯\\_(ツ)_/¯.
            ,
            counter: 0
        }});
        // The trace length is going to be the next power of two after index.
        // FIX ME - This need a trace length set, based on public input
        uint8 log_trace_length = 0;
        uint8[] memory fri_layout = default_fri_layout(log_trace_length);

        ProofTypes.ProofParameters memory params = ProofTypes.ProofParameters({{
            number_of_columns: NUM_COLUMNS,
            log_trace_length: log_trace_length,
            number_of_constraints: {},
            log_blowup: {},
            constraint_degree: CONSTRAINT_DEGREE,
            pow_bits: {},
            number_of_queries: {},
            fri_layout: fri_layout
        }});

        return (params, coin);
    }}

    function evaluate_oods_point(
        uint256 oods_point,
        uint256[] memory constraint_coeffiencts,
        PrimeField.EvalX memory eval,
        PublicInput memory public_input,
        ProofData memory data
    ) internal returns (uint256) {{
        uint256[] memory call_context = new uint256[]({});
        uint256 non_mont_oods = oods_point.fmul_mont(1);
        call_context[0] = non_mont_oods;
"
    };
}

macro_rules! wrapper_contract_end {
    () => {
        "
    uint256 current_index = {};
    // This array contains {} elements, 2 for each constraint
    for (uint256 i = 0; i < constraint_coeffiencts.length; i ++) {{
        call_context[current_index] = constraint_coeffiencts[i];
        current_index++;
    }}
    // This array contains {} elements, one for each trace offset in the layout
    for (uint256 i = 0; i < trace_oods_values.length; i++) {{
        call_context[current_index] = trace_oods_values[i].fmul_mont(1);
        current_index++;
    }}

    // The contract we are calling out to is a pure assembly contract
    // With its own hard coded memory structure so we use an assembly
    // call to send a non abi encoded array that will be loaded directly
    // into memory
    uint256 result;
    {{
    OddsPoly local_contract_address = constraint_poly;
        assembly {{
            let p := mload(0x40)
            // Note size is {}*32 because we have {} public inputs, {} constraint coefficients \
     and {} trace decommitments
            if iszero(call(not(0), local_contract_address, 0, add(call_context, 0x20), {}, p, \
     0x20)) {{
            revert(0, 0)
            }}
            result := mload(p)
        }}
    }}
    return result;
    }}
}}
"
    };
}

fn autogen_wrapper_contract(
    claim_polynomials: &[RationalExpression],
    periodic_polys: &[&RationalExpression],
    constraints: &Constraints,
    system_name: &str,
    output_directory: &str,
    trace_layout_len: usize,
) -> Result<(), std::io::Error> {
    use RationalExpression::*;

    let name = format!("{}/{}.sol", output_directory, system_name);
    let path = Path::new(&name);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    let num_constraints = constraints.expressions().len();
    let total_input_memory_size =
        1 + claim_polynomials.len() + periodic_polys.len() + 2 * num_constraints + trace_layout_len;

    // The macro invocation appears to trigger this clippy warning, but the
    // underlying string doesn't.
    #[allow(clippy::non_ascii_literal)]
    writeln!(
        &mut file,
        // Note - This has to be a marco instead of constant so that the
        // format locations are properly loaded [and it compiles]
        wrapper_contract_start!(),
        system_name,
        system_name,
        system_name,
        system_name,
        num_constraints,
        64 - constraints.blowup.leading_zeros(),
        constraints.pow_bits,
        constraints.num_queries,
        total_input_memory_size
    )?;

    // The initial index is one because of the oods point
    let mut index = 1;
    for public_input in claim_polynomials.iter() {
        match public_input {
            ClaimPolynomial(_, _, _, name) => {
                match name {
                    Some(known_name) => {
                        writeln!(
                            &mut file,
                            "    call_context[{}] = 0; // This public input is named: {}",
                            index, known_name
                        )?;
                        index += 1;
                    }
                    None => {
                        writeln!(
                            &mut file,
                            "    call_context[{}] = 0; // This public input is not named, please \
                             give it a name in Rust",
                            index
                        )?;
                        index += 1;
                    }
                }
            }
            _ => panic!("Rational expression should be a claim polynomial"),
        }
    }

    // In the periodic_exp we contain every different rational expression polynomial
    // That includes some with the same coefficients but different internal rational
    // expressions We only want to generate and enumerate polynomial contracts
    // with different coeffiencts. So we map (coefficient) => number and only
    // autogenerate when the set doesn't have the next set of coefficients
    let mut named_periodic_cols = BTreeMap::new();
    let mut seen_polys = 0;
    for periodic_exp in periodic_polys.iter() {
        match periodic_exp {
            Polynomial(coefficients, _) => {
                if !named_periodic_cols.contains_key(coefficients) {
                    let _ = named_periodic_cols.insert(coefficients, seen_polys);
                    autogen_periodic(
                        periodic_exp,
                        seen_polys,
                        &format!("periodic{}", seen_polys),
                        output_directory,
                    )?;
                    seen_polys += 1;
                }
            }
            _ => panic!("Incorrect rational expression in periodic_polys"),
        }
    }

    // Now that we have prepared the mapping and contracts we add the polynomial
    // expressions to the wrapper contract.
    for periodic_exp in periodic_polys.iter() {
        match periodic_exp {
            Polynomial(coefficients, internal_exp) => {
                writeln!(
                    &mut file,
                    "    call_context[{}] = periodic_col{}.evaluate(non_mont_oods.fpow({:?}));",
                    index,
                    named_periodic_cols.get(coefficients).unwrap(),
                    extract_power(internal_exp)
                )?;
                index += 1;
            }
            _ => panic!("Incorrect rational expression in periodic_polys"),
        }
    }

    // The macro invocation appears to trigger this clippy warning, but the
    // underlying string doesn't.
    #[allow(clippy::non_ascii_literal)]
    writeln!(
        &mut file,
        wrapper_contract_end!(),
        index,
        2 * num_constraints,
        trace_layout_len,
        index + 2 * num_constraints + trace_layout_len,
        index,
        2 * num_constraints,
        trace_layout_len,
        32 * (index + 2 * num_constraints + trace_layout_len)
    )?;
    Ok(())
}

fn extract_power(data: &RationalExpression) -> usize {
    use RationalExpression::*;
    match data {
        X => 1,
        Exp(sub_data, power) => power * extract_power(sub_data),
        _ => {
            panic!(
                "Unable to encode power for periodic col with non standard internal rational \
                 expression"
            )
        }
    }
}

// Please note this function assumes a rational expression which is a polynomial
// over X^len_poly
fn autogen_periodic(
    periodic: &RationalExpression,
    index: usize,
    name: &str,
    output_directory: &str,
) -> Result<(), std::io::Error> {
    // TODO - use this https://doc.rust-lang.org/std/path/struct.Path.html
    let name = format!("{}/{}.sol", output_directory, name);
    let path = Path::new(&name);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

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

fn autogen_oods_contract(
    constraints: &[RationalExpression],
    n_cols: usize,
    blowup: usize,
    output_directory: &str,
    system_name: &str,
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

    let trace_contract = autogen_trace_layout(&trace_keys, n_cols, max_degree, blowup, system_name);

    // TODO - Variable naming
    let name = format!("{}/{}Trace.sol", output_directory, system_name);
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
    system_name: &str,
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
abstract contract {}Trace is DefaultConstraintSystem({}, {}, {}, {}) {{",
        system_name,
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
        // This adds code which writes the 32*row and thirty two times
        // the index of the row to the kth and k+1-th positions.
        // this will let us lookup the row and index of the row
        // when we are using assembly
        // Note the factor of 32 is the because 32 bytes is the word size
        // of evm memory.
        trace_layout_contract.push_str(&format!(
            "    (result[{}], result[{}]) = ({}, {});",
            k,
            k + 1,
            32 * i,
            // TODO - Support negative rows
            32 * rows
                .iter()
                .position(|x| x == &(TryInto::<usize>::try_into(*j).unwrap()))
                .unwrap()
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
    let ifs: Vec<_> = rows
        .iter()
        .enumerate()
        .map(|(i, row)| format!("if (row == {}) {{return {};}}", row, i))
        .collect();
    ifs.join("\n else ")
}

// We declare these macros so that the code isn't inlined in the function
macro_rules! constraint_poly_start {
    () => {
        "
pragma solidity ^0.6.6;

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
            }}
"
    };
}
macro_rules! constraint_poly_batch_inv {
    () => {
        "{{
        // Compute the inverses of the denominators into denominator_invs using batch inverse.

        // Start by computing the cumulative product.
        // Let (d_0, d_1, d_2, ..., d_{{n-1}}) be the values in denominators. Then after this \
     loop
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
      }}"
    };
}

fn setup_call_memory(
    file: &mut File,
    num_constraints: usize,
    claim_polynomial_keys: &[RationalExpression],
    inverses: &[&RationalExpression],
    traces: &[&RationalExpression],
    periodic: &[&RationalExpression],
    adjustment_degrees: &[usize],
) -> Result<BTreeMap<RationalExpression, String>, std::io::Error> {
    let mut index = 1; // Note index 0 is taken by the oods_point
    let mut memory_lookups: BTreeMap<RationalExpression, String> = BTreeMap::new();
    for claim_polynomial in claim_polynomial_keys {
        let _ = memory_lookups.insert(claim_polynomial.clone(), format!("mload({})", index * 32));
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

    // The macro invocation appears to trigger this clippy warning, but the
    // underlying string doesn't.
    #[allow(clippy::non_ascii_literal)]
    writeln!(
        file,
        constraint_poly_start!(),
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

    // The macro invocation appears to trigger this clippy warning, but the
    // underlying string doesn't.
    #[allow(clippy::non_ascii_literal)]
    writeln!(
        file,
        constraint_poly_batch_inv!(),
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
