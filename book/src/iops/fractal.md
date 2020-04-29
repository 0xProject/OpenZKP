# Fractal

[Chiesa, Ojha, Spooner (2019)](https://eprint.iacr.org/2019/1076.pdf).

https://research.cryptium.ch/demystifying-fractal-part-1/

https://research.cryptium.ch/demystifying-fractal-part-ii/


Given square matrices $A$, $B$ and $C$ from $\F_p^{n × n}$. The predicate on the witness $\vec z$ is

$$
(A ⋅ \vec z) ∘ (B ⋅ \vec z) = C ⋅ \vec z
$$

where $∘$ is the [Hadamard product](https://en.wikipedia.org/wiki/Hadamard_product_(matrices)) (elementwise product).


Given witness $z$  that satisfies above constraints. Define

$$
\begin{aligned}
\vec a &= A ⋅ \vec z &
\vec b &= B ⋅ \vec z &
\vec c &= C ⋅ \vec z &
\end{aligned}
$$

These relations will be individually checked in a process called *lin-check*.

Finally we need to check $a ∘ b = c$, in a process called *row-check*.

The lin-checks are converted into a (rational) sumcheck, and this all combined into a low-degree-test.

Take $\vec u ∈ \F_p[X]^n$ a vector of linearly independent polynomials. Then the inner product 

$$
\vec u^T ⋅ \vec v_1 = \vec u^T ⋅ \vec v_2 \Leftrightarrow \vec v_1 = \vec v_2
$$

This applies with high probability if we instead evaluate only in a single random point.

$$
\vec a = A ⋅ \vec z
$$

$$
\vec u^T ⋅ a = \vec u^T ⋅ A ⋅ \vec z
$$

$$
\vec u^T ⋅ a = (A^T ⋅ \vec u)^T ⋅ \vec z
$$

$$
\vec u^T ⋅ a - (A^T ⋅ \vec u)^T ⋅ \vec z = \vec 0
$$

Now, encode the vectors as polynomials on a basis $⟨ω_n⟩$ such that $a(ω_n^i) = a_i$.

We can restate the equality in polynomials

$$
\vec x(X) =
\begin{pmatrix}
1 \\\\ X \\\\ X^2 \\\\ \vdots \\\\ X^n
\end{pmatrix}
$$

## Sum-check

If $S$ is a multiplicative coset, then $\sum_{s \in S} f(s) = \sigma$ iff $f(x) = x ⋅ g(x) + \sigma / \norm S$ where $\deg g < \norm S - 1$.

Generalization to rational functions

$$
\frac{P(X)}{Q(X)} =_S x ⋅ g(x) + \sigma / \norm S
$$

$$
P(X) =_S (x ⋅ g(x) + \sigma / \norm S) ⋅ Q(X)
$$

$$
0 =_S (x ⋅ g(x) + \sigma / \norm S) ⋅ Q(X) - P(X)
$$

$$
\frac{(x ⋅ g(x) + \sigma / \norm S) ⋅ Q(X) - P(X)}{Z_S(X)}
$$

---

$$
D_S(X, Y) = \frac{Z_S(X) - Z_S(Y)}{X - Y}
$$

On $S$, it is zero when $X \neq  Y$ but non-zero when $X = Y$.
