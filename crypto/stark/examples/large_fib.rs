#![warn(clippy::all)]
use log::info;
use std::{env, time::Instant};
use zkp_macros_decl::field_element;
use zkp_primefield::{FieldElement, Root};
use zkp_stark::{prove, verify, Constraints, Provable, RationalExpression, TraceTable, Verifiable};
use zkp_u256::U256;

struct Claim {
    index: usize,
    value: FieldElement,
}

struct Witness {
    secret: FieldElement,
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
        let every_row = || (X - g.pow(trace_length - 1)) / (X.pow(trace_length) - 1);

        Constraints::from_expressions((trace_length, 2), seed, vec![
            (Trace(0, 1) - Trace(1, 0)) * every_row(),
            (Trace(1, 1) - Trace(0, 0) - Trace(1, 0)) * every_row(),
            (Trace(0, 0) - 1) * on_row(0),
            (Trace(0, 0) - &self.value) * on_row(self.index),
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

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(args[1].parse::<usize>().expect("Invalid number supplied"))
            .build_global()
            .expect("Error building Rayon thread pool.");
    }
    info!("Starting Fibonacci benchmark...");

    let claim = Claim {
        index: 1_000_000,
        value: field_element!("02bcd597f68583c9c818cf260a35a64c09bbac70e485a0f857ba042808bff531"),
    };
    let witness = Witness {
        secret: field_element!("deadbeef"),
    };

    let start = Instant::now();
    let constraints = claim.constraints();
    let trace = claim.trace(&witness);
    // assert_eq!(claim.check(&witness), Ok(()));
    let proof = prove(&constraints, &trace).expect("Proof failed");
    let duration = start.elapsed();
    println!("Time elapsed in proof function is: {:?}", duration);
    println!("The proof length is {}", proof.as_bytes().len());
    println!(
        "The estimated size bound is: {}",
        constraints.max_proof_size()
    );

    verify(&constraints, &proof).expect("Verification failed");
}
