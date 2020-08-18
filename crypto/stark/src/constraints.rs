use crate::{polynomial::DensePolynomial, rational_expression::RationalExpression};
use itertools::Itertools;
use std::{collections::BTreeSet, fmt, prelude::v1::*};
use zkp_primefield::{FieldElement, Root};

#[derive(Clone, Debug)]
pub enum Error {
    InvalidTraceLength,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match *self {
            InvalidTraceLength => write!(f, "Invalid trace length (must be power of two)"),
        }
    }
}

/// Constraints for Stark proofs
///
/// Contains the constraint expressions that apply to the trace table in
/// addition to various tuning parameters that determine how proofs are
/// computed. These can trade off between security, prover time, verifier time
/// and proof size.
///
/// **Note**: This does not including the constraint system or anything
/// about the claim to be proven.
// TODO Implement PartialEq
#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Constraints {
    channel_seed:   Vec<u8>,
    trace_nrows:    usize,
    trace_ncolumns: usize,

    expressions: Vec<RationalExpression>,

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
    pub num_queries: usize,

    /// Number of FRI reductions between steps
    ///
    /// After the initial LDE polynomial is committed, several rounds of FRI
    /// degree reduction are done. Entries in the vector specify how many
    /// reductions are done between commitments.
    ///
    /// After `fri_layout.sum()` reductions are done, the remaining polynomial
    /// is written explicitly in coefficient form.
    pub fri_layout: Vec<usize>,

    /// To make autogeneration easier we have included a 'ClaimPolynomial'
    /// these claim polynomials need to be taken out of the expressions before
    /// they can be evaluated
    /// The following Vec of dense polys can be used to substitute claim
    /// polynomials inside of the prover.
    pub claim_polynomials: Vec<DensePolynomial>,
}

impl Constraints {
    fn default_fri_layout(trace_nrows: usize) -> Vec<usize> {
        // The binary logarithm of the final layer polynomial degree.
        const LOG2_TARGET: usize = 8;

        // Number of reductions to reach target degree
        // TODO: For very small traces we fold to a constant, but this is not
        // necessarily optimal.
        let log2_trace = trace_nrows.trailing_zeros() as usize;
        let num_reductions = if log2_trace > LOG2_TARGET {
            log2_trace - LOG2_TARGET
        } else {
            log2_trace
        };

        // Do as many three reductions as possible
        let mut fri_layout = vec![3; num_reductions / 3];
        if num_reductions % 3 != 0 {
            fri_layout.push(num_reductions % 3);
        }
        fri_layout
    }

    /// Requires all instances of `RationalExpression::ClaimPolynomial` in the
    /// expressions to have been replaced by
    /// `RationalExpression::DensePolynomial`.
    // False positive
    // TODO: Remove once [1] clears
    // [1]: <https://github.com/rust-lang/rust-clippy/issues/5351>
    #[allow(clippy::unused_self)]
    pub fn from_expressions(
        (trace_nrows, trace_ncolumns): (usize, usize),
        channel_seed: Vec<u8>,
        expressions: Vec<RationalExpression>,
    ) -> Result<Self, Error> {
        let _ = FieldElement::root(trace_nrows).ok_or(Error::InvalidTraceLength)?;
        // TODO: Hash expressions into channel seed
        // TODO - Examine if we want to up these security params further.
        // 22.5*4  + 0 queries = 90
        // TODO: Sensible default for pow_bits. For small proofs it should be small.
        Ok(Self {
            channel_seed,
            trace_nrows,
            trace_ncolumns,
            expressions,
            blowup: 16,
            pow_bits: 0,
            num_queries: 45,
            fri_layout: Self::default_fri_layout(trace_nrows),
            claim_polynomials: vec![],
        })
    }

    /// Requires all instances of `RationalExpression::ClaimPolynomial` in the
    /// expressions to have been replaced by
    /// `RationalExpression::DensePolynomial`.
    // False positive
    // TODO: Remove once [1] clears
    // [1]: <https://github.com/rust-lang/rust-clippy/issues/5351>
    #[allow(clippy::unused_self)]
    pub fn from_expressions_detailed(
        (trace_nrows, trace_ncolumns): (usize, usize),
        channel_seed: Vec<u8>,
        expressions: Vec<RationalExpression>,
        op_blowup: Option<usize>,
        op_pow_bits: Option<usize>,
        op_num_queries: Option<usize>,
        op_fri_layout: Option<Vec<usize>>,
    ) -> Result<Self, Error> {
        let _ = FieldElement::root(trace_nrows).ok_or(Error::InvalidTraceLength)?;
        // TODO: Hash expressions into channel seed
        // 15*4 + 30 queries = 90
        Ok(Self {
            channel_seed,
            trace_nrows,
            trace_ncolumns,
            expressions,
            blowup: match op_blowup {
                Some(x) => x,
                None => 16,
            },
            pow_bits: match op_pow_bits {
                Some(x) => x,
                None => {
                    if cfg!(test) {
                        0
                    } else {
                        20
                    }
                }
            },
            num_queries: match op_num_queries {
                Some(x) => x,
                None => 13,
            },
            fri_layout: match op_fri_layout {
                Some(x) => x,
                None => Self::default_fri_layout(trace_nrows),
            },
            claim_polynomials: vec![],
        })
    }

