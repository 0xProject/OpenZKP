# Stark Proof

* $\mathbb{F}$: Finite field with roots of unity $ω_n$ of order $n$.
* $N$: Trace table length.
* $W$: Number of columns.
* $g = ω_N$: Generator for the trace table.
* $G = \{g^i | i \in [0, N)\}$
* $β = 16$: Blowup factor.
* $ω$: Generator for the extended domain.
* $Ω = \{ω^i | i \in [0, β N)\}$ extended evaluation domain.
* $z$: Out of domain sampling value.
* $T \in \mathbb{F}^{N \times W}$: Trace table such tat $T_{ij}$ is the $j$th column of the $i$th row.
* $T_i(x) \in \mathbb{F}[x]$ column polynomial such that $T_i(g^j) = T_{ij}$.

## Constraints

The constraints are multivariate algebraic functions over the trace table. The constraints are parameterized by the public inputs.

A Stark proofs the claim that the prover knowns a trace table that satisfies all
the constraints.



## Proof system

Public inputs can be either

* field elements that show up in the constraints, or
* exponents that show up in the zeros-fraction.

Trace table:

The trace table has $N$ rows and some small (<1000) number of columns. Let the
$i$th column be interpolated on domain $G$ to produce a polynomial $T_i(x)$ such
that $T_i(g^j)$ is the value in the $i$th row and $j$th column.

Constraints:

$$
C_i(x) = B_i(T) \frac{N_i(x)}{D_i(x)}
$$

where $B_i$ is some relation between the trace table fields that is required to
be zero on certain rows and $N/D$ an expression that divides out these rows.
Typical values for $N/D$ are:

$$
\begin{aligned}
\frac{1}{x - g^i} && && &&
\frac{x - g^N}{x^N - 1} &&
\end{aligned}
$$

The first means the constraint only applies at row $i$, the second that it
applies everywhere. (TODO: Example where it applies every $m$th row.)

Typical expressions for $B$ are:

$$
\begin{aligned}
T_0(x) - c && && &&
T_1(x⋅g) - T_0(x) - T_1(x) &&
\end{aligned}
$$

Where the first means the value in column $0$ is equal to $c$ and the second
means the value in column $1$ in the next row is the sum of the values in
columns $0$ and $1$ in the current row.

## Proof transcript

0. Setup.

The prover and verifier agree on a constraint system, the Stark construction and
security parameters.

1. P->V: Public inputs.

Prover sends the public inputs.

1. P->V: Trace table commitment on LDE points.

The trace polynomials are evaluated on $Ω$ and the values committed to with a
merkle tree.

Each leaf is a vector of the columns evaluated at that leaf:

$$
[T_0(ω^i), T_1(ω^i), ... T_M(ω^i)]
$$

2. V->P: Constraint coefficients

Two for each constraint: one for the plain constraint and one for the 
degree adjusted constraint.

Later, in the FRI part we will proof that these polynomials have degree less than
or equal to $N$. Our constraints have could be of different degree, for example.

**Example.**

3. P->V: Combined constraint on LDE points

4. V->P: Out of domain sampling value

