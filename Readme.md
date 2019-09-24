# OpenZKP

[![Crates.io](https://img.shields.io/crates/l/openstark)](/License.md)
[![](https://docs.rs/openstark/badge.svg)](https://docs.rs/openstark)
[![CircleCI](https://img.shields.io/circleci/build/github/0xProject/starkcrypto)](https://circleci.com/gh/0xProject/starkcrypto)
[![Codecov](https://img.shields.io/codecov/c/gh/0xproject/starkcrypto)](https://codecov.io/gh/0xProject/starkcrypto)

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

| Package                                                        | Version                                                                                                             | Description                                                                                       |
| -------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| `utils/`                                                       |                                                                                                                     |                                                                                                   |
| [`criterion-utils`](/utils/criterion-utils)                    | [![Crates.io](https://img.shields.io/crates/v/criterion-utils?label=)](https://crates.io//)                         | Criterion helpers to benchmark over size and number of processors.                                |
| [`error-utils`](/utils/error-utils)                            | [![Crates.io](https://img.shields.io/crates/v/criterion-utils?label=)](https://crates.io//)                         | Assertion like macros for returning `Result::Err`.                                                |
| [`logging-allocator`](/utils/logging-allocator)                | [![Crates.io](https://img.shields.io/crates/v/criterion-utils?label=)](https://crates.io//)                         | Wrapper around the system allocator that logs large allocations.                                  |
| [`mmap-vec`](/utils/mmap-vec)                                  | [![Crates.io](https://img.shields.io/crates/v/criterion-utils?label=)](https://crates.io//)                         | Replacement for `Vec` that uses file-backed storage.                                              |
| [`macros-lib`](/utils/macros-lib)                              | ![Unpublished](https://img.shields.io/badge/-unpublished-lightgrey)                                                 | Library of procedural macros implemented using `proc_macro2`                                      |
| [`macros-impl`](/utils/macros-impl)                            | ![Unpublished](https://img.shields.io/badge/-unpublished-lightgrey)                                                 | Implementation crate for `proc_macro_hack`                                                        |
| [`macros-decl`](/utils/macros-decl)                            | ![Unpublished](https://img.shields.io/badge/-unpublished-lightgrey)                                                 | Procedural macros.                                                                                |
| `algebra/`                                                     |                                                                                                                     |                                                                                                   |
| [`u256`](/algebra/u256)                                        | ![Unpublished](https://img.shields.io/badge/-unpublished-lightgrey)                                                 | Implementation of 256-bit unsigned integers.                                                      |
| [`primefield`](/algebra/primefield)                            | ![Unpublished](https://img.shields.io/badge/-unpublished-lightgrey)                                                 | A 251-bit prime field suitable for FFTs.                                                                            |
| [`elliptic-curve`](/algebra/elliptic-curve)                    | ![Unpublished](https://img.shields.io/badge/-unpublished-lightgrey)                                                 | A crypto-grade elliptic curve over the `primefield`.                                              |
| `crypto/`                                                      |                                                                                                                     |                                                                                                   |
| [`elliptic-curve-crypto`](/crypto/elliptic-curve-crypto)       | ![Unpublished](https://img.shields.io/badge/-unpublished-lightgrey)                                                 | Pedersen commitments and digital signatures.                                                      |
| [`hash`](/crypto/hash)                                         | ![Unpublished](https://img.shields.io/badge/-unpublished-lightgrey)                                                 | Hash primitive used in `openstark`.                                                               |
| [`merkle-tree`](/crypto/merkle-tree)                           | [![Crates.io](https://img.shields.io/crates/v/criterion-utils?label=)](https://crates.io//)                         | Merkle tree based vector commitment.                                                              |
| [`openstark`](/crypto/openstark)                               | [![Crates.io](https://img.shields.io/crates/v/criterion-utils?label=)](https://crates.io//)                         | Implementation of the STARK ZK-proof system.                                                      |


## Example

Think of a secret number $n$ and start a sequence with $x_0 = 1$ and $x_1 = n$. Then proceed as a Fibonacci sequence: $x_i = x_{i-2} + x_{i-1}$. We want to proof someone we know $x_k$ without revealing $n$.

```rust
use openstark::{Provable, Verifiable};

struct Claim {
    index: usize,
    value: FieldElement,
}

struct Secret {
    seed: FieldElement,
}

impl Verifiable for Claim {
    fn constraints(&self) -> Constraints {
        Constraints {
            // TODO
        }
    }
}

impl Provable<Secret> for Claim {
    // TODO
}

pub fn main() {
    let claim = Claim {
        index: 5000,
        value: field_element!(""),
    };
    let secret = field_element!("");
    let proof = claim.proof(secret);

    claim.verify(proof)?;
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
