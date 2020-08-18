# Composition of cyclic proof system

## Introduction

The single most important theorem in modern proof systems is the Schwarz-Zippel lemma, so let's introduce it now:

**Theorem.** *(Schwartz–Zippel)*


**Definition.** *(Proof system). Given a claim $c \in \mathcal C$ and witness $w \in \mathcal W$*.

* `setup`: $(Entropy, \lambda) \rightarrow K$
* `proof`: $(\mathcal C, \mathcal W) \rightarrow \mathcal P$.
* `verify`: $(\mathcal C, \mathcal W) \rightarrow \mathcal P$.

In the remainder we assume we are talking about a specific instance of a proof, meaning the claim and witness are fixed.


**Definition.** *(Algebraic expressions). $\F[\{X_0, X_1, \dots\}]$* Is this a free-semiring?

**Definition.** *(Polynomial proof system). Given a field $\F$*

Parameters: $\F$, $\F[X]$.

Setup polynomials: $Q_0(X), Q_1(X), \dots$. (**to do**)

Witness polynomials: $P_0(X), P_1(X), \dots, P_M(X)$.

Constraint expressions: $f_0(X, P_0, P_1, \dots, P_M), f_1(\dots)$.

**Definition.** *(Cyclic polynomial proof system). A polynomial proof system where for each witness polynomial $P_i$ there is a primitive root of unity $\omega_{N_i}$ of order $N_i = \deg P_i + 1$. Setup and witness polynomials are defined by a vector $\vec w_i \in \F^{N_i}$ and $P_i$ is the polynomial resulting from interpolation of the vector on $⟨ω_{N_i}⟩$ such that $P_i(\omega_{N_i}^j) = w_{ij}$. All access to $P_i$'s in the constraint expression is of the form $P_i(\omega_{N_i}^k X)$ with $k \in \Z$. This allows us to relate witness value $w_{i,j}$ with $w_{i,j + k}$.*

**Note.** In the case of starks, the witness vectors $\vec w_i$ are all of the same length, called the *trace length*. This allows the interpolated values $w_{ij}$ to be layed out in an $N \times M$ matrix, called the *trace table*. The witness vectors are called *columns* or *registers*.

**To do.** Generallize to expressions of the form $P_i(\omega^k X^e)$ which will allow relating $i$ with $e \cdot i + k$, which in turn may allow the $2i$, $2i + 1$ offsets required for Vitalik's data availability proof.

**To do.** Show how to generalize 

**Definition.** *(Degree of constraints). The maximum degree of $f_i$ in the arguments $P$ (so excluding the degree of $X$).

We can write repeating polynomial relationships between witness values.

**Note.** In practice we want to keep the degree of these relationships low, typically just $2$ or $3$. It is always possible to re-write higher degree constraint systems to an equivalent lower degree one by introducing new witness polynomials and new constraints.

**Definition.** *(Constraint values). Unconstraint values in the RS that can be set to anything.*

**Definition.** *(Empty proof). Proof of given size where all values are unconstraint.*

## Transformations and compositions

We will start by introducing some obvious invariants of proof systems. For starters, adding unused witness polynomials maintains the proof:

**Lemma.** *(Extension). Adding more unconstraint witness polynomials.*

We can also change the order of the witness polynomials:

**Lemma.** *(Permutation of witness polynomials). An RS proof system is invariant under a permutation of the order of the witness polynomials.*

We can also rotate things around

**Lemma.** *(Scaling). An RS proof system is invariant under multiplication of each constraint expression by a constant.*

**Lemma.** *(Rotation). An RS proof system is invariant under an affine transform $X \mapsto \omega_N X^i$ with corresponding rotation of the witness polynomials.*

**Lemma.** *(Rotation). An RS proof system is invariant under an affine transform $X \mapsto \omega_N^k X^i$ in constraint polynomials witness access and $X \mapsto \omega_N^{-k} X$ for the $X$ argument.*

**Lemma.** *(Interpolation). An RS proof system is invariant under an affine transform $X \mapsto X^k$ where $k \vert N$ and (**to do**).*

