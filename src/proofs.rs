#![allow(non_snake_case)] //TODO - Migrate to naming system which the rust complier doesn't complain about
#![allow(clippy::zero_prefixed_literal)]

use crate::channel::*;
use crate::fft::*;
use crate::field::*;
use crate::merkle::*;
use crate::polynomial::*;
use crate::u256::U256;
use rayon::prelude::*;
use tiny_keccak::Keccak;

pub struct TraceTable {
    pub ROWS: usize,
    pub COLS: usize,
    pub elements: Vec<FieldElement>,
}

impl TraceTable {
    pub fn new(ROWS: usize, COLS: usize, elements: Vec<FieldElement>) -> Self {
        Self {
            ROWS,
            COLS,
            elements,
        }
    }
}

trait Reversable {
    fn bit_reverse(self) -> Self;
}
impl Reversable for u64 {
    fn bit_reverse(self) -> Self {
        let bits = 64;
        let mut x_hold = self;
        let mut y = 0;
        for _i in 0..bits {
            y = (y << 1) | (x_hold & 1);
            x_hold >>= 1;
        }
        y
    }
}
impl Reversable for usize {
    fn bit_reverse(self) -> Self {
        let bits = 64;
        let mut x_hold = self;
        let mut y = 0;
        for _i in 0..bits {
            y = (y << 1) | (x_hold & 1);
            x_hold >>= 1;
        }
        y
    }
}

//This struct contains two evaluation systems which allow diffrent functionality, first it contains a default function which directly evaluates the constraint funciton
//Second it constains a function desgined to be used as the core of a loop on precomputed values to get the C function.
//If the proof system wants to used a looped eval for speedup it can set the loop bool to true, otherwise the system will preform all computation directly
#[allow(clippy::type_complexity)]
pub struct Constraint<'a> {
    pub NCONSTRAINTS: usize,
    pub eval: &'a Fn(
        &FieldElement,
        &[&[FieldElement]],
        u64,
        FieldElement,
        &[FieldElement],
    ) -> FieldElement, //x point, polynomials, claim index, claim, constraint_coefficents
    pub eval_loop: Option<
        &'a Fn(
            &[&[FieldElement]], //Evaluated polynomials (LDEn)
            &[FieldElement],    //Constraint Coefficents
            u64,                //Claim index
            &FieldElement,      //Claim
        ) -> Vec<FieldElement>,
    >,
}

impl<'a> Constraint<'a> {
    #[allow(clippy::type_complexity)]
    pub fn new(
        NCONSTRAINTS: usize,
        eval: &'a Fn(
            &FieldElement,
            &[&[FieldElement]],
            u64,
            FieldElement,
            &[FieldElement],
        ) -> FieldElement,
        eval_loop: Option<
            &'a Fn(&[&[FieldElement]], &[FieldElement], u64, &FieldElement) -> Vec<FieldElement>,
        >,
    ) -> Self {
        Self {
            NCONSTRAINTS,
            eval,
            eval_loop,
        }
    }
}

