use crate::{constraints::*, trace_table::*};
use std::convert::TryInto;
use zkp_primefield::FieldElement;

pub(crate) fn check_constraints(
    constraints: &Constraints,
    table: &TraceTable,
) -> Result<(), (usize, usize)> {
    let trace_generator = FieldElement::root(table.num_rows()).unwrap();
    let mut current_root = FieldElement::ONE;

    for row in 0..table.num_rows() {
        // Note - Still in col row form
        let trace = |i: usize, j: isize| {
            let j: usize = j.try_into().unwrap();
            assert!(j == 0 || j == 1);
            table[((j + row) % table.num_rows(), i)].clone()
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
        x = FieldElement::ONE;
    } else {
        x = trace_generator.pow(row - 1)
    }

    let trace = |i: usize, j: isize| {
        let j: usize = j.try_into().unwrap();
        assert!(j == 0 || j == 1);
        table[((j + row) % table.num_rows(), i)].clone()
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
        trace[(800, 0)] = FieldElement::ZERO;
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
        trace[(0, 0)] = FieldElement::ZERO;
        assert_eq!(check_specific_constraint(&constraints, &trace, 0, 2), false);
    }
}
