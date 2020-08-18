#[cfg(feature = "prover")]
use crate::constraint_check::{check_constraints, check_specific_constraint};
#[cfg(feature = "prover")]
use crate::{prove, ProverError, TraceTable};
use crate::{verify, Constraints, Proof, VerifierError};

pub trait Verifiable {
    fn constraints(&self) -> Constraints;

    fn verify(&self, proof: &Proof) -> Result<(), VerifierError> {
        let constraints = self.constraints();
        verify(&constraints, proof)
    }
}

#[cfg(feature = "prover")]
pub trait Provable<T>: Verifiable {
    fn trace(&self, witness: T) -> TraceTable;

    fn prove(&self, witness: T) -> Result<Proof, ProverError> {
        let constraints = self.constraints();
        let trace = self.trace(witness);
        prove(&constraints, &trace)
    }

    fn check(&self, witness: T) -> Result<(), (usize, usize)> {
        let constraints = self.constraints();
        let trace = self.trace(witness);
        check_constraints(&constraints, &trace)
    }

    fn check_specified(&self, witness: T, row: usize, which_constraint: usize) -> Result<(), ()> {
        let constraints = self.constraints();
        let trace = self.trace(witness);
        if check_specific_constraint(&constraints, &trace, row, which_constraint) {
            Ok(())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::{polynomial::DensePolynomial, RationalExpression};
    use proptest::{collection::vec as prop_vec, prelude::*};
    use std::convert::TryInto;
    use zkp_primefield::{FieldElement, One, Pow, Root, Zero};

    // False positives on the Latex math.
    #[allow(clippy::doc_markdown)]
    /// Defines a constraint system for the recurrance relation $a_{n+2} =
    /// a_{n+1}.pow(exponent) + a_n$, where the claim is that I know a value for
    /// $a_1$ such that $a_{index} = value$.
    #[derive(Clone, PartialEq, Debug)]
    pub(crate) struct Recurrance {
        pub(crate) index:         usize,
        pub(crate) initial_value: FieldElement,
        pub(crate) exponent:      usize,
    }

    #[derive(Clone, PartialEq, Debug)]
    pub(crate) struct Claim {
        index:            usize,
        pub(crate) value: FieldElement,
        exponent:         usize,
    }

    #[derive(Clone, PartialEq, Debug)]
    pub(crate) struct Witness {
        secret: FieldElement,
    }

    impl Recurrance {
        pub(crate) fn claim(&self) -> Claim {
            Claim {
                index:    self.index,
                exponent: self.exponent,
                value:    self.index_value(),
            }
        }

        pub(crate) fn witness(&self) -> Witness {
            Witness {
                secret: self.initial_value.clone(),
            }
        }

        fn index_value(&self) -> FieldElement {
            let mut state = (FieldElement::one(), self.initial_value.clone());
            for _ in 0..self.index {
                state = (state.1.pow(self.exponent), state.0 + state.1);
            }
            state.0
        }
    }

    impl Claim {
        pub(crate) fn seed(&self) -> Vec<u8> {
            let mut seed = self.index.to_be_bytes().to_vec();
            seed.extend_from_slice(&self.value.as_montgomery().to_bytes_be());
            // For backwards compatibility, don't include exponent in seed when it's 1.
            if self.exponent != 1 {
                seed.extend_from_slice(&self.exponent.to_be_bytes());
            }
            seed
        }
    }

    impl Verifiable for Claim {
        fn constraints(&self) -> Constraints {
            use RationalExpression::*;

            // Constraint repetitions
            let trace_length = (self.index + 1).next_power_of_two();
            let trace_generator = FieldElement::root(trace_length).unwrap();
            let g = Constant(trace_generator);
            let on_row = |index| (X - g.pow(index)).inv();
            let every_row = || (X - g.pow(trace_length - 1)) / (X.pow(trace_length) - 1);

            // Constraints
            Constraints::from_expressions((trace_length, 2), self.seed(), vec![
                (Trace(0, 1) - Trace(1, 0).pow(self.exponent)) * every_row(),
                (Trace(1, 1) - Trace(0, 0) - Trace(1, 0)) * every_row(),
                (Trace(0, 0) - 1) * on_row(trace_length),
                (Trace(0, 0) - &self.value) * on_row(self.index),
            ])
            .unwrap()
        }
    }

    impl Provable<&Witness> for Claim {
        fn trace(&self, witness: &Witness) -> TraceTable {
            let trace_length = (self.index + 1).next_power_of_two();
            let mut trace = TraceTable::new(trace_length, 2);
            trace[(0, 0)] = 1.into();
            trace[(0, 1)] = witness.secret.clone();
            for i in 1..trace_length {
                trace[(i, 0)] = trace[(i - 1, 1)].pow(self.exponent);
                trace[(i, 1)] = &trace[(i - 1, 0)] + &trace[(i - 1, 1)];
            }
            trace
        }
    }

    #[derive(Clone, PartialEq, Debug)]
    pub(crate) struct Recurrance2 {
        pub(crate) index:          usize,
        pub(crate) initial_values: Vec<FieldElement>,
        pub(crate) coefficients:   Vec<FieldElement>,
        pub(crate) exponents:      Vec<usize>,
    }

    #[derive(Clone, PartialEq, Debug)]
    pub(crate) struct Claim2 {
        pub(crate) index:        usize,
        pub(crate) coefficients: Vec<FieldElement>,
        pub(crate) exponents:    Vec<usize>,
        pub(crate) value:        FieldElement,
    }

    #[derive(Clone, PartialEq, Debug)]
    pub(crate) struct Witness2 {
        initial_values: Vec<FieldElement>,
    }

    impl Recurrance2 {
        pub(crate) fn claim(&self) -> Claim2 {
            Claim2 {
                coefficients: self.coefficients.clone(),
                exponents:    self.exponents.clone(),
                index:        self.index,
                value:        self.index_value(),
            }
        }

        pub(crate) fn witness(&self) -> Witness2 {
            Witness2 {
                initial_values: self.initial_values.clone(),
            }
        }

        fn index_value(&self) -> FieldElement {
            let mut values = vec![FieldElement::zero(); self.index];
            for (i, initial_value) in self.initial_values.iter().enumerate() {
                values[i] = initial_value.clone();
            }
            let order = self.initial_values.len();
            for i in order..self.index {
                let mut next_value = FieldElement::zero();
                for ((value, coefficient), &exponent) in values[i - order..]
                    .iter()
                    .zip(&self.coefficients)
                    .zip(&self.exponents)
                {
                    next_value += coefficient * value.pow(exponent);
                }
                values[i] = next_value;
            }
            values[self.index - 1].clone()
        }
    }

    impl Claim2 {
        fn seed(&self) -> Vec<u8> {
            let mut seed = self.index.to_be_bytes().to_vec();
            for coefficient in &self.coefficients {
                seed.extend_from_slice(&coefficient.as_montgomery().to_bytes_be());
            }
            for exponent in &self.exponents {
                seed.extend_from_slice(&exponent.to_be_bytes());
            }
            seed.extend_from_slice(&self.index.to_be_bytes());
            seed.extend_from_slice(&self.value.as_montgomery().to_bytes_be());
            seed
        }

        fn trace_length(&self) -> usize {
            (self.index + 1).next_power_of_two()
        }

        fn claim_polynomials(&self) -> Vec<DensePolynomial> {
            vec![DensePolynomial::new(&[self.value.clone()])]
        }
    }

    impl Verifiable for Claim2 {
        fn constraints(&self) -> Constraints {
            use RationalExpression::*;

            let trace_length = self.trace_length();
            let trace_generator = Constant(
                FieldElement::root(trace_length).expect("trace length is not power of two"),
            );

            let on_row = |index| (X - trace_generator.pow(index)).inv();

            let mut constraints: Vec<RationalExpression> = vec![
                (Trace(0, 0) - ClaimPolynomial(0, 0, Box::new(X), None)) * on_row(self.index - 1),
            ];

            let mut recurrance_constraint = Constant(FieldElement::zero());
            for (i, (coefficient, exponent)) in
                self.coefficients.iter().zip(&self.exponents).enumerate()
            {
                recurrance_constraint = recurrance_constraint
                    + Trace(0, i.try_into().unwrap()).pow(*exponent) * coefficient;
            }
            recurrance_constraint =
                recurrance_constraint - Trace(0, self.coefficients.len().try_into().unwrap());
            recurrance_constraint = recurrance_constraint / (X.pow(trace_length) - 1);
            for i in 0..self.coefficients.len() {
                recurrance_constraint =
                    recurrance_constraint * (X - trace_generator.pow(i + 1).inv());
            }
            constraints.push(recurrance_constraint);
            let claim_polynomials = self.claim_polynomials();
            constraints = constraints
                .iter()
                .map(|c| c.substitute_claim(&claim_polynomials))
                .collect();

            Constraints::from_expressions((trace_length, 1), self.seed(), constraints).unwrap()
        }
    }

    impl Provable<&Witness2> for Claim2 {
        fn trace(&self, witness: &Witness2) -> TraceTable {
            let mut trace_table = TraceTable::new(self.trace_length(), 1);

            for (i, initial_value) in witness.initial_values.iter().enumerate() {
                trace_table[(i, 0)] = initial_value.clone();
            }
            let order = witness.initial_values.len();
            for i in order..self.trace_length() {
                let mut next_value = FieldElement::zero();
                for (j, (coefficient, &exponent)) in
                    self.coefficients.iter().zip(&self.exponents).enumerate()
                {
                    next_value += coefficient * trace_table[(i - order + j, 0)].pow(exponent);
                }
                trace_table[(i, 0)] = next_value;
            }
            trace_table
        }
    }

    impl Arbitrary for Recurrance {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        // TODO: handle 1 row trace tables.
        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            <(usize, FieldElement, usize)>::arbitrary()
                .prop_map(|(a, initial_value, b)| {
                    Self {
                        index: 1 + a % 10,
                        initial_value,
                        exponent: b % 6,
                    }
                })
                .boxed()
        }
    }

    impl Arbitrary for Recurrance2 {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            (1_usize..=12)
                .prop_flat_map(|order| {
                    (
                        (order..order + 10),
                        prop_vec(FieldElement::arbitrary(), order),
                        prop_vec(0_usize..6, order),
                        prop_vec(FieldElement::arbitrary(), order),
                    )
                })
                .prop_map(|(index, initial_values, exponents, coefficients)| {
                    Self {
                        index,
                        initial_values,
                        exponents,
                        coefficients,
                    }
                })
                .boxed()
        }
    }
}
