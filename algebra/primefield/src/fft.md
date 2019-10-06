
# DFT

The Number Theoretic Transform $b_i$ of an sequence input sequence $a_i$ is given by:

$$
b_i = \sum_j a_j \omega_n^{ij}
$$

**Observation.** If we interpret $x_i$ as coefficients in a polynomial $P(x) = a_0 + a_1 x + a_2 x^2 + \dots$, then the NTT is $b_i = P(\omega_n^I)$.

**Observation.** If we interpret $x_i$ as a vector, it is related to $y_i$ by the DFT matrix (also a Vandermonde matrix):

$$
\begin{pmatrix}
b_0 \\ b_1 \\ \vdots \\ b_{n-1}
\end{pmatrix}
=
\begin{pmatrix}
1 & 1 & 1 & \cdots & 1 \\
1 & \omega_n & \omega_n^2 & \dots&  \omega_n^{n-1} \\
\end{pmatrix}
\begin{pmatrix}
a_0 \\ a_1 \\ \vdots \\ a_{n-1}
\end{pmatrix}
$$

**Observation.** If we interpret $a_i$ as residues of some polynomial $P$ modulo $(x - \omega_n^i)$ then by the Chinese Remainder Theorem the values $b_i$ are the coefficients of $P$ modulo $(x^n - 1)$.

# Cooley-Tukey



$$
\begin{pmatrix}
x_0 \\ x_1 \\ \vdots \\ x_{30}
\end{pmatrix}
$$

Factor $30 = 5 * 6$ and transform matrix:

$$
\begin{pmatrix}
x_0 & x_1 & x_2 & x_3 & x_4 & x_5 \\
x_0 & x_1 & x_2 & x_3 & x_4 & x_5 \\
x_0 & x_1 & x_2 & x_3 & x_4 & x_5 \\
x_0 & x_1 & x_2 & x_3 & x_4 & x_5 \\
x_0 & x_1 & x_2 & x_3 & x_4 & x_5 \\
\end{pmatrix}
$$

Apply DFT on each column.

Multiply by twiddle factors.

Apply DFT on each row.