#[allow(clippy::cognitive_complexity)]
pub fn stark_proof(
    trace: &TraceTable,
    constraints: &Constraint,
    claim_index: u64,
    claim_value: FieldElement,
    beta: u64,
) -> Channel {
    let trace_len = (trace.elements.len() / trace.COLS) as u64;
    let omega = FieldElement::root(U256::from(trace_len * beta)).unwrap();
    let g = omega.pow(U256::from(beta)).unwrap();
    let eval_domain_size = trace_len * beta;
    let gen = FieldElement::GENERATOR;

    let mut eval_x = Vec::with_capacity((eval_domain_size) as usize);

    let mut hold = FieldElement::ONE;
    for _i in 0..(eval_domain_size) {
        eval_x.push(hold.clone());
        hold *= &omega
    }

    let mut TPn = vec![Vec::new(); trace.COLS];
    (0..trace.COLS)
        .into_par_iter()
        .map(|x| {
            let mut hold_col = Vec::with_capacity(trace_len as usize);
            for i in (0..trace.elements.len()).step_by(trace.COLS) {
                hold_col.push(trace.elements[x + i].clone());
            }
            ifft(g.clone(), hold_col.as_slice())
        })
        .collect_into_vec(&mut TPn);

    // debug_assert_eq!(
    //     eval_poly(trace_x[1000].clone(), TPn[0].as_slice()),
    //     trace.elements[1000_usize * trace.COLS]
    // );

    let mut LDEn = vec![vec![FieldElement::ZERO; eval_x.len()]; trace.COLS];
    //OPT - Use some system to make this occur inline instead of storing then processing
    #[allow(clippy::type_complexity)]
    let ret: Vec<(usize, Vec<(usize, Vec<FieldElement>)>)> = (0..(beta as usize))
        .into_par_iter()
        .map(|j| {
            (
                j,
                (0..(trace.COLS as usize))
                    .into_par_iter()
                    .map(|x| {
                        (
                            x,
                            fft_cofactor(
                                g.clone(),
                                TPn[x].as_slice(),
                                &gen * (&omega.pow(U256::from(j as u64)).unwrap()),
                            ),
                        )
                    })
                    .collect(),
            )
        })
        .collect();

    for j_element in ret {
        for x_element in j_element.1 {
            for i in 0..trace_len {
                LDEn[x_element.0]
                    [((i * beta + (j_element.0 as u64)) % (eval_domain_size)) as usize] =
                    x_element.1[i as usize].clone();
            }
        }
    }

    let mut leaves = Vec::with_capacity(eval_domain_size as usize);
    (0..(eval_domain_size as usize))
        .into_par_iter()
        .map(|i| leaf_list(i as u64, &LDEn))
        .collect_into_vec(&mut leaves);

    let leaf_pointer: Vec<&[U256]> = leaves.iter().map(|x| x.as_slice()).collect();
    let tree = make_tree(leaf_pointer.as_slice());
    let mut public_input = [claim_index.to_be_bytes()].concat();
    public_input.extend_from_slice(&claim_value.0.to_bytes_be());
    let mut proof = Channel::new(public_input.as_slice());
    proof.write(&tree[1]);
    let mut constraint_coefficients = Vec::with_capacity(constraints.NCONSTRAINTS);
    for _i in 0..constraints.NCONSTRAINTS {
        constraint_coefficients.push(proof.element());
    }

    let mut CC;
    let mut x = gen.clone();
    let sliced_poly: Vec<&[FieldElement]> = TPn.iter().map(|x| x.as_slice()).collect();
    let sliced_eval: Vec<&[FieldElement]> = LDEn.iter().map(|x| x.as_slice()).collect();

    match constraints.eval_loop {
        Some(x) => {
            CC = (x)(
                sliced_eval.as_slice(),
                constraint_coefficients.as_slice(),
                claim_index,
                &claim_value,
            )
        }
        None => {
            CC = vec![FieldElement::ZERO; eval_domain_size as usize];
            for i in 0..eval_domain_size {
                CC[i as usize] = (constraints.eval)(
                    //This will preform the polynomial evaluation on each step
                    &x,
                    sliced_poly.as_slice(),
                    claim_index,
                    claim_value.clone(),
                    constraint_coefficients.as_slice(),
                );

                x *= &omega;
            }
        }
    }
    let mut c_leaves = Vec::with_capacity(eval_domain_size as usize);
    (0..(eval_domain_size as usize))
        .into_par_iter()
        .map(|i| leaf_single(i as u64, &CC))
        .collect_into_vec(&mut c_leaves);

    let c_tree = make_tree(c_leaves.as_slice());
    proof.write(&c_tree[1]);
    let oods_point = proof.element();
    let oods_point_g = &oods_point * &g;
    let mut oods_values = Vec::with_capacity(2 * trace.COLS + 1);
    for item in TPn.iter() {
        let mut evaled = eval_poly(oods_point.clone(), item.as_slice());
        oods_values.push(evaled.clone());
        evaled = eval_poly(oods_point_g.clone(), item.as_slice());
        oods_values.push(evaled.clone());
    }

    oods_values.push((constraints.eval)(
        &oods_point,
        sliced_poly.as_slice(),
        claim_index,
        claim_value.clone(),
        constraint_coefficients.as_slice(),
    )); //Gets eval_C of the oods point via direct computation

    for v in oods_values.iter() {
        proof.write_element(v);
    }

    let mut oods_coeffiecnts = Vec::with_capacity(2 * trace.COLS + 1);
    for _i in 0..=2 * trace.COLS {
        oods_coeffiecnts.push(proof.element());
    }

    let mut CO = Vec::with_capacity(eval_domain_size as usize);
    let x = FieldElement::GENERATOR;
    let mut x_omega_cycle = Vec::with_capacity(eval_domain_size as usize);
    let mut x_oods_cycle: Vec<FieldElement> = Vec::with_capacity(eval_domain_size as usize);
    let mut x_oods_cycle_g: Vec<FieldElement> = Vec::with_capacity(eval_domain_size as usize);

    eval_x
        .par_iter()
        .map(|i| i * &x)
        .collect_into_vec(&mut x_omega_cycle);
    x_omega_cycle
        .par_iter()
        .map(|i| (i - &oods_point, i - &oods_point * &g))
        .unzip_into_vecs(&mut x_oods_cycle, &mut x_oods_cycle_g);

    let pool = vec![&x_oods_cycle, &x_oods_cycle_g];

    let mut held = Vec::with_capacity(3);
    pool.par_iter()
        .map(|i| invert_batch(i))
        .collect_into_vec(&mut held);

    x_oods_cycle_g = held.pop().unwrap();
    x_oods_cycle = held.pop().unwrap();

    (0..(eval_domain_size as usize))
        .into_par_iter()
        .map(|i| {
            let A = &x_oods_cycle[i as usize];
            let B = &x_oods_cycle_g[i as usize];
            let mut r = FieldElement::ZERO;

            for x in 0..trace.COLS {
                r += &oods_coeffiecnts[2 * x] * (&LDEn[x][i as usize] - &oods_values[2 * x]) * A;
                r += &oods_coeffiecnts[2 * x + 1]
                    * (&LDEn[x][i as usize] - &oods_values[2 * x + 1])
                    * B;
            }
            r += &oods_coeffiecnts[oods_coeffiecnts.len() - 1]
                * (&CC[i as usize] - &oods_values[oods_values.len() - 1])
                * A;

            r
        })
        .collect_into_vec(&mut CO);
    //Fri Layers
    let mut fri = Vec::with_capacity(64 - (eval_domain_size.leading_zeros() as usize)); //Since Eval domain size is power of two this is a log_2
    fri.push(CO);
    let fri_tree_1 = fri_tree(&(fri[0].as_slice()), 8);
    proof.write(&fri_tree_1[1]);

    let mut eval_point = proof.element();
    fri.push(fri_layer(
        &fri[0].as_slice(),
        &eval_point,
        eval_domain_size,
        eval_x.as_slice(),
    ));
    fri.push(fri_layer(
        &fri[1].as_slice(),
        &(eval_point.square()),
        eval_domain_size,
        eval_x.as_slice(),
    ));
    fri.push(fri_layer(
        fri[2].as_slice(),
        &(eval_point.square().square()),
        eval_domain_size,
        eval_x.as_slice(),
    ));
    let fri_tree_2 = fri_tree(&(fri[3].as_slice()), 4);

    proof.write(&fri_tree_2[1]);

    eval_point = proof.element();
    fri.push(fri_layer(
        &(fri[3].as_slice()),
        &eval_point,
        eval_domain_size,
        eval_x.as_slice(),
    ));
    fri.push(fri_layer(
        &(fri[4].as_slice()),
        &(eval_point.square()),
        eval_domain_size,
        eval_x.as_slice(),
    ));
    // Five fri layers have reduced the size of the evaluation domain and polynomail by 32x
    let last_layer_degree_bound = trace_len / 32;
    let mut last_layer_coefficents = ifft(
        FieldElement::root(U256::from(fri[5].len() as u64)).unwrap(),
        &(fri[5].as_slice()),
    );
    last_layer_coefficents.truncate(last_layer_degree_bound as usize);
    proof.write_element_list(last_layer_coefficents.as_slice());
    debug_assert_eq!(last_layer_coefficents.len() as u64, last_layer_degree_bound);
    //Security paramter proof of work is at 12 bits
    let proof_of_work = pow_find_nonce(12, &proof);
    debug_assert!(pow_verfiy(proof_of_work, 12, &proof));
    proof.write(&proof_of_work.to_be_bytes());

    //Security paramter number of queries is at 20
    let num_queries = 20;
    let mut query_indices = Vec::with_capacity(num_queries + 3);
    while query_indices.len() < num_queries {
        let val = U256::from_bytes_be(&proof.bytes());
        query_indices.push(((val.clone() >> (0x100 - 0x040)).c0 & (2_u64.pow(14) - 1)) as usize);
        query_indices.push(((val.clone() >> (0x100 - 0x080)).c0 & (2_u64.pow(14) - 1)) as usize);
        query_indices.push(((val.clone() >> (0x100 - 0x0C0)).c0 & (2_u64.pow(14) - 1)) as usize);
        query_indices.push((val.c0 & (2_u64.pow(14) - 1)) as usize);
    }
    query_indices.truncate(num_queries);
    (&mut query_indices).sort_unstable(); //Fast inplace sort that doesn't preserve the order of equal elements.

    for index in query_indices.iter() {
        for x in 0..trace.COLS {
            proof.write_element(
                &LDEn[x as usize][(index.clone()).bit_reverse()
                    >> ((LDEn[x].len().leading_zeros()) + 1) as usize],
            );
        }
    }

    let mut decommitment = crate::merkle::proof(&tree, &(query_indices.as_slice()));
    for x in decommitment.iter() {
        proof.write(x);
    }

    for index in query_indices.iter() {
        proof.write_element(
            &CC[index.clone().bit_reverse() >> ((CC.len().leading_zeros()) + 1) as usize],
        );
    }
    decommitment = crate::merkle::proof(&c_tree, &(query_indices.as_slice()));
    for x in decommitment.iter() {
        proof.write(x);
    }

    let fri_indices: Vec<usize> = query_indices
        .iter()
        .map(|x| x / ((beta / 2) as usize))
        .collect();
    for i in fri_indices.iter() {
        for j in 0..((beta / 2) as usize) {
            let n = i * ((beta / 2) as usize) + j;
            if query_indices.binary_search(&n).is_ok() {
                continue;
            } else {
                proof.write_element(&fri[0][((n as u64).bit_reverse() >> 50) as usize]);
            }
        }
    }
    decommitment = crate::merkle::proof(&fri_tree_1, &(fri_indices.as_slice()));
    for x in decommitment.iter() {
        proof.write(x);
    }

    let fri_low_indices: Vec<usize> = query_indices.iter().map(|x| x / 32).collect();
    for i in fri_low_indices.iter() {
        for j in 0..4 {
            let n = i * 4 + j;
            if fri_indices.binary_search(&n).is_ok() {
                continue;
            } else {
                proof.write_element(&fri[3][((n as u64).bit_reverse() >> 53) as usize]);
            }
        }
    }
    decommitment = crate::merkle::proof(&fri_tree_2, &(fri_low_indices.as_slice()));
    for x in decommitment.iter() {
        proof.write(x);
    }

    proof
}

