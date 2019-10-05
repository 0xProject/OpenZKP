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
    use zkp_primefield::FieldElement;

    #[derive(Clone, PartialEq, Debug)]
    pub(crate) struct Claim {
        pub(crate) index: usize,
        pub(crate) value: FieldElement,
    }

    #[derive(Clone, PartialEq, Debug)]
    pub(crate) struct Witness {
        pub(crate) secret: FieldElement,
    }

    impl Verifiable for Claim {
        fn constraints(&self) -> Constraints {
            use RationalExpression::*;

            // Seed
            let mut seed = self.index.to_be_bytes().to_vec();
            seed.extend_from_slice(&self.value.as_montgomery().to_bytes_be());

            // Constraint repetitions
            let trace_length = self.index.next_power_of_two();
            let trace_generator = FieldElement::root(trace_length).unwrap();
            let g = Constant(trace_generator);
            let on_row = |index| (X - g.pow(index)).inv();
            let every_row = || (X - g.pow(trace_length - 1)) / (X.pow(trace_length) - 1.into());

            // Constraints
            Constraints::from_expressions((trace_length, 2), seed, vec![
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
            let trace_length = self.index.next_power_of_two();
            let mut trace = TraceTable::new(trace_length, 2);
            trace[(0, 0)] = 1.into();
            trace[(0, 1)] = witness.secret.clone();
            for i in 0..(trace_length - 1) {
                trace[(i + 1, 0)] = trace[(i, 1)].clone();
                trace[(i + 1, 1)] = &trace[(i, 0)] + &trace[(i, 1)];
            }
            trace
        }
    }
}
