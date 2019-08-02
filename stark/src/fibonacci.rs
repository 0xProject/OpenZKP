use crate::{
    polynomial::Polynomial,
    proofs::{Constraint, TraceTable},
};
use primefield::FieldElement;
use u256::U256;

pub fn get_trace_table(length: usize, witness: FieldElement) -> TraceTable {
    let mut elements = vec![FieldElement::ONE, witness];
    for i in 1..length {
        elements.push(elements[2 * i - 1].clone());
        elements.push(&elements[2 * i - 2] + &elements[2 * i - 1]);
    }
    TraceTable::new(length, 2, elements)
}

pub fn get_fibonacci_constraints(
    trace_length: usize,
    claim_value: FieldElement,
    claim_index: usize,
) -> Vec<Constraint> {
    let trace_generator = FieldElement::root(U256::from(trace_length as u64)).unwrap();
    let no_rows = Polynomial::new(&[FieldElement::ONE]);
    let every_row =
        Polynomial::from_sparse(&[(trace_length, FieldElement::ONE), (0, -&FieldElement::ONE)]);
    let first_row = Polynomial::new(&[-&FieldElement::ONE, FieldElement::ONE]);
    let last_row = Polynomial::new(&[
        -&trace_generator.pow(U256::from(trace_length as u64 - 1)),
        FieldElement::ONE,
    ]);

    let claim_index_row = Polynomial::new(&[
        -&trace_generator.pow(U256::from(claim_index as u64)),
        FieldElement::ONE,
    ]);

    vec![
        Constraint {
            base:        Box::new(|tp, g| tp[0].shift(g) - tp[1].clone()),
            numerator:   last_row.clone(),
            denominator: every_row.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(|tp, g| tp[1].shift(g) - tp[1].clone() - tp[0].clone()),
            numerator:   last_row.clone(),
            denominator: every_row.clone(),
            adjustment:  Polynomial::from_sparse(&[(trace_length - 1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(|tp, _| tp[0].clone() - Polynomial::new(&[FieldElement::ONE])),
            numerator:   no_rows.clone(),
            denominator: first_row,
            adjustment:  Polynomial::from_sparse(&[(1, FieldElement::ONE)]),
        },
        Constraint {
            base:        Box::new(move |tp, _| {
                tp[0].clone() - Polynomial::new(&[claim_value.clone()])
            }),
            numerator:   no_rows,
            denominator: claim_index_row,
            adjustment:  Polynomial::from_sparse(&[(1, FieldElement::ONE)]),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proofs::{get_constraint_polynomial, interpolate_trace_table};
    use hex_literal::*;
    use u256::u256h;

    #[test]
    fn example_fibonacci_constraint_polynomial() {
        let witness = FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000000000000cafebabe"
        ));
        let trace_table = get_trace_table(1024, witness);
        let trace_polynomials = interpolate_trace_table(&trace_table);

        let claim = FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        ));
        let constraints = get_fibonacci_constraints(1024, claim, 1000usize);
        let constraint_coefficients = vec![FieldElement::ONE; 2 * constraints.len()];

        let constraint_polynomial =
            get_constraint_polynomial(&trace_polynomials, &constraints, &constraint_coefficients);

        let x = FieldElement::from(U256::from(12u64));
        let value = constraint_polynomial.evaluate(&x);
        let expected = FieldElement(u256h!(
            "030feab92ebd0e8349ebf27147138b100deecf4d2edec71a8e97abb2f71443d8"
        ));

        assert_eq!(value, expected);
    }
}
