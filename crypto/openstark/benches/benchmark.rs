#![warn(clippy::all)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use macros_decl::u256h;
use openstark::{
    check_proof,
    fibonacci::{get_fibonacci_constraints, get_trace_table, PrivateInput, PublicInput},
    stark_proof, ProofParams,
};
use primefield::FieldElement;
use u256::U256;

fn proof_make(crit: &mut Criterion) {
    let public = PublicInput {
        index: 1000,
        value: FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        )),
    };
    let private = PrivateInput {
        secret: FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000000000000cafebabe"
        )),
    };

    crit.bench_function("Making an abstracted Fibonacci proof", move |bench| {
        bench.iter(|| {
            black_box(stark_proof(
                &get_trace_table(1024, &private),
                &get_fibonacci_constraints(&public),
                &public,
                &ProofParams {
                    blowup:                   16,
                    pow_bits:                 12,
                    queries:                  20,
                    fri_layout:               vec![3, 2, 1],
                    constraints_degree_bound: 1,
                },
            ))
        })
    });
}

fn proof_check(crit: &mut Criterion) {
    let public = PublicInput {
        index: 1000,
        value: FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        )),
    };
    let private = PrivateInput {
        secret: FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000000000000cafebabe"
        )),
    };

    let proof = stark_proof(
        &get_trace_table(1024, &private),
        &get_fibonacci_constraints(&public),
        &public,
        &ProofParams {
            blowup:                   16,
            pow_bits:                 12,
            queries:                  20,
            fri_layout:               vec![3, 2, 1],
            constraints_degree_bound: 1,
        },
    );

    crit.bench_function("Checking a fib proof of len 1024", move |bench| {
        bench.iter(|| {
            black_box(check_proof(
                proof.proof.as_slice(),
                &get_fibonacci_constraints(&public),
                &public,
                &ProofParams {
                    blowup:                   16,
                    pow_bits:                 12,
                    queries:                  20,
                    fri_layout:               vec![3, 2, 1],
                    constraints_degree_bound: 1,
                },
                2,
                1024,
            ))
        })
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    proof_check(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_group! {
   name = slow_benches;
   config = Criterion::default().sample_size(20);
   targets = proof_make
}
criterion_main!(benches, slow_benches);
