# Constraint systems

## Fibbonacci

```rust
a = 1
b = input

for i in 0..1024 {
    a' = b
    b' = a + b
}

return b
```

## Perderson Merkle

```rust
left_source = 0
left_slope = 0
left_point_x = 0
left_point_y = 0
right_source = 0
right_slope = 0
right_point_x = 0
right_point_y = 0

for i in (0..).stepsize(256) {

    let other_hash = U256::from(&private_input.path[path_index]);
    let (x, _) = get_coordinates(&state.right.point);
    if !private_input.directions[path_index] {
        state = initialize_hash(U256::from(x), other_hash);
    } else {
        state = initialize_hash(other_hash, U256::from(x));
    }

    for j in (1..256) {
        // Copy previous right to left
        left_point_x' = right_point_x
        left_point_y' = right_point_y

        // Iterate bits on the left
        left_source' = left_source / 2
        if left_source % 1 == 1 {

            // Add constant point
            left_slope' = // (y_1 - y_2) / (x_1 - x_2)
            left_point_x' = // ...
            left_point_y' = // ...
        }

        // Copy left to right
        right_point_x' = left_point_x'
        right_point_y' = left_point_y'

        // Iterate bits on the right
        right_source' = right_source / 2
        if right_source % 1 == 1 {

            // Add constant point
            right_slope' = 
            right_point_x' = 
            right_point_y' = 
        }
    }
}
```
