# Code guidelines

## Automated linting

We use automated linting tools where possible and these are configured using
`.editorconfig` and `rustfmt.toml`. Code is linted using the following three
commands:

```
RUSTFLAGS="-Dwarnings" cargo build --all --all-targets
cargo +nightly fmt
cargo clippy
```

**NOTE**: We require a nightly build of `rustfmt`.

## External guidelines

We follow the

* ["Rust style guide"][rustfmt],
* ["Rust API Guidelines"][rustapi],
* ["Rust Book"][rustbook],
* ["Secure Rust Guideline"][sec-rs], and
* ["Libra Coding Guidelines"][libra].

[rustfmt]: https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md
[rustapi]: https://rust-lang-nursery.github.io/api-guidelines/about.html
[rustbook]: https://doc.rust-lang.org/book/title-page.html
[sec-rs]: https://anssi-fr.github.io/rust-guide/
[libra]: https://developers.libra.org/docs/community/coding-guidelines

Except,

* Unit tests are in the same file as the code covered and not in separate files.
* We use `proptest` for property based testing.

In addition to the above, we use [Criterion][criterion] extensively for statistical
benchmarking. See the section below for what is expected.

[criterion]: https://bheisler.github.io/criterion.rs/book/index.html

## Making exceptions

Sometimes clippy will give a false positive, where compliance will decrease the
safety and readability of the code. In this case a localized exception is in order.

Clippy exceptions should be added on the smallest reasonable scope. Often, this
will be a single function. An exception can be made for unit tests, where it is
okay to disable it for all tests in the file.

All exceptions need to have a comment line preceding explaining why the rule leads
to false positives.

## Testing

Be liberal with `debug_assert` statements.

In unit tests, the correct values are named `expected` and computed values are
named `actual`. When comparing, they are ordered `actual` then `expected` like
so:

```rust
let actual = square(2);
let expected = 4;
assert_eq!(actual, expected);
```

The expressions can be inlined as long as the above order is maintained.

```rust
assert_eq!(square(2), 4);
```

## Benchmarking

Benchmarks should be added for functions that are critical to performance.
