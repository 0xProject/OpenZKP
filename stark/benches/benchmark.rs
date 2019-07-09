#![warn(clippy::all)]
#![deny(warnings)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hex_literal::*;
use primefield::{u256h, FieldElement, U256};
use stark::{fft_cofactor, get_constraint, get_trace_table, make_tree, stark_proof, ProofParams};

fn merkle_proof_make(crit: &mut Criterion) {
    let depth = 6;
    let mut leaves = Vec::new();

    for i in 0..2_u64.pow(depth) {
        leaves.push(U256::from((i + 10).pow(3)));
    }
    crit.bench_function("Making depth 6 Merkle Tree", move |bench| {
        bench.iter(|| black_box(make_tree(leaves.as_slice())))
    });
}

fn fft_timing(crit: &mut Criterion) {
    let cofactor = FieldElement::from(u256h!(
        "07696b8ff70e8e9285c76bef95d3ad76cdb29e213e4b5d9a9cd0afbd7cb29b5c"
    ));
    let vector = vec![
        FieldElement::from(u256h!(
            "008ee28fdbe9f1a7983bc1b600dfb9177c2d82d825023022ab4965d999bd3faf"
        )),
        FieldElement::from(u256h!(
            "037fa3db272cc54444894042223dcf260e1d1ec73fa9baea0e4572817fdf5751"
        )),
        FieldElement::from(u256h!(
            "054483fc9bcc150b421fae26530f8d3d2e97cf1918f534e67ef593038f683241"
        )),
        FieldElement::from(u256h!(
            "005b695b9001e5e62549557c48a23fd7f1706c1acdae093909d81451cd455b43"
        )),
        FieldElement::from(u256h!(
            "025079cb6cb547b63b67614dd2c78474c8a7b17b3bc53f7f7276984b6b67b18a"
        )),
        FieldElement::from(u256h!(
            "044729b25360c0025d244d31a5f144917e59f728a3d03dd4685c634d2b0e7cda"
        )),
        FieldElement::from(u256h!(
            "079b0e14d0bae81ff4fe55328fb09c4117bcd961cb60581eb6f2a770a42240ed"
        )),
        FieldElement::from(u256h!(
            "06c0926a786abb30b8f6e0eb9ef2278b910862717ed4beb35121d4741717e0e0"
        )),
    ];
    crit.bench_function("Performing FFT", move |bench| {
        bench.iter(|| black_box(fft_cofactor(&vector, &cofactor)))
    });
}

fn abstracted_fib_proof_make(crit: &mut Criterion) {
    let claim_index = 1000_usize;
    let claim_fib = FieldElement::from(u256h!(
        "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
    ));
    let witness = FieldElement::from(u256h!(
        "00000000000000000000000000000000000000000000000000000000cafebabe"
    ));

    crit.bench_function("Making an abstracted Fibonacci proof", move |bench| {
        bench.iter(|| {
            black_box(stark_proof(
                &get_trace_table(1024, witness.clone()),
                &get_constraint(),
                claim_index,
                claim_fib.clone(),
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
    merkle_proof_make(c);
    fft_timing(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_group! {
   name = slow_benches;
   config = Criterion::default().sample_size(20);
   targets = abstracted_fib_proof_make
}
criterion_main!(benches, slow_benches);