fn leaf_list(i: u64, LDEn: &[Vec<FieldElement>]) -> Vec<U256> {
    let mut ret = Vec::with_capacity(LDEn.len());
    for item in LDEn.iter() {
        ret.push(
            item[(i.bit_reverse() >> (item.len().leading_zeros() + 1)) as usize]
                .0
                .clone(),
        )
    }
    ret
}

fn leaf_single(i: u64, CC: &[FieldElement]) -> U256 {
    CC[(i.bit_reverse() >> (CC.len().leading_zeros() + 1)) as usize]
        .0
        .clone()
}

fn fri_layer(
    previous: &[FieldElement],
    evaluation_point: &FieldElement,
    eval_domain_size: u64,
    eval_x: &[FieldElement],
) -> Vec<FieldElement> {
    let n = previous.len() as u64;
    let s = eval_domain_size / n;
    let mut next = vec![FieldElement::ZERO; (n / 2) as usize];
    (0..(n as usize) / 2)
        .into_par_iter()
        .map(|i| {
            let j = (n / 2 + (i as u64)) % n;
            let m = eval_x.len() as u64;
            let ind = ((m - (i as u64)) * s) % m;
            let x_inv = &eval_x[ind as usize];
            let a = &previous[i as usize];
            let b = &previous[j as usize];
            (a + b) + evaluation_point * x_inv * (a - b)
        })
        .collect_into_vec(&mut next);
    next
}

