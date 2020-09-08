## Composition

**Context.** We are given a prime field $\F_p$, a size parameter $n$ and the following:

* $\log_2 p > 250$ (the prime is of *cryptographic size*),
* $2^n \,\vert\, p - 1$ (the field has roots of unity $\omega_n$).
* $n = 2^k < 2^{28}$ (we can realistically compute operations that ar $O(n \log n)$, but not $O(n ^2)$).

**Note.** If it helps, you may assume additional restricions on the prime $p$, for example the existance of other roots of unity. (Depending on how common these primes are, the result may no longer be relevant in pairing cryptography, but it will still be relevant for FRI and DARK based constructions). You may even outright pick a prime.

We denote the set of polynomials of degree less than $k$ as $\F_{< k}[X]$.

As is common in Cryptography, we accept probabilistic results as long as the probability is larger than $1 - 2^{\lambda}$, where $\lambda$ is the number of *bits of security*. We typically require at least $\lambda > 80$, but higher is better.

Finally, we are working in interactive proofs where there is dialogue between a prover and a verifier. Given a polynomial $P \in \F_{< n}$ we have existing protocols that allow us to send an *oracle* for this polynomial to the verfier, and the verifier can then ask for evaluations of these polynomials. Note that this existing protocol only works for polynomials of degree less than $n$.

---

**Lemma.** (Polynomial identity test). Given two polynomials $P, Q \in \F_{< n}[X]$ we want to convice the verifier that they are equal. The prover sends oracles for $P$ and $Q$ to the verifier, the verifier responds by asking for evaluations on a random point $\alpha$ and checks that they are equal.

**Proof.** By the Schwartz-Sippel lemma.

---

**Problem.** (Efficient zeros). An *efficiently evaluable polynomial* of degree $n$ has an $\O(\log n)$ sized circuit of $+$ or $\times$ operations that evaluates it. We are looking for efficiently evaluable polynomials that have as roots a subbset of the roots of unity $\omega_n^i$.

For example $X^n -1$ is efficiently evaluable and has all the roots of unity as roots. $X^{n/2} -1$ is efficient and has a roots all even powers of $\omega_n^i$.

---

**Problem.** (Composition) Given $F,G,H \in \F_{< n}[X]$, we want to interactively proof that

$$
F(G(X)) \\!\\!\\!\\!\mod H(X) = 0
$$

We can send *oracles* to the verifier, but only for polynomials in $\F_{\le n}$.

The current proof protocols use

$$
F(G(X)) = H(X) Z(X)
$$

and then writes

$$
Z(X) = Z_0(X^n) + X \cdot Z_1(X^n) + \cdots + X^n Z_n(X^n)
$$

And sends oracles for $Z_0 \dots Z_n$ and uses identity testing. The problem is this uses $n$ operations, and we'd like a protocol that uses at most $\O(\log n)$ exchanges.

**Restrictions.** Feel free to assume any of

* $H(X) = X^n -1$ or any other efficient polynomial.
* $F(X) = X^n -1$ or any polynomial.

**Generalizations.** Same problem, but now with $F$ a multivariate over multiple $G$

$$
F(G_0(X), G_1(X), G_2(X), \dots, G_) = H(X) Z(X)
$$

---

Solution by *Reid Barton* for $F$ an efficient polynomial.

$$
\begin{aligned}
G_1(X) - G(X)^2 &= H(X) Z_1(X) \\\\
G_2(X) - G(X)^2 &= H(X) Z_2(X) \\\\
& \,\,\,\vdots \\\\
G_k(X) - G(X)^2 &= H(X) Z_k(X) \\\\
\end{aligned}
$$

## Polynomial composition

Both approaches above hit the problem that we need to low-degree-test an expression of the form $P(Q(X))$. The straightforward approach has degree bound $(\deg P)(\deg Q)$ which is quadratic. This will kill prover performance and more than double proof size.

**Goal.** *Find a way to LDE test polynomial compositions efficiently.*

