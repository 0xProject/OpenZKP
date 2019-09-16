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

// A default un-optimized 50 bit secure system.
#[allow(dead_code)]
fn params_suggestion(constraints_degree_bound: usize) -> ProofParams{
    ProofParams {
        blowup:                   16,
        pow_bits:                 12,
        queries:                  20,
        fri_layout:               vec![3, 2],
        constraints_degree_bound,
    }
}

#[allow(dead_code)]
fn conservative_bit_bound(blowup_log: usize, queries: usize, pow_bits: usize) -> usize {
    // Our conservative formula is (1/2^blowup_log)^(queries/2)*(1/2^pow_bits)
    // So the bit security should be blowup_log*(queries/2) + pow_bits
    blowup_log*(queries/2) + pow_bits
}

// Returns an upper bound on proof size in terms of bytes in the proof.
// Note we expect that actual sizes are compressed by the removal of overlaps in decommitments
// TODO - Improve bound by removing the elements of overlap in worst cases.
#[allow(dead_code)]
#[allow(clippy::cast_possible_truncation)]
pub fn decommitment_size_upper_bound(trace_len_log: usize, num_columns: usize, fri_layout: Vec<usize>, queries: usize) -> usize {
    // First we decommit two proofs for each query [one which is the evaluation domain decommitment and one is the constraints]
    let mut total_decommitment = queries*(trace_len_log*num_columns + trace_len_log);
    // Now we account for the first layer which is 8 elements [assuming the worst case we need to decommit 7 other elements].
    let mut current_size = trace_len_log - 3;
    total_decommitment += queries*(current_size + 7);

    for i in fri_layout {
        // This worst case assumes that only one in each group is from the previous layer.
        total_decommitment += queries*(current_size + 2_usize.pow(i as u32) - 1);
        current_size -= i;
    }
    // Decommits all of the remaining elements
    let final_list = 2_usize.pow(current_size as u32);
    if final_list > queries {
        total_decommitment += 2_usize.pow(current_size as u32) - queries;
    }
    32*total_decommitment
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::proofs::*;
    use crate::{
        fibonacci::{get_fibonacci_constraints, get_trace_table, PrivateInput, PublicInput},
        verifier::check_proof,
    };
    use u256::U256;
    use primefield::FieldElement;
    use macros_decl::{field_element, u256h, hex};

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
        assert!(actual.proof.len() < decommitment_size_upper_bound(12, 2, vec![2, 1, 4, 2], 20));p
    }
}