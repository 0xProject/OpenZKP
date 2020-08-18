## Efficient patterns of zeros of certain polynomials

### Context

In STARKs the computation is unrolled into a table:

| $x$ | $P_1(x)$ | $P_2(x)$ | $\dots$ | $P_n(x)$ |
|----|----|----|----|----|
| $\omega^0$ | a | b | $\cdots$ | c |
| $\omega^1$ | d | e | $\cdots$ | f |
| $\vdots$   | $\vdots$ | $\vdots$ | $\ddots$ | $\vdots$ |
| $\omega^{n-1}$ | g | h | $\cdots$ | i |

Think of the columns as registers and the rows as sequential states in the computation.

The columns in table are interpreted as polynomials evaluated on powers of a root of unity $\omega$.

Constraints are now expressed as rational functions. Consider the constraint that $P_2$ is the sum of previous $P_1$ and $P_2$ (Fibonacci constraint):

$$
\frac{
    P_2(\omega\cdot x) - P_1(x) - P_2(x)
}{
    (x^n - 1)/(x - \omega^{n-1})
}
$$

The numerator is an expression that is zero whenever the constraint holds between two rows. The denominator is zero whenever the constraint *should* hold. The division is exact only when the trace table is valid.

In STARKs, the verifier needs to evaluate the constraints once, given values of $P_i$. For an efficient proof system, this the expression should be evaluable in complexity at most logarithmic in $n$.

The above can be evaluated in $O(\log n)$ using binary exponentiation.

### Statement

Consider a large prime field $\mathbb{F}_p$ with a $n = 2^k$ order root of unity $\omega$. That is, $\omega^n = 1$.

We are interested in constructing efficient polynomials which have their zeros at certain powers of this root. Efficient here means it can be evaluated in $O(\log(n))$ given arbitrary preprocessing.

Which patterns of zeros can be efficiently computed?

### Positive examples

* $(x - ω^i)$  is zero at $ω^i$. It can be evaluated in $O(1)$.
* $(x^n - 1)$ is zeros at all powers of $ω$. It is efficient because with repeated squaring it can be evaluated in $O(log(n))$.
* $(x^{n/m} - 1)$ is zero at every $m$-th power of $ω$.

### Combinators

* $A(x) \cdot B(x)$ is zero whenever $A$ or $B$ is zero. This is efficient if $A$ and $B$ are. Be careful that overlap changes multiplicity.
* $A(x) / B(x)$ is zero whenever $A$ is zero, except when $B$ is zero assuming the multiplicity of the zeros is one.
* $A(ω^i x)$ has all zeros moved by $i$ places.

### Open questions

Is there an efficient evaluation of the polynomial that is zero at $\omega^0, \dots, \omega^{n/2}$, i.e. only the first half of the table.

What about arbitrary ranges?

### Prelimiary results

* Reed-Solomon theory will explain which patterns result in sparse polynomials. The example problem will result in a dense polynomial. Takeaway: the example problem will require a clever way to evaluate a dense polynomial. We know these exist for some polynomials. (from discussion with Dimitry)

* The general problem without precomputation is unsolvable. As we let $N$ grow to infinity, the number of patterns grows $2^n$, but the number of efficient evaluation circuits grows as $\log n$. This assumes we don't use constants besides $1$. Takeaway: Any generic solution will require more than $\log n$ precomputation for the constants. (from discussion with Dan)

### Appendix: why is this field interesting

Arithmetic circuits are a generalization of boolean circuits. Finite fields offer richter math than booleans and some researcher believe this has opertunities for solving hard complexity theory problems.

I also believe the results in the field are not as widely known as they should be, for example, many people believe Horner's evaluation is optimal. Given a polynomial

$$
P(x) = c_0 + c_1 x^1 + c_2 x^2 + \cdots  + c_n x^n
$$

Horner's schema allows this to be evaluated using $n$ multiplications and $n$ additions:

$$
\begin{aligned}
p_0 &= 1 \\\\
p_{i+1} &= p_{i} \cdot x + c_{n-i} \\\\
P(x) &= p_n
\end{aligned}
$$

Which takes $n$ multiplications. But [Rabin-Winograd (1972)][rw72] showed that any polynomial can be evaluated in $\frac{n}{2} + \log n$ multiplications. In the following, assume $n = 255$. The method generalizes for arbitrary degree in the paper.

[rw72]: https://doi.org/10.1002/cpa.3160250405

We first turn $P$ monic by dividing by the leading term, in the end we will multiply by it again:

$$
c'_i = \frac{c_i}{c_n}
$$

