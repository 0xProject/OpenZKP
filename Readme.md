# Crypto support library for StarkDEX

NOTE: Modular inversion is not constant time.

## Benchmark

Checkout master branch:

```
cargo bench --bench benchmark -- --save-baseline master
```

```
cargo bench --bench benchmark -- --baseline master
```

## TODO

-   Make function `const fn`.
-   Integrate a fuzzer.
-   Implement more algoritms:
    https://en.wikipedia.org/wiki/Template:Number-theoretic_algorithms
