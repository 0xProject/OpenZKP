# PLOOKUP

[Gabizon & Williamson (2020)](https://eprint.iacr.org/2020/315.pdf)

[Bootle, Cerulli, Groth, Jakobsen, & Maller (20180](https://link.springer.com/chapter/10.1007%2F978-3-030-03326-2_20)

**To do.** BCGJM contains many more constructions worth collecting.

Let $\vec t \in \F^d$ contain the lookup values.

and $\vec f \in \F^n$ the vector to test.

We want to test $\vec f ⊂ \vec t$, meaning all values of $f$ are in $t$.

$g$ a multiplicative generator of order $n+1$.

Define $f_i = f(g^i)$ and $f ∈ \F_{< n}[X]$.

$⟨g⟩$

When $\vec f ⊂ \vec t$ we say *f sorted by t* when the values appear in the same order.

Given $s ∈ \F^{ n+ d}$ define

$$
F(β, γ) = (1 + β)^n ⋅ \left( \prod^i_{[0,n]} (γ + f_i) \right) ⋅ \left(\prod^i_{[0,d)} γ ⋅ (1 + β) + t_i + β ⋅ t_{i+ 1} \right)
$$

$$
G(β, γ) = \prod^i_{[0, n + d)} \left( γ ⋅ (1 + β) + s_i + β ⋅ s_{i + 1} \right)
$$

$F = G$ iff

1. $\vec f ⊂ \vec t$
2. $s$ 


$F$ has zeros at

* $β = -1$, multiplicity $n + d$.
* $γ = -f_i$, multiplicity $n$.
* $γ ⋅ (1 + β) + t_i + β ⋅ t_{i+ 1} = 0$
  
$$
γ ⋅ (1 + β) + t_i + β ⋅ t_{i+ 1} = 0
$$

$$
t_i + β ⋅ t_{i+ 1} = -γ ⋅ (1 + β)
$$
