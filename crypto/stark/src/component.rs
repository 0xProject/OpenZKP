use crate::RationalExpression;
use crate::TraceTable;
use crate::primefield::FieldElement;
use std::convert::TryFrom;

// TODO: Introduce prover/verifier distinction

// OPT: Don't reallocate trace table so many times, instead use a view
// that is passed top-down for writing.

#[cfg_attr(feature = "std", derive(Debug))]
pub struct Component {
    trace:       TraceTable,
    constraints: Vec<RationalExpression>,
}

impl Component {

    /// Constructs an empty component of given size.
    /// 
    /// This is useful in combination with composition combinators to pad out a
    /// component to a required size.
    pub fn empty(rows: usize, columns: usize) -> Self {
        Component {
            trace:       TraceTable::new(rows, columns),
            constraints: Vec::new(),
        }
    }
}

// OPT: Use the degree of freedom provided by shift_x + shift_trace to
// minimize the number of trace values to reveal.

// OPT: In addition to this, we can also permute (rotate) the values in
// the trace table to add a further degree of freedom.

/// Change the order of the columns
///
/// `new_column_index = permutation[old_column_index]`
pub fn permute_columns(a: Component, permutation: Vec<usize>) -> Component {
    // Validate the permutation
    // TODO: Check that there are nu duplicate values in permutation.
    assert_eq!(a.trace.num_columns(), permutation.len());
    assert_eq!(permutation.iter().find(|&e| *e >= a.trace.num_columns()), None);

    // Create a new trace table that permutes the columns
    let mut trace = TraceTable::new(a.trace.num_rows(), a.trace.num_columns());
    for i in 0..trace.num_rows() {
        for j in 0..trace.num_columns() {
            trace[(i, permutation[j])] = a.trace[(i, j)].clone();
        }
    }
    
    // Permute the columns in the constraints
    use RationalExpression::*;
    let constraints = a.constraints.into_iter().map(|constraint| constraint.map(
        &mut |expression| match expression {
            Trace(column, offset) => Trace(permutation[column], offset),
            other                 => other,
        }
    )).collect(); 

    Component { trace, constraints }
}

/// Rotate around the row indices
/// 
/// `new_row_index = old_row_index + amount`
pub fn shift(a: Component, amount: isize) -> Component {
    // Normalize shift amount
    let amount_abs: usize = usize::try_from(
        amount.rem_euclid(isize::try_from(a.trace.num_rows()).unwrap())).unwrap();

    // Create a new trace table that permutes the columns
    let mut trace = TraceTable::new(a.trace.num_rows(), a.trace.num_columns());
    for i in 0..trace.num_rows() {
        for j in 0..trace.num_columns() {
            let new_row_index = (i + amount_abs) % trace.num_rows();
            trace[(new_row_index, j)] = a.trace[(i, j)].clone();
        }
    }

    // Adjust constraint system by replacing X
    // (alternatively, we could modify the Trace(...) offsets)
    use RationalExpression::*;
    let factor = FieldElement::root(trace.num_rows())
    .expect("No generator for trace length")
    .pow(-amount);
    let constraints = a.constraints.into_iter().map(
        |constraint| constraint.map(
            &mut |expression| match expression {
                X     => Constant(factor.clone()) * X,
                other => other,
            }
        )
    ).collect();

    Component { trace, constraints }
}

/// TODO: Reverse the order of the rows

/// Half the number of columns and double the number of rows.
/// 
/// Folds even columns into even rows and odd columns into odd rows.
/// 
/// **Note.** The number of columns is required to be even. To make it even,
/// you can horizontally compose with an empty component of size n x 1.
pub fn fold(a: Component) -> Component {
    assert_eq!(a.trace.num_columns() % 2, 0);

    // Create a new trace table that interleaves columns in odd and even rows
    let mut trace = TraceTable::new(2 * a.trace.num_rows(), a.trace.num_columns() / 2);
    for i in 0..a.trace.num_rows() {
        for j in 0..a.trace.num_columns() {
            let row = j % 2;
            let col = j / 2;
            trace[(2 * i + row, col)] = a.trace[(i, j)].clone();
        }
    }

    // Adjust constraint system by interpolating and changing the columns
    use RationalExpression::*;
    let constraints = a.constraints.into_iter().map(
        |constraint| constraint.map(
            &mut |expression| match expression {
                Trace(i, j) => {
                    let row = i % 2;
                    let col = i / 2;
                    Trace(col, 2 * j + isize::try_from(row).unwrap())
                },
                other => other,
            }
        )
    ).collect();

    Component { trace, constraints }
}

pub fn compose_horizontal(a: Component, b: Component) -> Component {
    assert_eq!(a.trace.num_rows(), b.trace.num_rows());

    // Create a new trace table that horizontally concatenates a and b
    let mut trace = TraceTable::new(a.trace.num_rows(), a.trace.num_columns() + b.trace.num_columns());
    for i in 0..a.trace.num_rows() {
        for j in 0..a.trace.num_columns() {
            trace[(i, j)] = a.trace[(i, j)].clone();
        }
        for j in 0..b.trace.num_columns() {
            trace[(i, j + a.trace.num_columns())] = b.trace[(i, j)].clone();
        }
    }

    // Shift b's constraints over by a's columns.
    use RationalExpression::*;
    let a_cols = a.trace.num_columns();
    let mut constraints = a.constraints;
    constraints.extend(b.constraints.into_iter().map(|constraint|
        constraint.map(&mut |expression| {
            match expression {
                Trace(i, j) => Trace(i + a_cols, j),
                other => other,
            }
        })
    ));

    Component { trace, constraints }
}

pub fn compose_vertical(a: Component, b: Component) -> Component {
    assert_eq!(a.trace.num_rows(), b.trace.num_rows());
    assert_eq!(a.trace.num_columns(), b.trace.num_columns());
    // TODO: assert_eq!(a.constraints, b.constraints);

    // Create a new trace table that vertically concatenates a and b
    let mut trace = TraceTable::new(2 * a.trace.num_rows(), a.trace.num_columns());
    for i in 0..a.trace.num_rows() {
        for j in 0..a.trace.num_columns() {
            trace[(i, j)] = a.trace[(i, j)].clone();
            trace[(i + a.trace.num_rows(), j)] = b.trace[(i, j)].clone();
        }
    }

    // Repeat a's constraints twice
    use RationalExpression::*;
    let constraints = a.constraints.into_iter().map(|constraint|
        constraint.map(&mut |expression| {
            match expression {
                X => X.pow(2),
                other => other,
            }
        })
    ).collect();

    Component { trace, constraints }
}


/// Fold a component a number of times, padding if necessary.
pub fn fold_many(a: Component, folds: usize) -> Component {
    let mut result = a;
    for _ in 0..folds {
        if result.trace.num_columns() % 2 == 1 {
            let rows = result.trace.num_rows();
            result = compose_horizontal(result, Component::empty(rows, 1))
        }
        result = fold(result)
    }
    result
}

/// Horizontally compose two components of potentially unequal length
pub fn compose_folded(a: Component, b: Component) -> Component {
    use std::cmp::Ordering::*;
    let a_len = a.trace.num_rows();
    let b_len = b.trace.num_rows();
    match a_len.cmp(&b_len) {
        Less => {
            let folds = usize::try_from((b_len / a_len).trailing_zeros()).unwrap();
            compose_folded(fold_many(a, folds), b)
        }
        Equal => compose_horizontal(a, b),
        Greater => {
            let folds = usize::try_from((a_len / b_len).trailing_zeros()).unwrap();
            compose_horizontal(a, fold_many(b, folds))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
}
