use crate::RationalExpression;
use crate::TraceTable;
use crate::primefield::FieldElement;
use std::convert::TryFrom;
use crate::Constraints;
use crate::constraint_check::check_constraints;

// TODO: Introduce prover/verifier distinction

// OPT: Don't reallocate trace table so many times, instead use a view
// that is passed top-down for writing.

#[derive(Clone)]
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

    pub fn check(&self) -> bool {
        let constraints = Constraints::from_expressions(
            (self.trace.num_rows(), self.trace.num_columns()),
            Vec::new(),
            self.constraints.clone()).unwrap();
        check_constraints(&constraints, &self.trace).is_ok()
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

#[cfg(feature = "test")]
use quickcheck::{Gen, Arbitrary};

#[cfg(feature = "test")]
impl Component {
    /// Creates an example constraint system of given size
    pub fn example(rows: usize, cols: usize, start: &FieldElement) -> Self {
        use RationalExpression::*;

        // Construct a sequence using the quadratic recurrence relation:
        //     x[0]   = start
        //     x[1]   = start.pow(3)
        //     x[i+2] = x[i] * x[i + 1] + start.pow(5)
        let mut x0 = start.clone();
        let mut x1 = start.pow(3);
        let offset = start.pow(5);
        let mut next = || {
            let result = x0.clone();
            let x2 = &x0 * &x1 + &offset;
            x0 = x1.clone();
            x1 = x2;
            result
        };

        // Fill in the trace table with the sequence
        // the sequence is written left-to-right, then top-to-bottom.
        let mut trace = TraceTable::new(rows, cols);
        for i in 0..(rows*cols) {
            trace[(i / cols, i % cols)] = next();
        }

        // Construct the constraint system for the sequence.
        let omega = Constant(FieldElement::root(rows).unwrap());
        let mut constraints = Vec::new();
        // x[0] = start
        if cols >= 1 {
            constraints.push(
                (Trace(0, 0) - start.into()) / (X - omega.pow(0)),
            );
        }
        // For each column we need to add a constraint
        for i in 0..cols {
            // Find the previous two cells in the table
            let (x0, x1) = match (i, cols) {
                (0, 1) => (Trace(0, -2), Trace(0, -1)),
                (0, _) => (Trace(cols - 2, -1), Trace(cols - 1, -1)),
                (1, _) => (Trace(cols - 1, -1), Trace(0, 0)),
                (_, _) => (Trace(i - 2, 0), Trace(i - 1, 0)),
            };
            // Exempt the first two cells
            let exceptions = match (i, cols) {
                (0, 1) => (X - omega.pow(0)) * (X - omega.pow(1)),
                (0, _) | (1, _) => (X - omega.pow(0)),
                (_, _) => 1.into(),
            };
            // x[i+2] = x[i] * x[i + 1] + offset
            constraints.push(
                (Trace(i, 0) - x0 * x1 - (&offset).into())
                * exceptions / (X.pow(rows) - 1.into())
            )
        }

        Component { trace, constraints }
    }
}

#[cfg(feature = "test")]
fn arbitrary_dimensions<G: Gen>(g: &mut G) -> (usize, usize) {
    // rows = 0 1 2 4 .. 1024
    let rows = (1 << (usize::arbitrary(g) % 12)) >> 1;
    // cols = 0 1 2 3 .. 10
    let cols = usize::arbitrary(g) %  11;
    (rows, cols)
}

#[cfg(feature = "test")]
impl Arbitrary for Component {
    /// Creates an arbitrary constraint system up to 1024x10
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let (rows, cols) = arbitrary_dimensions(g);
        let mut seed = FieldElement::arbitrary(g);
        while seed == FieldElement::ZERO || seed == FieldElement::ONE {
            seed += FieldElement::arbitrary(g);
        }
        Component::example(rows, cols, &seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::*;

    #[quickcheck]
    fn empty_check(rows: usize, cols: usize) -> bool {
        let rows = 1 << (rows % 11);
        let cols = cols % 10;
        Component::empty(rows, cols).check()
    }

    #[quickcheck]
    fn arbitrary_check(component: Component) -> bool {
        component.check()
    }

    // For transformations and combinations we would ideally
    // like to check that the new Component proofs the same claims
    // as the input ones. Unfortunately, this is difficult to verify.
    // Instead, we verify:
    //
    // * The new Component is internally consistent.
    // * Labeled values remain and have the same values.
    // * The new Component's trace has at least as many values as the
    //   input traces (although you could imagine an inlining transform
    //   that violates this.)
    // * The new Component has at least as many constraints as the inputs
    //   combined. Except for vertical composition, where the inputs have
    //   identical constraints and only one copy results. (We could also
    //   imagine an optimization pass that combines constraints if possible
    //   and removes redundant ones.)

}

