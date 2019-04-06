# Crypto support library for StarkDEX

NOTE: Modular inversion is not constant time.

## Benchmark

Checkout master branch:

```sh
cargo bench --bench benchmark -- --save-baseline master
```

```sh
cargo bench --bench benchmark -- --baseline master
open target/criterion/report/index.html
```

```sh
cargo install cargo-instruments
cargo instruments --release --bench benchmark --open
```

## TODO

-   Make function `const fn`.
-   Integrate a fuzzer.
-   Implement more algoritms:
    https://en.wikipedia.org/wiki/Template:Number-theoretic_algorithms
-   Migrate to libcore
    https://doc.rust-lang.org/core/

## Goals

-   Perfomance optimized for Native and WebAssembly
-   Generality
-   Later: Constant-time operations.
-   Prefer `const fn` over procedural macros.

For optimization, there are a few different scenarios:

-   Programmer time known fields.
-   Compiler time known fields.
-   Statically runtime known fields.
-   Dynamically runtime known fields.

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
