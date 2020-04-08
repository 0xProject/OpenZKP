# OpenZKP Stark workshop

## Getting started

```bash
$ mkdir openzkp-workshop; cd openzkp-workshop
$ cargo init
$ echo 'zkp-stark = "0.1.2"' >> Cargo.toml
$ cargo build
```

## ZKP recap

```rust
/// The thing to be proven (aka public input)
trait Claim {

    /// Prove the claim using the witness (aka private input)
    fn prove(&self, Witness) -> Proof;

    /// Verify the claim using the proof
    fn verify(&self, Proof) -> bool;
}
```

```rust
```

## STARKs

* The STARK protocol variant implemented in OpenZKP is close to the one developed by Starkware in the StarkDEX verifier contracts.

```rust
struct Claim {
    seed:        Vec<u8>,
    trace_size:  (usize, usize),
    constraints: Vec<RationalExpression>,
    // ... (advanced options with sensible defaults)
}
```

```rust
struct Witness {
    trace_table: Matrix<N, M>
}
```

## Trace table

## Constraints

## Repeating

## Fibonacci

## MIMC

## Future directions

* Constraints over multiple rows.
* More examples.
* EVM Verifier contracts.
* Higher level language.
* Even better performance.
* Aurora, Marlin, Plonk, Fractal, etc.

* YOUR IDEAS & CONTRIBUTIONS!

```python
def fib(index, secret):
    a = 1
    b = secret
    for i in 0..index:
        a, b = b, a + b
    return a


fib(200, ???) = 119...351
```

```python
K = [42, 43, 170, 2209, 16426, 78087, 279978, 823517, ...]

def mimc(initial):
    a = initial
    for i in 0..index:
        a, b = b, a + b
    return a


fib(200, ???) = 119...351
```


| i | `a` | `b` |
|---|-----|-------|
| 0 | 1   | 42 |
| 1 | 42  | 43 |
| 2 | 43  | 85 |
| 3 | 85  | 128 |
| . |  .. | .. |
| 200 | **119...351** | ...|
| .. | .. | .. |
| 255 | .. | .. |

asd

| $x$ | $P_0(x)$ | $P_1(x)$ |
|---|-----|-------|
| $ω^0$ | 1   | 42 |
| $ω^1$ | 42  | 43 |
| $ω^2$ | 43  | 85 |
| $ω^3$ | 85  | 128 |
| . |  .. | .. |
| $ω^{200}$ | **119...351** | ...|
| .. | .. | .. |
| $ω^{255}$ | .. | .. |


asd

$$
\begin{aligned}
T_{i, 0} &= 1  & i &= 0\\
T_{i, 0} &= 119\dots351  & i &= 200\\
T_{i+1, 0} &= T_{i, 1} & i &= 0, 1, 2, \dots \\
T_{i+1, 1} &= T_{i, 0} + T_{i, 1} & i &= 0, 1, 2, \dots\\
\end{aligned}
$$

asd

$$
\begin{aligned}
(T_{i, 0} - 1)  && i &= 0\\
(T_{i, 0} - 119\dots351)  && i &= 200\\
(T_{i+1, 0} - T_{i, 1}) && i &= 0, 1, 2, \dots \\
(T_{i+1, 1} - T_{i, 0} - T_{i, 1}) && i &= 0, 1, 2, \dots\\
\end{aligned}
$$

asd

$$
\begin{aligned}
(P_0(x) - 1)  && i &= 0 \\
(P_0(x) - 119\dots351)  && i &= 200 \\
(P_0(ω \cdot x) - P_1(x)) && i &= 0, 1, 2, \dots \\
(P_1(ω \cdot x) - P_0(x) - P_1(x)) && i &= 0, 1, 2, \dots\\
\end{aligned}
$$

asd

$$
(P_0(x) - 1) / (x - \omega^0) \\
(P_0(x) - 119\dots351) / (x - \omega^{200})\\
(P_0(ω \cdot x) - P_1(x)) (x - \omega^{255}) / (x^{256} - 1) \\
(P_1(ω \cdot x) - P_0(x) - P_1(x)) (x - \omega^{255}) / (x^{256} - 1)\\
$$

asd


$$
\begin{aligned}
(T_{i, 0} - 1)  && x &= ω^0 \\
(T_{i, 0} - 119\dots351)  && x &= ω^{200} \\
(T_{i+1, 0} - T_{i, 0}) && x &= ω^0, ω^1, ω^2, \dots \\
(T_{i+1, 1} - T_{i, 0} - T_{i, 1}) && x &= ω^0, ω^1, ω^2, \dots\\
\end{aligned}
$$

asd

$$
\frac{1}{x - \omega^i}
$$

every row:

$$
\frac{x - \omega^{n -1}}{x^n - 1}
$$

asd

$$
(T_{i, 0} - 1) / (x - \omega^0) \\
(T_{i, 0} - 119\dots351) / (x - \omega^{200})\\
(T_{i+1, 0} - T_{i, 0})(x - \omega^{n -1}) / (x^n - 1) \\
(T_{i+1, 1} - T_{i, 0} - T_{i, 1}) (x - \omega^{n -1}) / (x^n - 1)\\
$$
asd


asd

$$
\begin{aligned}
T_{1, 0} - 1 \\
T_{200, 0} &= 119\dots351 \\
T_{i+1, 0} &= T_{i, 0} \\
T_{i+1, 1} &= T_{i, 0} + T_{i, 1} \\
\end{aligned}
$$

```python
K = [7542, 1223, 3423, ...]

def mimc(x):
    for i in 0..256:
        x = x**3 + K[i % len(K)]
    return x
```

asd

| $x$ | $P_0(x)$ | $x^{16}$ | $K(x^{16})$ |
|---|-----|----|---|
| $ω^0$ | 3231  | $ω^0$ | $K_0$ |
| $ω^1$ | 4232  | $ω^{16}$ | $K_1$ |
| $ω^2$ | 5434  | $ω^{32}$ | $K_2$ |
| $ω^3$ | 7665  | $ω^{48}$ | $K_3$ |
| .. |  .. |
| $ω^{255}$ | 7855  | $ω^{240}$ | $K_{15}$ |

asd

$$
(T_{i+1, 0} - T_{i, 0}^3 - K_i)
$$

asd

$$
(P_0(\omega x) - P_0(x)^3 - K(x^{16}))
$$

asd

$$
K(x) = k_0 + k_1 x + \dots k_{15} x^{15}
$$