    pub fn channel_seed(&self) -> &[u8] {
        &self.channel_seed
    }

    pub fn trace_nrows(&self) -> usize {
        self.trace_nrows
    }

    pub fn trace_ncolumns(&self) -> usize {
        self.trace_ncolumns
    }

    pub fn len(&self) -> usize {
        self.expressions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.expressions().is_empty()
    }

    pub fn expressions(&self) -> &[RationalExpression] {
        &self.expressions
    }

    pub fn degree(&self) -> usize {
        self.expressions
            .iter()
            .map(|c| {
                let (numerator_degree, denominator_degree) = c.trace_degree();
                numerator_degree - denominator_degree
            })
            .max()
            .expect("no constraints")
    }

    // TODO: Better explanation with literature references.
    pub fn security_bits(&self) -> usize {
        // Our conservative formula is (1/2^blowup_log)^(queries/2)*(1/2^pow_bits)
        // So the bit security should be blowup_log*(queries/2) + pow_bits
        let blowup_log = (64 - (self.blowup as u64).leading_zeros()) as usize;
        blowup_log * (self.num_queries / 2) + self.pow_bits
    }

    // Returns an upper bound on proof size in terms of bytes in the proof.
    // Note we expect that actual sizes are compressed by the removal of overlaps in
    // decommitments
    // TODO - Improve bound by removing the elements of overlap in
    // worst cases.
    pub fn max_proof_size(&self) -> usize {
        let trace_len_log = self.trace_nrows().trailing_zeros() as usize;
        // First we decommit two proofs for each query [one which is the evaluation
        // domain decommitment and one is the constraints]
        let mut total_decommitment =
            self.num_queries * (trace_len_log * self.trace_ncolumns() + trace_len_log);
        // Now we account for the first layer which is 8 elements [assuming the worst
        // case we need to decommit 7 other elements].
        let mut current_size = trace_len_log - 3;
        total_decommitment += self.num_queries * (current_size + 7);

        for &i in &self.fri_layout {
            // This worst case assumes that only one in each group is from the previous
            // layer.
            current_size -= i;
            total_decommitment += self.num_queries * (current_size + (1 << i) - 1);
        }
        // Decommits all of the remaining elements
        let final_list = 1 << current_size;
        if final_list > self.num_queries {
            total_decommitment += final_list - self.num_queries;
        }
        32 * total_decommitment
    }

    pub fn combine(&self, constraint_coefficients: &[FieldElement]) -> RationalExpression {
        use RationalExpression::*;
        assert_eq!(2 * self.len(), constraint_coefficients.len());
        let target_degree = self.degree() * self.trace_nrows() - 1;

        self.expressions
            .iter()
            .zip(constraint_coefficients.iter().tuples())
            .map(
                |(constraint, (coefficient_low, coefficient_high))| -> RationalExpression {
                    let (num, den) = constraint.degree(self.trace_nrows() - 1);
                    let adjustment_degree = target_degree + den - num;
                    let adjustment = Constant(coefficient_low.clone())
                        + Constant(coefficient_high.clone()) * X.pow(adjustment_degree);
                    adjustment * constraint.clone()
                },
            )
            .sum()
    }

    pub fn trace_arguments(&self) -> Vec<(usize, isize)> {
        self.expressions
            .iter()
            .map(RationalExpression::trace_arguments)
            .fold(BTreeSet::new(), |x, y| &x | &y)
            .into_iter()
            .collect()
    }

    // This sets a the claim polynomials field
    // Note that since we didn't want to change the interface this is the
    // only way to set or change the field
    pub fn add_claim_polynomials(&mut self, polys: Vec<DensePolynomial>) {
        self.claim_polynomials = polys;
    }

    // This function if called on a set of constraints which has both
    // Rational Expression claim polynomials in the constraints
    // and has set a claim_polynomials constraint field, will use the
    // claim_polynomials constraint field to substitute out the
    // Rational Expression claim polynomials
    pub fn substitute(&mut self) {
        if !self.claim_polynomials.is_empty() {
            self.expressions = self
                .expressions
                .iter()
                .map(|x| x.substitute_claim(&self.claim_polynomials))
                .collect();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{prove, traits::tests::Recurrance, Provable, Verifiable};
    use zkp_macros_decl::field_element;
    use zkp_primefield::FieldElement;
    use zkp_u256::U256;

    #[test]
    fn size_estimate_test() {
        let recurrance = Recurrance {
            index:         4000,
            initial_value: field_element!("0f00dbabe0cafebabe"),
            exponent:      1,
        };
        let private = recurrance.witness();
        let public = recurrance.claim();

        let mut constraints = public.constraints();
        constraints.blowup = 16;
        constraints.pow_bits = 12;
        constraints.num_queries = 20;
        constraints.fri_layout = vec![2, 1, 4, 2];

        let actual = prove(&constraints, &public.trace(&private)).unwrap();
        assert!(actual.as_bytes().len() <= constraints.max_proof_size());
    }
}
