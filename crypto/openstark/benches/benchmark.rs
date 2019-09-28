#![warn(clippy::all)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use macros_decl::u256h;
use openstark::{fibonacci, proof, verify, ProofParams, Provable, Verifiable};
use primefield::FieldElement;
use u256::U256;

fn proof_make(crit: &mut Criterion) {
    let claim = fibonacci::Claim {
        index: 1000,
        value: FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        )),
    };
    let witness = fibonacci::Witness {
        secret: FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000000000000cafebabe"
        )),
    };
    let seed = Vec::from(&claim);
    let constraints = claim.constraints();
    let trace = claim.trace(&witness);

    crit.bench_function("Making an abstracted Fibonacci proof", move |bench| {
        bench.iter(|| {
            black_box(proof(&seed, &constraints, &trace, &ProofParams {
                blowup:     16,
                pow_bits:   12,
                queries:    20,
                fri_layout: vec![3, 2, 1],
            }))
        })
    });
}

fn proof_check(crit: &mut Criterion) {
    let claim = fibonacci::Claim {
        index: 1000,
        value: FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        )),
    };
    let witness = fibonacci::Witness {
        secret: FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000000000000cafebabe"
        )),
    };

    let seed = Vec::from(&claim);
    let constraints = claim.constraints();
    let trace = claim.trace(&witness);
    let proof = proof(&seed, &constraints, &trace, &ProofParams {
        blowup:     16,
        pow_bits:   12,
        queries:    20,
        fri_layout: vec![3, 2, 1],
    });

    crit.bench_function("Checking a fib proof of len 1024", move |bench| {
        bench.iter(|| {
            black_box(verify(
                &seed,
                proof.proof.as_slice(),
                &constraints,
                &ProofParams {
                    blowup:     16,
                    pow_bits:   12,
                    queries:    20,
                    fri_layout: vec![3, 2, 1],
                },
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
