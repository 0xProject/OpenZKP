use crate::{
    polynomial::eval_poly,
    proofs::{geometric_series, Constraint, TraceTable},
    utils::Reversible,
};
use hex_literal::*;
use primefield::{invert_batch, FieldElement};
use rayon::prelude::*;
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

// TODO: Naming
#[allow(non_snake_case)]
pub fn eval_whole_loop(
    LDEn: &[&[FieldElement]],
    constraint_coefficients: &[FieldElement],
    claim_index: usize,
    claim_fib: &FieldElement,
) -> Vec<FieldElement> {
    let eval_domain_size = LDEn[0].len();
    let beta = 2usize.pow(4);
    assert!(eval_domain_size % beta == 0);
    let trace_len = eval_domain_size / beta;

    let omega = FieldElement::root(U256::from(trace_len * beta)).unwrap();
    let g = omega.pow(U256::from(beta));
    let gen = FieldElement::GENERATOR;

    let mut CC = Vec::with_capacity(eval_domain_size);
    let g_trace = g.pow(U256::from(trace_len - 1));
    let g_claim = g.pow(U256::from(claim_index));
    let x = gen.clone();
    let x_trace = (&x).pow(U256::from(trace_len));
    let x_1023 = (&x).pow(U256::from(trace_len - 1));
    let omega_trace = (&omega).pow(U256::from(trace_len));
    let omega_1023 = (&omega).pow(U256::from(trace_len - 1));

    let x_omega_cycle = geometric_series(&x, &omega, eval_domain_size);
    let x_trace_cycle = geometric_series(&x_trace, &omega_trace, eval_domain_size);
    let x_1023_cycle = geometric_series(&x_1023, &omega_1023, eval_domain_size);

    let mut x_trace_sub_one: Vec<FieldElement> = Vec::with_capacity(eval_domain_size);
    let mut x_sub_one: Vec<FieldElement> = Vec::with_capacity(eval_domain_size);
    let mut x_g_claim_cycle: Vec<FieldElement> = Vec::with_capacity(eval_domain_size);

    x_omega_cycle
        .par_iter()
        .map(|i| (i - FieldElement::ONE, i - &g_claim))
        .unzip_into_vecs(&mut x_sub_one, &mut x_g_claim_cycle);

    x_trace_cycle
        .par_iter()
        .map(|i| i - FieldElement::ONE)
        .collect_into_vec(&mut x_trace_sub_one);

    let pool = vec![&x_trace_sub_one, &x_sub_one, &x_g_claim_cycle];

    let mut held = Vec::with_capacity(3);
    pool.par_iter()
        .map(|i| invert_batch(i))
        .collect_into_vec(&mut held);

    x_g_claim_cycle = held.pop().unwrap();
    x_sub_one = held.pop().unwrap();
    x_trace_sub_one = held.pop().unwrap();

    (0..eval_domain_size)
        .into_par_iter()
        .map(|reverse_index| {
            // OPT: Eliminate index by generating x_* cycles in bit-reversed order using
            // fft.
            let index = reverse_index.bit_reverse_at(eval_domain_size);
            let next_reverse_index =
                ((index + beta as usize) % eval_domain_size).bit_reverse_at(eval_domain_size);

            let P0 = LDEn[0][reverse_index].clone();
            let P1 = LDEn[1][reverse_index].clone();
            let P0n = LDEn[0][next_reverse_index].clone();
            let P1n = LDEn[1][next_reverse_index].clone();

            let A = x_trace_sub_one[index].clone();
            let C0 = (&P0n - &P1) * (&x_omega_cycle[index] - &g_trace) * &A;
            let C1 = (&P1n - &P0 - &P1) * (&x_omega_cycle[index] - &g_trace) * &A;
            let C2 = (&P0 - FieldElement::ONE) * &x_sub_one[index];
            let C3 = (&P0 - claim_fib) * &x_g_claim_cycle[index];

            let C0a = &C0 * &x_1023_cycle[index];
            let C1a = &C1 * &x_1023_cycle[index];
            let C2a = &C2 * &x_omega_cycle[index];
            let C3a = &C3 * &x_omega_cycle[index];

            let mut r = FieldElement::ZERO;
            r += &constraint_coefficients[0] * C0;
            r += &constraint_coefficients[1] * C0a;
            r += &constraint_coefficients[2] * C1;
            r += &constraint_coefficients[3] * C1a;
            r += &constraint_coefficients[4] * C2;
            r += &constraint_coefficients[5] * C2a;
            r += &constraint_coefficients[6] * C3;
            r += &constraint_coefficients[7] * C3a;

            r
        })
        .collect_into_vec(&mut CC);
    CC
}

// TODO: Naming
#[allow(non_snake_case)]
pub fn eval_c_direct(
    x: &FieldElement,
    polynomials: &[&[FieldElement]],
    claim_index: usize,
    claim: FieldElement,
    constraint_coefficients: &[FieldElement],
) -> FieldElement {
    let trace_len = 1024;
    let g = FieldElement::from(u256h!(
        "0659d83946a03edd72406af6711825f5653d9e35dc125289a206c054ec89c4f1"
    ));

    let eval_P0 = |x: FieldElement| -> FieldElement { eval_poly(x, polynomials[0]) };
    let eval_P1 = |x: FieldElement| -> FieldElement { eval_poly(x, polynomials[1]) };
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

pub fn get_constraint() -> Constraint<'static> {
    Constraint::new(20, &eval_c_direct, Some(&eval_whole_loop))
}
