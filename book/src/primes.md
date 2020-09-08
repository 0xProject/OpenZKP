# Special primes


**Solinas Primes.** $p = P(\cdot 2^{k})$ with $P$ a low-degree polynomial with tiny coefficients, usually ${-1,0,1}$.

*Note.* These are also known as Generalized Mersenne Primes.

https://en.wikipedia.org/wiki/Solinas_prime

https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.46.2133&rep=rep1&type=pdf

http://cacr.uwaterloo.ca/techreports/1999/corr99-39.pdf

**Pseudo-Mersenne Primes.** A subset of Solinas Primes of the form $p = 2^k - c$ with $c$ a small.

https://crypto.stackexchange.com/a/14807

https://crypto.stackexchange.com/questions/32222/difference-between-pseudo-mersenne-primes-and-generalized-mersenne-primes

**Proth Primes.** $p = c \cdot 2^{k} + 1$ with $2^{k} > c$ and $c$ small.

https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/modmul_no_precomp.pdf


### List of primes

https://safecurves.cr.yp.to/field.html

# Montgomery form for Proth primes

We are interested in arithmatic in a prime field $\F_p$ with roots of unity $\omega_n$ for large powers of two, $n = 2^k$. Due to math, this means that $2^k \vert p -1$ and so $p = c\cdot2^k +1$.

Prime numbers of the form $c\cdot2^k +1$ are known as [Proth primes](https://en.wikipedia.org/wiki/Proth_prime) after the related [Proth's theorem](https://en.wikipedia.org/wiki/Proth%27s_theorem).

**Theorem.** *(Proth's theorem) If $p = c\cdot 2^k + 1$ with $c$ odd and $2^k > c$ and there exist an integer $a$ such that $a^{\frac{p -1}{2}} = -1 \mod p$, then and only then $p$ is prime.*

We further specialize to the case where $p < 2^{254}$ and $k = 196$. This means that our prime has the following form in as a 256-bit binary number:

$$
0
\overbrace{c_{62} c_{61} \dots c_{0}}^{63}
\overbrace{0 0 0 0 0 0 0 \dots 0 0 0 0}^{195}
1
$$

or, represented using 64-bit words:

$$
c
0
0
1
$$

This special form will come in handy when we develop a Montgomery multiplication routine.

[Montgomery reduction](https://en.wikipedia.org/wiki/Montgomery_modular_multiplication) is an efficient way to compute $\frac{x}{2^{256}} \mod p$ where $x < p\cdot2^{256}$:


$$
M = -p^{-1} \mod 2^{64} = -1
$$

Since $p = 1 \mod 2^{64}$.

1. For i in 0 to 3:
3. $A = A + \left[-a_i \right]_{2^{64}} \cdot m \cdot (2^{64})^i$.

$$
\begin{matrix}
a_7 & a_6 & a_5 & a_4 & a_3 & a_2 & a_1 & a_0 \\\\
& & & & & & & -a_0 \\\\
& & & (-a_0 m_3)_1 & (-a_0 m_3)_0 & & & \\\\
& & & & & & -a_1' & \\\\
& & (-a_1' m_3)_1 & (-a_1' m_3)_0 & & & & \\\\
& & & & & -a_2'' & & \\\\
& (-a_2'' m_3)_1 & (-a_2'' m_3)_0 & & & & & \\\\
& & & & -a_3''' & & & \\\\
(-a_3''' m_3)_1 & (-a_3''' m_3)_0 & & & & & & \\\\
\end{matrix}
$$

Note, all negatives like $-a_0$ are to be taken in twos complement, so $2^{64} - a_0$.

Note that $a_1' = a_1 + 1$ and $-a_1'$. Instead of adding one and negating, we can take the binary complement (not) of the bits.

We can ignore computing the first three limbs altogether.

$-a_1' = !a_1$

$-a_2'' = !a_2$

$a_3''' = a_3 - a_0 m_3 + 1$

TODO: Above observations only hold if $a_0, a_1', a_2'', a_3'''$ non-zero

---

**To do.** The carry of $a_0 + [- a_0]_{2^{64}}$ is always one, except when $a_0$ is zero. What happens if we assume the carry is always one? In other words, we sometimes add an additional $2^{64}$ to $a_0$.

The mechanic of setting the lower bits to zero would still work, but we would substitute $2^{64} - a_0$ for $- a_0$. This means we add an additional $m_3$ to $a_4$, only if $a_0 = 0$.

$$
\begin{matrix}
a_7 & a_6 & a_5 & a_4 & a_3 & a_2 & a_1 & a_0 \\\\
& & & & & & & -a_0 \\\\
& & & (-a_0 m_3)_1 & (-a_0 m_3)_0 & & & \\\\
& & & m_3 & & & & \\\\
\end{matrix}
$$

The same goes for the other three limbs. The net affect is also that the final result is no longer within 2x of the modulus, but 3x.

Another option is to speculatively use (or not use) the carry, and correct after.

Not using:

$$
\begin{matrix}
a_7 & a_6 & a_5 & a_4 & a_3 & a_2 & a_1 & a_0 \\\\
& & & & & & & -a_0 \\\\
& & & (-a_0 m_3)_1 & (-a_0 m_3)_0 & & & \\\\
& & & & & & -a_1 & \\\\
& & (-a_1 m_3)_1 & (-a_1 m_3)_0 & & & & \\\\
& & & & & & -1 & & c_1\\\\
& & & -m_3 & & & & & c_1\\\\
\end{matrix}
$$

Using:

$$
\begin{matrix}
a_7 & a_6 & a_5 & a_4 & a_3 & a_2 & a_1 & a_0 \\\\
& & & & & & & -a_0 \\\\
& & & (-a_0 m_3)_1 & (-a_0 m_3)_0 & & & \\\\
& & & & & & -a_1 & \\\\
& & (-a_1 m_3)_1 & (-a_1 m_3)_0 & & & & \\\\
& & & & & & 1 & & c_1\\\\
& & & m_3 & & & & & c_1\\\\
\end{matrix}
$$


When do carries occur?
* $a_0$ will carry if $a_0 \ne 0$.
* $a_1$ will carry if $a_0 \ne 0 \land a_1 \ne -1$ or $a_0 = 0 \land a_1 \ne 0$.
* $a_2$ will carry if ...
