# OpenZKP

[![Crates.io](https://img.shields.io/crates/l/zkp-stark)](/License.md)
[![](https://docs.rs/zkp-stark/badge.svg)](https://docs.rs/zkp-stark)
[![CircleCI](https://img.shields.io/circleci/build/github/0xProject/OpenZKP)](https://circleci.com/gh/0xProject/OpenZKP)
[![Codecov](https://img.shields.io/codecov/c/gh/0xproject/OpenZKP)](https://codecov.io/gh/0xProject/OpenZKP)

OpenZKP - pure Rust implementations of Zero-Knowledge Proof systems.

## Overview

The current version has

* 🌞 a simple interface (see the [example](#example) below),
* 🗜️ succinct proofs,
* 🏎️ decent performance, and
* 🌐 webassembly support.

That being said, it also has a number of limitations, it has

* *no* high-level language,
* *no* comprehensive security audit,
* *no* perfect zero-knowledge,
* hard-coded field and hash function,

and some others, see [features and limitations](#features-and-limitations) below for details.

## Packages

| Package                                                        | Version                                                                                                                              | Description                                                                                       |
| -------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------- |
| `utils/`                                                       |                                                                                                                                      |                                                                                                   |
| [`criterion-utils`](/utils/criterion-utils)                    | [![Crates.io](https://img.shields.io/crates/v/zkp-criterion-utils?label=)](https://crates.io/crates/zkp-criterion-utils)             | Criterion helpers to benchmark over size and number of processors.                                |
| [`error-utils`](/utils/error-utils)                            | [![Crates.io](https://img.shields.io/crates/v/zkp-error-utils?label=)](https://crates.io/crates/zkp-error-utils)                     | Assertion like macros for returning `Result::Err`.                                                |
| [`logging-allocator`](/utils/logging-allocator)                | [![Crates.io](https://img.shields.io/crates/v/zkp-logging-allocator?label=)](https://crates.io/crates/zkp-logging-allocator)         | Wrapper around the system allocator that logs large allocations.                                  |
| [`mmap-vec`](/utils/mmap-vec)                                  | [![Crates.io](https://img.shields.io/crates/v/zkp-mmap-vec?label=)](https://crates.io/crates/zkp-mmap-vec)                           | Substitute for `Vec` that uses file-backed storage.                                               |
| [`macros-lib`](/utils/macros-lib)                              | [![Crates.io](https://img.shields.io/crates/v/zkp-macros-lib?label=)](https://crates.io/crates/zkp-macros-lib)                       | Library of procedural macros implemented using `proc_macro2`                                      |
| [`macros-impl`](/utils/macros-impl)                            | [![Crates.io](https://img.shields.io/crates/v/zkp-macros-impl?label=)](https://crates.io/crates/zkp-macros-impl)                     | Implementation crate for `proc_macro_hack`                                                        |
| [`macros-decl`](/utils/macros-decl)                            | [![Crates.io](https://img.shields.io/crates/v/zkp-macros-decl?label=)](https://crates.io/crates/zkp-macros-decl)                     | Procedural macros.                                                                                |
| `algebra/`                                                     |                                                                                                                                      |                                                                                                   |
| [`u256`](/algebra/u256)                                        | [![Crates.io](https://img.shields.io/crates/v/zkp-u256?label=)](https://crates.io/crates/zkp-u256)                                   | Implementation of 256-bit unsigned integers.                                                      |
| [`primefield`](/algebra/primefield)                            | [![Crates.io](https://img.shields.io/crates/v/zkp-primefield?label=)](https://crates.io/crates/zkp-primefield)                       | A 251-bit prime field suitable for FFTs.                                                          |
| [`elliptic-curve`](/algebra/elliptic-curve)                    | [![Crates.io](https://img.shields.io/crates/v/zkp-elliptic-curve?label=)](https://crates.io/crates/zkp-elliptic-curve)               | An elliptic curve over the `primefield`.                                                          |
| `crypto/`                                                      |                                                                                                                                      |                                                                                                   |
| [`elliptic-curve-crypto`](/crypto/elliptic-curve-crypto)       | [![Crates.io](https://img.shields.io/crates/v/zkp-elliptic-curve-crypto?label=)](https://crates.io/crates/zkp-elliptic-curve-crypto) | Pedersen commitments and digital signatures.                                                      |
| [`hash`](/crypto/hash)                                         | [![Crates.io](https://img.shields.io/crates/v/zkp-hash?label=)](https://crates.io/crates/zkp-hash)                                   | Hash primitive used in `zkp-stark`.                                                               |
| [`merkle-tree`](/crypto/merkle-tree)                           | [![Crates.io](https://img.shields.io/crates/v/zkp-merkle-tree?label=)](https://crates.io/crates/zkp-merkle-tree)                     | Merkle tree based vector commitment.                                                              |
| [`stark`](/crypto/stark)                                       | [![Crates.io](https://img.shields.io/crates/v/zkp-stark?label=)](https://crates.io/crates/zkp-stark)                                 | Implementation of the STARK ZK-proof system.                                                      |

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

## Contributing

See our [Contributing guideline](/Contributing.md) and [Code of conduct](/Code_of_conduct.md).

See CircleCI documentation on how to [run tests locally][cci-local].

[cci-local]: https://circleci.com/docs/2.0/local-cli/
