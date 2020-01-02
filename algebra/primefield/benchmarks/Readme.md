# Starkcrypto benchmark

```sh
cargo bench --bench benchmark
open target/criterion/report/index.html
```

## Comparisson

Note that all these libraries aim for side-channel resistance and thus use
slower constant-time algorithms.

-   Secp256k1 native
-   Secp256k1 bindings
-   Ed25519 dalek
-   Curve25519 donna bindings
-   TODO: https://github.com/AztecProtocol/barretenberg

- TODO: https://github.com/scipr-lab/zexe
