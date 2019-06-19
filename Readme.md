# Crypto support library for StarkDEX

NOTE: Modular inversion is not constant time.

## Testing

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

## TODO

-   Make function `const fn`.
-   Integrate a fuzzer.
-   Implement more algoritms:
    https://en.wikipedia.org/wiki/Template:Number-theoretic_algorithms
-   Migrate to libcore
    https://doc.rust-lang.org/core/
-   Use const genetics for modulus and const fn for field params?
    This RFC seems to explicitly not allow that:
    https://github.com/rust-lang/rfcs/blob/master/text/2000-const-generics.md

-   GCD http://www.csd.uwo.ca/~moreno/CS424/Ressources/ComparingSeveralGCDAlgorithms.Jebelean.1993.pdf
    https://pdfs.semanticscholar.org/a7e7/b01a3dd6ac0ec160b35e513c5efa38c2369e.pdf
    https://math.stackexchange.com/questions/2515148/lehmers-algorithm-for-finding-the-greatest-common-divisor

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
