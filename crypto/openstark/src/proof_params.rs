use std::prelude::v1::*;

/// Parameters for Stark proof generation
///
/// Contains various tuning parameters that determine how proofs are computed.
/// These can trade off between security, prover time, verifier time and
/// proof size.
///
/// **Note**: This does not including the constraint system or anything
/// about the claim to be proven.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofParams {
    /// The blowup factor
    ///
    /// The size of the low-degree-extension domain compared to the trace
    /// domain. Should be a power of two. Recommended values are 16, 32 or 64.
    pub blowup: usize,

    /// Proof of work difficulty
    ///
    /// The difficulty of the proof of work step in number of leading zero bits
    /// required.
    pub pow_bits: usize,

    /// Number of queries made to the oracles
    pub queries: usize,

    /// Number of FRI reductions between steps
    ///
    /// After the initial LDE polynomial is committed, several rounds of FRI
    /// degree reduction are done. Entries in the vector specify how many
    /// reductions are done between commitments.
    ///
    /// After `fri_layout.sum()` reductions are done, the remaining polynomial
    /// is written explicitly in coefficient form.
    pub fri_layout: Vec<usize>,

    /// The highest degree of any constraint polynomial.
    ///
    /// The polynomial constraints are not directly commited too on the trace
    /// domain, instead they are calculated via "Deep fri" which samples and
    /// commits too information outside of the domain.
    ///
    /// This information on constraint degree allows the out of domain sampling
    /// to provide the right number points.
    pub constraints_degree_bound: usize,
}

impl ProofParams {
    #[allow(dead_code)]
    fn suggested(constraints_degree_bound: usize, domain_size_log: usize) -> Self {
        let num_threes = (domain_size_log - 8) / 3;
        let mut fri_layout = vec![3; num_threes];
        if num_threes * 3 == (domain_size_log - 8) {
            fri_layout.push(domain_size_log - (8 + num_threes * 3));
        }

        // TODO - Examine if we want to up these security params further.
        // 15*4 + 30 = 90
        Self {
            blowup: 16,
            pow_bits: 30,
            queries: 30,
            fri_layout,
            constraints_degree_bound,
        }
    }
}

#[allow(dead_code)]
fn conservative_bit_bound(blowup_log: usize, queries: usize, pow_bits: usize) -> usize {
    // Our conservative formula is (1/2^blowup_log)^(queries/2)*(1/2^pow_bits)
    // So the bit security should be blowup_log*(queries/2) + pow_bits
    blowup_log * (queries / 2) + pow_bits
}

// Returns an upper bound on proof size in terms of bytes in the proof.
// Note we expect that actual sizes are compressed by the removal of overlaps in
// decommitments TODO - Improve bound by removing the elements of overlap in
// worst cases.
#[allow(dead_code)]
#[allow(clippy::cast_possible_truncation)]
pub fn decommitment_size_upper_bound(
    trace_len_log: usize,
    num_columns: usize,
    fri_layout: Vec<usize>,
    queries: usize,
) -> usize {
    // First we decommit two proofs for each query [one which is the evaluation
    // domain decommitment and one is the constraints]
    let mut total_decommitment = queries * (trace_len_log * num_columns + trace_len_log);
    // Now we account for the first layer which is 8 elements [assuming the worst
    // case we need to decommit 7 other elements].
    let mut current_size = trace_len_log - 3;
    total_decommitment += queries * (current_size + 7);

    for i in fri_layout {
        // This worst case assumes that only one in each group is from the previous
        // layer.
        current_size -= i;
        total_decommitment += queries * (current_size + 2_usize.pow(i as u32) - 1);
    }
    // Decommits all of the remaining elements
    let final_list = 2_usize.pow(current_size as u32);
    if final_list > queries {
        total_decommitment += 2_usize.pow(current_size as u32) - queries;
    }
    32 * total_decommitment
}

