#![warn(clippy::all)]
use log::info;
use std::time::Instant;
use zkp_macros_decl::field_element;
use zkp_primefield::{FieldElement, Root, SquareInline};
use zkp_stark::{Constraints, Provable, RationalExpression, TraceTable, Verifiable};
use zkp_u256::U256;

const R: FieldElement = field_element!("03");

#[derive(Debug)]
struct Claim {
    pub c0_start: FieldElement,
    pub c1_start: FieldElement,
    pub c0_end:   FieldElement,
    pub c1_end:   FieldElement,
}

impl Verifiable for Claim {
    fn constraints(&self) -> Constraints {
        use RationalExpression::*;

        // Seed
        let mut seed = self.c0_start.as_montgomery().to_bytes_be().to_vec();
        seed.extend_from_slice(&self.c1_start.as_montgomery().to_bytes_be());
        seed.extend_from_slice(&self.c0_end.as_montgomery().to_bytes_be());
        seed.extend_from_slice(&self.c1_end.as_montgomery().to_bytes_be());

        // Constraint repetitions
        let trace_length = 1_048_576;
        let trace_generator = FieldElement::root(trace_length).unwrap();
        let g = Constant(trace_generator);
        let on_row = |index| (X - g.pow(index)).inv();
        let every_row = || (X - g.pow(trace_length - 1)) / (X.pow(trace_length) - 1);

        Constraints::from_expressions((trace_length, 4), seed, vec![
            // Square (Trace(0,0), Trace(1, 0)) and check that it equals (Trace(2,0),
            // Trace(3,0))
            ((Trace(0, 0) * Trace(0, 0) + Constant(R) * Trace(1, 0) * Trace(1, 0) - Trace(2, 0))
                * every_row()),
            (Constant(2.into()) * Trace(0, 0) * Trace(1, 0) - Trace(3, 0)) * every_row(),
            // Multiply the square by the single and the square and enforce it on the next row
            ((Trace(0, 0) * Trace(2, 0) + Constant(R) * Trace(1, 0) * Trace(3, 0) - Trace(0, 1))
                * every_row()),
            (Trace(0, 0) * Trace(2, 0) + Trace(1, 0) * Trace(3, 0) - Trace(1, 1)) * every_row(),
            // Boundary Constraints
            (Trace(0, 0) - &self.c0_start) * on_row(0),
            (Trace(1, 0) - &self.c1_start) * on_row(0),
            (Trace(0, 0) - &self.c0_end) * on_row(trace_length - 1),
            (Trace(1, 0) - &self.c1_end) * on_row(trace_length - 1),
        ])
        .unwrap()
    }
}

impl Provable<()> for Claim {
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
    info!("Starting VDF benchmark...");
    let claim = Claim {
        c0_start: field_element!(
            "00a74f2a70da4ea3723cabd2acc55d03f9ff6d0e7acef0fc63263b12c10dd837"
        ),
        c1_start: field_element!(
            "02ba0d3dfeb1ee83889c5ad8534ba15723a42b306e2f44d5eee10bfa939ae756"
        ),
        c0_end:   field_element!(
            "00f96d03d6da1feaa7462bebd3d691bee9f74d237b5a7180d9274e6d4d8d43d9"
        ),
        c1_end:   field_element!(
            "008bbb2c325988ae685e5256b067da1e9f9bbb183bb25f0da1f1dbdb61eb5e76"
        ),
    };
    assert_eq!(claim.check(()), Ok(()));

    let start = Instant::now();
    let proof = claim.prove(()).expect("Proof failed.");
    let duration = start.elapsed();
    println!("Time elapsed in proof function is: {:?}", duration);
    println!("The proof length is {}", proof.as_bytes().len());

    let verified = claim.verify(&proof);
    println!("Checking the proof resulted in: {:?}", verified);
}
