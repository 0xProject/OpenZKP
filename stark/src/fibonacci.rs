use crate::{
    polynomial::Polynomial,
    proofs::{Constraint, TraceTable},
};
use primefield::FieldElement;
// use rayon::prelude::*;
use hex_literal::*;
use u256::{u256h, U256};

// TODO: Naming
#[allow(non_snake_case)]
pub fn get_trace_table(length: usize, witness: FieldElement) -> TraceTable {
    let mut T_0 = vec![FieldElement::ONE];
    let mut T_1 = vec![witness];
    for i in 1..length {
        T_0.push(T_1[(i - 1) as usize].clone());
        T_1.push(T_0[(i - 1) as usize].clone() + T_1[(i - 1) as usize].clone());
    }
    let mut final_vec = Vec::with_capacity(2 * length as usize);
    for i in 0..length {
        final_vec.push(T_0[i as usize].clone());
        final_vec.push(T_1[i as usize].clone());
    }
    TraceTable::new(length as usize, 2, final_vec)
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
        },
        Constraint {
            base:        Box::new(|tp, g| tp[1].shift(g) - tp[1].clone() - tp[0].clone()),
            numerator:   last_row.clone(),
            denominator: every_row.clone(),
        },
        Constraint {
            base:        Box::new(|tp, _| tp[0].clone() - Polynomial::new(&[FieldElement::ONE])),
            numerator:   no_rows.clone(),
            denominator: first_row,
        },
        Constraint {
            base:        Box::new(move |tp, _| {
                tp[0].clone() - Polynomial::new(&[claim_value.clone()])
            }),
            numerator:   no_rows,
            denominator: claim_index_row,
        },
    ]
}

// TODO: Naming
#[allow(non_snake_case, dead_code)]
pub fn eval_c_direct(
    x: &FieldElement,
    polynomials: &[Polynomial],
    claim_index: usize,
    claim: FieldElement,
    constraint_coefficients: &[FieldElement],
) -> FieldElement {
    let trace_len = 1024;
    let g = FieldElement::from(u256h!(
        "0659d83946a03edd72406af6711825f5653d9e35dc125289a206c054ec89c4f1"
    ));

    let eval_P0 = |x: FieldElement| -> FieldElement { polynomials[0].evaluate(&x) };
    let eval_P1 = |x: FieldElement| -> FieldElement { polynomials[1].evaluate(&x) };
    let eval_C0 = |x: FieldElement| -> FieldElement {
        ((eval_P0(&x * &g) - eval_P1(x.clone())) * (&x - &g.pow(U256::from(trace_len - 1))))
            / (&x.pow(U256::from(trace_len)) - FieldElement::ONE)
    };
    let eval_C1 = |x: FieldElement| -> FieldElement {
        ((eval_P1(&x * &g) - eval_P0(x.clone()) - eval_P1(x.clone()))
            * (&x - (&g.pow(U256::from(trace_len - 1)))))
            / (&x.pow(U256::from(trace_len)) - FieldElement::ONE)
    };
    let eval_C2 = |x: FieldElement| -> FieldElement {
        ((eval_P0(x.clone()) - FieldElement::ONE) * FieldElement::ONE) / (&x - FieldElement::ONE)
    };
    let eval_C3 = |x: FieldElement| -> FieldElement {
        (eval_P0(x.clone()) - claim) / (&x - &g.pow(U256::from(claim_index as u64)))
    };

    let deg_adj = |degree_bound: u64,
                   constraint_degree: u64,
                   numerator_degree: u64,
                   denominator_degree: u64|
     -> u64 {
        degree_bound + denominator_degree - 1 - constraint_degree - numerator_degree
    };

    let eval_C = |x: FieldElement| -> FieldElement {
        let composition_degree_bound = trace_len;
        let mut r = FieldElement::ZERO;
        r += &constraint_coefficients[0] * &eval_C0(x.clone());
        r += &constraint_coefficients[1]
            * &eval_C0(x.clone())
            * (&x).pow(U256::from(deg_adj(
                composition_degree_bound,
                trace_len - 1,
                1,
                trace_len,
            )));
        r += &constraint_coefficients[2] * &eval_C1(x.clone());
        r += &constraint_coefficients[3]
            * &eval_C1(x.clone())
            * (&x).pow(U256::from(deg_adj(
                composition_degree_bound,
                trace_len - 1,
                1,
                trace_len,
            )));
        r += &constraint_coefficients[4] * &eval_C2(x.clone());
        r += &constraint_coefficients[5]
            * &eval_C2(x.clone())
            * x.pow(U256::from(deg_adj(
                composition_degree_bound,
                trace_len - 1,
                0,
                1,
            )));
        r += &constraint_coefficients[6] * (eval_C3.clone())(x.clone());
        r += &constraint_coefficients[7]
            * &eval_C3(x.clone())
            * x.pow(U256::from(deg_adj(
                composition_degree_bound,
                trace_len - 1,
                0,
                1,
            )));
        r
    };
    eval_C(x.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::fibonacci::{get_fibonacci_constraints, get_trace_table};
    // use hex_literal::*;
    // use u256::{u256h, U256};
    use crate::proofs::{get_constraint_polynomial, interpolate_trace_table};
    #[test]
    fn mason() {
        let x = FieldElement::ZERO;
        let claim = FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        ));

        let witness = FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000000000000cafebabe"
        ));
        let trace_table = get_trace_table(1024, witness);
        let trace_polynomials = interpolate_trace_table(&trace_table);

        let mut constraint_coefficients = vec![FieldElement::ZERO; 20];
        constraint_coefficients[0] = FieldElement::ONE;
        constraint_coefficients[1] = FieldElement::ONE;
        constraint_coefficients[2] = FieldElement::ONE;
        constraint_coefficients[3] = FieldElement::ONE;
        constraint_coefficients[4] = FieldElement::ONE;
        constraint_coefficients[5] = FieldElement::ONE;
        constraint_coefficients[6] = FieldElement::ONE;

        let old = eval_c_direct(
            &x,
            &trace_polynomials,
            1000usize,
            claim.clone(),
            &constraint_coefficients,
        );

        let p = Polynomial::new(&[FieldElement::ONE, -&FieldElement::ONE]);
        assert_eq!(p.evaluate(&FieldElement::ONE), FieldElement::ZERO);

        let constraint_polynomial = get_constraint_polynomial(
            &trace_polynomials,
            &get_fibonacci_constraints(1024, claim, 1000usize),
            &constraint_coefficients,
        );
        let new = constraint_polynomial.evaluate(&x);

        assert_eq!(old, new);
    }
}
