use crate::{
    polynomial::eval_poly,
    proofs::{geometric_series, Constraint, TraceTable},
};
use hex_literal::*;
use primefield::{invert_batch, FieldElement};
use rayon::prelude::*;
use u256::{u256h, U256};

// TODO: Naming
#[allow(non_snake_case)]
pub fn get_trace_table(length: u64, witness: FieldElement) -> TraceTable {
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
    let eval_domain_size_usize = LDEn[0].len();
    let eval_domain_size = eval_domain_size_usize as u64;
    let beta = 2_u64.pow(4);
    let trace_len = eval_domain_size / beta;

    let omega = FieldElement::root(U256::from(trace_len * beta)).unwrap();
    let g = omega.pow(U256::from(beta));
    let gen = FieldElement::GENERATOR;

    let mut CC = Vec::with_capacity(eval_domain_size_usize);
    let g_trace = g.pow(U256::from(trace_len - 1));
    let g_claim = g.pow(U256::from(claim_index as u64));
    let x = gen.clone();
    let x_trace = (&x).pow(U256::from(trace_len));
    let x_1023 = (&x).pow(U256::from(trace_len - 1));
    let omega_trace = (&omega).pow(U256::from(trace_len));
    let omega_1023 = (&omega).pow(U256::from(trace_len - 1));

    let x_omega_cycle = geometric_series(&x, &omega, eval_domain_size_usize);
    let x_trace_cycle = geometric_series(&x_trace, &omega_trace, eval_domain_size_usize);
    let x_1023_cycle = geometric_series(&x_1023, &omega_1023, eval_domain_size_usize);

    let mut x_trace_sub_one: Vec<FieldElement> = Vec::with_capacity(eval_domain_size_usize);
    let mut x_sub_one: Vec<FieldElement> = Vec::with_capacity(eval_domain_size_usize);
    let mut x_g_claim_cycle: Vec<FieldElement> = Vec::with_capacity(eval_domain_size_usize);

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

    (0..eval_domain_size_usize)
        .into_par_iter()
        .map(|i| {
            let j = ((i as u64) + beta) % eval_domain_size;

            let P0 = LDEn[0][i as usize].clone();
            let P1 = LDEn[1][i as usize].clone();
            let P0n = LDEn[0][j as usize].clone();
            let P1n = LDEn[1][j as usize].clone();

            let A = x_trace_sub_one[i as usize].clone();
            let C0 = (&P0n - &P1) * (&x_omega_cycle[i as usize] - &g_trace) * &A;
            let C1 = (&P1n - &P0 - &P1) * (&x_omega_cycle[i as usize] - &g_trace) * &A;
            let C2 = (&P0 - FieldElement::ONE) * &x_sub_one[i as usize];
            let C3 = (&P0 - claim_fib) * &x_g_claim_cycle[i as usize];

            let C0a = &C0 * &x_1023_cycle[i as usize];
            let C1a = &C1 * &x_1023_cycle[i as usize];
            let C2a = &C2 * &x_omega_cycle[i as usize];
            let C3a = &C3 * &x_omega_cycle[i as usize];

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{channel::*, fft::*, merkle::*, utils::Reversible};
    use tiny_keccak::Keccak;

    #[test]
    // TODO - When the proofs function is changed to included functions for each
    // step of the proof, change this to make a set of unit tests of each of them
    // [then move to proofs].
    #[allow(clippy::cognitive_complexity)]
    // TODO: Naming
    #[allow(non_snake_case)]
    fn fib_proof_test() {
        let trace_len = 1024;
        let beta = 2_u64.pow(4);
        let omega = FieldElement::from(u256h!(
            "0393a32b34832dbad650df250f673d7c5edd09f076fc314a3e5a42f0606082e1"
        ));
        let g = FieldElement::from(u256h!(
            "0659d83946a03edd72406af6711825f5653d9e35dc125289a206c054ec89c4f1"
        ));
        let eval_domain_size = trace_len * beta;
        let eval_domain_size_usize = eval_domain_size as usize;

        assert_eq!(omega.pow(U256::from(eval_domain_size)), FieldElement::ONE);
        assert_eq!(g.pow(U256::from(trace_len)), FieldElement::ONE);

        let gen = FieldElement::from(U256::from(3_u64));
        let mut trace_x = Vec::with_capacity(trace_len as usize);
        let mut eval_x = Vec::with_capacity((eval_domain_size) as usize);
        let mut eval_offset_x = Vec::with_capacity((eval_domain_size) as usize);

        for i in 0..trace_len {
            trace_x.push(g.pow(U256::from(i)));
        }
        for i in 0..(eval_domain_size) {
            let hold = omega.pow(U256::from(i));
            eval_x.push(hold.clone());
            eval_offset_x.push(hold * &gen);
        }

        assert_eq!(
            U256::from(trace_x[500].clone()),
            u256h!("07fa71827697ee902dbd6e44720371cec088c639337b2302d9eec253038b9789")
        );
        assert_eq!(
            U256::from(eval_x[500].clone()),
            u256h!("068a24ef8b13c6b23a4fe31235667142494bc0eecbb59ed9866a44ac47fb2f6b")
        );
        assert_eq!(
            U256::from(eval_offset_x[500].clone()),
            u256h!("039e6ecea13b53f4aeefa936a03353c6dbe342cc6320dc8c933ece04d7f18e3f")
        );

        let witness = FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000000000000cafebabe"
        ));
        let claim_index = 1000_u64;
        let claim_fib = FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        ));

        let mut T_0 = vec![FieldElement::ONE];
        let mut T_1 = vec![witness];
        for i in 1..trace_len {
            T_0.push(T_1[(i - 1) as usize].clone());
            T_1.push(T_0[(i - 1) as usize].clone() + T_1[(i - 1) as usize].clone());
        }
        assert_eq!(T_0[1000], claim_fib);

        let TP0 = ifft(&T_0.as_slice());
        let TP1 = ifft(&T_1.as_slice());

        assert_eq!(eval_poly(trace_x[1000].clone(), &TP0.as_slice()), T_0[1000]);

        let mut LDE0 = vec![FieldElement::ZERO; eval_x.len()];
        let mut LDE1 = vec![FieldElement::ZERO; eval_x.len()];

        for j in 0..beta {
            let mut vals = fft_cofactor(&(TP0.as_slice()), &(&gen * &omega.pow(U256::from(j))));
            for i in 0..trace_len {
                LDE0[((i * beta + j) % (eval_domain_size)) as usize] = vals[i as usize].clone();
            }

            vals = fft_cofactor(&(TP1.as_slice()), &(&gen * &omega.pow(U256::from(j))));
            for i in 0..trace_len {
                LDE1[((i * beta + j) % (eval_domain_size)) as usize] = vals[i as usize].clone();
            }
        }
        assert_eq!(
            eval_poly(eval_offset_x[13644].clone(), &(TP0.as_slice())),
            LDE0[13644]
        );
        assert_eq!(
            eval_poly(eval_offset_x[13644].clone(), &(TP1.as_slice())),
            LDE1[13644]
        );

        let leaf = |i: u64| -> Vec<U256> {
            vec![
                LDE0[(i.bit_reverse() >> 50) as usize].0.clone(),
                LDE1[(i.bit_reverse() >> 50) as usize].0.clone(),
            ]
        };

        assert_eq!(
            (leaf(3243))[0].clone(),
            u256h!("01ddd9e389a326817ad1d2a5311e1bc2cf7fa734ebdc2961085b5acfa87a58ff")
        );
        assert_eq!(
            (leaf(3243))[1].clone(),
            u256h!("03dbc6c47df0606997c2cefb20c4277caf2b76bca1d31c13432f71cdd93b3718")
        );

        let mut leaves = Vec::with_capacity(eval_domain_size_usize);
        for i in 0..eval_domain_size {
            leaves.push(leaf(i));
        }
        let leaf_pointer: Vec<&[U256]> = leaves.iter().map(|x| x.as_slice()).collect();
        let tree = make_tree(leaf_pointer.as_slice());

        assert_eq!(
            tree[1],
            hex!("018dc61f748b1a6c440827876f30f63cb6c4c188000000000000000000000000")
        );

        let mut public_input = [claim_index.to_be_bytes()].concat();
        public_input.extend_from_slice(&claim_fib.0.to_bytes_be());

        let mut proof = Channel::new(&public_input.as_slice());
        assert_eq!(
            proof.digest,
            hex!("c891a11ddbc6c425fad523a7a4aeafa505d7aa1638cfffbd5b747100bc69e367")
        );
        proof.write(&tree[1]);
        assert_eq!(
            proof.digest,
            hex!("b7d80385fa0c8879473cdf987ea7970bb807aec78bb91af39a1504d965ad8e92")
        );
        let test_element: FieldElement = proof.read();
        assert_eq!(
            U256::from(test_element),
            u256h!("0529fc64b01be65623ef376bfa31d62b9a75ba2f51b5fda79e55e2ac05dfa80f")
        );

        let mut constraint_coefficients = Vec::with_capacity(8);
        for _ in 0..8 {
            constraint_coefficients.push(proof.read());
        }

        let eval_P0 = |x: FieldElement| -> FieldElement { eval_poly(x, &TP0) };
        let eval_P1 = |x: FieldElement| -> FieldElement { eval_poly(x, &TP1) };
        let eval_C0 = |x: FieldElement| -> FieldElement {
            ((eval_P0(&x * &g) - eval_P1(x.clone())) * (&x - &g.pow(U256::from(trace_len - 1))))
                / (&x.pow(U256::from(trace_len)) - FieldElement::ONE)
        };
        let eval_C1 = |x: FieldElement| -> FieldElement {
            ((eval_P1(&x * &g) - eval_P0(x.clone()) - eval_P1(x.clone()))
                * (&x - &g.pow(U256::from(trace_len - 1))))
                / (&x.pow(U256::from(trace_len)) - FieldElement::ONE)
        };
        let eval_C2 = |x: FieldElement| -> FieldElement {
            ((eval_P0(x.clone()) - FieldElement::ONE) * FieldElement::ONE)
                / (&x - FieldElement::ONE)
        };
        let eval_C3 = |x: FieldElement| -> FieldElement {
            (eval_P0(x.clone()) - claim_fib.clone()) / (&x - &g.pow(U256::from(claim_index)))
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
            r += &constraint_coefficients[6] * &eval_C3(x.clone());
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
        let z = FieldElement::from(u256h!(
            "0622cc7ddb1b061b1b59a071b74754245186e9b5eb3fbf43b2defab80a8cfe46"
        ));
        assert_eq!(
            U256::from(&eval_C(z)),
            u256h!("02e7cbb3fc164554f931e769b21e990d039b64385782b92955c33a6acff58956")
        );

        let mut CC = vec![FieldElement::ZERO; eval_domain_size_usize];
        let g_trace = g.pow(U256::from(trace_len - 1));
        let g_claim = g.pow(U256::from(claim_index));
        let x = gen.clone();
        let mut x_trace = (&x).pow(U256::from(trace_len));
        let mut x_1023 = (&x).pow(U256::from(1023_u64));
        let omega_trace = (&omega).pow(U256::from(trace_len));
        let omega_1023 = (&omega).pow(U256::from(1023_u64));

        let mut x = gen.clone();
        let mut x_omega_cycle = Vec::with_capacity(eval_domain_size_usize);
        let mut x_trace_cycle = Vec::with_capacity(eval_domain_size_usize);
        let mut x_1023_cycle = Vec::with_capacity(eval_domain_size_usize);

        let mut x_trace_sub_one: Vec<FieldElement> = Vec::with_capacity(eval_domain_size_usize);
        let mut x_sub_one: Vec<FieldElement> = Vec::with_capacity(eval_domain_size_usize);
        let mut x_g_claim_cycle: Vec<FieldElement> = Vec::with_capacity(eval_domain_size_usize);

        for _ in 0..(eval_domain_size_usize) {
            x_omega_cycle.push(x.clone());
            x_trace_cycle.push(x_trace.clone());
            x_1023_cycle.push(x_1023.clone());

            x_trace_sub_one.push(&x_trace - FieldElement::ONE);
            x_sub_one.push(&x - FieldElement::ONE);
            x_g_claim_cycle.push(&x - &g_claim);

            x_trace *= &omega_trace;
            x *= &omega;
            x_1023 *= &omega_1023;
        }

        x_trace_sub_one = invert_batch(&x_trace_sub_one);
        x_sub_one = invert_batch(&x_sub_one);
        x_g_claim_cycle = invert_batch(&x_g_claim_cycle);

        for i in 0..eval_domain_size {
            let j = (i + beta) % eval_domain_size;

            let P0 = LDE0[i as usize].clone();
            let P1 = LDE1[i as usize].clone();
            let P0n = LDE0[j as usize].clone();
            let P1n = LDE1[j as usize].clone();

            let A = x_trace_sub_one[i as usize].clone();
            let C0 = (&P0n - &P1) * (&x_omega_cycle[i as usize] - &g_trace) * &A;
            let C1 = (&P1n - &P0 - &P1) * (&x_omega_cycle[i as usize] - &g_trace) * &A;
            let C2 = (&P0 - FieldElement::ONE) * &x_sub_one[i as usize];
            let C3 = (&P0 - &claim_fib) * &x_g_claim_cycle[i as usize];

            let C0a = &C0 * &x_1023_cycle[i as usize];
            let C1a = &C1 * &x_1023_cycle[i as usize];
            let C2a = &C2 * &x_omega_cycle[i as usize];
            let C3a = &C3 * &x_omega_cycle[i as usize];

            let mut r = FieldElement::ZERO;
            r += &constraint_coefficients[0] * C0;
            r += &constraint_coefficients[1] * C0a;
            r += &constraint_coefficients[2] * C1;
            r += &constraint_coefficients[3] * C1a;
            r += &constraint_coefficients[4] * C2;
            r += &constraint_coefficients[5] * C2a;
            r += &constraint_coefficients[6] * C3;
            r += &constraint_coefficients[7] * C3a;

            CC[i as usize] = r;
        }
        assert_eq!(CC[123].clone(), eval_C(eval_offset_x[123].clone()));

        let leaf_constraint = |i: u64| -> U256 { CC[(i.bit_reverse() >> 50) as usize].0.clone() };
        let mut leaves_con = Vec::with_capacity(eval_domain_size_usize);
        for i in 0..eval_domain_size {
            leaves_con.push(leaf_constraint(i));
        }
        let ctree = make_tree(leaves_con.as_slice());
        assert_eq!(
            ctree[1],
            hex!("46318de7dbdafda87c1052d50989d15f8e61a5b8000000000000000000000000")
        );

        proof.write(&ctree[1]);
        let oods_point: FieldElement = proof.read();
        assert_eq!(
            U256::from(oods_point.clone()),
            u256h!("031dc8fc2f57e3f39f6951a04a04294a7c63c988573dc058eea4cbf3e6268353")
        );

        let oods_values = vec![
            (&eval_P0)(oods_point.clone()),
            (&eval_P0)(&oods_point * &g),
            (&eval_P1)(oods_point.clone()),
            (&eval_P1)(&oods_point * &g),
            (&eval_C)(oods_point.clone()),
        ];
        for element in oods_values.iter() {
            proof.write(element);
        }
        assert_eq!(
            proof.digest,
            hex!("f556f04f342598411b5626a797a114a64b3a15a5ab0d4f2a6b350b941d56d071")
        );

        let mut oods_coefficients = Vec::with_capacity(5);
        for _ in 0..5 {
            oods_coefficients.push(proof.read());
        }

        let oods = |x: &FieldElement| -> FieldElement {
            let mut r = FieldElement::ZERO;
            r += &oods_coefficients[0] * (eval_P0(x.clone()) - &oods_values[0]) / (x - &oods_point);
            r += &oods_coefficients[1] * (eval_P0(x.clone()) - &oods_values[1])
                / (x - &oods_point * &g);
            r += &oods_coefficients[2] * (eval_P1(x.clone()) - &oods_values[2]) / (x - &oods_point);
            r += &oods_coefficients[3] * (eval_P1(x.clone()) - &oods_values[3])
                / (x - &oods_point * &g);
            r += &oods_coefficients[4] * (eval_C(x.clone()) - &oods_values[4]) / (x - &oods_point);
            r
        };
        assert_eq!(
            U256::from(oods(&eval_offset_x[20])),
            u256h!("0362a57323b84f8eed48f0d0e68fe2282cd7333c46778f4bfb307f4317acea58")
        );

        let mut CO = vec![FieldElement::ZERO; eval_domain_size_usize];

        let mut x_oods_cycle: Vec<FieldElement> = Vec::with_capacity(eval_domain_size_usize);
        let mut x_oods_cycle_g: Vec<FieldElement> = Vec::with_capacity(eval_domain_size_usize);
        for i in 0..(eval_domain_size_usize) {
            x_omega_cycle.push(x_omega_cycle[i].clone());
            x_oods_cycle.push(&x_omega_cycle[i] - &oods_point);
            x_oods_cycle_g.push(&x_omega_cycle[i] - &oods_point * &g);
        }

        x_oods_cycle = invert_batch(x_oods_cycle.as_slice());
        x_oods_cycle_g = invert_batch(x_oods_cycle_g.as_slice());

        for i in 0..eval_domain_size {
            let A = FieldElement::ONE * &x_oods_cycle[i as usize];
            let B = FieldElement::ONE * &x_oods_cycle_g[i as usize];
            let mut r = FieldElement::ZERO;
            r += &oods_coefficients[0] * (&LDE0[i as usize] - &oods_values[0]) * &A;
            r += &oods_coefficients[1] * (&LDE0[i as usize] - &oods_values[1]) * &B;
            r += &oods_coefficients[2] * (&LDE1[i as usize] - &oods_values[2]) * &A;
            r += &oods_coefficients[3] * (&LDE1[i as usize] - &oods_values[3]) * &B;
            r += &oods_coefficients[4] * (&CC[i as usize] - &oods_values[4]) * &A;

            CO[i as usize] = r;
        }

        assert_eq!(CO[4321].clone(), oods(&eval_offset_x[4321]));

        let fri_layer =
            |previous: &[FieldElement], evaluation_point: &FieldElement| -> Vec<FieldElement> {
                let n = previous.len() as u64;
                let s = eval_domain_size / n;
                let mut next = vec![FieldElement::ZERO; (n / 2) as usize];
                for i in 0..n / 2 {
                    let j = (n / 2 + i) % n;
                    let m = eval_x.len() as u64;
                    let ind = ((m - i) * s) % m;
                    let x_inv = &eval_x[ind as usize];
                    let a = &previous[i as usize];
                    let b = &previous[j as usize];
                    let r = (a + b) + evaluation_point * x_inv * (a - b);
                    next[i as usize] = r;
                }
                next
            };
        let fri_tree = |layer: &[FieldElement], coset_size: u64| -> Vec<[u8; 32]> {
            let n = layer.len();
            let bits = 64 - (n as u64).leading_zeros(); // Floored base 2 log
            let mut internal_leaves = Vec::new();
            for i in (0..n).step_by(coset_size as usize) {
                let mut internal_leaf = Vec::new();
                for j in 0..coset_size {
                    internal_leaf.push(
                        layer[(((i as u64) + j).bit_reverse() >> (64 - bits + 1)) as usize]
                            .0
                            .clone(),
                    );
                }
                internal_leaves.push(internal_leaf);
            }
            let leaf_pointer: Vec<&[U256]> = internal_leaves.iter().map(|x| x.as_slice()).collect();
            make_tree(leaf_pointer.as_slice())
        };
        let mut fri = Vec::with_capacity(14);
        fri.push(CO);
        let fri_tree_1 = fri_tree(&(fri[0].as_slice()), 8);
        assert_eq!(
            fri_tree_1[1],
            hex!("f5110a80f0fabf114678f7e643a2be01f88661fe000000000000000000000000")
        );
        proof.write(&fri_tree_1[1]);

        let mut eval_point = proof.read();
        fri.push(fri_layer(&(fri[0].as_slice()), &eval_point));
        fri.push(fri_layer(&(fri[1].as_slice()), &(eval_point.square())));
        fri.push(fri_layer(
            &(fri[2].as_slice()),
            &(eval_point.square().square()),
        ));
        let fri_tree_2 = fri_tree(&(fri[3].as_slice()), 4);
        assert_eq!(
            fri_tree_2[1],
            hex!("27ad2f6a19d18a7e4535905f1ee0bf0d39e8e444000000000000000000000000")
        );
        proof.write(&fri_tree_2[1]);

        eval_point = proof.read();
        fri.push(fri_layer(&(fri[3].as_slice()), &eval_point));
        fri.push(fri_layer(&(fri[4].as_slice()), &(eval_point.square())));

        let mut last_layer_coefficents = ifft(&(fri[5].as_slice()));
        last_layer_coefficents.truncate(32);
        assert_eq!(
            eval_poly(
                FieldElement::from(u256h!(
                    "011d07d97880b4dc0234b46e6d4b9bd4a1f811a9e3bc8e32cc4074a0c13a1d0b"
                ))
                .pow(U256::from(42_u64)),
                &(last_layer_coefficents.as_slice())
            ),
            fri[5][42]
        );
        proof.write(last_layer_coefficents.as_slice());
        assert_eq!(
            proof.digest,
            hex!("e2c7e50f3d1dcaad74678d8abb489675849ead08e2f848429a136304d9550bb6")
        );

        let pow_find_nonce = |pow_bits: u32| -> u64 {
            let mut seed = vec![1_u8, 35_u8, 69_u8, 103_u8, 137_u8, 171_u8, 205_u8, 237_u8];
            seed.extend_from_slice(&proof.digest);
            for byte in pow_bits.to_be_bytes().iter() {
                if *byte > 0 {
                    seed.push(*byte);
                    break;
                }
            }
            let mut seed_res = [0_u8; 32];
            let mut sha3 = Keccak::new_keccak256();
            sha3.update(&seed);
            sha3.finalize(&mut seed_res);

            for n in 0..(u64::max_value() as usize) {
                let mut sha3 = Keccak::new_keccak256();
                let mut res = [0; 32];
                sha3.update(&seed_res);
                sha3.update(&(n.to_be_bytes()));
                sha3.finalize(&mut res);
                let final_int = U256::from_bytes_be(&res);
                if final_int.leading_zeros() >= pow_bits as usize {
                    // Only do the large int compare if the quick logs match
                    return n as u64;
                }
            }
            0
        };
        let nonce = pow_find_nonce(12);
        assert_eq!(nonce, 3465);
        proof.write(nonce);

        let num_queries = 20;
        let mut query_indices = Vec::with_capacity(num_queries + 3);
        while query_indices.len() < num_queries {
            let val: U256 = proof.read();
            query_indices
                .push(((val.clone() >> (0x100 - 0x040)).c0 & (2_u64.pow(14) - 1)) as usize);
            query_indices
                .push(((val.clone() >> (0x100 - 0x080)).c0 & (2_u64.pow(14) - 1)) as usize);
            query_indices
                .push(((val.clone() >> (0x100 - 0x0C0)).c0 & (2_u64.pow(14) - 1)) as usize);
            query_indices.push(
                ((
                    val
                    // >> (0x100 - 0x100)
                )
                    .c0
                    & (2_u64.pow(14) - 1)) as usize,
            );
        }
        query_indices.truncate(num_queries);
        assert_eq!(query_indices[19], 11541);
        (&mut query_indices).sort_unstable(); // Fast inplace sort that doesn't preserve the order of equal elements.

        for index in query_indices.iter() {
            proof.write(&LDE0[((*index as u64).bit_reverse() >> 50) as usize]);
            proof.write(&LDE1[((*index as u64).bit_reverse() >> 50) as usize]);
        }
        assert_eq!(
            proof.digest,
            hex!("664fa06e3244baea41f87dcfb26316c7ebf13e36d2f88bbd3f216197181db5d5")
        );

        let mut decommitment = crate::merkle::proof(&tree, &(query_indices.as_slice()));
        for x in decommitment.iter() {
            proof.write(x);
        }
        assert_eq!(
            proof.digest,
            hex!("804a12f5f778c9d2b076d07a8c516dd8e1a57c35ef2df10f55df58764812799d")
        );

        for index in query_indices.iter() {
            proof.write(&CC[((*index as u64).bit_reverse() >> 50) as usize]);
        }
        decommitment = crate::merkle::proof(&ctree, &(query_indices.as_slice()));
        for x in decommitment.iter() {
            proof.write(x);
        }
        assert_eq!(
            proof.digest,
            hex!("ea73885255f98e9a51f6549fb74e076181971e426190660cdc45bac337423cb6")
        );

        let fri_indices: Vec<usize> = query_indices.iter().map(|x| x / 8).collect();
        for i in fri_indices.iter() {
            for j in 0..8 {
                let n = i * 8 + j;
                if query_indices.binary_search(&n).is_ok() {
                    continue;
                } else {
                    proof.write(&fri[0][((n as u64).bit_reverse() >> 50) as usize]);
                }
            }
        }
        decommitment = crate::merkle::proof(&fri_tree_1, &(fri_indices.as_slice()));
        for x in decommitment.iter() {
            proof.write(x);
        }
        assert_eq!(
            proof.digest,
            hex!("ce932c02384abaa5e3cdc266812a0ea8880e490e174f7de55ba8e29dd1c01f88")
        );

        let fri_low_indices: Vec<usize> = query_indices.iter().map(|x| x / 32).collect();
        for i in fri_low_indices.iter() {
            for j in 0..4 {
                let n = i * 4 + j;
                if fri_indices.binary_search(&n).is_ok() {
                    continue;
                } else {
                    proof.write(&fri[3][((n as u64).bit_reverse() >> 53) as usize]);
                }
            }
        }
        decommitment = crate::merkle::proof(&fri_tree_2, &(fri_low_indices.as_slice()));
        for x in decommitment.iter() {
            proof.write(x);
        }
        assert_eq!(
            proof.digest,
            hex!("3d3b54ffd1c5e6f579648398b4a9bb67166d83d24c76e6adf74fa0feaf4e16d9")
        );
    }
}
