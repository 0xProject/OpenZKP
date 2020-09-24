use crate::{
    constraints::Constraints, polynomial::DensePolynomial, rational_expression::RationalExpression,
};
use serde::Serialize;
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
use thiserror::Error;
use tinytemplate::TinyTemplate;
use zkp_macros_decl::field_element;
use zkp_primefield::FieldElement;
use zkp_u256::U256;

const OODS_POLY_TEMPLATE: &str = include_str!("../assets/OodsPoly.sol");
const PERIODIC_TEMPLATE: &str = include_str!("../assets/Periodic.sol");
const TRACE_TEMPLATE: &str = include_str!("../assets/Trace.sol");
const WRAPPER_TEMPLATE: &str = include_str!("../assets/Wrapper.sol");

#[derive(Debug, Error)]
#[allow(variant_size_differences)]
pub enum GenerateError {
    #[error("Error writing contract")]
    IoError(#[from] std::io::Error),
    #[error("Error rendering template")]
    TemplateError(#[from] tinytemplate::error::Error),
    #[error("Bug: invalid expression ocurred")]
    InvalidExpression,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize)]
struct DegreeAdjustment {
    location: usize,
    exponent: usize,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize)]
struct BatchInvert {
    location:   usize,
    expression: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize)]
struct Constraint {
    first_coefficient_location:  usize,
    second_coefficient_location: usize,
    degree_adjustment_location:  String,
    expression:                  String,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize)]
struct OodsPolyContext {
    modulus:               String,
    x:                     String,
    degree_adjustments:    Vec<DegreeAdjustment>,
    batch_inverted:        Vec<BatchInvert>,
    constraints:           Vec<Constraint>,
    periodic_name:         String,
    periodic_coefficients: Vec<String>,

    // Locations
    modulus_location: usize,
    input_data_size:  usize,
    expmod_context:   usize,

    // Batch inverse parameters
    products_to_values:        usize,
    partial_product_end_ptr:   usize,
    partial_product_start_ptr: usize,
    first_partial_product_ptr: usize,
    last_partial_product_ptr:  usize,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize)]
struct PeriodicContext {
    name:         String,
    coefficients: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize)]
struct RowOffset {
    row:   usize,
    index: usize,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize)]
struct TraceContext {
    name:               String,
    constraint_degree:  usize,
    num_rows:           usize,
    num_cols:           usize,
    blowup:             usize,
    column_layout_size: usize,

    row_offsets:   Vec<RowOffset>,
    column_layout: Vec<usize>,
    row_layout:    Vec<usize>,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize)]
struct PeriodicColumnEvaluation {
    name:     String,
    index:    usize,
    exponent: usize,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize)]
struct WrapperContext {
    name:                    String,
    number_of_constraints:   usize,
    log_blowup:              usize,
    pow_bits:                usize,
    number_of_queries:       usize,
    total_input_memory_size: usize,
    trace_layout_len:        usize,
    number_of_public_inputs: usize,
    coefficient_offset:      usize,

    // Call input values
    public_input_names:          Vec<String>,
    periodic_column_evaluations: Vec<PeriodicColumnEvaluation>,

    constraint_input_size: usize,
}