#[allow(dead_code)]
fn total_search(domain_size: usize, num_cols: usize, queries: usize) -> (Vec<usize>, usize) {
    let mut current_min = usize::max_value();
    let mut current_partition = Vec::new();

    // Searches each possible total number of reductions
    for i in 1..(domain_size - 1) {
        let (min_vec, min_cost) = partion_search(i, domain_size, num_cols, queries);

        if min_cost < current_min {
            current_min = min_cost;
            current_partition = min_vec;
        }
    }
    (current_partition, current_min)
}

// Searches over all permutations of all partitions of the n provided to find
// min upper bound cost.
#[allow(dead_code)]
#[allow(clippy::cast_sign_loss)]
fn partion_search(
    n: usize,
    domain_size: usize,
    num_cols: usize,
    queries: usize,
) -> (Vec<usize>, usize) {
    let mut partion = vec![0; n]; // We know the max size is n ones
    partion[0] = n;
    let mut k: i32 = 0;

    let mut current_min = usize::max_value();
    let mut current_partition = Vec::new();

    loop {
        let trimmed: Vec<usize> = partion.clone().into_iter().filter(|x| *x != 0).collect();
        let upper_bound =
            decommitment_size_upper_bound(domain_size, num_cols, trimmed.clone(), queries);
        let (permuted, permuted_upper) = permutation_search(
            trimmed.as_slice(),
            upper_bound,
            domain_size,
            num_cols,
            queries,
        );

        if permuted_upper < current_min && !permuted.is_empty() {
            current_min = permuted_upper;
            current_partition = permuted;
        }

        let mut rem_value = 0;
        while k >= 0 && partion[k as usize] == 1 {
            rem_value += partion[k as usize];
            partion[k as usize] = 0;
            k -= 1;
        }

        if k < 0 {
            break;
        }

        partion[k as usize] -= 1;
        rem_value += 1;

        while rem_value > partion[k as usize] {
            partion[k as usize + 1] = partion[k as usize];
            rem_value -= partion[k as usize];
            k += 1;
        }

        partion[k as usize + 1] = rem_value;
        k += 1;
    }

    (current_partition, current_min)
}

#[allow(dead_code)]
fn permutation_search(
    partion: &[usize],
    cost: usize,
    domain_size: usize,
    num_cols: usize,
    queries: usize,
) -> (Vec<usize>, usize) {
    let mut mut_part = partion.to_vec();
    let n = mut_part.len();

    let mut current_min = cost;
    let mut current_partition = partion.to_vec();

    // This inefficient brute force search doubles up since swap(1, 2) = swap(2, 1)
    // Moreover since partitions have repeated elements often this transformation is
    // trivial We should monitor running time and if it becomes a problem fix
    // this.
    for i in 0..n {
        for j in 0..n {
            if i != j {
                mut_part.swap(i, j);
                let upper_bound =
                    decommitment_size_upper_bound(domain_size, num_cols, mut_part.clone(), queries);
                if upper_bound < current_min {
                    current_min = upper_bound;
                    current_partition = mut_part.clone();
                }
                mut_part.swap(i, j);
            }
        }
    }
    (current_partition, current_min)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        fibonacci::{get_fibonacci_constraints, get_trace_table, PrivateInput, PublicInput},
        proofs::*,
    };
    use macros_decl::u256h;
    use primefield::FieldElement;
    use u256::U256;

    #[test]
    fn size_estimate_test() {
        let private = PrivateInput {
            secret: FieldElement::from(u256h!(
                "00000000000000000000000000000000000000000000000f00dbabe0cafebabe"
            )),
        };
        let tt = get_trace_table(4096, &private);
        let public = PublicInput {
            index: 4000,
            value: tt[(4000, 0)].clone(),
        };
        let constraints = get_fibonacci_constraints(&public);
        let actual = stark_proof(&tt, &constraints, &public, &ProofParams {
            blowup:                   16,
            pow_bits:                 12,
            queries:                  20,
            fri_layout:               vec![2, 1, 4, 2],
            constraints_degree_bound: 1,
        });
        assert!(actual.proof.len() < decommitment_size_upper_bound(12, 2, vec![2, 1, 4, 2], 20));
    }
}
