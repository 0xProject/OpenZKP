# Montgomery ladder

https://www.ams.org/journals/mcom/1987-48-177/S0025-5718-1987-0866113-7/S0025-5718-1987-0866113-7.pdf

https://arxiv.org/pdf/1703.01863.pdf

See: https://www.hyperelliptic.org/EFD/g1p/auto-montgom-xz.html#ladder-mladd-1987-m

Montgomery curve:

$$
by^2 = x^3 + ax^2 + x
$$

where $b(a^2-4)$ is non-zero.

Additional constraints helps with addition formulas:

$$
4a^{24}=a+2
$$


The following constraint provides completeness guarantees for some doubling and addition formulas:

$$
a^2 - 4 \text{ not a square}
$$


## Doubling formula

$$
x' = \frac{(x^2-1)^2}{4x(x^2+a\cdot x+1)}
$$

Or, written out in full:

$$
x' = \frac{x^4 -2x^2  +1}{4x^3+4ax^2 + 4x}
$$

```
XX1 = X1^2
t0 = XX1-1
X3 = t0^2
t1 = a*X1
t2 = XX1+t1
t3 = t2+1
t4 = X1*t3
Z3 = 4*t4
```

Plus a final `X3 /= Z3` inversion.

## Differential Addition

## Application to short Weierstrass form

From: https://safecurves.cr.yp.to/ladder.html

> One can sometimes convert a short Weierstrass curve y^2=x^3+ax+b to a Montgomery curve as follows. Find r satisfying r^3+ar+b=0. Find s satisfying s^2=3r^2+a. Define u=(x-r)/s, B=1/s^3, and A=3r/s. Then By^2=u^3+Au^2+u. One can perform x-coordinate scalar multiplication on y^2=x^3+ax+b by converting x to u, performing u-coordinate scalar multiplication on By^2=u^3+Au^2+u with the Montgomery ladder, and converting back.
>
> The reason this does not always work is that, for the majority of curves, the field F_p does not contain suitable elements r and s. One can work around this by replacing F_p with an extension field, but this requires much less simple field operations inside scalar multiplication.
>
> In particular, curves of prime order or 2*prime order can never be converted to Montgomery curves over F_p: Montgomery curves always have order divisible by 4.


# Brier–Joye ladder

A generic ladder work for any curve.

http://joye.site88.net/papers/BJ02espa.pdf

### Doubling

Eq. 7 from the above paper:

$$
x' = \frac{(x^2-a)^2-8bx}{4(x^3+ax+b)}
$$

Or in rational form:

$$
x' = \frac{x^4 -2ax^2-8bx+a^2}{4x^3+4ax+4b}
$$

### Addition

Give $P = (x_1, y_1)$, $Q = (x_2, y_2)$ and $P-Q = (x,y)$ we can compute $P+Q = (x', y')$:

Taking eq. 6 from the above paper:

$$
x' = \frac{-4b(x_1 + x_2) + (x_1x_2 - a)^2}{x(x_1 - x_2)^2}
$$

Or in rational form:

$$
x' = \frac{-4bx_1 + -4bx_2 + x_1^2x_2^2 - 2a x_1x_2 + a^2}{xx_1^2 + x x_2^2 - 2x x_1 x_2}
$$