Combined, these rotation and interpolation have the effect of mapping $X=\omega_N^i$ to $X=\omega_N^{e \cdot i + k}$; it is an affine transform of the exponent with a corresponding inverse transform in the witness.

**Lemma.** *(Folding). An RS proof system with $M$ witness polynomials of degrees $n_i$ can be transformed into an RS proof system with $M/2$ polynomials of degree $2 n_i$.*

**To do.** Generalize to roots of unity other than binary powers of two.

**To do.** Generalize to roots of unity other than binary powers of two, allowing more complex foldings than halfing/doubling.

**Lemma.** *(Repeated folding). .*

---

**Lemma.** *(Projection). Given two RS proof systems such that the second fits in the unconstraint cells of the first, they can be merged.*

**Lemma.** *(Horizontal composition). Two proof systems can be composed horizontally.*

**Lemma.** *(Vertical composition). Two proof systems can be composed vertically if all their constraints are the same.*

The previous lemmas give us some leeway to make them the same.

**Lemma.** *(Matched horizontal composition). Can be composed horizontally.*

## Special cyclic polynomial proof systems

Vector equality

Set equality

Permutation check

Univariate rowcheck

Univariate sumcheck

Row check

Lincheck

## Example cyclic polynomial proof systems

MiMC/Poseidon/etc.

Plonk/RedShift

Aurora

Marlin

Fractal

## Overview of low degree tests

**Lemma.** *(Degree raising). Given a polynomial $P$ with degree bound $\deg P ≤ k$ and an LDE test for degree $N$ with $k ≤ N$, we can test the degree-bound of the polynomial.*

$$
P'(X) = (\alpha + \beta \cdot X^k) \cdot P(X)
$$

**Lemma.** *(Degree halving).*

$$
P(X) = P'(X^2) + X \cdot P''(X^2)
$$

**Note.** Degree halving doubles the number of polynomials to be LDE tested.

**Lemma.** *(Combining low degree tests).* 

$$
P'(X) = \sum_i \alpha_i P_i(X)
$$

Using the above three lemmas, a single low-degree test can verify arbitrary degree bounds on a set of polynomials.

### FRI

### Dark (on RSA or Classgroups)

### Kit commitment (Pairing)

# AIR Composibility

```rust,ignore
struct AirComponent {
    trace:       TraceTable,
    constraints: Vec<RationalExpression>,
    labels:      Vec<(String, RationalExpression)>
}
```

**Note.** Having labels on expressions allows direct labeling of trace cells  using `Trace(i, j)`. But it also allows labeling derived values, for example if the constraints are written such that the 'real' value is the difference of two columns, the labeled output can be `Trace(i, j) - Trace(i, k)`.

## Trace generation

**Alternative.** Instead of having the components supply the full trace table, we could also have them return a sparse table of $(i, j, value)$ tuples. The constraints would then be used to fill out the table. This may (or not) be more performant, but seems harder to implement. Perhaps instead we can provide helper functions to generate a trace table from constraints and sparse values.


## Air component combinators

```rust,ignore
fn compose_horizontal(A: AirComponent, B: AirComponent) -> AirComponent {
    require(A.rows == B.rows);

    // result.trace = [A.trace | B.trace]
    // B.constraints.shift_columns(A.num_columns)
    // result.constraints = union(A.constraints, B.constraints)
    // names are prefixed `left_` and `right_`.
}
```

```rust,ignore
fn compose_vertical(A: AirComponent, B: AirComponent) -> AirComponent {
    require(A.rows == B.rows);
    require(A.cols == B.cols);
    require(A.constraints == B.constraints)

    // result.trace = [ A.trace ]
    //                [ B.trace ]
    // A.constraints.repeat(2)
    // result.constraints = A.constraints
    // names are prefixed `top_` and `bottom_`.
}
```

