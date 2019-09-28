# Stark proof

## Input

* A `ConstraintSystem` which captures the claim that is made.
* A `TraceTable` which is the witness to this claim.
* A `ProofParams` object which configures the proof.

## Output

* A `ProverChannel`.

## Proof construction

A new `ProverChannel` is initialized with the public input.

### Step 1: Low degree extension of the trace table.

The trace table is interpolated to an evaluation domain that is larger by a factor `params.blowup`. It is also offset by a cofactor (currently fixed to the default generator of the field, `3`).

$$
T_{i, j} = P_j(\omega_{\text{trace}}^i)
$$

<!-- TODO: Introduce trace table -->

A merkle tree is constructed over this evaluation domain and commited to the channel.

$$
\text{Leaf}_i = (T_0(x_i), T_1(x_i), \dots )
$$

where $x_i = 3 \cdot \omega_{\mathrm{lde}}^i$.

<!-- TODO: The indices should be bit-reversed. -->

### Step 2: Constraint commitment

For each constraint, two random value $\alpha_i$ and $\beta_i$ are drawn from the channel. The constraints are combined as

$$
C(x) = \sum_i (\alpha_i + \beta_i \cdot x^{d_i}) \cdot C_i(x)
$$

<!-- TODO: Introduce constraints -->

where $d_i$ is the adjustment degree,

$$
d_i = \mathrm{target\\_degree} - \deg C_i
$$

The adjustment degrees are there to prevent make sure that the final polynomial is a sum of all constraint polynomials aligned on the lowest coefficient, and on the highest coefficient. This guarantees that constraint degrees are enforced exactly. (Non-enforcement on the low end would mean a term of negative degree $x^{-1}$ would be accepted).

<!-- TODO: Introduce target degree -->

The resulting polynomial $C$ is now split in $\mathrm{d}$ polynomials such that

$$
C(x) = A_0(x^{\mathrm{d}}) + x \cdot A_1(x^{\mathrm{d}}) + x^2 \cdot A_2(x^{\mathrm{d}}) + \cdots + x^{{\mathrm{d}} -1}\cdot A_{\mathrm{d}}(x^{\mathrm{d}})
$$

where $\deg A_i \leq \text{trace\_length}$.

For a linear constraint system this does nothing and we have $A_0 = C$, for a quadratic constraint system $A_0$ and $A_1$ contain all the odd and even coefficients of $C$ respectively.

A merkle tree is constructed over the LDE values of the $A$ polynomials and commited to the channel.

$$
\text{Leaf}_i = (A_0(x_i), A_1(x_i), \dots )
$$

### Step 3: Divide out the deep points and combine

A random value $z$ is drawn from the channel.

For each trace polynomial, $T_i(z)$ and $T_i(\omega \cdot z)$ are written to the proof. For each combined constraint polynomial, $A_i(z^{\mathrm{d}})$ is written to the proof.

The points are then divided out of the polynomials, with each trace polynomial being treated twice:

$$
T_i'(x) = \frac{T_i(x) - T_i(z)}{x - z}
$$

$$
T_i''(x) = \frac{T_i(x) - T_i(\omega \cdot z)}{x - \omega \cdot z}
$$

Similarly for the constraint polynomials:

$$
A_i'(x) = \frac{A_i(x) - A_i(z^{\mathrm{d}})}{x - z^{\mathrm{d}}}
$$

For each trace polynomial, two random values $\alpha_i$ and $\beta_i$ are drawn from the channel. For each constraint polynomial, one random value $\gamma_i$ is drawn.

All polynomial are combined in a single final polynomial:

$$
P(x) = \sum_i \left( \alpha_i \cdot T_i'(x) + \beta_i \cdot T_i''(x)\right) + \sum_i \gamma_i \cdot A_i'(x)
$$

<!-- TODO: Mention degree bounds on polynomials. Wouldn't this be -2 because of the divided out points? -->

### Step 4: Create FRI layers

The final polynomial $P$ is evaluated on the LDE domain. A Merkle tree is constructed of these values and committed to the proof.

A random value $\alpha$ is drawn from the channel. Take $P_0$ to be our final polynomial, then

$$
P_{i+1}(x^2) = \left( P_i(x) + P_i(-x) \right) + \frac{\alpha}{x} \left( P_i(x) - P_i(-x) \right)
$$

This is the same as taking all the odd coefficients, multiplying them by $\alpha$ and adding them to the even coefficients.

This reduction step can be repeated using $\alpha^2, \alpha^4, \dots$ instead of $\alpha$. Once sufficient reductions are made, a new Merkle tree is constructed, committed too, a new random value $\alpha$ is drawn and the FRI layering process repeats.

The number of reduction steps in between each commitment is specified using the `params.fri_layout` parameter. The default recommendation is to do three reductions between each layer, as this optimizes proof size.

Once the degree of the polynomial is sufficiently low, it is written to the channel in coefficient form.

### Step 5: Proof of work

A random challenge is drawn from the channel and a proof of work is solved. The solution is written to the channel. The difficulty is specified by the `params.pow_bits` parameter.

### Step 6: Decommit queries

Random values $x_i$ from the LDE domain are drawn from the channel to form our queries. The total number of queries is specified by `params.queries`. The values are sorted.

<!-- TODO: Sorted by bit-reversed index -->

The trace polynomial values at the query locations are written to the channel:

$$
T_0(x_0), T_1(x_0), \dots, T_0(x_1), T_1(x_1), \dots
$$

A merkle proof is provided linking these values to the earlier commitment.

Similarly, the combined constraint polynomial values are written to the channel:

$$
A_0(x_0), A_1(x_0), \dots, A_0(x_1), A_1(x_1), \dots
$$

And again a merkle proof is provided linking these values to the earlier commitment.

Then the values of the final polynomial are provided:

$$
P(x_0), P(x_1), \dots
$$

the merkle proof for these values links them to the commitment at the start of the FRI layer.

Now the set of points $x_i$ is squared while maintaining the sorted order. Duplicate points are removed. This is repeated unit we reach the reduction for the next FRI commitment.

Values for the next committed FRI layer are provided:

$$
P_i(x_0), P_i(x_1), \dots
$$

with merkle proofs to that layer. This process is repeated for all FRI layer commitments.
