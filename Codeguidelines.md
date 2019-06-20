# Code guidelines

## 0. Basis

```
cargo +nightly fmt
cargo clippy
```

* https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md
* https://developers.libra.org/docs/community/coding-guidelines
* https://rust-lang-nursery.github.io/api-guidelines/about.html

## 1 Clippy rules

### 1.1 Exceptions on the smallest block

Clippy exceptions should be added on the smallest reasonable scope. Often on functions.

An expection can be made for unit tests, where it is okay to disable it for all tests in the file.

### 1.1 Document exceptions

All exceptions need to have a comment line preceding explaining why the rule leads to false positives.

## 2 Testing

In unit tests, the correct values are named `expected` and computed values are
named `actual`. When comparing, they are ordered `actual` then `expected` like so:

```rust
let actual = square(2);
let expected = 4;
assert_eq!(actual, expected);
```

The expressions can be inlined as long as the order is maintained.

```rust
assert_eq!(square(2), 4);
```


