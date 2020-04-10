# Introduction to ZK-STARKs

$$
\gdef\F{\mathtt {F}}
\gdef\X{\mathtt {X}}
\gdef\Y{\mathtt {Y}}
\gdef\Z{\mathtt {Z}}
$$

## Disclaimer: contains math

* If you don't understand something
  * Not your fault, this stuff is hard
  * Nobody understands it fully
* If you don't understand anything
  * My fault, anything can be explained at some level
* If you *do* understand everything
  * Collect your Turing Award & Fields Medal
  * Many open questions

## Zero knowledge proofs

We know some algorithm $\F(\X, \Y)$.

I give you $\X$ and $\Z$ and proof that “I know an $\Y$ such that $\F(\X, \Y) = \Z$” without revealing $\Y$.

* $\X$ public input, old balances.
* $\Y$ secret input, trades.
* $\Z$ public output, new balances.

### Scalable DEX

“I know an $\Y$ such that $\F(\X, \Y) = \Z$”

* public input $\X$: (merkle root of) old balances.
* secret input $\Y$: trades.
* public output $\Z$: (merkle root of) new balances.

$\F$ verifies maker and taker signatures on the trades and updates the balances.

### Naive solution

* I give you $\X$, $\Y$ and $\Z$.
* You compute $\F(\X, \Y)$ and verify that it is $\Z$.

*Problems*:

* 📀 I need to send data size $O(\X + \Y + \Z)$, i.e. all the trades.\
  💾 We want $O(\X + \Z + \F)$, only merkle roots.
* ⏳ You need to do computations $O(\F)$.\
  ⌛ We want constant gas.
* 🤫 You now know $\Y$, the secret input.\
  🤷 We don't care.

## Math refresher: Polynomials

----

| | |
|------------|--------|
| Constant   | $a_0$ |
| Linear     | $a_0 + a_1 x$ |
| Parabola   | $a_0 + a_1 x + a_2 x^2$ |
| Cubic      | $a_0 + a_1 x + a_2 x^2 + a_3 x^3$ |
| Quartic    | $a_0 + a_1 x + a_2 x^2 + a_3 x^3 + a_4 x^4$ |
| ...        | $a_0 + a_1 x + a_2 x^2 + \cdots + a_n x^n$ |

Can be uniquely described in three ways:

* $n + 1$ Coefficients
* $n + 1$ Points
* $n$ Zeros* and a scaling factor


(\* Zeros might be imaginary.)

Can do math with them:

* Add $\deg (P + Q) = \max (\deg P, \deg Q)$.
* Multiply $\deg (P \times Q) = \deg P + \deg Q$.
* Divide $\deg \frac{P}{Q} = \deg P - \deg Q$
* Division works when zeros match.

## Toy example: Fibonnacci

We want to prove the 1000-th Fibonacci number starting from a public and a secret value. Take $\F(\X, \Y) = \Z$ to mean the following:

$$
\begin{aligned}
F_0 &:= \X &
F_i &:= F_{i - 2} + F_{i - 1} \\\\
F_1 &:= \Y &
\Z  &:= F_{1000} \\\\
\end{aligned}
$$

## Computational trace

Computation with $n$ steps and $w$ *registers*. The trace $T$ is a $n × w$ table.
Here $n = 1000$ and $w = 2$. Restate algorithm as constraints on $T_{i}$

Example: $\X = 3$, $\Y = 4$:

| n | $T_{n, 0}$ | $T_{n, 1}$ |
|---|----|----|
| 0 |  3 |  4 |
| 1 |  4 |  7 |
| 2 |  7 | 11 |
| 3 | 11 | 18 |
|... | ... | ... |
| 999 | $F_{999}$ | $F_{1000}$ |

Encode the algorithm as a set of *transition constraints*:

$$
\begin{aligned}
T_{i + 1, 0} &= T_{i, 1} &
T_{i + 1, 1} &= T_{i, 0} + T_{i, 1}
\end{aligned}
$$

and *boundary constraints*:

$$
\begin{aligned}
T_{0, 0} &= \X &
T_{999, 1} &= \Z &
\end{aligned}
$$

‟I know $y$ such that $f(x,y)=z$.”

$⇔$

‟I know a trace $T$ such that the constraints hold.”

## Trace polynomials

For each register $j$, create a polynomial $P_j(x)$ of degree $999$ such that
$P_j(i) = T_{i, j}$ for $i = 0 … 999$.

(Actual implementation uses $P_j(ω^i) = T_{i, j}$ with $ω$ a $n$-root of unity to allow $O(n \log n)$ FFT and FRI. Also rounds $n$ up to the next power of two. Ignore for now.)

Consider the constraint $T_{i + 1, 1} = T_{i, 0} + T_{i, 1}$ for $i = 0 … 999$:

$⇔ P_1(i + 1) = P_0(i) + P_1(i)$ for $i = 0 … 999$

$⇔ P_1(i + 1) - (P_0(i) + P_1(i)) = 0$ for $i = 0 … 999$

$⇔ Q(x) = P_1(x + 1) - (P_0(x) + P_1(x))$ is zero when $x$ is an integer $0 … 999$.

$R(x) = (x - 0) ⋅ (x - 1)⋅ (x - 2) ⋯ (x - 999)$ is a polynomial and is zero *only* when $x$ is an integer $0 … 999$.

