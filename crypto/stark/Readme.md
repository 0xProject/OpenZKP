# OpenZKP Stark

[![Crates.io](https://img.shields.io/crates/l/zkp-stark)](/License.md)
[![](https://docs.rs/zkp-stark/badge.svg)](https://docs.rs/zkp-stark)
[![CircleCI](https://img.shields.io/circleci/build/github/0xProject/OpenZKP)](https://circleci.com/gh/0xProject/OpenZKP)
[![Codecov](https://img.shields.io/codecov/c/gh/0xproject/OpenZKP)](https://codecov.io/gh/0xProject/OpenZKP)

Implementation of STARK zero-knowledge-proofs.

## Example

```rust
use zkp_stark::{*, primefield::*};

struct FibonacciClaim {
    index: usize,
    value: FieldElement,
}

impl Verifiable for FibonacciClaim {
    fn constraints(&self) -> Constraints {
        use RationalExpression::*;

        // Seed
        let mut seed = self.index.to_be_bytes().to_vec();
        seed.extend_from_slice(&self.value.as_montgomery().to_bytes_be());

        // Constraint repetitions
        let trace_length = self.index.next_power_of_two();
        let g = Constant(FieldElement::root(trace_length).unwrap());
        let on_row = |index| (X - g.pow(index)).inv();
        let every_row = || (X - g.pow(trace_length - 1)) / (X.pow(trace_length) - 1.into());

        let mut c = Constraints::from_expressions((trace_length, 2), seed, vec![
            (Trace(0, 1) - Trace(1, 0)) * every_row(),
            (Trace(1, 1) - Trace(0, 0) - Trace(1, 0)) * every_row(),
            (Trace(0, 0) - 1.into()) * on_row(0),
            (Trace(0, 0) - (&self.value).into()) * on_row(self.index),
        ])
        .unwrap()
    }
}

impl Provable<&FieldElement> for FibonacciClaim {
    fn trace(&self, witness: &FieldElement) -> TraceTable {
        let trace_length = self.index.next_power_of_two();
        let mut trace = TraceTable::new(trace_length, 2);
        trace[(0, 0)] = 1.into();
        trace[(0, 1)] = witness.clone();
        for i in 0..(trace_length - 1) {
            trace[(i + 1, 0)] = trace[(i, 1)].clone();
            trace[(i + 1, 1)] = &trace[(i, 0)] + &trace[(i, 1)];
        }
        trace
    }
}

pub fn main() {
    let claim = FibonacciClaim {
        index: 5000,
        value: FieldElement::from_hex_str("069673d708ad3174714a2c27ffdb56f9b3bfb38c1ea062e070c3ace63e9e26eb"),
    };
    let secret = FieldElement::from(42);
    let proof = claim.prove(&secret).unwrap();
    claim.verify(&proof).unwrap();
}
```

## Features and Limitations

### Features

**A simple interface.** The public interface is simple and is considered [semver-stable](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md). Future versions are expected to add functionality without breaking this interface.

**Succinct proofs.** For a given security parameter, the proof size is close to minimal. Significant improvements here would require innovations in the way constraint systems are designed or in the underlying cryptography.

**Decent performance.** All steps of the proof are using asymptotically optimal algorithms and all of the major steps are multi-threaded. There are no hard memory requirements. We can expect a good amount of performance improvements by fine-tuning, but we don't expect orders of magnitude improvements.

**Webassembly support.** The verifier can be used in a WebAssembly environment without the Rust `std` lib. The prover will work too, but has not been a priority.

### Limitations

**No high-level language.** Constraints are specified using their algebraic expressions. This requires complicated and careful design from the library user and is easy to do wrong, leading to insecure systems. A high level language would help make development simpler and safer and facilitate re-use of components.

**No comprehensive security audit.** While development is done with the best security practices in mind, it is still very early stage and has not had the amount of expert peer review required for a production grade system.

**No perfect zero-knowledge.** The current implementation provides succinct proofs but not perfect zero knowledge. While non-trivial, it is theoretically possible to learn something about the secret. Achieving perfect zero-knowledge is possible and can be implemented.

**No side-channel resistance.** The implementation favours performance over side-channel resistance. While this is common in zero-knowledge proof system, you should be aware that his might leak intermediate computations. Side-channel resistance can be implemented.

**Hard-coded field and hash.** The current implementation uses a particular [prime field](/algebra/primefield) and a particular [hash function](/crypto/hash). These are optimized for verification in the Ethereum Virtual Machine. This can be generalized to other primitives optimized for other use cases.
