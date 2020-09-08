# The Kate Scheme and Variants

[Kate, Zaverucha, Golderg (2010)](https://www.iacr.org/archive/asiacrypt2010/6477178/6477178.pdf)

[Boneh, Drake, Fish, Gabizon (2020)](https://eprint.iacr.org/2020/081.pdf)

## Pairing curve

Given elliptic curves $\G_1$, $\G_2$ and $\G_T$ all of order $p$ with generators $G_1$, $G_2$ and $G_T$ respectively and a *pairing function* $e: \G_1 \times \G_2 \rightarrow \G_T$ that is a bilinear map:

$$
e(G_1, G_2) = G_T
$$

$$
e(α ⋅ A, β ⋅ B) = (α ⋅ β)⋅ e(A, B)
$$

$$
e(A + B, C) = e(A, C) + e(B, C)
$$

$$
e(A, B + C) = e(A, B) + e(A, C)
$$

**Note.** All three groups are written additively. In some literature $\G_T$ or all three groups are written multiplicatively.

## Trusted Setup

Given paring curve with generator $G$ of prime order $p$, pick a secret scalar $α ∈ \F_p$ and construct the common reference string $\set G$:

$$
\set G_1 = \\{ α^i ⋅ G_1 \vert i ∈ [0,n] \\}
$$

$$
\set G_2 = \\{ α^i ⋅ G_2 \vert i ∈ [0,m] \\}
$$

## Commitment

Given a polynomial $f(X) ∈ \F_{\le n}[X]$, using the reference string we can compute (without knowledge of $α$):

$$
f(α) ⋅ G
$$

If $f(X)$ is expressed in coefficients, then:

$$
f(α) ⋅ G
= \left( \sum_i f_i ⋅ α^i \right) ⋅ G
= \sum_i f_i ⋅ \left( α^i ⋅ G \right)
$$

If $f(X)$ is expressed in a set of basis polynomials $l_i(X) ∈ \F_{\le n}[X]$ , then:

$$
f(α) ⋅ G
= \left( \sum_i f_i ⋅ l_i(α) \right) ⋅ G
= \sum_i f_i ⋅ \left( l_i(α) ⋅ G \right)
$$

where the values $l_i(α) ⋅ G$ can be precomputed. This is useful when the polynomials are not specified by coefficients, but by their values on a particular set of points. In this case the $l_i(X)$ are the Langrage basis polynomials for that set of points.  

This value $f(α) ⋅ G$ is our commitment. The above process only works up to degree $n$ so the low-degree-ness of $f$ is implied.

## Opening

### Single point

In Kate (2010) a protocol is developed to proof $f(x) = y$ given a commitment to $f$.

Given a commitment $f(α) ⋅ G$, the verifier wants to prove that $f(x) = y$.

The verifier sends $(x, y, f(α) ⋅ G, f'(α) ⋅ G)$ to the prover where

$$
f'(X) = \frac{f(X) - y}{X - x}
$$

where $f'$ is a polynomial with $\deg f' = \deg f - 1$ if and only if $f(x) = y$.

It is sufficient to check this equality in the random point $α$

$$
f'(α) = \frac{f(α) - y}{α - x}
$$

move the denominator to the other side

$$
f'(α) ⋅ (α - x) = f(α) - y
$$

Multiply both sides by $e(G_1, G_2)$

$$
f'(α) ⋅ (α - x) ⋅ e(G_1, G_2) = (f(α) - y) ⋅ e(G_1, G_2)
$$

Use the bilinear map identities:

$$
e(f'(α) ⋅ G_1, α ⋅ G_2 - x ⋅ G_2) = e(f(α) ⋅ G_1, G_2) - y ⋅ e(G_1, G_2)
$$

and this equation can be checked by the verifier using provided commitments.

### Multiple polynomials and points

In Boneh(2020) this is generalized to multiple polynomials $f_i$ and mutiple evaluations $x_i$, $y_i$.

The commitments are now a set of points $f_i(α) ⋅ G_1$.

Construct $r_i$ that interpolate $f_i$ on evaluation points for f_i$:

$$
r_i(x) = f_i(x)
$$

And $z_i(x) = 0$ on those points (but nowhere else).

Then, like before, we can divide out the evaluations:

$$
f_i'(X) = \frac{f_i(X) - r_i(X)}{z_i(X)}
$$

Combine all polynomials

$$
f(X) = \sum_i \gamma^{i -1} ⋅ f_i(X)
$$

$$
f'(X) = \sum_i \gamma^{i -1} ⋅ f_i'(X)
$$

The commitment is $f'(α) ⋅ G_1$.

To verify, we need to check the identity

$$
f'(X) = \sum_i \gamma^{i -1} ⋅ \frac{f_i(X) - r_i(X)}{z_i(X)}
$$

Again, we cross-multiply. Let $z(X) = \mathrm{lcm}(z_i(X))$ the least common multiple of $z_i$. Multiply both sides by $z(X)$

$$
f'(X) ⋅ z(X) = \sum_i \gamma^{i -1} ⋅ (f_i(X) - r_i(X)) ⋅ \bar z_i(X)
$$

where $\bar z_i(X) = \frac{z(X)}{z_i(X)}$ is a small polynomial, the 'complement' of $z_i(X)$ in $z(X)$.

We again check it only for $X = α$:

$$
f'(α) ⋅ z(α) = \sum_i γ^i ⋅ (f_i(α) - r_i(α)) ⋅ \bar z_i(α)
$$

multiply both sides by $e(G_1, G_2)$:

$$
f'(α) ⋅ z(α) ⋅ e(G_1, G_2) = \sum_i γ^i ⋅ \left( f_i(α) - r_i(α) \right) ⋅ \bar z_i(α) ⋅ e(G_1, G_2)
$$

apply the identities

$$
e(f'(α) ⋅ G_1, z(α) ⋅ G_2) = \sum_i γ^i ⋅ \left( f_i(α) - r_i(α) \right) ⋅ e(G_1, \bar z_i(α) ⋅ G_2)
$$

$$
e(f'(α) ⋅ G_1, z(α) ⋅ G_2) = \sum_i γ^i ⋅ \left(
     e(f_i(α) ⋅ G_1, \bar z_i(α) ⋅ G_2) -
     e(r_i(α) ⋅ G_1, \bar z_i(α) ⋅ G_2)
\right)
$$

$$
e(f'(α) ⋅ G_1, z(α) ⋅ G_2) =
\sum_i e(γ^i ⋅ (f_i(α) ⋅ G_1 - r_i(α) ⋅ G_1), \bar z_i(α) ⋅ G_2)
$$

and this is again an identity in terms of quantities known to the verifier.

## Optimized protocol

The prover commits to $f'(α) ⋅ G_1$ as before.

Channel creates random $β$.

The prover computes

$$
l(X) = \frac{1}{X - β} \\( \sum_i γ^i ⋅ \bar z_i(β) ⋅ (f_i(X) - r_i(β))  - z(β) ⋅ f'(X)\\)
$$

The prover commits to $l(α) ⋅ G_1$.

To verify, we again need to check

$$
f'(X) = \sum_i \gamma^{i -1} ⋅ \frac{f_i(X) - r_i(X)}{z_i(X)}
$$

Recall the identity from before:

$$
f'(X) ⋅ z(X) = \sum_i \gamma^{i -1} ⋅ (f_i(X) - r_i(X)) ⋅ \bar z_i(X)
$$

Let's move it all to the right hand side

$$
0 = \sum_i \gamma^{i -1} ⋅ (f_i(X) - r_i(X)) ⋅ \bar z_i(X) - f'(X) ⋅ z(X)
$$

replace some of the occurrences of $X$ with a new variable $Y$:

**To do.** Why is this a valid move? Why these occurrences?

$$
g(X, Y) = \sum_i \gamma^{i -1} ⋅ (f_i(X) - r_i(Y)) ⋅ \bar z_i(Y) - f'(X) ⋅ z(Y)
$$

we need to demonstrate that $g(X, X) = 0$, or equivalently $g(X, β)$ has a root at $X = β$ or

$$
g'(X) = \frac{g(X, β)}{X - β}
$$

is a polynomial.

The prover commits to $g'(α)$

The verifier needs to check

$$
g'(X) = \frac{g(X, β)}{X - β}
$$

evaluate at $α$:

$$
g'(α) = \frac{g(α, β)}{α - β}
$$

cross multiply

$$
g'(α) ⋅ (α - β) = g(α, β)
$$

add a $β ⋅ g'(α)$ term (this is to avoid an addition in $\G_2$ later)

$$
g'(α) ⋅ α = g(α, β)  + β ⋅ g'(α)
$$


multiply by $e(G_1, G_2)$

$$
g'(α) ⋅ α ⋅ e(G_1, G_2) = (g(α, β)  + β ⋅ g'(α)) ⋅ e(G_1, G_2)
$$

apply the identities

$$
e(g'(α) ⋅ G_1, α ⋅ G_2) = e(g(α, β) ⋅ G_1, G_2)
$$

where

$$
g(α, β) ⋅ G_1 = \sum_i \gamma^{i -1} ⋅ (f_i(α) ⋅ G_1 - r_i(β) ⋅ G_1) ⋅ \bar z_i(β) - f'(α) ⋅ G_1 ⋅ z(β) + β ⋅ g'(α) ⋅ G_1
$$

with all quantities known to the verifier.

**To do.** I have $+ β ⋅ g'(α) ⋅ G_1$ but in the paper this term is negative.

---

<https://blog.cloudflare.com/a-relatively-easy-to-understand-primer-on-elliptic-curve-cryptography/

<https://medium.com/@VitalikButerin/exploring-elliptic-curve-pairings-c73c1864e627>

<https://hackmd.io/@tompocock/shplonk>

<https://www.iacr.org/archive/asiacrypt2010/6477178/6477178.pdf>

## BN254 (aka BN128)

Defined over $F_q$ and $F_q[i]/(i^2 + 1)$ such that $\norm G_1 = \norm G_2 = p$.

<https://eips.ethereum.org/EIPS/eip-196>

<https://eips.ethereum.org/EIPS/eip-197>
