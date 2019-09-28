# Stark verify

## Input

* A `VerifierChannel` containing the proof.
* A `ConstraintSystem` which captures the claim that is made.
* A `ProofParams` object which configures the proof.

## Verification process

### Step 1: Read all commitments and draw random values

* Read the trace polynomial commitment commitment.
* Draw the constraint combination coefficients $\alpha_i$ and $\beta_i$.
* Read the combined constraint polynomial commitment.
* Draw the deep point $z$.
* Read the deep values of the trace polynomials $T_i(z)$ ,$T_i(\omega \cdot z)$.
* Read the deep values of the combined constraint polynomial $A_i(z^\mathrm{d})$
* Draw the coefficients for the final combination $\alpha_i$, $\beta_i$ and $\gamma_i$.
* Read the final polynomial commitment.
* Draw the FRI folding coefficient.
* Repeatedly read the FRI layer commitments and folding coefficients.
* Read the final FRI polynomial.

### Step 2: Verify proof of work

* Draw proof of work challenge.
* Read proof of work solution.
* Verify proof of work solution.

### Step 3: Read query decommitments

* Draw query indices
* Read evaluations of trace polynomial
  $T_0(x_0), T_1(x_0), \dots, T_0(x_1), T_1(x_1), \dots$
* Read and verify merkle decommitments for trace polynomial
* Read evaluations of the combined constraint polynomial
  $A_0(x_0), A_1(x_0), \dots, A_0(x_1), A_1(x_1), \dots$
* Read and verify merkle decommitments for combined constraint polynomial

### Step 4: FRI decommitments and final layer verification

<!-- TODO -->

### Step 5: Verify deep point evaluation

Using the disclosed values of $T_i(z)$ and $T_i(\omega \cdot z)$, compute the combined constraint polynomial at the deep point $C(z)$.

$$
C(z) = \sum_i (\alpha_i + \beta_i \cdot z^{d_i}) \cdot C_i(z, T_0(z), T_0(\omega \cdot z), T_1(z), \dots)
$$

Using the disclosed values of $A_i(z^{\mathrm{d}})$ compute $C(z)$.

$$
C'(z) = \sum_i z^i \cdot A_i(z^{\mathrm{d}})
$$

Verify that $C(z) = C'(z)$.

### Step 6: Compute first FRI layer values

Divide out the deep point from the trace and constraint decommitments

$$
T_i'(x_j) = \frac{T_i(x_j) - T_i(z)}{x_j - z}
$$

$$
T_i''(x_j) = \frac{T_i(x_j) - T_i(\omega \cdot z)}{x_j - \omega \cdot z}
$$

$$
A_i'(x_j) = \frac{A_i(x_j) - A_i(z^{\mathrm{d}})}{x_j - z^{\mathrm{d}}}
$$

and combine to create evaluations of the final polynomial $P(x_i)$.

$$
P(x_j) = \sum_i \left(\alpha_i \cdot T_i'(x_j) + \beta_i \cdot T_i''(x_j) \right) + \sum_i \gamma_i \cdot A_i'(x_j)
$$

### Step 7: Verify FRI proof

* Draw coeffient
* Reduce layer $n$ times
* Read and verify decommitments
* Repeat
* Evaluate the final layer

<!-- TODO: ellaborate FRI verification -->