$$
P(x) = c_n (c'_0 + c'_1 \cdot x + c'_2 \cdot x^2 + \dots + x^n)
$$

Now we pick $i = \frac{n + 1}{2} = 128$ and split the polynomial in two:

$$
P(x) = Q(x) \cdot (x^{128} + a) + R(x)
$$

Where $Q$ and $R$ are monic with degree $127$. The coefficients of $Q$ are simply $c'_{129}, \dots, c'_{255}$. The value of $a$ is $c_{128} - 1$. From this we can compute the coefficients of $R$, they are $r_i = c'_i - c \cdot q_i$ and $r_{127} = 1$.

We apply this splitting recursively untill we are left with many monic polynomials of degree 3: $S(x) = s_0 + s_1 x + s_2 x^2 + x^3$. These are computed using:

$$
S(x) = (x^2 + s_1) \cdot (x + s_2) + b
$$

where $b$ is $s_0 - s_1 s_2$. All in all, the method requires about $n/2$ multiplications, about half of Horners method. To this we need to add the $\log n$ squarings required to get the required powers of $x$.


---

With $N$ a power of two:

$$
P(X) = (X - \omega_N^0) (X - \omega_N^1) \cdots (X - \omega_N^{N/2})
$$

$$
P(X) = (X - 1) (X - \omega_N)(X - \omega_N^2) \cdots (X + 1)
$$

**Conjecture.** *In $\mathbb C$ the coefficients of $P$ alternate between real multiples of $\omega_4^0, \omega_4^1, \omega_4^2, \omega_4^3$.*

**Conjecture.** *Furthermore, the scalar factors are symmetric in the sens that $c_i = c_{N/2 - i}$.*

**Conjecture.** *Furthermore, the scalar factors follow a bell curve.*

**To do.** Propose concrete formula for coefficients.

**To do.** This seems to generalize to composite $N$.


*Symmetries*: Take $P(X)$ to be the polynomial with zeros on $\omega_N^0, \dots \omega_N^{N/2}$. It should have the following symmetries based on observations of patterns of roots:

* Translation by one

    $$
    (X - \omega_N^0) P(\omega_N X) = (X - \omega_N^{N/2 + 1}) P(X)
    $$

    and observing that $\omega_N^{N/2 + 1} = \omega_N \cdot \omega_N^{N/2} = -\omega_N$:

    $$
    (X - 1) P(\omega_N X) = (X + \omega_N) P(X)
    $$


* Complement.

    $$
    P(X)P(-X) = (X+1)(X-1)(X^N -1)
    $$

    $$
    P(X)P(-X) = (X^2-1)(X^N -1)
    $$

* (Conjectured)

    $$
    P(X) = P\left(\frac{\omega_N}{X}\right)X^{N/2}
    $$



---

<https://helper.ipam.ucla.edu/publications/ccgtut/ccgtut_11787.pdf>

<https://en.wikipedia.org/wiki/Vieta%27s_formulas>

<https://en.wikipedia.org/wiki/Gauss%E2%80%93Lucas_theorem>

<https://en.wikipedia.org/wiki/Geometrical_properties_of_polynomial_roots>

<https://en.wikipedia.org/wiki/Polynomial_decomposition>

---

Decomposition:

**To do**. Try decomposition methods. Observe that $X^{2^k}-1 = (X^2 -1) \circ (X^2) \circ \cdots \circ (X^2)$. The rhs can be evaluated in $O(k)$ operations.

<https://web.archive.org/web/20150924101735/http://www.sigsam.org/bulletin/articles/187/Polynomial_time_decomposition_pp13-23.pdf>

<https://arxiv.org/pdf/1107.0687.pdf>

Does not seem to work for small examples: <https://www.wolframalpha.com/input/?i=Decompose%5B%28x-i%29%28x-1%29%28x%2B1%29%2C+x%5D even though $X^4-1$ works https://www.wolframalpha.com/input/?i=Decompose%5B%28x-i%29%28x-1%29%28x%2B1%29%28x%2Bi%29%2C+x%5D>

---

Interpolation

$$
\begin{aligned}
P(z) &= 
\sum_i a_i ⋅ L_i(z) \\\\&=
\sum_i a_i ⋅ \frac{ω^i}{n} \frac{z^n - 1}{z - ω^i} \\\\&=
\frac{z^n - 1}{n} \sum_i \frac{a_i ⋅ ω^i}{z - ω^i}
\end{aligned}
$$

where $a_i = \begin{cases} 0 & i \le N/2 \\ \ne 0 & otherwise \end{cases}$.

$$
\frac{z^n - 1}{n} \sum_{i \in (N/2, N)} \frac{a_i \cdot ω^i}{z - ω^i}
$$

This reduces the problem to computing the sum in $\log N$ time, plus we are allowed to multiply each term of the sum by a nonzero factor of our choosing. First of, let's absorb the other factors in $a_i$:

$$
(z^n - 1) \sum_{i \in (N/2, N)} \frac{a_i}{z - ω^i}
$$

The integral related to the sum seems solvable: <https://www.wolframalpha.com/input/?i=Integrate%5B1%2F%28a-b%5Ex%29%2C+x%5D>

The series seems related to the digamma function: <https://en.wikipedia.org/wiki/Digamma_function#Evaluation_of_sums_of_rational_functions http://mathworld.wolfram.com/q-PolygammaFunction.html>

There is potentially a closed form solution, at least for complex numbers: 

<https://www.wolframalpha.com/input/?i=Sum%5B1%2F%28z-b%5Ei%29%2C+%7Bi%2C+n+%2C++2*n%7D%5D>

<https://en.wikipedia.org/wiki/Lambert_series>

