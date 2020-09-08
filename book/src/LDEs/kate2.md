# Bra-Kate notation

## Pairing Curve

$$
\gdef\bra#1{\left⟨ #1 \right|}
\gdef\ket#1{\left| #1 \right⟩}
\gdef\braket#1#2{\left⟨ #1 \middle| #2 \right⟩}
$$

Given elliptic curve groups $\G_1$, $\G_2$ and $\G_T$ all of order $p$ with generators $G_1$, $G_2$ and $G_T$ respectively and a *pairing function* $e: \G_1 × \G_2 → \G_T$ that is a bilinear map:

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

**Note.** All three groups are written additively. In literature the convention can be multiplicative or even mixed.

A [bilinear map](https://en.wikipedia.org/wiki/Bilinear_map) is a generalization of an [inner product](https://en.wikipedia.org/wiki/Inner_product_space). The [bra-ket notation](https://en.wikipedia.org/wiki/Bra%E2%80%93ket_notation) popularized in quantum mechanics is great for working with these, so let's try it. While we are at it, let's also generalize our maps from $\F_p$ to $\F_p[X]$ by fixing some $\alpha \in \F_p$. Define the following:

$$
\begin{aligned}
\bra ⋅ &: \F_p[X] → \G_1 &
\bra f &≜ f(α) ⋅ G_1
\\\\
\ket ⋅ &: \F_p[X] → \G_2 &
\ket f &≜ f(α) ⋅ G_2
\\\\
\braket ⋅ ⋅ &: \F_p[X] × \F_p[X] → \G_T &
\braket f g &≜ e\left(\bra f, \ket g\right)
\end{aligned}
$$

This implies $\bra 1 = G_1$, $\braket 1 1 = G_T$, $\ket {X^2} = α^2 ⋅ G_2$, etc.

## Trusted setup

Given paring curve with generator $G$ of prime order $p$, pick a secret scalar $α ∈ \F_p$ and construct the common reference string $(\set G_1, \set G_2)$:

$$
\begin{aligned}
\set G_1 &= \left\\{ \bra {X^i} \middle\vert i ∈ [0,n] \right\\}
&
\set G_2 &= \\{ \ket {X^i} \vert i ∈ [0,m] \\}
\end{aligned}
$$

## Commitment

Given a polynomial $f(X) ∈ \F_{\le n}[X]$, using the reference string we can compute $\bra f$:

If $f(X)$ is expressed in coefficients, then:

$$
\bra f
= \bra{\sum_i f_i ⋅ X^i}
= \sum_i f_i ⋅ \bra{X^i}
$$

If $f(X)$ is expressed in a set of basis polynomials $l_i(X) ∈ \F_{\le n}[X]$ , then:

$$
\bra f
= \bra{ \sum_i f_i ⋅ l_i(X) }
= \sum_i f_i ⋅ \bra{l_i}
$$

where the values $\bra{l_i}$ can be precomputed. This is useful when the polynomials are not specified by coefficients, but by their values on a particular set of points. In this case the $l_i(X)$ are the Langrage basis polynomials for that set of points.  

This value $\bra f$ is our commitment. The above process only works up to degree $n$ so the low-degree-ness of $f$ is implied.

---

To commit a polynomial $f$, send $\bra f$.

multiply by $\braket 1 1$:

$$
g'(α) ⋅ α ⋅ \braket 1 1 = (g(α, β)  + β ⋅ g'(α)) ⋅ \braket 1 1
$$

apply the identities

$$
\braket{g'}{X} = \braket{g(X, β)}{1}
$$

where

$$
\bra{g(X, β)} = β ⋅ \bra{g'} - z(β) ⋅ \bra{f'} + \sum_i \gamma^{i -1} ⋅ \bar z_i(β) ⋅ (\bra{f_i} - \bra{r_i(β)})
$$