fn fri_tree(layer: &[FieldElement], coset_size: u64) -> Vec<[u8; 32]> {
    let n = layer.len();
    let bits = 64 - (n as u64).leading_zeros(); //Floored base 2 log
    let mut internal_leaves = Vec::new();
    for i in (0..n).step_by(coset_size as usize) {
        let mut internal_leaf = Vec::with_capacity(coset_size as usize);
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
}

fn pow_find_nonce(pow_bits: u32, proof: &Channel) -> u64 {
    let mut seed = vec![01_u8, 35_u8, 69_u8, 103_u8, 137_u8, 171_u8, 205_u8, 237_u8];
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

    let test_value = U256::from(2_u64).pow(u64::from(256 - pow_bits)).unwrap();
    for n in 0..(u64::max_value() as usize) {
        let mut sha3 = Keccak::new_keccak256();
        let mut res = [0; 32];
        sha3.update(&seed_res);
        sha3.update(&(n.to_be_bytes()));
        sha3.finalize(&mut res);
        let final_int = U256::from_bytes_be(&res);
        if final_int.leading_zeros() == pow_bits as usize && final_int < test_value {
            //Only do the large int compare if the quick logs match
            return n as u64;
        }
    }
    0
}
//TODO - Make tests compatible with the proof of work values from this function
#[allow(dead_code)]
fn pow_find_nonce_threaded(pow_bits: u32, proof: &Channel) -> u64 {
    let mut seed = vec![01_u8, 35_u8, 69_u8, 103_u8, 137_u8, 171_u8, 205_u8, 237_u8];
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

    let test_value = U256::from(2_u64).pow(u64::from(256 - pow_bits)).unwrap();
    let ret = (0..(u64::max_value() as usize))
        .into_par_iter()
        .find_any(|n| -> bool {
            let mut sha3 = Keccak::new_keccak256();
            let mut res = [0; 32];
            sha3.update(&seed_res);
            sha3.update(&(n.to_be_bytes()));
            sha3.finalize(&mut res);
            let final_int = U256::from_bytes_be(&res);
            if final_int.leading_zeros() == pow_bits as usize {
                final_int < test_value
            } else {
                false
            }
        });
    ret.unwrap() as u64
}

fn pow_verfiy(n: u64, pow_bits: u32, proof: &Channel) -> bool {
    let mut seed = vec![01_u8, 35_u8, 69_u8, 103_u8, 137_u8, 171_u8, 205_u8, 237_u8];
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

    let test_value = U256::from(2_u64).pow(u64::from(256 - pow_bits)).unwrap();
    let mut sha3 = Keccak::new_keccak256();
    let mut res = [0; 32];
    sha3.update(&seed_res);
    sha3.update(&(n.to_be_bytes()));
    sha3.finalize(&mut res);
    let final_int = U256::from_bytes_be(&res);
    final_int < test_value
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::channel::*;
    use crate::fibonacci::*;
    use crate::field::*;
    use crate::u256::U256;
    use crate::u256h;
    use hex_literal::*;

    #[test]
    fn fib_abstraction_test() {
        let claim_index = 1000_u64;
        let claim_fib = FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        ));
        let witness = FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000000000000cafebabe"
        ));
        let correct_proof = fib_proof(witness.clone());
        let potential_proof = stark_proof(
            &get_trace_table(1024, witness),
            &get_constraint(),
            claim_index,
            claim_fib,
            2_u64.pow(4),
        );

        assert_eq!(correct_proof.digest, potential_proof.digest);
    }
}