This means

$$
C(x) = \frac{Q(x)}{R(x)}
$$

is also a polynomial.

Create functions that are polynomial *only* when the constraints are satisfied:

Transition constraints:

$$
\begin{aligned}
T_{i + 1, 0} &= T_{i, 1}
&⇒&&
C_0(x) &= \frac
    {P_0(x + 1) - P_1(x)}
    {\prod^i_{[0 … 998]}\left( x - i\right)}
\\\\
T_{i + 1, 1} &= T_{i, 0} + T_{i, 1}
&⇒&&
C_1(x) &= \frac
    {P_1(x + 1) - (P_0(x) + P_1(x))}
    {\prod^i_{[0\dots998]}\, (x - i)}
\end{aligned}
$$

Boundary constraints:

$$
\begin{aligned}
T_{0, 0} &= X
&⇒&&
C_2(x) &= \frac
    {P_0(x) - X}
    {x - 0}
\\\\
T_{999, 1} &= Z
&⇒&&
C_3(x) &= \frac
    {P_1(x) - Z}
    {x - 999} \\\\
\end{aligned}
$$


‟I know $y$ such that $f(x,y)=z$.”

$⇔$

‟I know a trace $T$ such that the constraints hold.”

$⇔$

‟I know polynomials $P_0$ and $P_1$ such that $C_0$, $C_1$, $C_2$, $C_3$ are polynomial.”

## Interactive proof

I give you $\X$, $\Z$ and a merkle roots of $P_0$ and $P_1$.

You give me random values $α_0$, $α_1$, $α_2$, $α_3$.

## Fast Reed-Solomon Interactive Oracle Proof II

$$
P(x) = a_0 + a_1 x + a_2 x^2 + a_3 x ^3 \cdots + a_n x^n
$$

Given a random number $β$, we can fold the coefficients and get a polynomial of degree $\frac{n}{2}$.

$$
P'(x) = (a_0 + a_1 β) + (a_2 + a_3 β) x + \cdots + ( a_{n-1} + a_n β) x^{\frac n2}
$$

This can be computed using:

$$
P'(x) = P(x) + \left( \frac{β}{2x} - \frac{1}{2}\right) \left(P(x) - P(-x) \right)
$$

$$
P(x) = a_0 + a_1 x + a_2 x^2 + a_3 x ^3 \cdots + a_n x^n
$$

Given a random number $β$, we can fold the coefficients and get a polynomial of degree $\frac{n}{2}$.

$$
P'(x) = (a_0 + a_1 β) + (a_2 + a_3 β) x + \cdots + ( a_{n-1} a_n β) x^{\frac n2}
$$

$$
P'(x) = P(x) + \left( \frac{β}{2x} - \frac{1}{2}\right) \left(P(x) - P(-x) \right)
$$

$$
\begin{aligned}
P(x)  ={}&
a_0 &{}+{}& a_1 x &{}+{}& a_2 x^2 &{}+{}& a_3 x ^3 &{}+{}& \cdots &{}+{}& a_{n-1} x^{n-1} &{}+{}& a_n x^n
\\\\
P(-x)  ={}&
a_0 &{}-{}& a_1 x &{}+{}& a_2 x^2 &{}-{}& a_3 x ^3 &{}+{}& \cdots &{}-{}& a_{n-1} x^{n-1} &{}+{}& a_n x^n 
\\\\
P(x) - P(-x) ={}&
&& 2a_1 x && &{}+{}& 2a_3 x ^3 &{}+{}& \cdots &{}+{}& 2 a_{n-1} x^{n-1} \\\\
\\\\
\frac{β}{2x} \left(P(x) - P(-x)\right) ={}&
 a_1 β && &{}+{}& a_3 β x^2 && &{}+{}& \cdots &{}+{}& a_{n-1} β x^{n-2} \\\\
 \\\\
\frac{1}{2} \left(P(x) - P(-x)\right) ={}&
 a_1 x && &{}+{}& a_3 x^3 && &{}+{}& \cdots &{}+{}& a_{n-1} β x^{n-1} \\\\
  \\\\
(\frac{β}{2x}-\frac{1}{2}) \left(P(x) - P(-x)\right) ={}&
 a_1 β &{}-{}& a_1 x &{}+{}& a_3 β x^2 &{}-{}& a_3 x^3 &{}+{}& \cdots &{}+{}& a_{n-1} β x^{n-1} \\\\
\end{aligned}
$$

$$
P'(x) = (a_0 + a_1 β) + (a_2 + a_3 β) x + \cdots + ( a_{n-1} + a_n β) x^{\frac n2}
$$

I compute $C(x) = α_0 ⋅ C_0(x) + α_1 ⋅ C_1(x) + α_2 ⋅ C_2(x) + α_3 ⋅ C_3(x)$.

I give you the merkle root of $C$ and claim $\deg C = 1024$.

You give me a random value $β_0$.

I give you the merkle root of $C'$ and claim $\deg C' = 512$.

You give me a random value $β_1$.

...

I give you the constant $C''$.


You verify $C''$ using $\X$, $\Y$, the $α$s and the $β$s.


## Fiat-Shamir transform

All you do is give me random numbers. Why don't I replace you by a pseudo random number generator!

Seed PRNG with all prover messages, extract random 'verfier' messages.

Send all the proof at once.
