# DEEP-ALI

$$
\gdef\gen#1{\left\langle #1 \right\rangle}
$$

Denote with $\omega_n$ consistent primitive roots of unity such that $\omega_n^k = \omega_{n / k}$ when $k \vert n$.

Denote with $\vec x = \gen α$ the vector $x_i = α^i$ with length equal to the order of $α$.

*Vectorize* notation of rational expressions in an elementwise way, such that $\vec y = P(\vec x)$ has values $y_i = P(x_i)$.

1. $P → V: \mathrm{Commit}(P(\gen {ω_{k ⋅ N}} ))$.
2. $P \leftarrow V: α \in \F_p$.
3. $P → V: \mathrm{Commit}(P'(\gen {ω_{k ⋅ N / 2}} ))$
4. $⋮$

where

$$
P'(X^2) = P(X) + \frac{α}{x} ⋅ P(-X)
$$