**To do.** Specify goal more concretely.

https://en.wikipedia.org/wiki/Polynomial_decomposition

https://www.math.mcgill.ca/rickards/PDFs/amer.math.monthly.118.04.358-rickards.pdf

https://www.cs.cornell.edu/~kozen/Papers/poly.pdf

Chebyshev polynomials have the nesting property $T_n(T_m(X)) = T_{mn}(X)$.

https://dspace.mit.edu/bitstream/handle/1721.1/71792/Kedlaya-2011-FAST%20POLYNOMIAL%20FACT.pdf?sequence=1&isAllowed=y
https://www.cse.iitk.ac.in/users/nitin/courses/CS681-2016-17-II/pdfs/slides-dwivedi.pdf
Evaluates $f(g(x)) \mod h(x)$ in less then $\mathcal O(n^2)$ time using FFTs. => Read.
http://users.cms.caltech.edu/~umans/papers/U07.pdf

https://www.cse.iitk.ac.in/users/nitin/courses/CS681-2016-17-II/pdfs/slides-dwivedi.pdf

https://citeseer.ist.psu.edu/viewdoc/download;jsessionid=611B98690C1028968AED2736F9E1E77C?doi=10.1.1.51.3154&rep=rep1&type=pdf

https://arxiv.org/pdf/0807.3578.pdf


---


Aside: https://en.wikipedia.org/wiki/Bruun's_FFT_algorithm

## Composition proof

$$
\def\F{\mathbb F}
$$

**Context.** Are values are from a 

**Goal.** *Given $P\in\F_{< N}[X]$ Proof the following constraint using Schwartz-Sippel and a $\le N$ low degree test:*

$$
F(G(X)) \mod H(X) = 0
$$



$$
\frac{P(X)^{2^{64}} - 1}{X^N - 1}
$$

**Lemma.** *Polynomials satisfying the above constraint satisfy $P(\omega_N^i) = \omega_{2^{64}}^{k_i}$ for $i \in [0,N)$.*

**Definition.** *$\F_{< N}[X] \sim \F[X]/ X^N$*

---

Assume for the moment $N = 2^{64}$, we can generalize later

$$
\frac{P(X)^N - 1}{X^N - 1}
$$

This puts some constraints on the coefficients of $P$.

$$
P(X) = \sum_{i\in[0,N)} p_i X^i
$$

$$
\frac{P(X)^N - 1}{X^N - 1} =
\frac{\left(\sum_{i\in[0,N)} p_i X^i\right)^N - 1}{X^N - 1}
$$

We can now expand using something like the https://en.wikipedia.org/wiki/Binomial_theorem, i.e. https://en.wikipedia.org/wiki/Multinomial_theorem.

**To do.** Look into differentials.

### Simplifications

We can further restrict the problem to the case $h(x) = x^n - 1$. This can help because we can now factor $h$ as

$$
h(x) = (x - ω_n^0) (x - ω_n^1) (x - ω_n^2) \cdots (x - ω_n^{n-1})
$$

or (thanks Yan Zhang)

$$
h(x) = (x - 1) (x + 1) (x^2 + 1) (x^4 + 1) \cdots (x^n + 1)
$$

The problem could be broken solved modulo these factors and then invoke CRT.

---

### References

* J. F. Ritt (1921). "Prime and Composite Polynomials".
  [link](https://www.ams.org/journals/tran/1922-023-01/S0002-9947-1922-1501189-9/S0002-9947-1922-1501189-9.pdf).
* Raoul Blankertz (2014). "A polynomial time algorithm for computing all minimal decompositions of a polynomial"
  [link](https://web.archive.org/web/20150924101735/http://www.sigsam.org/bulletin/articles/187/Polynomial_time_decomposition_pp13-23.pdf)
* K. S. Kedlaya, C. Umans (2011). "Fast polynomial factorization and modular composition"
 [link](http://users.cms.caltech.edu/~umans/papers/KU08-final.pdf)
