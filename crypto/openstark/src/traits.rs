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
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::RationalExpression;
    use primefield::FieldElement;
    use quickcheck::{Arbitrary, Gen};

    /// Helper struct for
    #[derive(Clone, PartialEq, Debug)]
    pub(crate) struct Recurrance {
        pub(crate) index:         usize,
        pub(crate) initial_value: FieldElement,
    }

    #[derive(Clone, PartialEq, Debug)]
    pub(crate) struct Claim {
        index: usize,
        value: FieldElement,
    }

    #[derive(Clone, PartialEq, Debug)]
    pub(crate) struct Witness {
        secret: FieldElement,
    }

    impl Recurrance {
        pub(crate) fn claim(&self) -> Claim {
            Claim {
                index: self.index,
                value: self.index_value(),
            }
        }

        pub(crate) fn witness(&self) -> Witness {
            Witness {
                secret: self.initial_value.clone(),
            }
        }

        fn index_value(&self) -> FieldElement {
            let mut state = (FieldElement::ONE, self.initial_value.clone());
            for _ in 0..self.index {
                state = (state.1.clone(), state.0 + state.1);
            }
            state.0
        }
    }

    impl Claim {
        pub(crate) fn seed(&self) -> Vec<u8> {
            let mut seed = self.index.to_be_bytes().to_vec();
            seed.extend_from_slice(&self.value.as_montgomery().to_bytes_be());
            seed
        }
    }

    impl Arbitrary for Recurrance {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Recurrance {
                // TODO: handle 1 row trace tables.
                index:         1 + usize::arbitrary(g),
                initial_value: FieldElement::arbitrary(g),
            }
        }
    }

    impl Verifiable for Claim {
        fn constraints(&self) -> Constraints {
            use RationalExpression::*;

            // Constraint repetitions
            let trace_length = (self.index + 1).next_power_of_two();
            let trace_generator = FieldElement::root(trace_length).unwrap();
            let g = Constant(trace_generator);
            let on_row = |index| (X - g.pow(index)).inv(); // this is both
            let every_row = || (X - g.pow(trace_length - 1)) / (X.pow(trace_length) - 1.into());

            // Constraints
            Constraints::from_expressions((trace_length, 2), self.seed(), vec![
                (Trace(0, 1) - Trace(1, 0)) * every_row(),
                (Trace(1, 1) - Trace(0, 0) - Trace(1, 0)) * every_row(),
                (Trace(0, 0) - 1.into()) * on_row(0),
                (Trace(0, 0) - (&self.value).into()) * on_row(self.index),
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
                trace[(i, 0)] = trace[(i - 1, 1)].clone();
                trace[(i, 1)] = &trace[(i - 1, 0)] + &trace[(i - 1, 1)];
            }
            trace
        }
    }
}