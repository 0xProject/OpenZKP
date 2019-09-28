#![warn(clippy::all)]
use env_logger;
use log::info;
use openstark::{proof, verify, ProofParams};
use primefield::FieldElement;
use std::time::Instant;

use macros_decl::field_element;
use openstark::{
    constraints::Constraints, rational_expression::RationalExpression, Provable, TraceTable,
    Verifiable,
};
use u256::U256;

#[derive(Debug)]
pub struct Claim {
    pub c0_start: FieldElement,
    pub c1_start: FieldElement,
    pub c0_end:   FieldElement,
    pub c1_end:   FieldElement,
}

pub const R: FieldElement = field_element!("03");

impl From<&Claim> for Vec<u8> {
    fn from(input: &Claim) -> Self {
        let mut ret = input.c0_start.as_montgomery().to_bytes_be().to_vec();
        ret.extend_from_slice(&input.c1_start.as_montgomery().to_bytes_be());
        ret.extend_from_slice(&input.c0_end.as_montgomery().to_bytes_be());
        ret.extend_from_slice(&input.c1_end.as_montgomery().to_bytes_be());
        ret
    }
}

impl Verifiable for Claim {
    #[allow(clippy::possible_missing_comma)]
    fn constraints(&self) -> Constraints {
        use RationalExpression::*;

        let trace_length = 1_048_576;
        let trace_generator = FieldElement::root(trace_length).unwrap();

        // Constraint repetitions
        let g = Constant(trace_generator);
        let on_row = |index| (X - g.pow(index)).inv();
        let reevery_row = || (X - g.pow(trace_length - 1)) / (X.pow(trace_length) - 1.into());

        Constraints::from_expressions((trace_length, 4), vec![
            // Square (Trace(0,0), Trace(1, 0)) and check that it equals (Trace(2,0), Trace(3,0))
            (Trace(0, 0) * Trace(0, 0) + Constant(R) * Trace(1, 0) * Trace(1, 0) - Trace(2, 0))
                * reevery_row(),
            (Constant(2.into()) * Trace(0, 0) * Trace(1, 0) - Trace(3, 0)) * reevery_row(),
            // Multiply the square by the single and the square and enforce it on the next row
            (Trace(0, 0) * Trace(2, 0) + Constant(R) * Trace(1, 0) * Trace(3, 0) - Trace(0, 1))
                * reevery_row(),
            (Trace(0, 0) * Trace(2, 0) + Trace(1, 0) * Trace(3, 0) - Trace(1, 1)) * reevery_row(),
            // Boundary Constraints
            (Trace(0, 0) - (&self.c0_start).into()) * on_row(0),
            (Trace(1, 0) - (&self.c1_start).into()) * on_row(0),
            (Trace(0, 0) - (&self.c0_end).into()) * on_row(trace_length - 1),
            (Trace(1, 0) - (&self.c1_end).into()) * on_row(trace_length - 1),
        ])
        .unwrap()
    }
}

impl Provable<()> for Claim {
    #[cfg(feature = "prover")]
    fn trace(&self, _witness: ()) -> TraceTable {
        let mut trace = TraceTable::new(1_048_576, 4);

        let mut prev_c0 = self.c0_start.clone();
        let mut prev_c1 = self.c1_start.clone();
        for i in 0..1_048_576 {
            trace[(i, 0)] = prev_c0.clone();
            trace[(i, 1)] = prev_c1.clone();
            trace[(i, 2)] = (&trace[(i, 0)]).square() + &R * (&trace[(i, 1)].square());
            trace[(i, 3)] = FieldElement::from(2) * &trace[(i, 0)] * &trace[(i, 1)];
            prev_c0 = &trace[(i, 0)] * &trace[(i, 2)] + &R * &trace[(i, 1)] * &trace[(i, 3)];
            prev_c1 = &trace[(i, 0)] * &trace[(i, 2)] + &trace[(i, 1)] * &trace[(i, 3)];
        }
        assert_eq!(trace[(1_048_576 - 1, 0)], self.c0_end);
        assert_eq!(trace[(1_048_576 - 1, 1)], self.c1_end);
        trace
    }
}

fn main() {
    env_logger::init();
    info!("Starting Fibonacci benchmark...");

    let c0_start =
        field_element!("00a74f2a70da4ea3723cabd2acc55d03f9ff6d0e7acef0fc63263b12c10dd837");
    let c1_start =
        field_element!("02ba0d3dfeb1ee83889c5ad8534ba15723a42b306e2f44d5eee10bfa939ae756");
    let c0_end = field_element!("00f96d03d6da1feaa7462bebd3d691bee9f74d237b5a7180d9274e6d4d8d43d9");
    let c1_end = field_element!("008bbb2c325988ae685e5256b067da1e9f9bbb183bb25f0da1f1dbdb61eb5e76");
    let input = Claim {
        c0_start,
        c1_start,
        c0_end,
        c1_end,
    };
    let params = ProofParams::suggested(20);
    let start = Instant::now();
    let seed = Vec::from(&input);
    let constraints = input.constraints();
    let trace = input.trace(());
    let potential_proof = proof(&seed, &constraints, &trace, &params);
    let duration = start.elapsed();
    println!("{:?}", potential_proof.coin.digest);
    println!("Time elapsed in proof function is: {:?}", duration);
    println!("The proof length is {}", potential_proof.proof.len());

    let verified = verify(
        &seed,
        potential_proof.proof.as_slice(),
        &constraints,
        &params,
    );
    println!("Checking the proof resulted in: {:?}", verified);
}

// TODO - Find a way to test from example files
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{proof_params::ProofParams, proofs::stark_proof, verifier::check_proof};
    use macros_decl::field_element;

    #[ignore]
    #[test]
    fn matter_vfd_test() {
        let c0_start =
            field_element!("00a74f2a70da4ea3723cabd2acc55d03f9ff6d0e7acef0fc63263b12c10dd837");
        let c1_start =
            field_element!("02ba0d3dfeb1ee83889c5ad8534ba15723a42b306e2f44d5eee10bfa939ae756");
        let c0_end =
            field_element!("00f96d03d6da1feaa7462bebd3d691bee9f74d237b5a7180d9274e6d4d8d43d9");
        let c1_end =
            field_element!("008bbb2c325988ae685e5256b067da1e9f9bbb183bb25f0da1f1dbdb61eb5e76");
        let input = Claim {
            c0_start,
            c1_start,
            c0_end,
            c1_end,
        };
        let params = ProofParams::suggested(20);
        let potential_proof = stark_proof(&input, &(), &params);
        assert_eq!(
            check_proof(potential_proof.proof.as_slice(), &input, &params),
            Ok(())
        );
    }
}
