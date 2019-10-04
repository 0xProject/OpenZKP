#![warn(clippy::all)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use macros_decl::u256h;
use openstark::{prove, verify, Constraints, Provable, RationalExpression, TraceTable, Verifiable};
use primefield::FieldElement;
use u256::U256;

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

fn bench_prove(crit: &mut Criterion) {
    let claim = Claim {
        index: 1000,
        value: FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        )),
    };
    let witness = Witness {
        secret: FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000000000000cafebabe"
        )),
    };
    let constraints = claim.constraints();
    let trace = claim.trace(&witness);

    crit.bench_function("Making an abstracted Fibonacci proof", move |bench| {
        bench.iter(|| black_box(prove(&constraints, &trace)))
    });
}

fn bench_verify(crit: &mut Criterion) {
    let claim = Claim {
        index: 1000,
        value: FieldElement::from(u256h!(
            "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
        )),
    };
    let witness = Witness {
        secret: FieldElement::from(u256h!(
            "00000000000000000000000000000000000000000000000000000000cafebabe"
        )),
    };

    let constraints = claim.constraints();
    let trace = claim.trace(&witness);
    let proof = prove(&constraints, &trace).unwrap();

    crit.bench_function("Checking a fib proof of len 1024", move |bench| {
        bench.iter(|| black_box(verify(&constraints, &proof)))
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_verify(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_group! {
   name = slow_benches;
   config = Criterion::default().sample_size(20);
   targets = bench_prove
}
criterion_main!(benches, slow_benches);
