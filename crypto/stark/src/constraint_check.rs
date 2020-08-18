use crate::{constraints::Constraints, trace_table::TraceTable};
use std::convert::TryInto;
use zkp_primefield::{FieldElement, One, Pow, Root};

#[allow(clippy::doc_markdown)]
/// # Check a set of constraints on a trace table
///
/// ## Input
///
/// A `ConstraintSystem` which captures the claim that is made.
/// A `TraceTable` which is the witness to this claim.
///
/// ## Output
///
/// A Result which indicated the constraint and row which failed
///
/// ## Constraint Checking
///
/// The function iterates through the rows of the table and the rational
/// expression constraints and runs a check function on the rational expression
/// which checks that it is well defined. This will return the evaluated value
/// and true if the expression either contains no division by zero or each
/// division by zero is multiplied by a zero [which we set as the proper
/// evaluation of the statement]. It is not a 100% accurate check and may have
/// false negatives when complex nested inverted constraints are used, best
/// practice in using it is to simplify all fractions in the constraint
/// expression. Moreover this system of checking is not guaranteed to work for
/// every expression that could be committed too. The best test that a
/// constraint system holds is to try to run a complete proof. The performance
/// of this function also depends heavily on the size of the system so for the
/// best experience using it to check constraints while developing it is best to
/// limit the trace table to the smallest meeting your needs.

pub fn check_constraints(
    constraints: &Constraints,
    table: &TraceTable,
) -> Result<(), (usize, usize)> {
    let trace_generator = FieldElement::root(table.num_rows()).unwrap();
    let mut current_root = FieldElement::one();
    let len = table.num_rows();

    for row in 0..len {
        // Note - Still in col row form
        let trace = |i: usize, j: isize| {
            if j.is_positive() {
                let j: usize = j.try_into().unwrap();
                table[((j + row) % len, i)].clone()
            } else {
                let j: usize = j.abs().try_into().unwrap();
                if row < j {
                    table[(len + row - j, i)].clone()
                } else {
                    table[(row - j, i)].clone()
                }
            }
        };
        for (which, expression) in constraints.expressions().iter().enumerate() {
            if !expression.check(&current_root, &trace).1 {
                return Err((row, which));
            }
        }
        current_root *= &trace_generator;
    }
    Ok(())
}

pub(crate) fn check_specific_constraint(
    constraints: &Constraints,
    table: &TraceTable,
    row: usize,
    which_constraint: usize,
) -> bool {
    let trace_generator = FieldElement::root(table.num_rows()).unwrap();
    let x;
    if row == 0 {
        x = FieldElement::one();
    } else {
        x = trace_generator.pow(row - 1)
    }
    let len = table.num_rows();

    let trace = |i: usize, j: isize| {
        if j.is_positive() {
            let j: usize = j.try_into().unwrap();
            table[((j + row) % len, i)].clone()
        } else {
            let j: usize = j.abs().try_into().unwrap();
            if row < j {
                table[(len + row - j, i)].clone()
            } else {
                table[(row - j, i)].clone()
            }
        }
    };

    constraints.expressions()[which_constraint]
        .check(&x, &trace)
        .1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{traits::tests::Recurrance, Provable, Verifiable};
    use zkp_macros_decl::field_element;
    use zkp_primefield::Zero;
    use zkp_u256::U256;

    #[test]
    fn checker_test() {
        let recurrance = Recurrance {
            index:         1000,
            initial_value: field_element!("cafebabe"),
            exponent:      1,
        };
        let witness = recurrance.witness();
        let claim = recurrance.claim();

        let mut constraints = claim.constraints();
        let mut trace = claim.trace(&witness);
        constraints.blowup = 16;
        constraints.pow_bits = 12;
        constraints.num_queries = 20;
        constraints.fri_layout = vec![3, 2];
        assert_eq!(check_constraints(&constraints, &trace), Ok(()));
        trace[(800, 0)] = FieldElement::zero();
        assert_eq!(check_constraints(&constraints, &trace), Err((799, 0)));
    }

    #[test]
    fn specific_constraint_checker() {
        let recurrance = Recurrance {
            index:         1000,
            initial_value: field_element!("cafebabe"),
            exponent:      1,
        };
        let witness = recurrance.witness();
        let claim = recurrance.claim();

        let mut constraints = claim.constraints();
        let mut trace = claim.trace(&witness);
        constraints.blowup = 16;
        constraints.pow_bits = 12;
        constraints.num_queries = 20;
        constraints.fri_layout = vec![3, 2];
        assert_eq!(
            check_specific_constraint(&constraints, &trace, 1000, 1),
            true
        );
        trace[(0, 0)] = FieldElement::zero();
        assert_eq!(check_specific_constraint(&constraints, &trace, 0, 2), false);
    }
}
