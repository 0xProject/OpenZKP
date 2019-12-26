# Unsigned 256-bit integers

[![Crates.io](https://img.shields.io/crates/l/zkp-u256)](/License.md)
[![](https://docs.rs/zkp-u256/badge.svg)](https://docs.rs/zkp-stark)
[![CircleCI](https://img.shields.io/circleci/build/github/0xProject/OpenZKP)](https://circleci.com/gh/0xProject/OpenZKP)
[![Codecov](https://img.shields.io/codecov/c/gh/0xproject/OpenZKP)](https://codecov.io/gh/0xProject/OpenZKP)

Implementation of 256-bit unsigned integers.

**Warning.** Side-channel resistance is currently not implemented. This library
is optimized for performance and does not use slower side-channel resistant
algorithms. Please evaluate the risks before using with sensitive data.

**Note.** Code coverage in Rust is still very early days. The above number is
likely inaccurate. Please view the coverage report for details.

## Feature flags

* `std` Build using libstd. (enabled by default)
* `inline` Inline small operations like bitshifts, addition, multiplication, etc. This leads to better performance at the cost of larger code size. You can always force inlining by using the `_inline` suffixed version of the operations. (enabled by default)
* `use_rand` Add support for the `rand` crate to generate random numbers.

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