```rust,ignore
fn compose_interleaved(A: AirComponent, B: AirComponent) -> AirComponent {
    require(A.rows == B.rows);
    require(A.cols == B.cols);

    // result.trace = [ A_0 ]
    //                [ B_0 ]
    //                [ A_1 ]
    //                [ B_1 ]
    //                [ ... ]
    // A.constraints.interleave(2);
    // B.constraints.interleave(2).shift(1);
    // result.constraints = union(A.constraints, B.constraints)
    // names are prefixed `odd_` and `even_`.
    
    // TODO: It is possible that a pair of constraints on odd/even
    // rows are the same and can be replaced by a single constraint
    // repeated.
}
```

```rust,ignore
fn fold(A: AirComponent) -> AirComponent {
    require(A.cols % 2 == 0);

    // result.trace = [ A_(0, 0...n) ]
    //                [ A_(0, n..2n) ]
    //                [ A_(1, 0...n) ]
    //                [ A_(1, n..2n) ]
    //                [     ...      ]
    // A.constraints.interleave(2).shift_half(1);
    // result.constraints = A.constraints
    // names are unchanged
}
```

**To do.** We can skip renaming if there are no collisions. This will have to be done on a global basis, not a per name, basis. This can lead to breakage when a subcomponent adds a new name.

## Further combinators

It's useful to add a helper function creating no-op components. This allows doing things like `fold(compose_horizontal(A, empty(A.rows, 1))` to do a fold where `A` has an odd number of columns.

```rust,ignore
fn empty(rows: usize, cols: usize) -> AirComponent;
```

Using these, we can implement more complex operations:

```rust,ignore
fn fold_padded(A: AirComponent, repeats: usize) -> AirComponent {
    
}
```

```rust,ignore
fn fit_horizontal(A: AirComponent, B: AirComponent) -> AirComponent {
    // Same as compose_horizontal, but it will do whatever folds
    // and paddings are necessary to make A.rows == B.rows.
}
```


## Example

```rust,ignore

fn transaction(
    initial_balances: Balances,
    txs: Vec<Transaction>
) -> (Balances, AirComponent) {
    let mut air = fit_horizontal(
        compose_vertical(
            compose_vertical(
                compose_horizontal(
                    merkle_proof()
                    .relabel("root", "old_maker_buy_root")
                    .relabel("leaf", "old_maker_buy_leaf"),
                    merkle_proof()
                    .relabel("root", "new_maker_buy_root")
                    .relabel("leaf", "new_maker_buy_leaf"),
                ),
                compose_horizontal(
                    merkle_proof()
                    .relabel("root", "old_maker_sell_root")
                    .relabel("leaf", "old_maker_sell_leaf"),
                    merkle_proof()
                    .relabel("root", "new_maker_sell_root")
                    .relabel("leaf", "new_maker_sell_leaf"),
                ),
            ),
            compose_vertical(
                compose_horizontal(
                    merkle_proof(),
                    merkle_proof(),
                ),
                compose_horizontal(
                    merkle_proof(),
                    merkle_proof(),
                ),
            ),
        ),
    );
    air.add_constraint(
        (air["old_maker_buy_root"] 
         - air["old_maker_sell_root"]
        ) * air.on_row(0)
    );
    air.relabel("old_maker_buy_root", "initial_balance_root");
    air.relabel("new_maker_sell_root", "final_balance_root");
    // TODO: Drop all other labels. Alternatively, replace the
    // set of labels in one operation.
}

fn starkdex(
    initial_balances: Balances,
    txs: Vec<Transaction>
) -> (Balances, AirComponent) {
    let mut component = txs
    .pad_to_power_of_two(EMPTY_TRANSACTION)
    .map(transaction)
    .binary_tree(compose_vertical)
    
    // TODO: Add constraints tying roots together.
}
```

---

```rust,ignore
struct Projection {

}

impl Projection {
    fn trace(usize, isize) -> RationalExpression;
}

trait Component {
    type Claim;
    type Witness;
    
    fn dimensions(&self) -> [usize];
    
    fn set_projection(projection: &[(usize, (usize, isize))]);
    
    fn constraints(&self, claim: &Claim) -> Vec<RationalExpression>;
    
    fn witness() -> Claim;
}
```