impl RationalExpression {
    #[cfg(feature = "std")]
    pub fn soldity_encode(&self, memory_layout: &BTreeMap<Self, String>) -> String {
        use RationalExpression::*;

        match memory_layout.get(self) {
            Some(s) => s.clone(),
            None => {
                match self {
                    Constant(c) => format!("0x{}", U256::from(c).to_string()),
                    Add(a, b) => {
                        format!(
                            "addmod({}, {}, mload(callvalue()))",
                            a.soldity_encode(memory_layout),
                            b.soldity_encode(memory_layout)
                        )
                    }
                    Neg(a) => {
                        format!(
                            "sub(mload(callvalue()) , {})",
                            a.soldity_encode(memory_layout)
                        )
                    }
                    Mul(a, b) => {
                        format!(
                            "mulmod({}, {}, mload(callvalue()))",
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
                                        "small_expmod({}, {})",
                                        a.soldity_encode(memory_layout),
                                        e.to_string()
                                    )
                                } else {
                                    format!(
                                        "expmod({}, {})",
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
) -> Result<(), GenerateError> {
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
    )?;

    // Write OodsPoly contract
    let output_directory = Path::new(output_directory);
    let filename = format!("{}ConstraintPoly.sol", system_name);
    let mut file = File::create(&output_directory.join(filename))?;
    write_oods_poly(
        &mut file,
        constraint_expressions.len(),
        &claim_polynomial_keys,
        inverse_keys.as_slice(),
        trace_keys.as_slice(),
        periodic_keys.as_slice(),
        adjustment_degrees.as_slice(),
        constraint_expressions,
    )?;

    Ok(())
}

fn autogen_wrapper_contract(
    claim_polynomials: &[RationalExpression],
    periodic_polys: &[&RationalExpression],
    constraints: &Constraints,
    system_name: &str,
    output_directory: &str,
    trace_layout_len: usize,
) -> Result<(), GenerateError> {
    use RationalExpression::*;

    let mut context = WrapperContext {
        name: system_name.to_owned(),
        number_of_constraints: constraints.expressions().len(),
        log_blowup: (64 - constraints.blowup.leading_zeros()) as usize,
        pow_bits: constraints.pow_bits,
        number_of_queries: constraints.num_queries,
        total_input_memory_size: 1
            + claim_polynomials.len()
            + periodic_polys.len()
            + 2 * constraints.expressions().len()
            + trace_layout_len,
        trace_layout_len,
        coefficient_offset: 1 + claim_polynomials.len() + periodic_polys.len(),
        number_of_public_inputs: claim_polynomials.len(),
        ..WrapperContext::default()
    };

    let name = format!("{}/{}.sol", output_directory, system_name);
    let path = Path::new(&name);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    let num_constraints = constraints.expressions().len();

    // The initial index is one because of the oods point
    for public_input in claim_polynomials.iter() {
        context.public_input_names.push(match public_input {
            ClaimPolynomial(_, _, _, Some(name)) => (*name).to_string(),
            ClaimPolynomial(_, _, _, None) => String::default(),
            _ => return Err(GenerateError::InvalidExpression),
        })
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
            _ => return Err(GenerateError::InvalidExpression),
        }
    }

    // Now that we have prepared the mapping and contracts we add the polynomial
    // expressions to the wrapper contract.
    let mut index = 1 + claim_polynomials.len();
    for periodic_exp in periodic_polys.iter() {
        match periodic_exp {
            Polynomial(coefficients, internal_exp) => {
                context
                    .periodic_column_evaluations
                    .push(PeriodicColumnEvaluation {
                        index,
                        name: format!(
                            "periodic_col{}",
                            named_periodic_cols.get(coefficients).unwrap()
                        ),
                        exponent: extract_power(internal_exp),
                    });
                index += 1;
            }
            _ => return Err(GenerateError::InvalidExpression),
        }
    }
    context.constraint_input_size = 32 * (index + 2 * num_constraints + trace_layout_len);

    // Render template
    let mut tt = TinyTemplate::new();
    tt.add_template("wrapper", WRAPPER_TEMPLATE)?;
    write!(file, "{}", tt.render("wrapper", &context)?)?;

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
) -> Result<(), GenerateError> {
    let mut tt = TinyTemplate::new();
    tt.add_template("oods_poly", OODS_POLY_TEMPLATE)?;
    tt.add_template("periodic", PERIODIC_TEMPLATE)?;
    tt.add_template("trace", TRACE_TEMPLATE)?;

    let output_directory = Path::new(output_directory);
    let filename = format!("{}.sol", name);
    let mut file = File::create(&output_directory.join(filename))?;

    let poly = match periodic {
        RationalExpression::Polynomial(poly, _) => poly,
        _ => return Err(GenerateError::InvalidExpression),
    };

    let mut context = PeriodicContext::default();
    context.name = index.to_string();
    context.coefficients = poly
        .coefficients()
        .iter()
        .rev()
        .map(|c| U256::from(c).to_string())
        .collect();

    // Render Periodic template
    let rendered = tt.render("periodic", &context)?;
    write!(file, "{}", &rendered)?;
    Ok(())
}

fn autogen_oods_contract(
    constraints: &[RationalExpression],
    n_cols: usize,
    blowup: usize,
    output_directory: &str,
    system_name: &str,
) -> Result<(), GenerateError> {
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

    let trace_contract =
        autogen_trace_layout(&trace_keys, n_cols, max_degree, blowup, system_name)?;

    // TODO - Variable naming
    let name = format!("{}/{}Trace.sol", output_directory, system_name);
    let path = Path::new(&name);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    writeln!(&mut file, "{}", trace_contract)?;

    Ok(())
}

// TODO - Support negative row offsets
fn autogen_trace_layout(
    trace_keys: &[&RationalExpression],
    n_cols: usize,
    constraint_degree: usize,
    blowup: usize,
    system_name: &str,
) -> Result<String, GenerateError> {
    let mut tt = TinyTemplate::new();
    tt.add_template("oods_poly", OODS_POLY_TEMPLATE)?;
    tt.add_template("periodic", PERIODIC_TEMPLATE)?;
    tt.add_template("trace", TRACE_TEMPLATE)?;

    let mut context = TraceContext {
        name: system_name.to_owned(),
        constraint_degree,
        column_layout_size: 2 * trace_keys.len(),
        num_cols: n_cols,
        blowup,
        ..TraceContext::default()
    };

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
    context.num_rows = rows.len();

    // Mapping for the row_to_offset function
    let identity_map = rows.iter().enumerate().all(|(index, item)| index == *item);
    if !identity_map {
        context.row_offsets = rows
            .iter()
            .enumerate()
            .map(|(index, &row)| RowOffset { index, row })
            .collect();
    }

    // This defines the trace layout function in solidity
    // This adds code which writes the 32*row and thirty two times
    // the index of the row to the kth and k+1-th positions.
    // this will let us lookup the row and index of the row
    // when we are using assembly
    // Note the factor of 32 is the because 32 bytes is the word size
    // of evm memory.
    for k in (0..2 * trace_keys.len()).step_by(2) {
        let (i, j) = match trace_keys[k / 2] {
            RationalExpression::Trace(i, j) => (i, j),
            _ => return Err(GenerateError::InvalidExpression),
        };
        let row_location = i * 32;
        let index_location = 32
            * rows
                .iter()
                .position(|x| x == &(TryInto::<usize>::try_into(*j).unwrap()))
                .unwrap();
        context.column_layout.push(row_location);
        context.column_layout.push(index_location);
    }

    // Row layout
    context.row_layout = rows;

    Ok(tt.render("trace", &context)?)
}

// TODO: Simplify
#[allow(clippy::too_many_arguments)]
fn write_oods_poly(
    file: &mut File,
    num_constraints: usize,
    claim_polynomial_keys: &[RationalExpression],
    inverses: &[&RationalExpression],
    traces: &[&RationalExpression],
    periodic: &[&RationalExpression],
    adjustment_degrees: &[usize],
    constraint_expressions: &[RationalExpression],
) -> Result<(), GenerateError> {
    let mut tt = TinyTemplate::new();
    tt.add_template("oods_poly", OODS_POLY_TEMPLATE)?;
    tt.add_template("periodic", PERIODIC_TEMPLATE)?;
    tt.add_template("trace", TRACE_TEMPLATE)?;

    let mut context = OodsPolyContext::default();
    context.modulus = "mload(callvalue())".to_owned();
    context.x = "calldataload(callvalue())".to_owned();

    // Initialize a memory map
    let mut memory_lookups: BTreeMap<RationalExpression, String> = BTreeMap::new();

    // Add X to memory map
    let _ = memory_lookups.insert(RationalExpression::X, format!("calldataload(callvalue())"));

    // Add public input to memory map
    let mut index = 1;
    for claim_polynomial in claim_polynomial_keys {
        let _ = memory_lookups.insert(
            claim_polynomial.clone(),
            format!("calldataload({})", index * 32),
        );
        index += 1;
    }

    // Add periodic column evaluations to memory map
    for &exp in periodic.iter() {
        let _ = memory_lookups.insert(exp.clone(), format!("calldataload({})", index * 32));
        index += 1;
    }

    // Add constraint coefficients to memory map
    index += num_constraints * 2;
    // Note that the trace values must be the last inputs from the contract to make
    // the memory layout defaults work.
    for &exp in traces.iter() {
        let _ = memory_lookups.insert(exp.clone(), format!("calldataload({})", index * 32));
        index += 1;
    }

    // End of input, switch from calldata to memory
    let in_data_size = index;
    index = 1; // 0 is reserved for the modulus

    // Here we need to add an output which writelns denominator storage and batch
    // inversion

    // We put the degree adjustment calculation into the memory map: Note this means
    // that if the exp used is used in non adjustment places in the constraints
    // those will now load this [in some cases]
    for &degree in adjustment_degrees {
        let implied_expression = RationalExpression::Exp(RationalExpression::X.into(), degree);
        // TODO - Clean this pattern
        #[allow(clippy::map_entry)]
        let flag = !memory_lookups.contains_key(&implied_expression);
        if flag {
            context.degree_adjustments.push(DegreeAdjustment {
                location: index * 32,
                exponent: degree,
            });
            let _ = memory_lookups.insert(implied_expression, format!("mload({})", index * 32));
            index += 1;
        }
    }

    let inverse_start_index = index;
    index += inverses.len();

    // Various offsets that appear in the header
    context.modulus_location = (index + inverses.len() + 6) * 32;
    context.input_data_size = in_data_size * 32;
    context.expmod_context = (index + inverses.len()) * 32;

    let mut inverse_position = inverse_start_index;
    for &exp in inverses.iter() {
        if let RationalExpression::Inv(a) = exp {
            context.batch_inverted.push(BatchInvert {
                location:   index * 32,
                expression: a.soldity_encode(&memory_lookups),
            });
        } else {
            panic!("Inverse search returned a non inverse");
        }

        // Out batch inversion will place the final inverted product before the
        // calculated denom
        let _ = memory_lookups.insert(exp.clone(), format!("mload({})", inverse_position * 32));
        inverse_position += 1;
        index += 1;
    }

    // Set batch inverse parameters in context
    context.products_to_values = inverses.len() * 32;
    context.partial_product_end_ptr = (inverse_start_index + inverses.len()) * 32;
    context.partial_product_start_ptr = (inverse_start_index) * 32;
    context.first_partial_product_ptr = inverse_start_index * 32;
    context.last_partial_product_ptr = (inverse_start_index + inverses.len()) * 32;

    // Add constraints to context
    let mut coefficient_index = 1 + claim_polynomial_keys.len() + periodic.len();
    for (exp, &degree) in constraint_expressions.iter().zip(adjustment_degrees.iter()) {
        let degree_adjustment_location = memory_lookups
            .get(&RationalExpression::Exp(
                RationalExpression::X.into(),
                degree,
            ))
            .unwrap()
            .to_owned();
        context.constraints.push(Constraint {
            first_coefficient_location: coefficient_index * 32,
            second_coefficient_location: (coefficient_index + 1) * 32,
            degree_adjustment_location,
            expression: exp.soldity_encode(&memory_lookups),
        });
        coefficient_index += 2;
    }

    // Render OodsPoly template
    let rendered = tt.render("oods_poly", &context)?;
    write!(file, "{}", &rendered)?;

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
