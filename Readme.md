# OpenStark
[![Crates.io](https://img.shields.io/crates/l/openstark)](/License.md)
[![](https://docs.rs/openstark/badge.svg)](https://docs.rs/openstark)
[![CircleCI](https://img.shields.io/circleci/build/github/0xProject/starkcrypto)](https://circleci.com/gh/0xProject/starkcrypto)
[![Codecov](https://img.shields.io/codecov/c/gh/0xproject/starkcrypto)](https://codecov.io/gh/0xProject/starkcrypto)

A pure-rust implementation of the STARK Zero-Knowledge Proof system.

See the [example](#example) below. The current version has

* 🌞 a simple interface,
* 🗜️ succinct proofs,
* 🏎️ decent performance,
* 🌐 webassembly support,
* ❌ *no* high-level language,
* ❌ *no* comprehensive security audit,
* ❌ *no* perfect zero-knowledge.

See [state](#state) below for details.

## Packages

This repository is a workspace containing many small packages that could be used.
The main package is  [`crypto/openstark`](/crypto/openstark) [![Crates.io](https://img.shields.io/crates/v/openstark)](https://crates.io/project/openstark/).

| Package                                                        | Version                                                                                                             | Description                                                                                       |
| -------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| [`criterion-utils`](/utils/criterion-utils) | [![Crates.io](https://img.shields.io/crates/v/criterion-utils?label=)](https://crates.io/project/0x-contract-addresses/) | A tiny utility library for getting known deployed contract addresses for a particular network     |
| [`error-utils`](/utils/error-utils) | [![PyPI](https://img.shields.io/pypi/v/0x-contract-artifacts.svg)](https://pypi.org/project/0x-contract-artifacts/) | 0x smart contract compilation artifacts                                                           |
| [`0x-contract-wrappers`](/python-packages/contract_wrappers)   | [![PyPI](https://img.shields.io/pypi/v/0x-contract-wrappers.svg)](https://pypi.org/project/0x-contract-wrappers/)   | 0x smart contract wrappers                                                                        |
| [`0x-json-schemas`](/python-packages/json_schemas)             | [![PyPI](https://img.shields.io/pypi/v/0x-json-schemas.svg)](https://pypi.org/project/0x-json-schemas/)             | 0x-related JSON schemas                                                                           |
| [`0x-order-utils`](/python-packages/order_utils)               | [![PyPI](https://img.shields.io/pypi/v/0x-order-utils.svg)](https://pypi.org/project/0x-order-utils/)               | A set of utilities for generating, parsing, signing and validating 0x orders                      |
| [`0x-sra-client`](/python-packages/sra_client)                 | [![PyPI](https://img.shields.io/pypi/v/0x-sra-client.svg)](https://pypi.org/project/0x-sra-client/)                 | A Python client for interacting with servers conforming to the Standard Relayer API specification |

## Example

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

## State

**A simple interface.** The public interface is simple and is considered stable. Future versions are expected to add functionality without breaking this interface.

**Succinct proofs.** For a given security parameter, the proof size is close to minimal. Significant improvements here would require innovations in the way
constraint systems are designed or in the underlying cryptography.

**Decent performance.** All steps of the proof are using asymptotically optimal algorithms and
all of the major steps are multi-threaded. There are no hard memory requirements. We can expect a good amount of performance improvements by fine-tuning, but we don't expect orders of magnitude improvements.

**Webassembly support.** All packages 

**No high-level language.** Constraints are specified using their algebraic expressions. This requires complicated and careful design from the library user and is easy to do wrong, leading to insecure systems. A high level language would help make development simpler and safer and facilitate re-use of components.

**No comprehensive security audit.** While development is done with the best security practices in mind, it is still very early stage and has not had the amount of expert peer review required for a production grade system.

**No perfect zero-knowledge.** The current implementation provides succinct proofs but not perfect zero knownledge. While non-trivial, it is theoretically possible to learn about the secret . We expect to solve this soon.


## Contributing

See our contributing guideline.

**TODO.** Contributing guideline.

**TODO.** Contributing guideline.

**TODO.**  Issue templates.

**TODO.**  Pull request template 



See CircleCI documentation on how to [run tests locally][cci-local].

[cci-local]: https://circleci.com/docs/2.0/local-cli/

## Benchmark

Checkout master branch:

```sh
cargo bench --bench benchmark -- --save-baseline master
```

```sh
cargo bench --bench benchmark -- --baseline master
open target/criterion/report/index.html
```

Benchmarking using Mac OS' instrumentation. For this we need the `cargo-instruments` plugin for Cargo.

```sh
cargo install cargo-instruments
```

You can then run tests under profiling. It is recommended to filter for a specific test.

```sh
cargo instruments --release --bench benchmark --open [test name]
```
## Goals

-   Perfomance optimized for Native and WebAssembly
-   Generality
-   Later: Constant-time operations.
-   Prefer `const fn` over procedural macros.

For optimization, there are a few different scenarios:

Note: The modulus is always assumed to be 256bit or less.

-   Programmer time known fields. The programmer can supply hand tuned optimized
    implementations of various algorithms. Ideally well performing defaults are
    provided.
-   Compiler time known fields.
    The compiler can compute constants, for example for Montgomery
    representation. The field parameters should be inlined.
-   Statically runtime known fields.
    Modulus is not known during compilation (but it's size is). Element
    membership of a particular field is known at compile time. The field
    parameters should statically allocated and the pointers inlined.
-   Dynamically runtime known fields.
    Modulus is not known during compilation (but its size is). Element
    membership of a particular field is not known at compile time. The field
    element should carry a pointer to the field parameters.

## References and benchmarks

-   A sophisticated rust implementation of Curve25519.
    https://github.com/dalek-cryptography/curve25519-dalek
    -   Implementation using AVX512
        https://medium.com/@hdevalence/even-faster-edwards-curves-with-ifma-8b1e576a00e9
        https://doc-internal.dalek.rs/develop/curve25519_dalek/backend/vector/ifma/index.html
-   A rust library for constant time algorithms.
    https://github.com/dalek-cryptography/subtle
-   Probably the most tuned curve out there.
    https://github.com/bitcoin-core/secp256k1
    -   Rust bindings: https://crates.io/crates/secp256k1
    -   Rust port: https://crates.io/crates/libsecp256k1
    -   A fork of secp256k1 favouring performance over constant-timeness.
        https://github.com/llamasoft/secp256k1_fast_unsafe
-   ZCash implementation of Sappling:
    https://github.com/zkcrypto/bellman
    -   https://crates.io/crates/pairing
    -   Fork by Matter Labs.
        -   https://crates.io/crates/ff_ce
        -   https://crates.io/crates/pairing_ce
        -   https://crates.io/crates/bellman_ce
-   Fast implementation of zksnark in java
    https://github.com/scipr-lab/dizk

## References

-   Handbook of Applied Cryptography
    http://cacr.uwaterloo.ca/hac/
-   Guide to Elliptic Curve Cryptography
    https://cdn.preterhuman.net/texts/cryptography/Hankerson,%20Menezes,%20Vanstone.%20Guide%20to%20elliptic%20curve%20cryptography%20(Springer,%202004)(ISBN%20038795273X)(332s)_CsCr_.pdf
