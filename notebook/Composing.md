# AIR Composibility

```rust
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

```rust
fn compose_horizontal(A: AirComponent, B: AirComponent) -> AirComponent {
    require(A.rows == B.rows);

    // result.trace = [A.trace | B.trace]
    // B.constraints.shift_columns(A.num_columns)
    // result.constraints = union(A.constraints, B.constraints)
    // names are prefixed `left_` and `right_`.
}
```

```rust
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

```rust
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

```rust
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

```rust
fn empty(rows: usize, cols: usize) -> AirComponent;
```

Using these, we can implement more complex operations:

```rust
fn fold_padded(A: AirComponent, repeats: usize) -> AirComponent {
    
}
```

```rust
fn fit_horizontal(A: AirComponent, B: AirComponent) -> AirComponent {
    // Same as compose_horizontal, but it will do whatever folds
    // and paddings are necessary to make A.rows == B.rows.
}
```


## Example

```rust

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
